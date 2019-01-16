// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib::object::IsA;
use glib::translate::*;
use gst;
use BaseSrc;

pub trait BaseSrcExtManual: 'static {
    fn get_segment(&self) -> gst::Segment;

    fn start_complete(&self, ret: Result<gst::FlowSuccess, gst::FlowError>);

    fn start_wait(&self) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn wait_playing(&self) -> Result<gst::FlowSuccess, gst::FlowError>;
}

impl<O: IsA<BaseSrc>> BaseSrcExtManual for O {
    fn get_segment(&self) -> gst::Segment {
        unsafe {
            let src: &ffi::GstBaseSrc = &*(self.as_ptr() as *const _);
            ::utils::MutexGuard::lock(&src.element.object.lock);
            from_glib_none(&src.segment as *const _)
        }
    }

    fn start_complete(&self, ret: Result<gst::FlowSuccess, gst::FlowError>) {
        let ret: gst::FlowReturn = ret.into();
        unsafe {
            ffi::gst_base_src_start_complete(self.as_ref().to_glib_none().0, ret.to_glib());
        }
    }

    fn start_wait(&self) -> Result<gst::FlowSuccess, gst::FlowError> {
        let ret: gst::FlowReturn =
            unsafe { from_glib(ffi::gst_base_src_start_wait(self.as_ref().to_glib_none().0)) };
        ret.into_result()
    }

    fn wait_playing(&self) -> Result<gst::FlowSuccess, gst::FlowError> {
        let ret: gst::FlowReturn = unsafe {
            from_glib(ffi::gst_base_src_wait_playing(
                self.as_ref().to_glib_none().0,
            ))
        };
        ret.into_result()
    }
}
