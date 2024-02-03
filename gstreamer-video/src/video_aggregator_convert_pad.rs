use std::mem::transmute;

use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};

use crate::auto::VideoAggregatorConvertPad;

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::VideoAggregatorConvertPad>> Sealed for T {}
}

pub trait VideoAggregatorConvertPadExtManual:
    sealed::Sealed + IsA<VideoAggregatorConvertPad> + 'static
{
    #[doc(alias = "converter-config")]
    fn converter_config(&self) -> Option<crate::VideoConverterConfig> {
        ObjectExt::property::<Option<gst::Structure>>(self.as_ref(), "converter-config")
            .map(|c| c.try_into().unwrap())
    }

    #[doc(alias = "converter-config")]
    fn set_converter_config(&self, converter_config: Option<&crate::VideoConverterConfig>) {
        ObjectExt::set_property(
            self.as_ref(),
            "converter-config",
            converter_config.map(|s| s.as_ref()),
        )
    }

    #[doc(alias = "converter-config")]
    fn connect_converter_config_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_converter_config_trampoline<
            P: IsA<VideoAggregatorConvertPad>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstVideoAggregatorConvertPad,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(VideoAggregatorConvertPad::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::converter-config\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_converter_config_trampoline::<Self, F> as *const (),
                )),
                Box::into_raw(f),
            )
        }
    }
}

impl<O: IsA<VideoAggregatorConvertPad>> VideoAggregatorConvertPadExtManual for O {}
