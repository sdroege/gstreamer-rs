// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use Segment;
use SegmentFlags;
use Format;

use glib::translate::{from_glib, ToGlib, ToGlibPtr, ToGlibPtrMut};

impl Segment {
    pub fn set_flags(&mut self, flags: SegmentFlags) {
        unsafe {
            (*self.to_glib_none_mut().0).flags = flags.to_glib();
        }
    }

    pub fn get_flags(&self) -> SegmentFlags {
        unsafe {
            from_glib((*self.to_glib_none().0).flags)
        }
    }

    pub fn set_rate(&mut self, rate: f64) {
        unsafe {
            (*self.to_glib_none_mut().0).rate = rate;
        }
    }

    pub fn get_rate(&self) -> f64 {
        unsafe {
            (*self.to_glib_none().0).rate
        }
    }

    pub fn set_applied_rate(&mut self, applied_rate: f64) {
        unsafe {
            (*self.to_glib_none_mut().0).applied_rate = applied_rate;
        }
    }

    pub fn get_applied_rate(&self) -> f64 {
        unsafe {
            (*self.to_glib_none().0).applied_rate
        }
    }

    pub fn set_format(&mut self, format: Format) {
        unsafe {
            (*self.to_glib_none_mut().0).format = format.to_glib();
        }
    }

    pub fn get_format(&self) -> Format {
        unsafe {
            from_glib((*self.to_glib_none().0).format)
        }
    }

    pub fn set_base(&mut self, base: u64) {
        unsafe {
            (*self.to_glib_none_mut().0).base = base;
        }
    }

    pub fn get_base(&self) -> u64 {
        unsafe {
            (*self.to_glib_none().0).base
        }
    }

    pub fn set_offset(&mut self, offset: u64) {
        unsafe {
            (*self.to_glib_none_mut().0).offset = offset;
        }
    }

    pub fn get_offset(&self) -> u64 {
        unsafe {
            (*self.to_glib_none().0).offset
        }
    }

    pub fn set_start(&mut self, start: u64) {
        unsafe {
            (*self.to_glib_none_mut().0).start = start;
        }
    }

    pub fn get_start(&self) -> u64 {
        unsafe {
            (*self.to_glib_none().0).start
        }
    }

    pub fn set_stop(&mut self, stop: u64) {
        unsafe {
            (*self.to_glib_none_mut().0).stop = stop;
        }
    }

    pub fn get_stop(&self) -> u64 {
        unsafe {
            (*self.to_glib_none().0).stop
        }
    }

    pub fn set_time(&mut self, time: u64) {
        unsafe {
            (*self.to_glib_none_mut().0).time = time;
        }
    }

    pub fn get_time(&self) -> u64 {
        unsafe {
            (*self.to_glib_none().0).time
        }
    }

    pub fn set_position(&mut self, position: u64) {
        unsafe {
            (*self.to_glib_none_mut().0).position = position;
        }
    }

    pub fn get_position(&self) -> u64 {
        unsafe {
            (*self.to_glib_none().0).position
        }
    }

    pub fn set_duration(&mut self, duration: u64) {
        unsafe {
            (*self.to_glib_none_mut().0).duration = duration;
        }
    }

    pub fn get_duration(&self) -> u64 {
        unsafe {
            (*self.to_glib_none().0).duration
        }
    }
}
