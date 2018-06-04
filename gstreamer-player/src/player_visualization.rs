// Copyright (C) 2018 Philippe Normand <philn@igalia.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::translate::*;
use std::ffi::CStr;

use PlayerVisualization;

impl PlayerVisualization {
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
