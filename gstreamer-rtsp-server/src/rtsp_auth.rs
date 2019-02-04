use ffi;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;

use std::boxed::Box as Box_;
use std::mem::transmute;

use RTSPAuth;
use RTSPToken;

pub trait RTSPAuthExtManual: 'static {
    fn set_default_token<'a, P: Into<Option<&'a mut RTSPToken>>>(&self, token: P);

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
    fn set_default_token<'a, P: Into<Option<&'a mut RTSPToken>>>(&self, token: P) {
        let mut token = token.into();
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
                Some(transmute(accept_certificate_trampoline::<Self, F> as usize)),
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
    connection: *mut gio_ffi::GTlsConnection,
    peer_cert: *mut gio_ffi::GTlsCertificate,
    errors: gio_ffi::GTlsCertificateFlags,
    f: glib_ffi::gpointer,
) -> glib_ffi::gboolean
where
    P: IsA<RTSPAuth>,
{
    let f: &F = transmute(f);
    match f(
        &RTSPAuth::from_glib_borrow(this).unsafe_cast(),
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
    .to_glib()
}
