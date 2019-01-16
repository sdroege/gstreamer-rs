use ffi;
use glib::object::IsA;
use glib::translate::*;
use RTSPAuth;
use RTSPToken;

pub trait RTSPAuthExtManual: 'static {
    fn set_default_token<'a, P: Into<Option<&'a mut RTSPToken>>>(&self, token: P);
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
}
