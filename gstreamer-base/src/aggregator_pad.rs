// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::AggregatorPad;
use glib::object::IsA;
use glib::translate::*;

pub trait AggregatorPadExtManual: 'static {
    fn get_segment(&self) -> gst::Segment;
}

impl<O: IsA<AggregatorPad>> AggregatorPadExtManual for O {
    fn get_segment(&self) -> gst::Segment {
        unsafe {
            let ptr: &ffi::GstAggregatorPad = &*(self.as_ptr() as *const _);
            let _guard = crate::utils::MutexGuard::lock(&ptr.parent.object.lock);
            from_glib_none(&ptr.segment as *const gst::ffi::GstSegment)
        }
    }
}
