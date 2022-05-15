// Take a look at the license at the top of the repository in the LICENSE file.

use crate::PlaySignalAdapter;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::ObjectType;
use std::boxed::Box as Box_;
use std::mem::transmute;

impl PlaySignalAdapter {
    #[doc(alias = "duration-changed")]
    pub fn connect_duration_changed<
        F: Fn(&PlaySignalAdapter, Option<gst::ClockTime>) + Send + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn duration_changed_trampoline<
            F: Fn(&PlaySignalAdapter, Option<gst::ClockTime>) + Send + 'static,
        >(
            this: *mut ffi::GstPlaySignalAdapter,
            object: u64,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), FromGlib::from_glib(object))
        }
        #[allow(clippy::cast_ptr_alignment)]
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"duration-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    duration_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "position-updated")]
    pub fn connect_position_updated<
        F: Fn(&PlaySignalAdapter, Option<gst::ClockTime>) + Send + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn position_updated_trampoline<
            F: Fn(&PlaySignalAdapter, Option<gst::ClockTime>) + Send + 'static,
        >(
            this: *mut ffi::GstPlaySignalAdapter,
            object: u64,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), FromGlib::from_glib(object))
        }
        #[allow(clippy::cast_ptr_alignment)]
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"position-updated\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    position_updated_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "seek-done")]
    pub fn connect_seek_done<F: Fn(&PlaySignalAdapter, gst::ClockTime) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn seek_done_trampoline<
            F: Fn(&PlaySignalAdapter, gst::ClockTime) + Send + 'static,
        >(
            this: *mut ffi::GstPlaySignalAdapter,
            object: u64,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                try_from_glib(object).expect("undefined seek position"),
            )
        }
        #[allow(clippy::cast_ptr_alignment)]
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"seek-done\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    seek_done_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
