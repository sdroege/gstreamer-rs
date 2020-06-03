// Copyright (C) 2020 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::translate::*;
use gst_web_rtc_sys;
use WebRTCDataChannel;

use std::mem;

impl WebRTCDataChannel {
    pub fn on_error(&self, error: glib::Error) {
        let error = mem::ManuallyDrop::new(error);
        unsafe {
            gst_web_rtc_sys::gst_webrtc_data_channel_on_error(
                self.to_glib_none().0,
                mut_override(error.to_glib_none().0),
            );
        }
    }
}
