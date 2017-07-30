// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use miniobject::*;
use structure::*;

use std::ptr;
use std::mem;
use std::ffi::CStr;

use glib;
use glib::translate::{from_glib, from_glib_none, from_glib_full, FromGlibPtrContainer, ToGlibPtr, ToGlib};

#[repr(C)]
pub struct EventRef(ffi::GstEvent);

pub type Event = GstRc<EventRef>;

unsafe impl MiniObject for EventRef {
    type GstType = ffi::GstEvent;
}

impl EventRef {
    pub fn get_seqnum(&self) -> u32 {
        unsafe { ffi::gst_event_get_seqnum(self.as_mut_ptr()) }
    }

    pub fn get_running_time_offset(&self) -> i64 {
        unsafe { ffi::gst_event_get_running_time_offset(self.as_mut_ptr()) }
    }


    pub fn get_structure(&self) -> &StructureRef {
        unsafe {
            let structure = ffi::gst_event_get_structure(self.as_mut_ptr());
            StructureRef::from_glib_borrow(structure)
        }
    }

    pub fn is_upstream(&self) -> bool {
        unsafe {
            ((*self.as_ptr()).type_ as u32) & (ffi::GST_EVENT_TYPE_UPSTREAM.bits()) != 0
        }
    }

    pub fn is_downstream(&self) -> bool {
        unsafe {
            ((*self.as_ptr()).type_ as u32) & (ffi::GST_EVENT_TYPE_DOWNSTREAM.bits()) != 0
        }
    }

    pub fn is_serialized(&self) -> bool {
        unsafe {
            ((*self.as_ptr()).type_ as u32) & (ffi::GST_EVENT_TYPE_SERIALIZED.bits()) != 0
        }
    }

    pub fn is_sticky(&self) -> bool {
        unsafe {
            ((*self.as_ptr()).type_ as u32) & (ffi::GST_EVENT_TYPE_STICKY.bits()) != 0
        }
    }

    pub fn is_sticky_multi(&self) -> bool {
        unsafe {
            ((*self.as_ptr()).type_ as u32) & (ffi::GST_EVENT_TYPE_STICKY_MULTI.bits()) != 0
        }
    }

    pub fn view(&self) -> EventView {
        let type_ = unsafe { (*self.as_ptr()).type_ };

        if type_ == ffi::GST_EVENT_FLUSH_START {
            EventView::FlushStart(FlushStart(self))
        } else if type_ == ffi::GST_EVENT_FLUSH_STOP {
            EventView::FlushStop(FlushStop(self))
        } else if type_ == ffi::GST_EVENT_STREAM_START {
            EventView::StreamStart(StreamStart(self))
        } else if type_ == ffi::GST_EVENT_CAPS {
            EventView::Caps(Caps(self))
        } else if type_ == ffi::GST_EVENT_SEGMENT {
            EventView::Segment(Segment(self))
        } else if type_ == ffi::GST_EVENT_STREAM_COLLECTION {
            EventView::StreamCollection(StreamCollection(self))
        } else if type_ == ffi::GST_EVENT_TAG {
            EventView::Tag(Tag(self))
        } else if type_ == ffi::GST_EVENT_BUFFERSIZE {
            EventView::BufferSize(BufferSize(self))
        } else if type_ == ffi::GST_EVENT_SINK_MESSAGE {
            EventView::SinkMessage(SinkMessage(self))
        } else if type_ == ffi::GST_EVENT_STREAM_GROUP_DONE {
            EventView::StreamGroupDone(StreamGroupDone(self))
        } else if type_ == ffi::GST_EVENT_EOS {
            EventView::Eos(Eos(self))
        } else if type_ == ffi::GST_EVENT_TOC {
            EventView::Toc(Toc(self))
        } else if type_ == ffi::GST_EVENT_PROTECTION {
            EventView::Protection(Protection(self))
        } else if type_ == ffi::GST_EVENT_SEGMENT_DONE {
            EventView::SegmentDone(SegmentDone(self))
        } else if type_ == ffi::GST_EVENT_GAP {
            EventView::Gap(Gap(self))
        } else if type_ == ffi::GST_EVENT_QOS {
            EventView::Qos(Qos(self))
        } else if type_ == ffi::GST_EVENT_SEEK {
            EventView::Seek(Seek(self))
        } else if type_ == ffi::GST_EVENT_NAVIGATION {
            EventView::Navigation(Navigation(self))
        } else if type_ == ffi::GST_EVENT_LATENCY {
            EventView::Latency(Latency(self))
        } else if type_ == ffi::GST_EVENT_STEP {
            EventView::Step(Step(self))
        } else if type_ == ffi::GST_EVENT_RECONFIGURE {
            EventView::Reconfigure(Reconfigure(self))
        } else if type_ == ffi::GST_EVENT_TOC_SELECT {
            EventView::TocSelect(TocSelect(self))
        } else if type_ == ffi::GST_EVENT_SELECT_STREAMS {
            EventView::SelectStreams(SelectStreams(self))
        } else if type_ == ffi::GST_EVENT_CUSTOM_UPSTREAM {
            EventView::CustomUpstream(CustomUpstream(self))
        } else if type_ == ffi::GST_EVENT_CUSTOM_DOWNSTREAM {
            EventView::CustomDownstream(CustomDownstream(self))
        } else if type_ == ffi::GST_EVENT_CUSTOM_DOWNSTREAM_OOB {
            EventView::CustomDownstreamOob(CustomDownstreamOob(self))
        } else if type_ == ffi::GST_EVENT_CUSTOM_DOWNSTREAM_STICKY {
            EventView::CustomDownstreamSticky(CustomDownstreamSticky(self))
        } else if type_ == ffi::GST_EVENT_CUSTOM_BOTH {
            EventView::CustomBoth(CustomBoth(self))
        } else if type_ == ffi::GST_EVENT_CUSTOM_BOTH_OOB {
            EventView::CustomBothOob(CustomBothOob(self))
        } else {
            EventView::Other
        }
    }
}

