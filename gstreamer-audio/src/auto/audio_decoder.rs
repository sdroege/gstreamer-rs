// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::AudioInfo;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;

glib::wrapper! {
    pub struct AudioDecoder(Object<ffi::GstAudioDecoder, ffi::GstAudioDecoderClass>) @extends gst::Element, gst::Object;

    match fn {
        type_ => || ffi::gst_audio_decoder_get_type(),
    }
}

unsafe impl Send for AudioDecoder {}
unsafe impl Sync for AudioDecoder {}

pub const NONE_AUDIO_DECODER: Option<&AudioDecoder> = None;

pub trait AudioDecoderExt: 'static {
    #[doc(alias = "gst_audio_decoder_allocate_output_buffer")]
    fn allocate_output_buffer(&self, size: usize) -> Result<gst::Buffer, glib::BoolError>;

    #[doc(alias = "gst_audio_decoder_get_audio_info")]
    fn audio_info(&self) -> Option<AudioInfo>;

    #[doc(alias = "gst_audio_decoder_get_delay")]
    fn delay(&self) -> i32;

    #[doc(alias = "gst_audio_decoder_get_drainable")]
    fn is_drainable(&self) -> bool;

    #[doc(alias = "gst_audio_decoder_get_estimate_rate")]
    fn estimate_rate(&self) -> i32;

    #[doc(alias = "gst_audio_decoder_get_latency")]
    fn latency(&self) -> (gst::ClockTime, gst::ClockTime);

    #[doc(alias = "gst_audio_decoder_get_max_errors")]
    fn max_errors(&self) -> i32;

    #[doc(alias = "gst_audio_decoder_get_min_latency")]
    fn min_latency(&self) -> gst::ClockTime;

    #[doc(alias = "gst_audio_decoder_get_needs_format")]
    fn needs_format(&self) -> bool;

    #[doc(alias = "gst_audio_decoder_get_parse_state")]
    fn parse_state(&self) -> (bool, bool);

    #[doc(alias = "gst_audio_decoder_get_plc")]
    fn is_plc(&self) -> bool;

    #[doc(alias = "gst_audio_decoder_get_plc_aware")]
    fn plc_aware(&self) -> i32;

    #[doc(alias = "gst_audio_decoder_get_tolerance")]
    fn tolerance(&self) -> gst::ClockTime;

    #[doc(alias = "gst_audio_decoder_merge_tags")]
    fn merge_tags(&self, tags: Option<&gst::TagList>, mode: gst::TagMergeMode);

    #[doc(alias = "gst_audio_decoder_proxy_getcaps")]
    fn proxy_getcaps(&self, caps: Option<&gst::Caps>, filter: Option<&gst::Caps>) -> gst::Caps;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "gst_audio_decoder_set_allocation_caps")]
    fn set_allocation_caps(&self, allocation_caps: Option<&gst::Caps>);

    #[doc(alias = "gst_audio_decoder_set_drainable")]
    fn set_drainable(&self, enabled: bool);

    #[doc(alias = "gst_audio_decoder_set_estimate_rate")]
    fn set_estimate_rate(&self, enabled: bool);

    #[doc(alias = "gst_audio_decoder_set_latency")]
    fn set_latency(&self, min: gst::ClockTime, max: gst::ClockTime);

    #[doc(alias = "gst_audio_decoder_set_max_errors")]
    fn set_max_errors(&self, num: i32);

    #[doc(alias = "gst_audio_decoder_set_min_latency")]
    fn set_min_latency(&self, num: gst::ClockTime);

    #[doc(alias = "gst_audio_decoder_set_needs_format")]
    fn set_needs_format(&self, enabled: bool);

    #[doc(alias = "gst_audio_decoder_set_plc")]
    fn set_plc(&self, enabled: bool);

    #[doc(alias = "gst_audio_decoder_set_plc_aware")]
    fn set_plc_aware(&self, plc: bool);

    #[doc(alias = "gst_audio_decoder_set_tolerance")]
    fn set_tolerance(&self, tolerance: gst::ClockTime);

    #[doc(alias = "gst_audio_decoder_set_use_default_pad_acceptcaps")]
    fn set_use_default_pad_acceptcaps(&self, use_: bool);

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn connect_property_max_errors_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_min_latency_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_plc_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_tolerance_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<AudioDecoder>> AudioDecoderExt for O {
    fn allocate_output_buffer(&self, size: usize) -> Result<gst::Buffer, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_audio_decoder_allocate_output_buffer(
                self.as_ref().to_glib_none().0,
                size,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to allocate output buffer"))
        }
    }

    fn audio_info(&self) -> Option<AudioInfo> {
        unsafe {
            from_glib_full(ffi::gst_audio_decoder_get_audio_info(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn delay(&self) -> i32 {
        unsafe { ffi::gst_audio_decoder_get_delay(self.as_ref().to_glib_none().0) }
    }

    fn is_drainable(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_audio_decoder_get_drainable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn estimate_rate(&self) -> i32 {
        unsafe { ffi::gst_audio_decoder_get_estimate_rate(self.as_ref().to_glib_none().0) }
    }

    fn latency(&self) -> (gst::ClockTime, gst::ClockTime) {
        unsafe {
            let mut min = mem::MaybeUninit::uninit();
            let mut max = mem::MaybeUninit::uninit();
            ffi::gst_audio_decoder_get_latency(
                self.as_ref().to_glib_none().0,
                min.as_mut_ptr(),
                max.as_mut_ptr(),
            );
            let min = min.assume_init();
            let max = max.assume_init();
            (from_glib(min), from_glib(max))
        }
    }

    fn max_errors(&self) -> i32 {
        unsafe { ffi::gst_audio_decoder_get_max_errors(self.as_ref().to_glib_none().0) }
    }

    fn min_latency(&self) -> gst::ClockTime {
        unsafe {
            from_glib(ffi::gst_audio_decoder_get_min_latency(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn needs_format(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_audio_decoder_get_needs_format(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn parse_state(&self) -> (bool, bool) {
        unsafe {
            let mut sync = mem::MaybeUninit::uninit();
            let mut eos = mem::MaybeUninit::uninit();
            ffi::gst_audio_decoder_get_parse_state(
                self.as_ref().to_glib_none().0,
                sync.as_mut_ptr(),
                eos.as_mut_ptr(),
            );
            let sync = sync.assume_init();
            let eos = eos.assume_init();
            (from_glib(sync), from_glib(eos))
        }
    }

    fn is_plc(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_audio_decoder_get_plc(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn plc_aware(&self) -> i32 {
        unsafe { ffi::gst_audio_decoder_get_plc_aware(self.as_ref().to_glib_none().0) }
    }

    fn tolerance(&self) -> gst::ClockTime {
        unsafe {
            from_glib(ffi::gst_audio_decoder_get_tolerance(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn merge_tags(&self, tags: Option<&gst::TagList>, mode: gst::TagMergeMode) {
        unsafe {
            ffi::gst_audio_decoder_merge_tags(
                self.as_ref().to_glib_none().0,
                tags.to_glib_none().0,
                mode.into_glib(),
            );
        }
    }

    fn proxy_getcaps(&self, caps: Option<&gst::Caps>, filter: Option<&gst::Caps>) -> gst::Caps {
        unsafe {
            from_glib_full(ffi::gst_audio_decoder_proxy_getcaps(
                self.as_ref().to_glib_none().0,
                caps.to_glib_none().0,
                filter.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    fn set_allocation_caps(&self, allocation_caps: Option<&gst::Caps>) {
        unsafe {
            ffi::gst_audio_decoder_set_allocation_caps(
                self.as_ref().to_glib_none().0,
                allocation_caps.to_glib_none().0,
            );
        }
    }

    fn set_drainable(&self, enabled: bool) {
        unsafe {
            ffi::gst_audio_decoder_set_drainable(
                self.as_ref().to_glib_none().0,
                enabled.into_glib(),
            );
        }
    }

    fn set_estimate_rate(&self, enabled: bool) {
        unsafe {
            ffi::gst_audio_decoder_set_estimate_rate(
                self.as_ref().to_glib_none().0,
                enabled.into_glib(),
            );
        }
    }

    fn set_latency(&self, min: gst::ClockTime, max: gst::ClockTime) {
        unsafe {
            ffi::gst_audio_decoder_set_latency(
                self.as_ref().to_glib_none().0,
                min.into_glib(),
                max.into_glib(),
            );
        }
    }

    fn set_max_errors(&self, num: i32) {
        unsafe {
            ffi::gst_audio_decoder_set_max_errors(self.as_ref().to_glib_none().0, num);
        }
    }

    fn set_min_latency(&self, num: gst::ClockTime) {
        unsafe {
            ffi::gst_audio_decoder_set_min_latency(self.as_ref().to_glib_none().0, num.into_glib());
        }
    }

    fn set_needs_format(&self, enabled: bool) {
        unsafe {
            ffi::gst_audio_decoder_set_needs_format(
                self.as_ref().to_glib_none().0,
                enabled.into_glib(),
            );
        }
    }

    fn set_plc(&self, enabled: bool) {
        unsafe {
            ffi::gst_audio_decoder_set_plc(self.as_ref().to_glib_none().0, enabled.into_glib());
        }
    }

    fn set_plc_aware(&self, plc: bool) {
        unsafe {
            ffi::gst_audio_decoder_set_plc_aware(self.as_ref().to_glib_none().0, plc.into_glib());
        }
    }

    fn set_tolerance(&self, tolerance: gst::ClockTime) {
        unsafe {
            ffi::gst_audio_decoder_set_tolerance(
                self.as_ref().to_glib_none().0,
                tolerance.into_glib(),
            );
        }
    }

    fn set_use_default_pad_acceptcaps(&self, use_: bool) {
        unsafe {
            ffi::gst_audio_decoder_set_use_default_pad_acceptcaps(
                self.as_ref().to_glib_none().0,
                use_.into_glib(),
            );
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn connect_property_max_errors_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_errors_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut ffi::GstAudioDecoder,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<AudioDecoder>,
        {
            let f: &F = &*(f as *const F);
            f(&AudioDecoder::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-errors\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_errors_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_min_latency_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_min_latency_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut ffi::GstAudioDecoder,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<AudioDecoder>,
        {
            let f: &F = &*(f as *const F);
            f(&AudioDecoder::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::min-latency\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_min_latency_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_plc_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_plc_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut ffi::GstAudioDecoder,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<AudioDecoder>,
        {
            let f: &F = &*(f as *const F);
            f(&AudioDecoder::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::plc\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_plc_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_tolerance_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_tolerance_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut ffi::GstAudioDecoder,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<AudioDecoder>,
        {
            let f: &F = &*(f as *const F);
            f(&AudioDecoder::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::tolerance\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tolerance_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
