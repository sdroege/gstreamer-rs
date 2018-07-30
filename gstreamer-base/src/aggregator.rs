// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib::translate::*;
use glib::IsA;
use gst;
use Aggregator;

pub trait AggregatorExtManual {
    fn finish_buffer(&self, buffer: gst::Buffer) -> gst::FlowReturn;
}

impl<O: IsA<Aggregator>> AggregatorExtManual for O {
    fn finish_buffer(&self, buffer: gst::Buffer) -> gst::FlowReturn {
        unsafe {
            from_glib(ffi::gst_aggregator_finish_buffer(
                self.to_glib_none().0,
                buffer.into_ptr(),
            ))
        }
    }
}
