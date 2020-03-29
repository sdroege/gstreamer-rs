// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::object::IsA;
use glib::translate::*;
use gst;
use gst_base_sys;
use std::mem;
use std::ptr;
use BaseTransform;

pub trait BaseTransformExtManual: 'static {
    fn get_allocator(&self) -> (Option<gst::Allocator>, gst::AllocationParams);

    fn get_segment(&self) -> gst::Segment;
}

impl<O: IsA<BaseTransform>> BaseTransformExtManual for O {
    fn get_allocator(&self) -> (Option<gst::Allocator>, gst::AllocationParams) {
        unsafe {
            let mut allocator = ptr::null_mut();
            let mut params = mem::zeroed();
            gst_base_sys::gst_base_transform_get_allocator(
                self.as_ref().to_glib_none().0,
                &mut allocator,
                &mut params,
            );
            (from_glib_full(allocator), params.into())
        }
    }

    fn get_segment(&self) -> gst::Segment {
        unsafe {
            let trans: &gst_base_sys::GstBaseTransform = &*(self.as_ptr() as *const _);
            let _guard = ::utils::MutexGuard::lock(&trans.element.object.lock);
            from_glib_none(&trans.segment as *const _)
        }
    }
}
