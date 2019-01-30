// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use Caps;
use Plugin;
use TypeFindFactory;
use TypeFindProbability;

use glib;
use glib::translate::*;
use glib_ffi;
use std::marker::PhantomData;
use std::mem;
use std::ptr;
use std::slice;

#[repr(C)]
#[derive(Debug)]
pub struct TypeFind<'a>(ffi::GstTypeFind, PhantomData<&'a ()>);

pub trait TypeFindImpl {
    fn peek(&mut self, offset: i64, size: u32) -> Option<&[u8]>;
    fn suggest(&mut self, probability: TypeFindProbability, caps: &Caps);
    fn get_length(&self) -> Option<u64> {
        None
    }
}

impl<'a> TypeFind<'a> {
    pub fn register<
        'b,
        'c,
        'd,
        P: Into<Option<&'b Plugin>>,
        R: Into<Option<&'c str>>,
        S: Into<Option<&'d Caps>>,
        F,
    >(
        plugin: P,
        name: &str,
        rank: u32,
        extensions: R,
        possible_caps: S,
        func: F,
    ) -> Result<(), glib::error::BoolError>
    where
        F: Fn(&mut TypeFind) + Send + Sync + 'static,
    {
        let plugin = plugin.into();
        let extensions = extensions.into();
        let possible_caps = possible_caps.into();
        unsafe {
            let func: Box<F> = Box::new(func);
            let func = Box::into_raw(func);

            let res = ffi::gst_type_find_register(
                plugin.to_glib_none().0,
                name.to_glib_none().0,
                rank,
                Some(type_find_trampoline::<F>),
                extensions.to_glib_none().0,
                possible_caps.to_glib_none().0,
                func as *mut _,
                Some(type_find_closure_drop::<F>),
            );

            glib_result_from_gboolean!(res, "Failed to register typefind factory")
        }
    }

    pub fn peek(&mut self, offset: i64, size: u32) -> Option<&[u8]> {
        unsafe {
            let data = ffi::gst_type_find_peek(&mut self.0, offset, size);
            if data.is_null() {
                None
            } else {
                Some(slice::from_raw_parts(data, size as usize))
            }
        }
    }

    pub fn suggest(&mut self, probability: TypeFindProbability, caps: &Caps) {
        unsafe {
            ffi::gst_type_find_suggest(
                &mut self.0,
                probability.to_glib() as u32,
                caps.to_glib_none().0,
            );
        }
    }
}

unsafe impl<'a> Send for TypeFind<'a> {}

impl TypeFindFactory {
    pub fn call_function(&self, find: &mut TypeFindImpl) {
        unsafe {
            let find_ptr = &find as *const &mut TypeFindImpl as glib_ffi::gpointer;
            let mut find = ffi::GstTypeFind {
                peek: Some(type_find_peek),
                suggest: Some(type_find_suggest),
                data: find_ptr,
                get_length: Some(type_find_get_length),
                _gst_reserved: [ptr::null_mut(); 4],
            };

            ffi::gst_type_find_factory_call_function(self.to_glib_none().0, &mut find)
        }
    }
}

unsafe extern "C" fn type_find_trampoline<F: Fn(&mut TypeFind) + Send + Sync + 'static>(
    find: *mut ffi::GstTypeFind,
    user_data: glib_ffi::gpointer,
) {
    #[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
    let func: &F = mem::transmute(user_data);
    func(&mut *(find as *mut TypeFind));
}

unsafe extern "C" fn type_find_closure_drop<F: Fn(&mut TypeFind) + Send + Sync + 'static>(
    data: glib_ffi::gpointer,
) {
    Box::<F>::from_raw(data as *mut _);
}

unsafe extern "C" fn type_find_peek(data: glib_ffi::gpointer, offset: i64, size: u32) -> *const u8 {
    let find: &mut &mut TypeFindImpl = &mut *(data as *mut &mut TypeFindImpl);
    match find.peek(offset, size) {
        None => ptr::null(),
        Some(data) => data.as_ptr(),
    }
}