impl Event {
    pub fn new_flush_start() -> FlushStartBuilder {
        FlushStartBuilder::new()
    }

    pub fn new_flush_stop(reset_time: bool) -> FlushStopBuilder {
        FlushStopBuilder::new(reset_time)
    }

    pub fn new_stream_start(stream_id: &str) -> StreamStartBuilder {
        StreamStartBuilder::new(stream_id)
    }

    pub fn new_caps(caps: &::Caps) -> CapsBuilder {
        CapsBuilder::new(caps)
    }

    pub fn new_segment(segment: &::Segment) -> SegmentBuilder {
        SegmentBuilder::new(segment)
    }

    #[cfg(feature = "v1_10")]
    pub fn new_stream_collection(stream_collection: &::StreamCollection) -> StreamCollectionBuilder {
        StreamCollectionBuilder::new(stream_collection)
    }

    pub fn new_tag(tags: ::TagList) -> TagBuilder {
        TagBuilder::new(tags)
    }

    pub fn new_buffer_size(format: ::Format, minsize: i64, maxsize: i64, async: bool) -> BufferSizeBuilder {
        BufferSizeBuilder::new(format, minsize, maxsize, async)
    }

    pub fn new_sink_message<'a>(name: &'a str, msg: &'a ::Message) -> SinkMessageBuilder<'a> {
        SinkMessageBuilder::new(name, msg)
    }

    #[cfg(feature = "v1_10")]
    pub fn new_stream_group_done(group_id: u32) -> StreamGroupDoneBuilder {
        StreamGroupDoneBuilder::new(group_id)
    }

    pub fn new_eos() -> EosBuilder {
        EosBuilder::new()
    }

    pub fn new_toc(toc: (), updated: bool) -> TocBuilder {
        TocBuilder::new(toc, updated)
    }

    pub fn new_protection<'a>(system_id: &'a str, data: &'a ::Buffer, origin: &'a str) -> ProtectionBuilder<'a> {
        ProtectionBuilder::new(system_id, data, origin)
    }

    pub fn new_segment_done(format: ::Format, position: i64) -> SegmentDoneBuilder {
        SegmentDoneBuilder::new(format, position)
    }

    pub fn new_gap(timestamp: u64, duration: u64) -> GapBuilder {
        GapBuilder::new(timestamp, duration)
    }

    pub fn new_qos(type_: ::QOSType, proportion: f64, diff: i64, timestamp: u64) -> QosBuilder {
        QosBuilder::new(type_, proportion, diff, timestamp)
    }

    pub fn new_seek(rate: f64, format: ::Format, flags: ::SeekFlags, start_type: ::SeekType, start: i64, stop_type: ::SeekType, stop: i64) -> SeekBuilder {
        SeekBuilder::new(rate, format, flags, start_type, start, stop_type, stop)
    }

    pub fn new_navigation(structure: ::Structure) -> NavigationBuilder {
        NavigationBuilder::new(structure)
    }

    pub fn new_latency(latency: u64) -> LatencyBuilder {
        LatencyBuilder::new(latency)
    }

    pub fn new_step(format: ::Format, amount: u64, rate: f64, flush: bool, intermediate: bool) -> StepBuilder {
        StepBuilder::new(format, amount, rate, flush, intermediate)
    }

    pub fn new_reconfigure() -> ReconfigureBuilder {
        ReconfigureBuilder::new()
    }

    pub fn new_toc_select(uid: &str) -> TocSelectBuilder {
        TocSelectBuilder::new(uid)
    }

    #[cfg(feature = "v1_10")]
    pub fn new_select_streams<'a>(streams: &'a [&'a str]) -> SelectStreamsBuilder<'a> {
        SelectStreamsBuilder::new(streams)
    }

    pub fn new_custom_upstream(structure: ::Structure) -> CustomUpstreamBuilder {
        CustomUpstreamBuilder::new(structure)
    }

    pub fn new_custom_downstream(structure: ::Structure) -> CustomDownstreamBuilder {
        CustomDownstreamBuilder::new(structure)
    }

    pub fn new_custom_downstream_oob(structure: ::Structure) -> CustomDownstreamOobBuilder {
        CustomDownstreamOobBuilder::new(structure)
    }

    pub fn new_custom_downstream_sticky(structure: ::Structure) -> CustomDownstreamStickyBuilder {
        CustomDownstreamStickyBuilder::new(structure)
    }

    pub fn new_custom_both(structure: ::Structure) -> CustomBothBuilder {
        CustomBothBuilder::new(structure)
    }

    pub fn new_custom_both_oob(structure: ::Structure) -> CustomBothOobBuilder {
        CustomBothOobBuilder::new(structure)
    }
}

