// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use Bin;
use BinFlags;
use Element;
use LoggableError;

use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;

use gst_sys;

use std::boxed::Box as Box_;
use std::mem::transmute;
use std::path;

pub trait GstBinExtManual: 'static {
    fn add_many<E: IsA<Element>>(&self, elements: &[&E]) -> Result<(), glib::BoolError>;
    fn remove_many<E: IsA<Element>>(&self, elements: &[&E]) -> Result<(), glib::BoolError>;

    fn connect_do_latency<F: Fn(&Self) -> Result<(), LoggableError> + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    fn iterate_all_by_element_factory_name(&self, factory_name: &str) -> ::Iterator<Element>;
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

    fn set_bin_flags(&self, flags: BinFlags);

    fn unset_bin_flags(&self, flags: BinFlags);

    fn get_bin_flags(&self) -> BinFlags;
}

impl<O: IsA<Bin>> GstBinExtManual for O {
    fn add_many<E: IsA<Element>>(&self, elements: &[&E]) -> Result<(), glib::BoolError> {
        for e in elements {
            unsafe {
                glib_result_from_gboolean!(
                    gst_sys::gst_bin_add(
                        self.as_ref().to_glib_none().0,
                        e.as_ref().to_glib_none().0
                    ),
                    "Failed to add elements"
                )?;
            }
        }

        Ok(())
    }

    fn remove_many<E: IsA<Element>>(&self, elements: &[&E]) -> Result<(), glib::BoolError> {
        for e in elements {
            unsafe {
                glib_result_from_gboolean!(
                    gst_sys::gst_bin_remove(
                        self.as_ref().to_glib_none().0,
                        e.as_ref().to_glib_none().0,
                    ),
                    "Failed to remove elements"
                )?;
            }
        }

        Ok(())
    }

    fn connect_do_latency<F: Fn(&Self) -> Result<(), LoggableError> + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"do-latency\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    do_latency_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    fn iterate_all_by_element_factory_name(&self, factory_name: &str) -> ::Iterator<Element> {
        unsafe {
            from_glib_full(gst_sys::gst_bin_iterate_all_by_element_factory_name(
                self.as_ref().to_glib_none().0,
                factory_name.to_glib_none().0,
            ))
        }
    }

    fn iterate_all_by_interface(&self, iface: glib::types::Type) -> ::Iterator<Element> {
        unsafe {
            from_glib_full(gst_sys::gst_bin_iterate_all_by_interface(
                self.as_ref().to_glib_none().0,
                iface.to_glib(),
            ))
        }
    }

    fn iterate_elements(&self) -> ::Iterator<Element> {
        unsafe {
            from_glib_full(gst_sys::gst_bin_iterate_elements(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn iterate_recurse(&self) -> ::Iterator<Element> {
        unsafe {
            from_glib_full(gst_sys::gst_bin_iterate_recurse(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn iterate_sinks(&self) -> ::Iterator<Element> {
        unsafe {
            from_glib_full(gst_sys::gst_bin_iterate_sinks(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn iterate_sorted(&self) -> ::Iterator<Element> {
        unsafe {
            from_glib_full(gst_sys::gst_bin_iterate_sorted(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn iterate_sources(&self) -> ::Iterator<Element> {
        unsafe {
            from_glib_full(gst_sys::gst_bin_iterate_sources(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_children(&self) -> Vec<Element> {
        unsafe {
            let bin: &gst_sys::GstBin = &*(self.as_ptr() as *const _);
            let _guard = ::utils::MutexGuard::lock(&bin.element.object.lock);
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

    fn set_bin_flags(&self, flags: BinFlags) {
        unsafe {
            let ptr: *mut gst_sys::GstObject = self.as_ptr() as *mut _;
            let _guard = ::utils::MutexGuard::lock(&(*ptr).lock);
            (*ptr).flags |= flags.to_glib();
        }
    }

    fn unset_bin_flags(&self, flags: BinFlags) {
        unsafe {
            let ptr: *mut gst_sys::GstObject = self.as_ptr() as *mut _;
            let _guard = ::utils::MutexGuard::lock(&(*ptr).lock);
            (*ptr).flags &= !flags.to_glib();
        }
    }

    fn get_bin_flags(&self) -> BinFlags {
        unsafe {
            let ptr: *mut gst_sys::GstObject = self.as_ptr() as *mut _;
            let _guard = ::utils::MutexGuard::lock(&(*ptr).lock);
            from_glib((*ptr).flags)
        }
    }
}

unsafe extern "C" fn do_latency_trampoline<
    P,
    F: Fn(&P) -> Result<(), LoggableError> + Send + Sync + 'static,
>(
    this: *mut gst_sys::GstBin,
    f: glib_sys::gpointer,
) -> glib_sys::gboolean
where
    P: IsA<Bin>,
{
    let f: &F = &*(f as *const F);
    match f(&Bin::from_glib_borrow(this).unsafe_cast_ref()) {
        Ok(()) => true,
        Err(err) => {
            err.log_with_object(&*Bin::from_glib_borrow(this));
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
        bin.add(&::ElementFactory::make("identity", Some("identity0")).unwrap())
            .unwrap();
        bin.add(&::ElementFactory::make("identity", Some("identity1")).unwrap())
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
