// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;

use std::boxed::Box as Box_;
use std::mem::transmute;

use crate::RTSPAuth;
use crate::RTSPToken;

pub trait RTSPAuthExtManual: 'static {
    #[doc(alias = "gst_rtsp_auth_set_default_token")]
    fn set_default_token(&self, token: Option<&mut RTSPToken>);

    fn connect_accept_certificate<
        F: Fn(
                &Self,
                &gio::TlsConnection,
                &gio::TlsCertificate,
                gio::TlsCertificateFlags,
            ) -> Result<(), gst::LoggableError>
            + Send
            + Sync
            + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<RTSPAuth>> RTSPAuthExtManual for O {
    fn set_default_token(&self, mut token: Option<&mut RTSPToken>) {
        unsafe {
            ffi::gst_rtsp_auth_set_default_token(
                self.as_ref().to_glib_none().0,
                token.to_glib_none_mut().0,
            );
        }
    }

    fn connect_accept_certificate<
        F: Fn(
                &Self,
                &gio::TlsConnection,
                &gio::TlsCertificate,
                gio::TlsCertificateFlags,
            ) -> Result<(), gst::LoggableError>
            + Send
            + Sync
            + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"accept-certificate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    accept_certificate_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe extern "C" fn accept_certificate_trampoline<
    P,
    F: Fn(
            &P,
            &gio::TlsConnection,
            &gio::TlsCertificate,
            gio::TlsCertificateFlags,
        ) -> Result<(), gst::LoggableError>
        + Send
        + Sync
        + 'static,
>(
    this: *mut ffi::GstRTSPAuth,
    connection: *mut gio::ffi::GTlsConnection,
    peer_cert: *mut gio::ffi::GTlsCertificate,
    errors: gio::ffi::GTlsCertificateFlags,
    f: glib::ffi::gpointer,
) -> glib::ffi::gboolean
where
    P: IsA<RTSPAuth>,
{
    let f: &F = &*(f as *const F);
    match f(
        &RTSPAuth::from_glib_borrow(this).unsafe_cast_ref(),
        &from_glib_borrow(connection),
        &from_glib_borrow(peer_cert),
        from_glib(errors),
    ) {
        Ok(()) => true,
        Err(err) => {
            err.log();
            false
        }
    }
    .into_glib()
}
