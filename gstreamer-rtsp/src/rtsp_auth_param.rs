use glib::translate::*;

impl RTSPAuthParam {
    pub fn name(&self) -> Option<&str> {
        let ptr: *mut GstRTSPAuthParam = self.to_glib_none().0;
        unsafe {
            if (*ptr).name.is_null() {
                None
            } else {
                std::ffi::CStr::from_ptr((*ptr).name).to_str().ok()
            }
        }
    }

    pub fn value(&self) -> Option<&str> {
        let ptr: *mut GstRTSPAuthParam = self.to_glib_none().0;
        unsafe {
            if (*ptr).value.is_null() {
                None
            } else {
                std::ffi::CStr::from_ptr((*ptr).value).to_str().ok()
            }
        }
    }
}
