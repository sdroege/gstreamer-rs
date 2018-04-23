use ffi;
use glib;
use glib::StaticType;
use glib::translate::*;
use glib::value::ToSendValue;
use gst;
use gst_ffi;

use gst::miniobject::{GstRc, MiniObject};

pub trait GstRcRTSPTokenExt<T: MiniObject> {
    fn new_empty() -> Self;
    fn new(values: &[(&str, &ToSendValue)]) -> Self;
}

pub type RTSPToken = GstRc<RTSPTokenRef>;
pub struct RTSPTokenRef(ffi::GstRTSPToken);

unsafe impl MiniObject for RTSPTokenRef {
    type GstType = ffi::GstRTSPToken;
}

impl GstRcRTSPTokenExt<RTSPTokenRef> for GstRc<RTSPTokenRef> {
    fn new_empty() -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_rtsp_token_new_empty()) }
    }

    fn new(values: &[(&str, &ToSendValue)]) -> Self {
        let token = RTSPToken::new_empty();

        {
            let structure = token.writable_structure().unwrap();

            for &(f, v) in values {
                structure.set_value(f, v.to_send_value());
            }
        }

        token
    }
}

impl RTSPTokenRef {
    pub fn get_string(&self, field: &str) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_rtsp_token_get_string(
                self.as_mut_ptr(),
                field.to_glib_none().0,
            ))
        }
    }

    pub fn get_structure(&self) -> Option<gst::Structure> {
        unsafe { from_glib_none(ffi::gst_rtsp_token_get_structure(self.as_mut_ptr())) }
    }

    pub fn is_allowed(&self, field: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_token_is_allowed(
                self.as_mut_ptr(),
                field.to_glib_none().0,
            ))
        }
    }

    pub fn writable_structure(&self) -> Option<&mut gst::StructureRef> {
        unsafe {
            let structure = ffi::gst_rtsp_token_writable_structure(self.as_mut_ptr());
            if structure.is_null() {
                None
            } else {
                Some(gst::StructureRef::from_glib_borrow_mut(structure))
            }
        }
    }
}

impl ToOwned for RTSPTokenRef {
    type Owned = GstRc<RTSPTokenRef>;

    fn to_owned(&self) -> GstRc<RTSPTokenRef> {
        unsafe {
            from_glib_full(gst_ffi::gst_mini_object_copy(self.as_ptr() as *const _) as *mut _)
        }
    }
}

impl StaticType for RTSPTokenRef {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gst_rtsp_token_get_type()) }
    }
}
