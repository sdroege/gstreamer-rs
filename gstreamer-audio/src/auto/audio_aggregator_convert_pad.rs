// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::AudioAggregatorPad;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GstAudioAggregatorConvertPad")]
    pub struct AudioAggregatorConvertPad(Object<ffi::GstAudioAggregatorConvertPad, ffi::GstAudioAggregatorConvertPadClass>) @extends AudioAggregatorPad, gst_base::AggregatorPad, gst::Object;

    match fn {
        type_ => || ffi::gst_audio_aggregator_convert_pad_get_type(),
    }
}

impl AudioAggregatorConvertPad {
    pub const NONE: Option<&'static AudioAggregatorConvertPad> = None;
}

unsafe impl Send for AudioAggregatorConvertPad {}
unsafe impl Sync for AudioAggregatorConvertPad {}

pub trait AudioAggregatorConvertPadExt: 'static {
    //#[doc(alias = "converter-config")]
    //fn converter_config(&self) -> /*Ignored*/Option<gst::Structure>;

    //#[doc(alias = "converter-config")]
    //fn set_converter_config(&self, converter_config: /*Ignored*/Option<&gst::Structure>);

    #[doc(alias = "converter-config")]
    fn connect_converter_config_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<AudioAggregatorConvertPad>> AudioAggregatorConvertPadExt for O {
    //fn converter_config(&self) -> /*Ignored*/Option<gst::Structure> {
    //    glib::ObjectExt::property(self.as_ref(), "converter-config")
    //}

    //fn set_converter_config(&self, converter_config: /*Ignored*/Option<&gst::Structure>) {
    //    glib::ObjectExt::set_property(self.as_ref(),"converter-config", &converter_config)
    //}

    fn connect_converter_config_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_converter_config_trampoline<
            P: IsA<AudioAggregatorConvertPad>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAudioAggregatorConvertPad,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AudioAggregatorConvertPad::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::converter-config\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_converter_config_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
