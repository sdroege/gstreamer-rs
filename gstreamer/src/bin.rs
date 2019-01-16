// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use Bin;
use Element;

use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;

use ffi;

use std::boxed::Box as Box_;
use std::mem::transmute;
use std::path;

pub trait GstBinExtManual: 'static {
    fn add_many<E: IsA<Element>>(&self, elements: &[&E]) -> Result<(), glib::BoolError>;
    fn remove_many<E: IsA<Element>>(&self, elements: &[&E]) -> Result<(), glib::BoolError>;

    fn connect_do_latency<F: Fn(&Self) -> Result<(), glib::BoolError> + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn iterate_all_by_interface(&self, iface: glib::types::Type) -> ::Iterator<Element>;
    fn iterate_elements(&self) -> ::Iterator<Element>;
    fn iterate_recurse(&self) -> ::Iterator<Element>;
    fn iterate_sinks(&self) -> ::Iterator<Element>;
    fn iterate_sorted(&self) -> ::Iterator<Element>;
    fn iterate_sources(&self) -> ::Iterator<Element>;
    fn get_children(&self) -> Vec<Element>;

    fn debug_to_dot_data(&self, details: ::DebugGraphDetails) -> GString;
    fn debug_to_dot_file<Q: AsRef<path::Path>>(&self, details: ::DebugGraphDetails, file_name: Q);
    fn debug_to_dot_file_with_ts<Q: AsRef<path::Path>>(
        &self,
        details: ::DebugGraphDetails,
        file_name: Q,
    );
}

impl<O: IsA<Bin>> GstBinExtManual for O {
    fn add_many<E: IsA<Element>>(&self, elements: &[&E]) -> Result<(), glib::BoolError> {
        for e in elements {
            unsafe {
                let ret: bool = from_glib(ffi::gst_bin_add(
                    self.as_ref().to_glib_none().0,
                    e.as_ref().to_glib_none().0,
                ));
                if !ret {
                    return Err(glib_bool_error!("Failed to add elements"));
                }
            }
        }

        Ok(())
    }

    fn remove_many<E: IsA<Element>>(&self, elements: &[&E]) -> Result<(), glib::BoolError> {
        for e in elements {
            unsafe {
                let ret: bool = from_glib(ffi::gst_bin_remove(
                    self.as_ref().to_glib_none().0,
                    e.as_ref().to_glib_none().0,
                ));
                if !ret {
                    return Err(glib_bool_error!("Failed to remove elements"));
                }
            }
        }

        Ok(())
    }

    fn connect_do_latency<F: Fn(&Self) -> Result<(), glib::BoolError> + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) -> Result<(), glib::BoolError> + Send + Sync + 'static>> =
                Box_::new(Box_::new(f));
            connect_raw(
                self.as_ptr() as *mut _,
                b"do-latency\0".as_ptr() as *const _,
                transmute(do_latency_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn iterate_all_by_interface(&self, iface: glib::types::Type) -> ::Iterator<Element> {
        unsafe {
            from_glib_full(ffi::gst_bin_iterate_all_by_interface(
                self.as_ref().to_glib_none().0,
                iface.to_glib(),
            ))
        }
    }

    fn iterate_elements(&self) -> ::Iterator<Element> {
        unsafe {
            from_glib_full(ffi::gst_bin_iterate_elements(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn iterate_recurse(&self) -> ::Iterator<Element> {
        unsafe { from_glib_full(ffi::gst_bin_iterate_recurse(self.as_ref().to_glib_none().0)) }
    }

    fn iterate_sinks(&self) -> ::Iterator<Element> {
        unsafe { from_glib_full(ffi::gst_bin_iterate_sinks(self.as_ref().to_glib_none().0)) }
    }

    fn iterate_sorted(&self) -> ::Iterator<Element> {
        unsafe { from_glib_full(ffi::gst_bin_iterate_sorted(self.as_ref().to_glib_none().0)) }
    }

    fn iterate_sources(&self) -> ::Iterator<Element> {
        unsafe { from_glib_full(ffi::gst_bin_iterate_sources(self.as_ref().to_glib_none().0)) }
    }

    fn get_children(&self) -> Vec<Element> {
        unsafe {
            let bin: &ffi::GstBin = &*(self.as_ptr() as *const _);
            ::utils::MutexGuard::lock(&bin.element.object.lock);
            FromGlibPtrContainer::from_glib_none(bin.children)
        }
    }

    fn debug_to_dot_data(&self, details: ::DebugGraphDetails) -> GString {
        ::debug_bin_to_dot_data(self, details)
    }

    fn debug_to_dot_file<Q: AsRef<path::Path>>(&self, details: ::DebugGraphDetails, file_name: Q) {
        ::debug_bin_to_dot_file(self, details, file_name)
    }

    fn debug_to_dot_file_with_ts<Q: AsRef<path::Path>>(
        &self,
        details: ::DebugGraphDetails,
        file_name: Q,
    ) {
        ::debug_bin_to_dot_file_with_ts(self, details, file_name)
    }
}

unsafe extern "C" fn do_latency_trampoline<P>(
    this: *mut ffi::GstBin,
    f: glib_ffi::gpointer,
) -> glib_ffi::gboolean
where
    P: IsA<Bin>,
{
    let f: &&(Fn(&P) -> Result<(), glib::BoolError> + Send + Sync + 'static) = transmute(f);
    match f(&Bin::from_glib_borrow(this).unsafe_cast()) {
        Ok(()) => true,
        Err(err) => {
            gst_error!(::CAT_RUST, obj: &Bin::from_glib_borrow(this), "{}", err);
            false
        }
    }
    .to_glib()
}

#[cfg(test)]
mod tests {
    use super::*;
    use prelude::*;

    #[test]
    fn test_get_children() {
        ::init().unwrap();

        let bin = ::Bin::new(None);
        bin.add(&::ElementFactory::make("identity", "identity0").unwrap())
            .unwrap();
        bin.add(&::ElementFactory::make("identity", "identity1").unwrap())
            .unwrap();

        let mut child_names = bin
            .get_children()
            .iter()
            .map(|c| c.get_name())
            .collect::<Vec<GString>>();
        child_names.sort();
        assert_eq!(
            child_names,
            vec![String::from("identity0"), String::from("identity1")]
        );
    }
}
