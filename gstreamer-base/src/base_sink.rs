// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib::object::{IsA, IsClassFor};
use glib::translate::*;
use gst;
use std::mem;
use std::ops;
use BaseSink;

pub trait BaseSinkExtManual: 'static {
    fn get_segment(&self) -> gst::Segment;

    fn wait(
        &self,
        time: gst::ClockTime,
    ) -> (Result<gst::FlowSuccess, gst::FlowError>, gst::ClockTimeDiff);

    fn wait_preroll(&self) -> Result<gst::FlowSuccess, gst::FlowError>;
}

impl<O: IsA<BaseSink>> BaseSinkExtManual for O {
    fn get_segment(&self) -> gst::Segment {
        unsafe {
            let sink: &ffi::GstBaseSink = &*(self.as_ptr() as *const _);
            ::utils::MutexGuard::lock(&sink.element.object.lock);
            from_glib_none(&sink.segment as *const _)
        }
    }

    fn wait(
        &self,
        time: gst::ClockTime,
    ) -> (Result<gst::FlowSuccess, gst::FlowError>, gst::ClockTimeDiff) {
        unsafe {
            let mut jitter = mem::uninitialized();
            let ret: gst::FlowReturn = from_glib(ffi::gst_base_sink_wait(
                self.as_ref().to_glib_none().0,
                time.to_glib(),
                &mut jitter,
            ));
            (ret.into_result(), jitter)
        }
    }

    fn wait_preroll(&self) -> Result<gst::FlowSuccess, gst::FlowError> {
        let ret: gst::FlowReturn = unsafe {
            from_glib(ffi::gst_base_sink_wait_preroll(
                self.as_ref().to_glib_none().0,
            ))
        };
        ret.into_result()
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