impl glib::types::StaticType for GstRc<EventRef> {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(ffi::gst_event_get_type()) }
    }
}

pub enum EventView<'a> {
    FlushStart(FlushStart<'a>),
    FlushStop(FlushStop<'a>),
    StreamStart(StreamStart<'a>),
    Caps(Caps<'a>),
    Segment(Segment<'a>),
    StreamCollection(StreamCollection<'a>),
    Tag(Tag<'a>),
    BufferSize(BufferSize<'a>),
    SinkMessage(SinkMessage<'a>),
    StreamGroupDone(StreamGroupDone<'a>),
    Eos(Eos<'a>),
    Toc(Toc<'a>),
    Protection(Protection<'a>),
    SegmentDone(SegmentDone<'a>),
    Gap(Gap<'a>),
    Qos(Qos<'a>),
    Seek(Seek<'a>),
    Navigation(Navigation<'a>),
    Latency(Latency<'a>),
    Step(Step<'a>),
    Reconfigure(Reconfigure<'a>),
    TocSelect(TocSelect<'a>),
    SelectStreams(SelectStreams<'a>),
    CustomUpstream(CustomUpstream<'a>),
    CustomDownstream(CustomDownstream<'a>),
    CustomDownstreamOob(CustomDownstreamOob<'a>),
    CustomDownstreamSticky(CustomDownstreamSticky<'a>),
    CustomBoth(CustomBoth<'a>),
    CustomBothOob(CustomBothOob<'a>),
    Other,
    __NonExhaustive,
}

pub struct FlushStart<'a>(&'a EventRef);

pub struct FlushStop<'a>(&'a EventRef);
impl<'a> FlushStop<'a> {
    pub fn get_reset_time(&self) -> bool {
        unsafe {
            let mut reset_time = mem::uninitialized();

            ffi::gst_event_parse_flush_stop(self.0.as_mut_ptr(), &mut reset_time);

            from_glib(reset_time)
        }
    }
}

pub struct StreamStart<'a>(&'a EventRef);
impl<'a> StreamStart<'a> {
    pub fn get_stream_id(&self) -> Option<&'a str> {
        unsafe {
            let mut stream_id = ptr::null();

            ffi::gst_event_parse_stream_start(self.0.as_mut_ptr(), &mut stream_id);
            if stream_id.is_null() {
                None
            } else {
                Some((CStr::from_ptr(stream_id).to_str().unwrap()))
            }
        }
    }

    pub fn get_stream_flags(&self) -> ::StreamFlags {
        unsafe {
            let mut stream_flags = mem::uninitialized();

            ffi::gst_event_parse_stream_flags(self.0.as_mut_ptr(), &mut stream_flags);

            from_glib(stream_flags)
        }
    }

    pub fn get_group_id(&self) -> u32 {
        unsafe {
            let mut group_id = mem::uninitialized();

            ffi::gst_event_parse_group_id(self.0.as_mut_ptr(), &mut group_id);

            group_id
        }
    }
}

pub struct Caps<'a>(&'a EventRef);
impl<'a> Caps<'a> {
    pub fn get_caps(&self) -> &'a ::CapsRef {
        unsafe {
            let mut caps = ptr::null_mut();

            ffi::gst_event_parse_caps(self.0.as_mut_ptr(), &mut caps);
            ::CapsRef::from_ptr(caps)
        }
    }
}

pub struct Segment<'a>(&'a EventRef);
impl<'a> Segment<'a> {
    pub fn get_segment(&self) -> ::Segment {
        unsafe {
            let mut segment = ptr::null();

            ffi::gst_event_parse_segment(self.0.as_mut_ptr(), &mut segment);
            from_glib_none(segment as *mut _)
        }
    }
}

pub struct StreamCollection<'a>(&'a EventRef);
impl<'a> StreamCollection<'a> {
    #[cfg(feature = "v1_10")]
    pub fn get_stream_collection(&self) -> ::StreamCollection {
        unsafe {
            let mut stream_collection = ptr::null_mut();

            ffi::gst_event_parse_stream_collection(self.0.as_mut_ptr(), &mut stream_collection);
            from_glib_full(stream_collection)
        }
    }
}

pub struct Tag<'a>(&'a EventRef);
impl<'a> Tag<'a> {
    pub fn get_tag(&self) -> &'a ::TagListRef {
        unsafe {
            let mut tags = ptr::null_mut();

            ffi::gst_event_parse_tag(self.0.as_mut_ptr(), &mut tags);
            ::TagListRef::from_ptr(tags)
        }
    }
}

pub struct BufferSize<'a>(&'a EventRef);
impl<'a> BufferSize<'a> {
    pub fn get(&self) -> (::Format, i64, i64, bool) {
        unsafe {
            let mut fmt = mem::uninitialized();
            let mut minsize = mem::uninitialized();
            let mut maxsize = mem::uninitialized();
            let mut async = mem::uninitialized();

            ffi::gst_event_parse_buffer_size(self.0.as_mut_ptr(), &mut fmt, &mut minsize, &mut maxsize, &mut async);
            (from_glib(fmt), minsize, maxsize, from_glib(async))
        }
    }
}

pub struct SinkMessage<'a>(&'a EventRef);
impl<'a> SinkMessage<'a> {
    pub fn get_message(&self) -> ::Message {
        unsafe {
            let mut msg = ptr::null_mut();

            ffi::gst_event_parse_sink_message(self.0.as_mut_ptr(), &mut msg);
            from_glib_full(msg)
        }
    }
}

pub struct StreamGroupDone<'a>(&'a EventRef);
impl<'a> StreamGroupDone<'a> {
    #[cfg(feature = "v1_10")]
    pub fn get_group_id(&self) -> u32 {
        unsafe {
            let mut group_id = mem::uninitialized();

            ffi::gst_event_parse_stream_group_done(self.0.as_mut_ptr(), &mut group_id);

            group_id
        }
    }
}

pub struct Eos<'a>(&'a EventRef);

// TODO
pub struct Toc<'a>(&'a EventRef);

pub struct Protection<'a>(&'a EventRef);
impl<'a> Protection<'a> {
    pub fn get(&self) -> (&'a str, &'a ::BufferRef, &'a str) {
        unsafe {
            let mut system_id = ptr::null();
            let mut buffer = ptr::null_mut();
            let mut origin = ptr::null();

            ffi::gst_event_parse_protection(self.0.as_mut_ptr(), &mut system_id, &mut buffer, &mut origin);

            (CStr::from_ptr(system_id).to_str().unwrap(), ::BufferRef::from_ptr(buffer), CStr::from_ptr(origin).to_str().unwrap())
        }
    }
}

pub struct SegmentDone<'a>(&'a EventRef);
impl<'a> SegmentDone<'a> {
    pub fn get(&self) -> (::Format, i64) {
        unsafe {
            let mut fmt = mem::uninitialized();
            let mut position = mem::uninitialized();

            ffi::gst_event_parse_segment_done(self.0.as_mut_ptr(), &mut fmt, &mut position);

            (from_glib(fmt), position)
        }
    }
}

pub struct Gap<'a>(&'a EventRef);
impl<'a> Gap<'a> {
    pub fn get(&self) -> (u64, u64) {
        unsafe {
            let mut timestamp = mem::uninitialized();
            let mut duration = mem::uninitialized();

            ffi::gst_event_parse_gap(self.0.as_mut_ptr(), &mut timestamp, &mut duration);

            (timestamp, duration)
        }
    }
}

pub struct Qos<'a>(&'a EventRef);
impl<'a> Qos<'a> {
    pub fn get(&self) -> (::QOSType, f64, i64, u64) {
        unsafe {
            let mut type_ = mem::uninitialized();
            let mut proportion = mem::uninitialized();
            let mut diff = mem::uninitialized();
            let mut timestamp = mem::uninitialized();

            ffi::gst_event_parse_qos(self.0.as_mut_ptr(), &mut type_, &mut proportion, &mut diff, &mut timestamp);

            (from_glib(type_), proportion, diff, timestamp)
        }
    }
}


pub struct Seek<'a>(&'a EventRef);
impl<'a> Seek<'a> {
    pub fn get(&self) -> (f64, ::Format, ::SeekFlags, ::SeekType, i64, ::SeekType, i64) {
        unsafe {
            let mut rate = mem::uninitialized();
            let mut fmt = mem::uninitialized();
            let mut flags = mem::uninitialized();
            let mut start_type = mem::uninitialized();
            let mut start = mem::uninitialized();
            let mut stop_type = mem::uninitialized();
            let mut stop = mem::uninitialized();

            ffi::gst_event_parse_seek(self.0.as_mut_ptr(), &mut rate, &mut fmt, &mut flags, &mut start_type, &mut start, &mut stop_type, &mut stop);

            (rate, from_glib(fmt), from_glib(flags), from_glib(start_type), start, from_glib(stop_type), stop)
        }
    }
}

pub struct Navigation<'a>(&'a EventRef);

pub struct Latency<'a>(&'a EventRef);
impl<'a> Latency<'a> {
    pub fn get_latency(&self) -> u64 {
        unsafe {
            let mut latency = mem::uninitialized();

            ffi::gst_event_parse_latency(self.0.as_mut_ptr(), &mut latency);

            latency
        }
    }
}

pub struct Step<'a>(&'a EventRef);
impl<'a> Step<'a> {
    pub fn get(&self) -> (::Format, u64, f64, bool, bool) {
        unsafe {
            let mut fmt = mem::uninitialized();
            let mut amount = mem::uninitialized();
            let mut rate = mem::uninitialized();
            let mut flush = mem::uninitialized();
            let mut intermediate = mem::uninitialized();

            ffi::gst_event_parse_step(self.0.as_mut_ptr(), &mut fmt, &mut amount, &mut rate, &mut flush, &mut intermediate);

            (from_glib(fmt), amount, rate, from_glib(flush), from_glib(intermediate))
        }
    }
}

pub struct Reconfigure<'a>(&'a EventRef);

pub struct TocSelect<'a>(&'a EventRef);
impl<'a> TocSelect<'a> {
    pub fn get_uid(&self) -> &'a str {
        unsafe {
            let mut uid = ptr::null_mut();

            ffi::gst_event_parse_toc_select(self.0.as_mut_ptr(), &mut uid);

            CStr::from_ptr(uid).to_str().unwrap()
        }
    }
}

pub struct SelectStreams<'a>(&'a EventRef);
impl<'a> SelectStreams<'a> {
    #[cfg(feature = "v1_10")]
    pub fn get_streams(&self) -> Vec<String> {
        unsafe {
            let mut streams = ptr::null_mut();

            ffi::gst_event_parse_select_streams(self.0.as_mut_ptr(), &mut streams);

            FromGlibPtrContainer::from_glib_full(streams)
        }
    }
}

pub struct CustomUpstream<'a>(&'a EventRef);
pub struct CustomDownstream<'a>(&'a EventRef);
pub struct CustomDownstreamOob<'a>(&'a EventRef);
pub struct CustomDownstreamSticky<'a>(&'a EventRef);
pub struct CustomBoth<'a>(&'a EventRef);
pub struct CustomBothOob<'a>(&'a EventRef);

