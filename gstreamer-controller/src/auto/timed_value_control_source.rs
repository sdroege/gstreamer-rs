// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::ControlPoint;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::wrapper! {
    pub struct TimedValueControlSource(Object<ffi::GstTimedValueControlSource, ffi::GstTimedValueControlSourceClass>) @extends gst::ControlSource, gst::Object;

    match fn {
        type_ => || ffi::gst_timed_value_control_source_get_type(),
    }
}

unsafe impl Send for TimedValueControlSource {}
unsafe impl Sync for TimedValueControlSource {}

pub const NONE_TIMED_VALUE_CONTROL_SOURCE: Option<&TimedValueControlSource> = None;

pub trait TimedValueControlSourceExt: 'static {
    //#[doc(alias = "gst_timed_value_control_source_find_control_point_iter")]
    //fn find_control_point_iter(&self, timestamp: gst::ClockTime) -> /*Ignored*/Option<glib::SequenceIter>;

    //#[doc(alias = "gst_timed_value_control_source_get_all")]
    //fn all(&self) -> /*Ignored*/Vec<gst::TimedValue>;

    #[doc(alias = "gst_timed_value_control_source_get_count")]
    fn count(&self) -> i32;

    #[doc(alias = "gst_timed_value_control_source_set")]
    fn set(&self, timestamp: gst::ClockTime, value: f64) -> bool;

    //#[doc(alias = "gst_timed_value_control_source_set_from_list")]
    //fn set_from_list(&self, timedvalues: /*Ignored*/&[&gst::TimedValue]) -> bool;

    #[doc(alias = "gst_timed_value_control_source_unset")]
    fn unset(&self, timestamp: gst::ClockTime) -> bool;

    #[doc(alias = "gst_timed_value_control_source_unset_all")]
    fn unset_all(&self);

    fn connect_value_added<F: Fn(&Self, &ControlPoint) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_value_changed<F: Fn(&Self, &ControlPoint) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_value_removed<F: Fn(&Self, &ControlPoint) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<TimedValueControlSource>> TimedValueControlSourceExt for O {
    //fn find_control_point_iter(&self, timestamp: gst::ClockTime) -> /*Ignored*/Option<glib::SequenceIter> {
    //    unsafe { TODO: call ffi:gst_timed_value_control_source_find_control_point_iter() }
    //}

    //fn all(&self) -> /*Ignored*/Vec<gst::TimedValue> {
    //    unsafe { TODO: call ffi:gst_timed_value_control_source_get_all() }
    //}

    fn count(&self) -> i32 {
        unsafe { ffi::gst_timed_value_control_source_get_count(self.as_ref().to_glib_none().0) }
    }

    fn set(&self, timestamp: gst::ClockTime, value: f64) -> bool {
        unsafe {
            from_glib(ffi::gst_timed_value_control_source_set(
                self.as_ref().to_glib_none().0,
                timestamp.into_glib(),
                value,
            ))
        }
    }

    //fn set_from_list(&self, timedvalues: /*Ignored*/&[&gst::TimedValue]) -> bool {
    //    unsafe { TODO: call ffi:gst_timed_value_control_source_set_from_list() }
    //}

    fn unset(&self, timestamp: gst::ClockTime) -> bool {
        unsafe {
            from_glib(ffi::gst_timed_value_control_source_unset(
                self.as_ref().to_glib_none().0,
                timestamp.into_glib(),
            ))
        }
    }

    fn unset_all(&self) {
        unsafe {
            ffi::gst_timed_value_control_source_unset_all(self.as_ref().to_glib_none().0);
        }
    }

    fn connect_value_added<F: Fn(&Self, &ControlPoint) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn value_added_trampoline<
            P,
            F: Fn(&P, &ControlPoint) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstTimedValueControlSource,
            timed_value: *mut ffi::GstControlPoint,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<TimedValueControlSource>,
        {
            let f: &F = &*(f as *const F);
            f(
                &TimedValueControlSource::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(timed_value),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"value-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    value_added_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_value_changed<F: Fn(&Self, &ControlPoint) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn value_changed_trampoline<
            P,
            F: Fn(&P, &ControlPoint) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstTimedValueControlSource,
            timed_value: *mut ffi::GstControlPoint,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<TimedValueControlSource>,
        {
            let f: &F = &*(f as *const F);
            f(
                &TimedValueControlSource::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(timed_value),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"value-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    value_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_value_removed<F: Fn(&Self, &ControlPoint) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn value_removed_trampoline<
            P,
            F: Fn(&P, &ControlPoint) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstTimedValueControlSource,
            timed_value: *mut ffi::GstControlPoint,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<TimedValueControlSource>,
        {
            let f: &F = &*(f as *const F);
            f(
                &TimedValueControlSource::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(timed_value),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"value-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    value_removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
