// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib::translate::*;
use glib::{IsA, IsClassFor};
use gst;
use std::ops;
use BaseSink;

pub trait BaseSinkExtManual {
    fn get_segment(&self) -> gst::Segment;
}

impl<O: IsA<BaseSink>> BaseSinkExtManual for O {
    fn get_segment(&self) -> gst::Segment {
        unsafe {
            let stash = self.to_glib_none();
            let sink: &ffi::GstBaseSink = &*stash.0;
            ::utils::MutexGuard::lock(&sink.element.object.lock);
            from_glib_none(&sink.segment as *const _)
        }
    }
}

#[repr(C)]
pub struct BaseSinkClass(ffi::GstBaseSinkClass);

unsafe impl IsClassFor for BaseSinkClass {
    type Instance = BaseSink;
}

unsafe impl Send for BaseSinkClass {}
unsafe impl Sync for BaseSinkClass {}

impl ops::Deref for BaseSinkClass {
    type Target = gst::ElementClass;

    fn deref(&self) -> &Self::Target {
        self.upcast_ref()
    }
}

impl ops::DerefMut for BaseSinkClass {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.upcast_ref_mut()
    }
}
