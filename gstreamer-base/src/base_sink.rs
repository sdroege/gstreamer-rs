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
use BaseSink;

pub trait BaseSinkExtManual: 'static {
    fn get_segment(&self) -> gst::Segment;

    fn wait(
        &self,
        time: gst::ClockTime,
    ) -> (Result<gst::FlowSuccess, gst::FlowError>, gst::ClockTimeDiff);

    fn wait_preroll(&self) -> Result<gst::FlowSuccess, gst::FlowError>;
    fn wait_clock(
        &self,
        time: gst::ClockTime,
    ) -> (
        Result<gst::ClockSuccess, gst::ClockError>,
        gst::ClockTimeDiff,
    );
}

impl<O: IsA<BaseSink>> BaseSinkExtManual for O {
    fn get_segment(&self) -> gst::Segment {
        unsafe {
            let sink: &gst_base_sys::GstBaseSink = &*(self.as_ptr() as *const _);
            ::utils::MutexGuard::lock(&sink.element.object.lock);
            from_glib_none(&sink.segment as *const _)
        }
    }

    fn wait(
        &self,
        time: gst::ClockTime,
    ) -> (Result<gst::FlowSuccess, gst::FlowError>, gst::ClockTimeDiff) {
        unsafe {
            let mut jitter = 0;
            let ret: gst::FlowReturn = from_glib(gst_base_sys::gst_base_sink_wait(
                self.as_ref().to_glib_none().0,
                time.to_glib(),
                &mut jitter,
            ));
            (ret.into_result(), jitter)
        }
    }

    fn wait_preroll(&self) -> Result<gst::FlowSuccess, gst::FlowError> {
        let ret: gst::FlowReturn = unsafe {
            from_glib(gst_base_sys::gst_base_sink_wait_preroll(
                self.as_ref().to_glib_none().0,
            ))
        };
        ret.into_result()
    }

    fn wait_clock(
        &self,
        time: gst::ClockTime,
    ) -> (
        Result<gst::ClockSuccess, gst::ClockError>,
        gst::ClockTimeDiff,
    ) {
        unsafe {
            let mut jitter = 0;
            let ret: gst::ClockReturn = from_glib(gst_base_sys::gst_base_sink_wait_clock(
                self.as_ref().to_glib_none().0,
                time.to_glib(),
                &mut jitter,
            ));
            (ret.into_result(), jitter)
        }
    }
}
