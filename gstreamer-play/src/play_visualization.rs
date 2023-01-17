// Take a look at the license at the top of the repository in the LICENSE file.

use std::ffi::CStr;

use crate::PlayVisualization;

impl PlayVisualization {
    pub fn name(&self) -> &str {
        unsafe { CStr::from_ptr((*self.as_ptr()).name).to_str().unwrap() }
    }

    pub fn description(&self) -> &str {
        unsafe {
            CStr::from_ptr((*self.as_ptr()).description)
                .to_str()
                .unwrap()
        }
    }
}
