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
use glib::IsA;
use glib::translate::{from_glib, from_glib_full, FromGlibPtrContainer, ToGlib, ToGlibPtr};

use ffi;

pub trait BinExtManual {
    fn add_many<E: IsA<Element>>(&self, elements: &[&E]) -> Result<(), glib::BoolError>;
    fn remove_many<E: IsA<Element>>(&self, elements: &[&E]) -> Result<(), glib::BoolError>;

    fn iterate_all_by_interface(&self, iface: glib::types::Type) -> ::Iterator<Element>;
    fn iterate_elements(&self) -> ::Iterator<Element>;
    fn iterate_recurse(&self) -> ::Iterator<Element>;
    fn iterate_sinks(&self) -> ::Iterator<Element>;
    fn iterate_sorted(&self) -> ::Iterator<Element>;
    fn iterate_sources(&self) -> ::Iterator<Element>;
    fn get_children(&self) -> Vec<Element>;
}

impl<O: IsA<Bin>> BinExtManual for O {
    fn add_many<E: IsA<Element>>(&self, elements: &[&E]) -> Result<(), glib::BoolError> {
        for e in elements {
            unsafe {
                let ret: bool =
                    from_glib(ffi::gst_bin_add(self.to_glib_none().0, e.to_glib_none().0));
                if !ret {
                    return Err(glib::BoolError("Failed to add elements"));
                }
            }
        }

        Ok(())
    }

    fn remove_many<E: IsA<Element>>(&self, elements: &[&E]) -> Result<(), glib::BoolError> {
        for e in elements {
            unsafe {
                let ret: bool = from_glib(ffi::gst_bin_remove(
                    self.to_glib_none().0,
                    e.to_glib_none().0,
                ));
                if !ret {
                    return Err(glib::BoolError("Failed to add elements"));
                }
            }
        }

        Ok(())
    }

    fn iterate_all_by_interface(&self, iface: glib::types::Type) -> ::Iterator<Element> {
        unsafe {
            from_glib_full(ffi::gst_bin_iterate_all_by_interface(
                self.to_glib_none().0,
                iface.to_glib(),
            ))
        }
    }

    fn iterate_elements(&self) -> ::Iterator<Element> {
        unsafe { from_glib_full(ffi::gst_bin_iterate_elements(self.to_glib_none().0)) }
    }

    fn iterate_recurse(&self) -> ::Iterator<Element> {
        unsafe { from_glib_full(ffi::gst_bin_iterate_recurse(self.to_glib_none().0)) }
    }

    fn iterate_sinks(&self) -> ::Iterator<Element> {
        unsafe { from_glib_full(ffi::gst_bin_iterate_sinks(self.to_glib_none().0)) }
    }

    fn iterate_sorted(&self) -> ::Iterator<Element> {
        unsafe { from_glib_full(ffi::gst_bin_iterate_sorted(self.to_glib_none().0)) }
    }

    fn iterate_sources(&self) -> ::Iterator<Element> {
        unsafe { from_glib_full(ffi::gst_bin_iterate_sources(self.to_glib_none().0)) }
    }

    fn get_children(&self) -> Vec<Element> {
        unsafe {
            let stash = self.to_glib_none();
            let bin: &ffi::GstBin = &*stash.0;
            ::utils::MutexGuard::lock(&bin.element.object.lock);
            FromGlibPtrContainer::from_glib_none(bin.children)
        }
    }
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

        let mut child_names = bin.get_children()
            .iter()
            .map(|c| c.get_name())
            .collect::<Vec<String>>();
        child_names.sort();
        assert_eq!(
            child_names,
            vec![String::from("identity0"), String::from("identity1")]
        );
    }
}
