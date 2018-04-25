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
use BaseTransform;

pub trait BaseTransformExtManual {
    fn get_segment(&self) -> gst::Segment;
}

impl<O: IsA<BaseTransform>> BaseTransformExtManual for O {
    fn get_segment(&self) -> gst::Segment {
        unsafe {
            let stash = self.to_glib_none();
            let trans: &ffi::GstBaseTransform = &*stash.0;
            ::utils::MutexGuard::lock(&trans.element.object.lock);
            from_glib_none(&trans.segment as *const _)
        }
    }
}