unsafe extern "C" fn type_find_suggest(
    data: glib_ffi::gpointer,
    probability: u32,
    caps: *mut ffi::GstCaps,
) {
    let find: &mut &mut TypeFindImpl = &mut *(data as *mut &mut TypeFindImpl);
    find.suggest(from_glib(probability as i32), &from_glib_borrow(caps));
}

unsafe extern "C" fn type_find_get_length(data: glib_ffi::gpointer) -> u64 {
    use std::u64;

    let find: &mut &mut TypeFindImpl = &mut *(data as *mut &mut TypeFindImpl);
    find.get_length().unwrap_or(u64::MAX)
}

#[derive(Debug)]
pub struct SliceTypeFind<T: AsRef<[u8]>> {
    pub probability: Option<TypeFindProbability>,
    pub caps: Option<Caps>,
    data: T,
}

impl<T: AsRef<[u8]>> SliceTypeFind<T> {
    pub fn new(data: T) -> SliceTypeFind<T> {
        SliceTypeFind {
            probability: None,
            caps: None,
            data,
        }
    }

    pub fn run(&mut self) {
        let factories = TypeFindFactory::get_list();

        for factory in factories {
            factory.call_function(self);
            if let Some(prob) = self.probability {
                if prob >= TypeFindProbability::Maximum {
                    break;
                }
            }
        }
    }

    pub fn type_find(data: T) -> (TypeFindProbability, Option<Caps>) {
        let mut t = SliceTypeFind {
            probability: None,
            caps: None,
            data,
        };

        t.run();

        (t.probability.unwrap_or(TypeFindProbability::None), t.caps)
    }
}

impl<T: AsRef<[u8]>> TypeFindImpl for SliceTypeFind<T> {
    fn peek(&mut self, offset: i64, size: u32) -> Option<&[u8]> {
        let data = self.data.as_ref();
        let len = data.len();

        let offset = if offset >= 0 {
            offset as usize
        } else {
            if len < offset.abs() as usize {
                return None;
            }

            len - (offset.abs() as usize)
        };

        let size = size as usize;
        if offset + size <= len {
            Some(&data[offset..(offset + size)])
        } else {
            None
        }
    }

    fn suggest(&mut self, probability: TypeFindProbability, caps: &Caps) {
        match self.probability {
            None => {
                self.probability = Some(probability);
                self.caps = Some(caps.clone());
            }
            Some(old_probability) if old_probability < probability => {
                self.probability = Some(probability);
                self.caps = Some(caps.clone());
            }
            _ => (),
        }
    }
    fn get_length(&self) -> Option<u64> {
        Some(self.data.as_ref().len() as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typefind_call_function() {
        ::init().unwrap();

        let xml_factory = TypeFindFactory::get_list()
            .iter()
            .cloned()
            .find(|f| {
                f.get_caps()
                    .map(|c| {
                        c.get_structure(0)
                            .map(|s| s.get_name() == "application/xml")
                            .unwrap_or(false)
                    })
                    .unwrap_or(false)
            })
            .unwrap();

        let data = b"<?xml version=\"1.0\"?><test>test</test>";
        let data = &data[..];
        let mut typefind = SliceTypeFind::new(&data);
        xml_factory.call_function(&mut typefind);

        assert_eq!(
            typefind.caps,
            Some(Caps::new_simple("application/xml", &[]))
        );
        assert_eq!(typefind.probability, Some(TypeFindProbability::Minimum));
    }

    #[test]
    fn test_typefind_register() {
        ::init().unwrap();

        TypeFind::register(
            None,
            "test_typefind",
            ::Rank::Primary.to_glib() as u32,
            None,
            &Caps::new_simple("test/test", &[]),
            |typefind| {
                let mut found = false;
                if let Some(data) = typefind.peek(0, 8) {
                    if data == b"abcdefgh" {
                        found = true;
                    }
                }

                if found {
                    typefind.suggest(
                        TypeFindProbability::Likely,
                        &Caps::new_simple("test/test", &[]),
                    );
                }
            },
        )
        .unwrap();

        let data = b"abcdefgh";
        let data = &data[..];
        let (probability, caps) = SliceTypeFind::type_find(&data);

        assert_eq!(caps, Some(Caps::new_simple("test/test", &[])));
        assert_eq!(probability, TypeFindProbability::Likely);
    }
}
