// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::translate::*;
use gst;
use gst_player_sys;
use std::mem;
use PlayerVideoInfo;

impl PlayerVideoInfo {
    pub fn get_framerate(&self) -> gst::Fraction {
        unsafe {
            let mut fps_n = mem::MaybeUninit::uninit();
            let mut fps_d = mem::MaybeUninit::uninit();
            gst_player_sys::gst_player_video_info_get_framerate(
                self.to_glib_none().0,
                fps_n.as_mut_ptr(),
                fps_d.as_mut_ptr(),
            );
            (fps_n.assume_init() as i32, fps_d.as_mut_ptr() as i32).into()
        }
    }

    pub fn get_pixel_aspect_ratio(&self) -> gst::Fraction {
        unsafe {
            let mut par_n = mem::MaybeUninit::uninit();
            let mut par_d = mem::MaybeUninit::uninit();
            gst_player_sys::gst_player_video_info_get_pixel_aspect_ratio(
                self.to_glib_none().0,
                par_n.as_mut_ptr(),
                par_d.as_mut_ptr(),
            );
            (par_n.assume_init() as i32, par_d.assume_init() as i32).into()
        }
    }
}