macro_rules! event_builder_generic_impl {
    ($new_fn:expr) => {
        pub fn seqnum(self, seqnum: u32) -> Self {
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

        pub fn build(mut self) -> Event {
            assert_initialized_main_thread!();
            unsafe {
                let event = $new_fn(&mut self);
                if let Some(seqnum) = self.seqnum {
                    ffi::gst_event_set_seqnum(event, seqnum);
                }

                if let Some(running_time_offset) = self.running_time_offset {
                    ffi::gst_event_set_running_time_offset(event, running_time_offset);
                }

                from_glib_full(event)
            }
        }
    }
}

pub struct FlushStartBuilder {
    seqnum: Option<u32>,
    running_time_offset: Option<i64>,
}
impl FlushStartBuilder {
    pub fn new() -> Self {
        Self {
            seqnum: None,
            running_time_offset: None,
        }
    }

    event_builder_generic_impl!(|_| ffi::gst_event_new_flush_start());
}

pub struct FlushStopBuilder {
    seqnum: Option<u32>,
    running_time_offset: Option<i64>,
    reset_time: bool
}
impl FlushStopBuilder {
    pub fn new(reset_time: bool) -> Self {
        Self {
            seqnum: None,
            running_time_offset: None,
            reset_time: reset_time,
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_flush_stop(s.reset_time.to_glib()));
}

pub struct StreamStartBuilder<'a> {
    seqnum: Option<u32>,
    running_time_offset: Option<i64>,
    stream_id: &'a str,
    flags: Option<::StreamFlags>,
    group_id: Option<u32>,
}
impl<'a> StreamStartBuilder<'a> {
    pub fn new(stream_id: &'a str) -> Self {
        Self {
            seqnum: None,
            running_time_offset: None,
            stream_id: stream_id,
            flags: None,
            group_id: None,
        }
    }

    pub fn flags(self, flags: ::StreamFlags) -> Self {
        Self {
            flags: Some(flags),
            .. self
        }
    }

    pub fn group_id(self, group_id: u32) -> Self {
        Self {
            group_id: Some(group_id),
            .. self
        }
    }

    event_builder_generic_impl!(|s: &Self| {
        let ev = ffi::gst_event_new_stream_start(s.stream_id.to_glib_none().0);
        if let Some(flags) = s.flags {
            ffi::gst_event_set_stream_flags(ev, flags.to_glib());
        }
        if let Some(group_id) = s.group_id {
            ffi::gst_event_set_group_id(ev, group_id);
        }
        ev
    });
}

pub struct CapsBuilder<'a> {
    seqnum: Option<u32>,
    running_time_offset: Option<i64>,
    caps: &'a ::Caps,
}
impl<'a> CapsBuilder<'a> {
    pub fn new(caps: &'a ::Caps) -> Self {
        Self {
            seqnum: None,
            running_time_offset: None,
            caps: caps,
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_caps(s.caps.as_mut_ptr()));
}

pub struct SegmentBuilder<'a> {
    seqnum: Option<u32>,
    running_time_offset: Option<i64>,
    segment: &'a ::Segment,
}
impl<'a> SegmentBuilder<'a> {
    pub fn new(segment: &'a ::Segment) -> Self {
        Self {
            seqnum: None,
            running_time_offset: None,
            segment: segment,
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_segment(s.segment.to_glib_none().0));
}

#[cfg(feature = "v1_10")]
pub struct StreamCollectionBuilder<'a> {
    seqnum: Option<u32>,
    running_time_offset: Option<i64>,
    stream_collection: &'a ::StreamCollection,
}
#[cfg(feature = "v1_10")]
impl<'a> StreamCollectionBuilder<'a> {
    pub fn new(stream_collection: &'a ::StreamCollection) -> Self {
        Self {
            seqnum: None,
            running_time_offset: None,
            stream_collection: stream_collection,
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_stream_collection(s.stream_collection.to_glib_none().0));
}

pub struct TagBuilder {
    seqnum: Option<u32>,
    running_time_offset: Option<i64>,
    tags: Option<::TagList>,
}
impl TagBuilder {
    pub fn new(tags: ::TagList) -> Self {
        Self {
            seqnum: None,
            running_time_offset: None,
            tags: Some(tags),
        }
    }

