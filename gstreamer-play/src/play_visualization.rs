// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use std::ffi::CStr;

use crate::PlayVisualization;

impl PlayVisualization {
    pub fn name(&self) -> &str {
        unsafe {
            CStr::from_ptr((*self.to_glib_none().0).name)
                .to_str()
                .unwrap()
        }
    }

    pub fn description(&self) -> &str {
        unsafe {
            CStr::from_ptr((*self.to_glib_none().0).description)
                .to_str()
                .unwrap()
        }
    }
}
