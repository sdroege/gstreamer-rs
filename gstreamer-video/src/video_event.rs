// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use gst_ffi;

use gst;
use gst::MiniObject;
use glib::translate::{from_glib, from_glib_full, ToGlib};
use glib::ToSendValue;

pub fn is_force_key_unit_event(event: &gst::EventRef) -> bool {
    unsafe { from_glib(ffi::gst_video_event_is_force_key_unit(event.as_mut_ptr())) }
}

// FIXME: Copy from gstreamer/src/event.rs
macro_rules! event_builder_generic_impl {
    ($new_fn:expr) => {
        pub fn seqnum(self, seqnum: gst::Seqnum) -> Self {
            Self {
                seqnum: Some(seqnum),
                .. self
            }
        }

        pub fn running_time_offset(self, running_time_offset: i64) -> Self {
            Self {
                running_time_offset: Some(running_time_offset),
                .. self
            }
        }

        pub fn other_fields(self, other_fields: &[(&'a str, &'a ToSendValue)]) -> Self {
            Self {
                other_fields: self.other_fields.iter().cloned()
                    .chain(other_fields.iter().cloned())
                    .collect(),
                .. self
            }
        }

        pub fn build(mut self) -> gst::Event {
            assert_initialized_main_thread!();
            unsafe {
                let event = $new_fn(&mut self);
                if let Some(seqnum) = self.seqnum {
                    gst_ffi::gst_event_set_seqnum(event, seqnum.to_glib());
                }

                if let Some(running_time_offset) = self.running_time_offset {
                    gst_ffi::gst_event_set_running_time_offset(event, running_time_offset);
                }

                {
                    let s = gst::StructureRef::from_glib_borrow_mut(
                        gst_ffi::gst_event_writable_structure(event)
                    );

                    for (k, v) in self.other_fields {
                        s.set_value(k, v.to_send_value());
                    }
                }

                from_glib_full(event)
            }
        }
    }
}

pub fn new_downstream_force_key_unit_event<'a>(
    timestamp: gst::ClockTime,
    stream_time: gst::ClockTime,
    running_time: gst::ClockTime,
    all_headers: bool,
    count: u32,
) -> DownstreamForceKeyUnitEventBuilder<'a> {
    DownstreamForceKeyUnitEventBuilder::new(
        timestamp,
        stream_time,
        running_time,
        all_headers,
        count,
    )
}

pub struct DownstreamForceKeyUnitEventBuilder<'a> {
    seqnum: Option<gst::Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    timestamp: gst::ClockTime,
    stream_time: gst::ClockTime,
    running_time: gst::ClockTime,
    all_headers: bool,
    count: u32,
}

impl<'a> DownstreamForceKeyUnitEventBuilder<'a> {
    fn new(
        timestamp: gst::ClockTime,
        stream_time: gst::ClockTime,
        running_time: gst::ClockTime,
        all_headers: bool,
        count: u32,
    ) -> Self {
        skip_assert_initialized!();
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
            timestamp: timestamp,
            stream_time: stream_time,
            running_time: running_time,
            all_headers: all_headers,
            count: count,
        }
    }

    event_builder_generic_impl!(|s: &mut Self| {
        ffi::gst_video_event_new_downstream_force_key_unit(
            s.timestamp.to_glib(),
            s.stream_time.to_glib(),
            s.running_time.to_glib(),
            s.all_headers.to_glib(),
            s.count,
        )
    });
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

pub fn new_upstream_force_key_unit_event<'a>(
    running_time: gst::ClockTime,
    all_headers: bool,
    count: u32,
) -> UpstreamForceKeyUnitEventBuilder<'a> {
    UpstreamForceKeyUnitEventBuilder::new(running_time, all_headers, count)
}

pub struct UpstreamForceKeyUnitEventBuilder<'a> {
    seqnum: Option<gst::Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    running_time: gst::ClockTime,
    all_headers: bool,
    count: u32,
}

impl<'a> UpstreamForceKeyUnitEventBuilder<'a> {
    fn new(running_time: gst::ClockTime, all_headers: bool, count: u32) -> Self {
        skip_assert_initialized!();
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
            running_time: running_time,
            all_headers: all_headers,
            count: count,
        }
    }

    event_builder_generic_impl!(|s: &mut Self| {
        ffi::gst_video_event_new_upstream_force_key_unit(
            s.running_time.to_glib(),
            s.all_headers.to_glib(),
            s.count,
        )
    });
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

pub fn new_still_frame_event<'a>(in_still: bool) -> StillFrameEventBuilder<'a> {
    StillFrameEventBuilder::new(in_still)
}

pub struct StillFrameEventBuilder<'a> {
    seqnum: Option<gst::Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    in_still: bool,
}

impl<'a> StillFrameEventBuilder<'a> {
    fn new(in_still: bool) -> Self {
        skip_assert_initialized!();
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
            in_still: in_still,
        }
    }

    event_builder_generic_impl!(|s: &mut Self| {
        ffi::gst_video_event_new_still_frame(s.in_still.to_glib())
    });
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