    event_builder_generic_impl!(|s: &mut Self| {
        let tags = s.tags.take().unwrap();
        ffi::gst_event_new_tag(tags.into_ptr())
    });
}

pub struct BufferSizeBuilder {
    seqnum: Option<u32>,
    running_time_offset: Option<i64>,
    fmt: ::Format,
    minsize: i64,
    maxsize: i64,
    async: bool,
}
impl BufferSizeBuilder {
    pub fn new(fmt: ::Format, minsize: i64, maxsize: i64, async: bool) -> Self {
        Self {
            seqnum: None,
            running_time_offset: None,
            fmt: fmt,
            minsize: minsize,
            maxsize: maxsize,
            async: async,
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_buffer_size(s.fmt.to_glib(), s.minsize, s.maxsize, s.async.to_glib()));
}

pub struct SinkMessageBuilder<'a> {
    seqnum: Option<u32>,
    running_time_offset: Option<i64>,
    name: &'a str,
    msg: &'a ::Message,
}
impl<'a> SinkMessageBuilder<'a> {
    pub fn new(name: &'a str, msg: &'a ::Message) -> Self {
        Self {
            seqnum: None,
            running_time_offset: None,
            name: name,
            msg: msg,
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_sink_message(s.name.to_glib_none().0, s.msg.as_mut_ptr()));
}

#[cfg(feature = "v1_10")]
pub struct StreamGroupDoneBuilder {
    seqnum: Option<u32>,
    running_time_offset: Option<i64>,
    uid: u32,
}
#[cfg(feature = "v1_10")]
impl StreamGroupDoneBuilder {
    pub fn new(uid: u32) -> Self {
        Self {
            seqnum: None,
            running_time_offset: None,
            uid: uid,
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_stream_group_done(s.uid));
}

pub struct EosBuilder {
    seqnum: Option<u32>,
    running_time_offset: Option<i64>,
}
impl EosBuilder {
    pub fn new() -> Self {
        Self {
            seqnum: None,
            running_time_offset: None,
        }
    }

