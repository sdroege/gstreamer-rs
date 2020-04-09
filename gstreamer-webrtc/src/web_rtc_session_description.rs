// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::translate::*;
use gst_sdp;
use gst_web_rtc_sys;
use std::mem;
use WebRTCSDPType;
use WebRTCSessionDescription;

impl WebRTCSessionDescription {
    pub fn new(type_: WebRTCSDPType, sdp: gst_sdp::SDPMessage) -> WebRTCSessionDescription {
        assert_initialized_main_thread!();
        unsafe {
            let mut sdp = mem::ManuallyDrop::new(sdp);
            let desc = from_glib_full(gst_web_rtc_sys::gst_webrtc_session_description_new(
                type_.to_glib(),
                sdp.to_glib_none_mut().0,
            ));
            desc
        }
    }

    pub fn get_type(&self) -> ::WebRTCSDPType {
        unsafe { from_glib((*self.to_glib_none().0).type_) }
    }

    pub fn get_sdp(&self) -> gst_sdp::SDPMessage {
        unsafe { from_glib_none((*self.to_glib_none().0).sdp) }
    }
}
