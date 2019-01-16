// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use PtpClock;

use glib::object::Cast;
use glib::translate::*;
use gst;

impl PtpClock {
    pub fn new<'a, P: Into<Option<&'a str>>>(name: P, domain: u32) -> PtpClock {
        assert_initialized_main_thread!();
        let name = name.into();
        let name = name.to_glib_none();
        let (major, minor, _, _) = gst::version();
        if (major, minor) > (1, 12) {
            unsafe {
                gst::Clock::from_glib_full(ffi::gst_ptp_clock_new(name.0, domain)).unsafe_cast()
            }
        } else {
            // Workaround for bad floating reference handling in 1.12. This issue was fixed for 1.13
            unsafe {
                gst::Clock::from_glib_none(ffi::gst_ptp_clock_new(name.0, domain)).unsafe_cast()
            }
        }
    }
}