    event_builder_generic_impl!(|_| ffi::gst_event_new_eos());
}

pub struct TocBuilder {
    seqnum: Option<u32>,
    running_time_offset: Option<i64>,
    toc: (),
    updated: bool,
}
impl TocBuilder {
    pub fn new(toc: (), updated: bool) -> Self {
        Self {
            seqnum: None,
            running_time_offset: None,
            toc: toc,
            updated: updated,
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_toc(ptr::null_mut(), s.updated.to_glib()));
}

pub struct ProtectionBuilder<'a> {
    seqnum: Option<u32>,
    running_time_offset: Option<i64>,
    system_id: &'a str,
    data: &'a ::Buffer,
    origin: &'a str,
}
impl<'a> ProtectionBuilder<'a> {
    pub fn new(system_id: &'a str, data: &'a ::Buffer, origin: &'a str) -> Self {
        Self {
            seqnum: None,
            running_time_offset: None,
            system_id: system_id,
            data: data,
            origin: origin,
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_protection(s.system_id.to_glib_none().0, s.data.as_mut_ptr(), s.origin.to_glib_none().0));
}

pub struct SegmentDoneBuilder {
    seqnum: Option<u32>,
    running_time_offset: Option<i64>,
    fmt: ::Format,
    position: i64,
}
impl SegmentDoneBuilder {
    pub fn new(fmt: ::Format, position: i64) -> Self {
        Self {
            seqnum: None,
            running_time_offset: None,
            fmt: fmt,
            position: position,
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_segment_done(s.fmt.to_glib(), s.position));
}

pub struct GapBuilder {
    seqnum: Option<u32>,
    running_time_offset: Option<i64>,
    timestamp: u64,
    duration: u64,
}
impl GapBuilder {
    pub fn new(timestamp: u64, duration: u64) -> Self {
        Self {
            seqnum: None,
            running_time_offset: None,
            timestamp: timestamp,
            duration: duration,
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_gap(s.timestamp, s.duration));
}

pub struct QosBuilder {
    seqnum: Option<u32>,
    running_time_offset: Option<i64>,
    type_: ::QOSType,
    proportion: f64,
    diff: i64,
    timestamp: u64,
}
impl QosBuilder {
    pub fn new(type_: ::QOSType, proportion: f64, diff: i64, timestamp: u64) -> Self {
        Self {
            seqnum: None,
            running_time_offset: None,
            type_: type_,
            proportion: proportion,
            diff: diff,
            timestamp: timestamp,
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_qos(s.type_.to_glib(), s.proportion, s.diff, s.timestamp));
}

pub struct SeekBuilder {
    seqnum: Option<u32>,
    running_time_offset: Option<i64>,
    rate: f64,
    fmt: ::Format,
    flags: ::SeekFlags,
    start_type: ::SeekType,
    start: i64,
    stop_type: ::SeekType,
    stop: i64,
}
impl SeekBuilder {
    pub fn new(rate: f64, fmt: ::Format, flags: ::SeekFlags, start_type: ::SeekType, start: i64, stop_type: ::SeekType, stop: i64) -> Self {
        Self {
            seqnum: None,
            running_time_offset: None,
            rate: rate,
            fmt: fmt,
            flags: flags,
            start_type,
            start,
            stop_type,
            stop,
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_seek(s.rate, s.fmt.to_glib(), s.flags.to_glib(), s.start_type.to_glib(), s.start, s.stop_type.to_glib(), s.stop));
}

pub struct NavigationBuilder {
    seqnum: Option<u32>,
    running_time_offset: Option<i64>,
    structure: Option<Structure>,
}
impl NavigationBuilder {
    pub fn new(structure: Structure) -> Self {
        Self {
            seqnum: None,
            running_time_offset: None,
            structure: Some(structure),
        }
    }

