use glib::{prelude::*, translate::*};

use crate::RTPBaseDepayload;

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::RTPBaseDepayload>> Sealed for T {}
}

pub trait RTPBaseDepayloadExtManual: sealed::Sealed + IsA<RTPBaseDepayload> + 'static {
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
            P: IsA<RTPBaseDepayload>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTPBaseDepayload,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTPBaseDepayload::from_glib_borrow(this).unsafe_cast_ref())
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
            let elt = &*(self.as_ptr() as *const ffi::GstRTPBaseDepayload);
            &*(&elt.sinkpad as *const *mut gst::ffi::GstPad as *const gst::Pad)
        }
    }

    fn src_pad(&self) -> &gst::Pad {
        unsafe {
            let elt = &*(self.as_ptr() as *const ffi::GstRTPBaseDepayload);
            &*(&elt.srcpad as *const *mut gst::ffi::GstPad as *const gst::Pad)
        }
    }
}

impl<O: IsA<RTPBaseDepayload>> RTPBaseDepayloadExtManual for O {}
