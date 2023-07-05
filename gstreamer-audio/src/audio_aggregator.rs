#[cfg(feature = "v1_18")]
use std::mem::transmute;

#[cfg(feature = "v1_18")]
use glib::object::Cast;
#[cfg(feature = "v1_18")]
use glib::signal::{connect_raw, SignalHandlerId};
use glib::{object::IsA, translate::*};
use gst::prelude::*;

use crate::auto::{AudioAggregator, AudioAggregatorPad};

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::AudioAggregator>> Sealed for T {}
}

pub trait AudioAggregatorExtManual: sealed::Sealed + IsA<AudioAggregator> + 'static {
    #[doc(alias = "gst_audio_aggregator_set_sink_caps")]
    fn set_sink_caps(&self, pad: &impl IsA<AudioAggregatorPad>, caps: &gst::CapsRef) {
        unsafe {
            ffi::gst_audio_aggregator_set_sink_caps(
                self.as_ref().to_glib_none().0,
                pad.as_ref().to_glib_none().0,
                caps.as_mut_ptr(),
            );
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "output-buffer-duration-fraction")]
    fn output_buffer_duration_fraction(&self) -> gst::Fraction {
        glib::ObjectExt::property(self.as_ref(), "output-buffer-duration-fraction")
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "output-buffer-duration-fraction")]
    fn set_output_buffer_duration_fraction(&self, output_buffer_duration_fraction: gst::Fraction) {
        glib::ObjectExt::set_property(
            self.as_ref(),
            "output-buffer-duration-fraction",
            output_buffer_duration_fraction,
        )
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "output-buffer-duration-fraction")]
    fn connect_output_buffer_duration_fraction_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_output_buffer_duration_fraction_trampoline<
            P: IsA<AudioAggregator>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAudioAggregator,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AudioAggregator::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::output-buffer-duration-fraction\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_output_buffer_duration_fraction_trampoline::<Self, F> as *const (),
                )),
                Box::into_raw(f),
            )
        }
    }

    fn current_caps(&self) -> Option<gst::Caps> {
        unsafe {
            let ptr = self.as_ptr() as *mut ffi::GstAudioAggregator;
            let _guard = self.as_ref().object_lock();
            from_glib_none((*ptr).current_caps)
        }
    }

    fn current_audio_info(&self) -> Option<crate::AudioInfo> {
        self.current_caps()
            .and_then(|caps| crate::AudioInfo::from_caps(&caps).ok())
    }
}

impl<O: IsA<AudioAggregator>> AudioAggregatorExtManual for O {}