    event_builder_generic_impl!(|s: &mut Self| {
        let structure = s.structure.take();
        let ev = ffi::gst_event_new_navigation(structure.to_glib_none().0);
        mem::forget(structure);

        ev
    });
}

pub struct LatencyBuilder {
    seqnum: Option<u32>,
    running_time_offset: Option<i64>,
    latency: u64,
}
impl LatencyBuilder {
    pub fn new(latency: u64) -> Self {
        Self {
            seqnum: None,
            running_time_offset: None,
            latency: latency,
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_latency(s.latency));
}

pub struct StepBuilder {
    seqnum: Option<u32>,
    running_time_offset: Option<i64>,
    fmt: ::Format,
    amount: u64,
    rate: f64,
    flush: bool,
    intermediate: bool,
}
impl StepBuilder {
    pub fn new(fmt: ::Format, amount: u64, rate: f64, flush: bool, intermediate: bool) -> Self {
        Self {
            seqnum: None,
            running_time_offset: None,
            fmt: fmt,
            amount: amount,
            rate: rate,
            flush: flush,
            intermediate: intermediate,
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_step(s.fmt.to_glib(), s.amount, s.rate, s.flush.to_glib(), s.intermediate.to_glib()));
}

pub struct ReconfigureBuilder {
    seqnum: Option<u32>,
    running_time_offset: Option<i64>,
}
impl ReconfigureBuilder {
    pub fn new() -> Self {
        Self {
            seqnum: None,
            running_time_offset: None,
        }
    }

