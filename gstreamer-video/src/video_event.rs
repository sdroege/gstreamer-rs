// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;

use gst;
use gst::MiniObject;
use glib::translate::{from_glib, from_glib_full, ToGlib};

pub fn is_force_key_unit_event(event: &gst::EventRef) -> bool {
    unsafe { from_glib(ffi::gst_video_event_is_force_key_unit(event.as_mut_ptr())) }
}

pub fn new_downstream_force_key_unit_event(
    timestamp: gst::ClockTime,
    stream_time: gst::ClockTime,
    running_time: gst::ClockTime,
    all_headers: bool,
    count: u32,
) -> gst::Event {
    unsafe {
        from_glib_full(ffi::gst_video_event_new_downstream_force_key_unit(
            timestamp.to_glib(),
            stream_time.to_glib(),
            running_time.to_glib(),
            all_headers.to_glib(),
            count,
        ))
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DownstreamForceKeyUnitEvent {
    pub timestamp: gst::ClockTime,
    pub stream_time: gst::ClockTime,
    pub running_time: gst::ClockTime,
    pub all_headers: bool,
    pub count: u32,
}

pub fn parse_downstream_force_key_unit_event(
    event: &gst::EventRef,
) -> Option<DownstreamForceKeyUnitEvent> {
    unsafe {
        let mut timestamp = 0;
        let mut stream_time = 0;
        let mut running_time = 0;
        let mut all_headers = 0;
        let mut count = 0;

        let res: bool = from_glib(ffi::gst_video_event_parse_downstream_force_key_unit(
            event.as_mut_ptr(),
            &mut timestamp,
            &mut stream_time,
            &mut running_time,
            &mut all_headers,
            &mut count,
        ));
        if res {
            Some(DownstreamForceKeyUnitEvent {
                timestamp: from_glib(timestamp),
                stream_time: from_glib(stream_time),
                running_time: from_glib(running_time),
                all_headers: from_glib(all_headers),
                count: count,
            })
        } else {
            None
        }
    }
}

pub fn new_upstream_force_key_unit_event(
    running_time: gst::ClockTime,
    all_headers: bool,
    count: u32,
) -> gst::Event {
    unsafe {
        from_glib_full(ffi::gst_video_event_new_upstream_force_key_unit(
            running_time.to_glib(),
            all_headers.to_glib(),
            count,
        ))
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UpstreamForceKeyUnitEvent {
    pub running_time: gst::ClockTime,
    pub all_headers: bool,
    pub count: u32,
}

pub fn parse_upstream_force_key_unit_event(
    event: &gst::EventRef,
) -> Option<UpstreamForceKeyUnitEvent> {
    unsafe {
        let mut running_time = 0;
        let mut all_headers = 0;
        let mut count = 0;

        let res: bool = from_glib(ffi::gst_video_event_parse_upstream_force_key_unit(
            event.as_mut_ptr(),
            &mut running_time,
            &mut all_headers,
            &mut count,
        ));
        if res {
            Some(UpstreamForceKeyUnitEvent {
                running_time: from_glib(running_time),
                all_headers: from_glib(all_headers),
                count: count,
            })
        } else {
            None
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ForceKeyUnitEvent {
    Downstream(DownstreamForceKeyUnitEvent),
    Upstream(UpstreamForceKeyUnitEvent),
}

pub fn parse_force_key_unit_event(event: &gst::EventRef) -> Option<ForceKeyUnitEvent> {
    if event.is_upstream() {
        parse_upstream_force_key_unit_event(event).map(ForceKeyUnitEvent::Upstream)
    } else {
        parse_downstream_force_key_unit_event(event).map(ForceKeyUnitEvent::Downstream)
    }
}

pub fn new_still_frame_event(in_still: bool) -> gst::Event {
    unsafe { from_glib_full(ffi::gst_video_event_new_still_frame(in_still.to_glib())) }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StillFrameEvent {
    pub in_still: bool,
}

pub fn parse_still_frame_event(event: &gst::EventRef) -> Option<StillFrameEvent> {
    unsafe {
        let mut in_still = 0;

        let res: bool = from_glib(ffi::gst_video_event_parse_still_frame(
            event.as_mut_ptr(),
            &mut in_still,
        ));
        if res {
            Some(StillFrameEvent {
                in_still: from_glib(in_still),
            })
        } else {
            None
        }
    }
}
