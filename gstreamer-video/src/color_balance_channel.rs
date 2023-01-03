// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, translate::*};

use crate::ColorBalanceChannel;

impl ColorBalanceChannel {
    pub fn label(&self) -> glib::GString {
        unsafe { from_glib_none((*self.as_ptr()).label) }
    }

    pub fn min_value(&self) -> i32 {
        unsafe { (*self.as_ptr()).min_value }
    }

    pub fn max_value(&self) -> i32 {
        unsafe { (*self.as_ptr()).max_value }
    }
}