    event_builder_generic_impl!(|_| ffi::gst_event_new_reconfigure());
}

pub struct TocSelectBuilder<'a> {
    seqnum: Option<u32>,
    running_time_offset: Option<i64>,
    uid: &'a str,
}
impl<'a> TocSelectBuilder<'a> {
    pub fn new(uid: &'a str) -> Self {
        Self {
            seqnum: None,
            running_time_offset: None,
            uid: uid,
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_toc_select(s.uid.to_glib_none().0));
}

#[cfg(feature = "v1_10")]
pub struct SelectStreamsBuilder<'a> {
    seqnum: Option<u32>,
    running_time_offset: Option<i64>,
    streams: &'a [&'a str],
}
#[cfg(feature = "v1_10")]
impl<'a> SelectStreamsBuilder<'a> {
    pub fn new(streams: &'a [&'a str]) -> Self {
        Self {
            seqnum: None,
            running_time_offset: None,
            streams: streams,
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_select_streams(s.streams.to_glib_full()));
}

pub struct CustomUpstreamBuilder {
    seqnum: Option<u32>,
    running_time_offset: Option<i64>,
    structure: Option<Structure>,
}
impl CustomUpstreamBuilder {
    pub fn new(structure: Structure) -> Self {
        Self {
            seqnum: None,
            running_time_offset: None,
            structure: Some(structure),
        }
    }

    event_builder_generic_impl!(|s: &mut Self| {
        let structure = s.structure.take();
        let ev = ffi::gst_event_new_custom(ffi::GST_EVENT_CUSTOM_UPSTREAM, structure.to_glib_none().0);
        mem::forget(structure);

        ev
    });
}

pub struct CustomDownstreamBuilder {
    seqnum: Option<u32>,
    running_time_offset: Option<i64>,
    structure: Option<Structure>,
}
impl CustomDownstreamBuilder {
    pub fn new(structure: Structure) -> Self {
        Self {
            seqnum: None,
            running_time_offset: None,
            structure: Some(structure),
        }
    }

    event_builder_generic_impl!(|s: &mut Self| {
        let structure = s.structure.take();
        let ev = ffi::gst_event_new_custom(ffi::GST_EVENT_CUSTOM_DOWNSTREAM, structure.to_glib_none().0);
        mem::forget(structure);

        ev
    });
}

pub struct CustomDownstreamOobBuilder {
    seqnum: Option<u32>,
    running_time_offset: Option<i64>,
    structure: Option<Structure>,
}
impl CustomDownstreamOobBuilder {
    pub fn new(structure: Structure) -> Self {
        Self {
            seqnum: None,
            running_time_offset: None,
            structure: Some(structure),
        }
    }

    event_builder_generic_impl!(|s: &mut Self| {
        let structure = s.structure.take();
        let ev = ffi::gst_event_new_custom(ffi::GST_EVENT_CUSTOM_DOWNSTREAM_OOB, structure.to_glib_none().0);
        mem::forget(structure);

        ev
    });
}

pub struct CustomDownstreamStickyBuilder {
    seqnum: Option<u32>,
    running_time_offset: Option<i64>,
    structure: Option<Structure>,
}
impl CustomDownstreamStickyBuilder {
    pub fn new(structure: Structure) -> Self {
        Self {
            seqnum: None,
            running_time_offset: None,
            structure: Some(structure),
        }
    }

    event_builder_generic_impl!(|s: &mut Self| {
        let structure = s.structure.take();
        let ev = ffi::gst_event_new_custom(ffi::GST_EVENT_CUSTOM_DOWNSTREAM_STICKY, structure.to_glib_none().0);
        mem::forget(structure);

        ev
    });
}

pub struct CustomBothBuilder {
    seqnum: Option<u32>,
    running_time_offset: Option<i64>,
    structure: Option<Structure>,
}
impl CustomBothBuilder {
    pub fn new(structure: Structure) -> Self {
        Self {
            seqnum: None,
            running_time_offset: None,
            structure: Some(structure),
        }
    }

    event_builder_generic_impl!(|s: &mut Self| {
        let structure = s.structure.take();
        let ev = ffi::gst_event_new_custom(ffi::GST_EVENT_CUSTOM_BOTH, structure.to_glib_none().0);
        mem::forget(structure);

        ev
    });
}

pub struct CustomBothOobBuilder {
    seqnum: Option<u32>,
    running_time_offset: Option<i64>,
    structure: Option<Structure>,
}
impl CustomBothOobBuilder {
    pub fn new(structure: Structure) -> Self {
        Self {
            seqnum: None,
            running_time_offset: None,
            structure: Some(structure),
        }
    }

    event_builder_generic_impl!(|s: &mut Self| {
        let structure = s.structure.take();
        let ev = ffi::gst_event_new_custom(ffi::GST_EVENT_CUSTOM_BOTH_OOB, structure.to_glib_none().0);
        mem::forget(structure);

        ev
    });
}
