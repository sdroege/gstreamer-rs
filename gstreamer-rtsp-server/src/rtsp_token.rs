// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use glib::value::ToSendValue;

use std::fmt;

gst::mini_object_wrapper!(RTSPToken, RTSPTokenRef, ffi::GstRTSPToken, || {
    ffi::gst_rtsp_token_get_type()
});

impl RTSPToken {
    #[doc(alias = "gst_rtsp_token_new_empty")]
    pub fn new_empty() -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_rtsp_token_new_empty()) }
    }

    pub fn new(values: &[(&str, &(dyn ToSendValue + Sync))]) -> Self {
        assert_initialized_main_thread!();
        let mut token = RTSPToken::new_empty();

        {
            let token = token.get_mut().unwrap();
            let structure = token.structure_mut();

            for &(f, v) in values {
                structure.set_value(f, v.to_send_value());
            }
        }

        token
    }
}

impl RTSPTokenRef {
    #[doc(alias = "get_string")]
    #[doc(alias = "gst_rtsp_token_get_string")]
    pub fn string(&self, field: &str) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_rtsp_token_get_string(
                self.as_mut_ptr(),
                field.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "get_structure")]
    #[doc(alias = "gst_rtsp_token_get_structure")]
    pub fn structure(&self) -> Option<gst::Structure> {
        unsafe { from_glib_none(ffi::gst_rtsp_token_get_structure(self.as_mut_ptr())) }
    }

    #[doc(alias = "gst_rtsp_token_is_allowed")]
    pub fn is_allowed(&self, field: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_token_is_allowed(
                self.as_mut_ptr(),
                field.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "get_mut_structure")]
    pub fn structure_mut(&mut self) -> &mut gst::StructureRef {
        unsafe {
            let structure = ffi::gst_rtsp_token_writable_structure(self.as_mut_ptr());
            gst::StructureRef::from_glib_borrow_mut(structure)
        }
    }
}

impl fmt::Debug for RTSPToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        RTSPTokenRef::fmt(self, f)
    }
}

impl fmt::Debug for RTSPTokenRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("RTSPToken")
            .field("structure", &self.structure())
            .finish()
    }
}
