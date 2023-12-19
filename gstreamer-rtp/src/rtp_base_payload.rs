use std::ptr;

use glib::{prelude::*, translate::*};

use crate::RTPBasePayload;

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::RTPBasePayload>> Sealed for T {}
}

pub trait RTPBasePayloadExtManual: sealed::Sealed + IsA<RTPBasePayload> + 'static {
    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_rtp_base_payload_set_outcaps_structure")]
    #[doc(alias = "gst_rtp_base_payload_set_outcaps")]
    fn set_outcaps(&self, s: Option<&gst::StructureRef>) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_rtp_base_payload_set_outcaps_structure(
                    self.as_ref().to_glib_none().0,
                    s.as_ref()
                        .map(|s| s.as_ptr() as *mut _)
                        .unwrap_or(ptr::null_mut()),
                ),
                "Failed to negotiate by setting outcaps structure"
            )
        }
    }

    #[cfg(feature = "v1_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
    fn extensions(&self) -> Vec<crate::RTPHeaderExtension> {
        let extensions = self.as_ref().property::<gst::Array>("extensions");

        extensions
            .iter()
            .map(|v| v.get::<crate::RTPHeaderExtension>().unwrap())
            .collect()
    }

    #[cfg(feature = "v1_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_24")))]
    #[doc(alias = "extensions")]
    fn connect_extensions_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        unsafe extern "C" fn notify_extensions_trampoline<
            P: IsA<RTPBasePayload>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTPBasePayload,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTPBasePayload::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            glib::signal::connect_raw(
                self.as_ptr() as *mut _,
                b"notify::extensions\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_extensions_trampoline::<Self, F> as *const (),
                )),
                Box::into_raw(f),
            )
        }
    }

    fn sink_pad(&self) -> &gst::Pad {
        unsafe {
            let elt = &*(self.as_ptr() as *const ffi::GstRTPBasePayload);
            &*(&elt.sinkpad as *const *mut gst::ffi::GstPad as *const gst::Pad)
        }
    }

    fn src_pad(&self) -> &gst::Pad {
        unsafe {
            let elt = &*(self.as_ptr() as *const ffi::GstRTPBasePayload);
            &*(&elt.srcpad as *const *mut gst::ffi::GstPad as *const gst::Pad)
        }
    }
}

impl<O: IsA<RTPBasePayload>> RTPBasePayloadExtManual for O {}
