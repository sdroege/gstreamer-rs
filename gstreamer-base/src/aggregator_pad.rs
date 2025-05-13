// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, translate::*};
use gst::prelude::*;

use crate::{ffi, AggregatorPad};

pub trait AggregatorPadExtManual: IsA<AggregatorPad> + 'static {
    #[doc(alias = "get_segment")]
    fn segment(&self) -> gst::Segment {
        unsafe {
            let ptr: &ffi::GstAggregatorPad = &*(self.as_ptr() as *const _);
            let _guard = self.as_ref().object_lock();
            from_glib_none(&ptr.segment as *const gst::ffi::GstSegment)
        }
    }

    #[cfg(feature = "v1_28")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
    #[doc(alias = "current-level-time")]
    fn current_level_time(&self) -> Option<gst::ClockTime> {
        ObjectExt::property(self.as_ref(), "current-level-time")
    }

    #[cfg(feature = "v1_28")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
    #[doc(alias = "current-level-time")]
    fn connect_current_level_time_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        unsafe extern "C" fn notify_current_level_time_trampoline<
            P: IsA<AggregatorPad>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAggregatorPad,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AggregatorPad::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            glib::signal::connect_raw(
                self.as_ptr() as *mut _,
                c"notify::current-level-time".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_current_level_time_trampoline::<Self, F> as *const (),
                )),
                Box::into_raw(f),
            )
        }
    }
}

impl<O: IsA<AggregatorPad>> AggregatorPadExtManual for O {}
