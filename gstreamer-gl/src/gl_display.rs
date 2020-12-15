// Take a look at the license at the top of the repository in the LICENSE file.

use std::ffi::CStr;

use once_cell::sync::Lazy;

pub static GL_DISPLAY_CONTEXT_TYPE: Lazy<&'static str> = Lazy::new(|| unsafe {
    CStr::from_ptr(ffi::GST_GL_DISPLAY_CONTEXT_TYPE)
        .to_str()
        .unwrap()
});
