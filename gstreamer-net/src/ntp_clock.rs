// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::NtpClock;

use glib::object::Cast;
use glib::translate::*;

impl NtpClock {
    pub fn new(
        name: Option<&str>,
        remote_address: &str,
        remote_port: i32,
        base_time: gst::ClockTime,
    ) -> NtpClock {
        assert_initialized_main_thread!();
        let name = name.to_glib_none();
        let (major, minor, _, _) = gst::version();
        if (major, minor) > (1, 12) {
            unsafe {
                gst::Clock::from_glib_full(ffi::gst_ntp_clock_new(
                    name.0,
                    remote_address.to_glib_none().0,
                    remote_port,
                    base_time.to_glib(),
                ))
                .unsafe_cast()
            }
        } else {
            // Workaround for bad floating reference handling in 1.12. This issue was fixed for 1.13
            unsafe {
                gst::Clock::from_glib_none(ffi::gst_ntp_clock_new(
                    name.0,
                    remote_address.to_glib_none().0,
                    remote_port,
                    base_time.to_glib(),
                ))
                .unsafe_cast()
            }
        }
    }
}
