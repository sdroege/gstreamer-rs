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
use GenericFormattedValue;

use std::ptr;
use std::mem;
use std::cmp;
use std::fmt;
use std::ffi::CStr;
use std::ops::Deref;

use glib;
use glib::value::ToSendValue;
use glib::translate::{from_glib, from_glib_full, FromGlib, ToGlib, ToGlibPtr};

#[cfg(any(feature = "v1_10", feature = "dox"))]
use glib::translate::FromGlibPtrContainer;

use EventType;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Seqnum(pub u32);
pub const SEQNUM_INVALID: Seqnum = Seqnum(0);

impl ToGlib for Seqnum {
    type GlibType = u32;

    fn to_glib(&self) -> u32 {
        self.0
    }
}

impl FromGlib<u32> for Seqnum {
    fn from_glib(val: u32) -> Seqnum {
        skip_assert_initialized!();
        Seqnum(val)
    }
}

impl Into<u32> for Seqnum {
    fn into(self) -> u32 {
        self.0
    }
}

impl From<u32> for Seqnum {
    fn from(v: u32) -> Seqnum {
        Seqnum(v)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GroupId(pub u32);
pub const GROUP_ID_INVALID: GroupId = GroupId(0);

impl ToGlib for GroupId {
    type GlibType = u32;

    fn to_glib(&self) -> u32 {
        self.0
    }
}

impl Into<u32> for GroupId {
    fn into(self) -> u32 {
        self.0
    }
}

impl From<u32> for GroupId {
    fn from(v: u32) -> GroupId {
        GroupId(v)
    }
}

impl FromGlib<u32> for GroupId {
    fn from_glib(val: u32) -> GroupId {
        skip_assert_initialized!();
        GroupId(val)
    }
}

#[repr(C)]
pub struct EventRef(ffi::GstEvent);

pub type Event = GstRc<EventRef>;

unsafe impl Sync for EventRef {}
unsafe impl Send for EventRef {}

unsafe impl MiniObject for EventRef {
    type GstType = ffi::GstEvent;
}

impl EventType {
    pub fn is_upstream(&self) -> bool {
        (self.to_glib() as u32) & (ffi::GST_EVENT_TYPE_UPSTREAM.bits()) != 0
    }

    pub fn is_downstream(&self) -> bool {
        (self.to_glib() as u32) & (ffi::GST_EVENT_TYPE_DOWNSTREAM.bits()) != 0
    }

    pub fn is_serialized(&self) -> bool {
        (self.to_glib() as u32) & (ffi::GST_EVENT_TYPE_SERIALIZED.bits()) != 0
    }

    pub fn is_sticky(&self) -> bool {
        (self.to_glib() as u32) & (ffi::GST_EVENT_TYPE_STICKY.bits()) != 0
    }

    pub fn is_sticky_multi(&self) -> bool {
        (self.to_glib() as u32) & (ffi::GST_EVENT_TYPE_STICKY_MULTI.bits()) != 0
    }
}

impl PartialOrd for EventType {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        if !self.is_serialized() || !other.is_serialized() {
            return None;
        }

        let v1 = self.to_glib() as u32;
        let v2 = other.to_glib() as u32;

        let stream_start = ffi::GST_EVENT_STREAM_START as u32;
        let segment = ffi::GST_EVENT_SEGMENT as u32;
        let eos = ffi::GST_EVENT_EOS as u32;

        // Strictly ordered range between stream_start and segment,
        // and EOS is bigger than everything else
        if v1 >= stream_start && v1 <= segment || v2 >= stream_start && v2 <= segment {
            Some(v1.cmp(&v2))
        // If one is EOS, the other is definitely less or equal
        } else if v1 == eos || v2 == eos {
            if v1 == v2 {
                Some(cmp::Ordering::Equal)
            } else if v1 == eos {
                Some(cmp::Ordering::Greater)
            } else {
                Some(cmp::Ordering::Less)
            }
        } else {
            None
        }
    }
}

impl EventRef {
    pub fn get_seqnum(&self) -> Seqnum {
        unsafe { from_glib(ffi::gst_event_get_seqnum(self.as_mut_ptr())) }
    }

    pub fn get_running_time_offset(&self) -> i64 {
        unsafe { ffi::gst_event_get_running_time_offset(self.as_mut_ptr()) }
    }

    pub fn set_running_time_offset(&mut self, offset: i64) {
        unsafe { ffi::gst_event_set_running_time_offset(self.as_mut_ptr(), offset) }
    }

    pub fn get_structure(&self) -> Option<&StructureRef> {
        unsafe {
            let structure = ffi::gst_event_get_structure(self.as_mut_ptr());
            if structure.is_null() {
                None
            } else {
                Some(StructureRef::from_glib_borrow(structure))
            }
        }
    }

    pub fn is_upstream(&self) -> bool {
        self.get_type().is_upstream()
    }

    pub fn is_downstream(&self) -> bool {
        self.get_type().is_downstream()
    }

    pub fn is_serialized(&self) -> bool {
        self.get_type().is_serialized()
    }

    pub fn is_sticky(&self) -> bool {
        self.get_type().is_sticky()
    }

    pub fn is_sticky_multi(&self) -> bool {
        self.get_type().is_sticky_multi()
    }

    pub fn get_type(&self) -> EventType {
        unsafe { from_glib((*self.as_ptr()).type_) }
    }

    pub fn view(&self) -> EventView {
        let type_ = unsafe { (*self.as_ptr()).type_ };

        match type_ {
            ffi::GST_EVENT_FLUSH_START => EventView::FlushStart(FlushStart(self)),
            ffi::GST_EVENT_FLUSH_STOP => EventView::FlushStop(FlushStop(self)),
            ffi::GST_EVENT_STREAM_START => EventView::StreamStart(StreamStart(self)),
            ffi::GST_EVENT_CAPS => EventView::Caps(Caps(self)),
            ffi::GST_EVENT_SEGMENT => EventView::Segment(Segment(self)),
            ffi::GST_EVENT_STREAM_COLLECTION => EventView::StreamCollection(StreamCollection(self)),
            ffi::GST_EVENT_TAG => EventView::Tag(Tag(self)),
            ffi::GST_EVENT_BUFFERSIZE => EventView::BufferSize(BufferSize(self)),
            ffi::GST_EVENT_SINK_MESSAGE => EventView::SinkMessage(SinkMessage(self)),
            ffi::GST_EVENT_STREAM_GROUP_DONE => EventView::StreamGroupDone(StreamGroupDone(self)),
            ffi::GST_EVENT_EOS => EventView::Eos(Eos(self)),
            ffi::GST_EVENT_TOC => EventView::Toc(Toc(self)),
            ffi::GST_EVENT_PROTECTION => EventView::Protection(Protection(self)),
            ffi::GST_EVENT_SEGMENT_DONE => EventView::SegmentDone(SegmentDone(self)),
            ffi::GST_EVENT_GAP => EventView::Gap(Gap(self)),
            ffi::GST_EVENT_QOS => EventView::Qos(Qos(self)),
            ffi::GST_EVENT_SEEK => EventView::Seek(Seek(self)),
            ffi::GST_EVENT_NAVIGATION => EventView::Navigation(Navigation(self)),
            ffi::GST_EVENT_LATENCY => EventView::Latency(Latency(self)),
            ffi::GST_EVENT_STEP => EventView::Step(Step(self)),
            ffi::GST_EVENT_RECONFIGURE => EventView::Reconfigure(Reconfigure(self)),
            ffi::GST_EVENT_TOC_SELECT => EventView::TocSelect(TocSelect(self)),
            ffi::GST_EVENT_SELECT_STREAMS => EventView::SelectStreams(SelectStreams(self)),
            ffi::GST_EVENT_CUSTOM_UPSTREAM => EventView::CustomUpstream(CustomUpstream(self)),
            ffi::GST_EVENT_CUSTOM_DOWNSTREAM => EventView::CustomDownstream(CustomDownstream(self)),
            ffi::GST_EVENT_CUSTOM_DOWNSTREAM_OOB => {
                EventView::CustomDownstreamOob(CustomDownstreamOob(self))
            }
            ffi::GST_EVENT_CUSTOM_DOWNSTREAM_STICKY => {
                EventView::CustomDownstreamSticky(CustomDownstreamSticky(self))
            }
            ffi::GST_EVENT_CUSTOM_BOTH => EventView::CustomBoth(CustomBoth(self)),
            ffi::GST_EVENT_CUSTOM_BOTH_OOB => EventView::CustomBothOob(CustomBothOob(self)),
            _ => EventView::Other,
        }
    }
}

impl GstRc<EventRef> {
    pub fn new_flush_start<'a>() -> FlushStartBuilder<'a> {
        assert_initialized_main_thread!();
        FlushStartBuilder::new()
    }

    pub fn new_flush_stop<'a>(reset_time: bool) -> FlushStopBuilder<'a> {
        assert_initialized_main_thread!();
        FlushStopBuilder::new(reset_time)
    }

    pub fn new_stream_start(stream_id: &str) -> StreamStartBuilder {
        assert_initialized_main_thread!();
        StreamStartBuilder::new(stream_id)
    }

    pub fn new_caps(caps: &::Caps) -> CapsBuilder {
        assert_initialized_main_thread!();
        CapsBuilder::new(caps)
    }

    pub fn new_segment<F: ::FormattedValue>(segment: &::FormattedSegment<F>) -> SegmentBuilder {
        assert_initialized_main_thread!();
        SegmentBuilder::new(segment.as_ref())
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn new_stream_collection(
        stream_collection: &::StreamCollection,
    ) -> StreamCollectionBuilder {
        assert_initialized_main_thread!();
        StreamCollectionBuilder::new(stream_collection)
    }

    pub fn new_tag<'a>(tags: ::TagList) -> TagBuilder<'a> {
        assert_initialized_main_thread!();
        TagBuilder::new(tags)
    }

    pub fn new_buffer_size<'a, V: Into<GenericFormattedValue>>(
        minsize: V,
        maxsize: V,
        async: bool,
    ) -> BufferSizeBuilder<'a> {
        assert_initialized_main_thread!();
        let minsize = minsize.into();
        let maxsize = maxsize.into();
        assert_eq!(minsize.get_format(), maxsize.get_format());

        BufferSizeBuilder::new(minsize, maxsize, async)
    }

    pub fn new_sink_message<'a>(name: &'a str, msg: &'a ::Message) -> SinkMessageBuilder<'a> {
        assert_initialized_main_thread!();
        SinkMessageBuilder::new(name, msg)
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn new_stream_group_done<'a>(group_id: GroupId) -> StreamGroupDoneBuilder<'a> {
        assert_initialized_main_thread!();
        StreamGroupDoneBuilder::new(group_id)
    }

    pub fn new_eos<'a>() -> EosBuilder<'a> {
        assert_initialized_main_thread!();
        EosBuilder::new()
    }

    pub fn new_toc(toc: &::Toc, updated: bool) -> TocBuilder {
        assert_initialized_main_thread!();
        TocBuilder::new(toc, updated)
    }

    pub fn new_protection<'a>(system_id: &'a str, data: &'a ::Buffer) -> ProtectionBuilder<'a> {
        assert_initialized_main_thread!();
        ProtectionBuilder::new(system_id, data)
    }

    pub fn new_segment_done<'a, V: Into<GenericFormattedValue>>(
        position: V,
    ) -> SegmentDoneBuilder<'a> {
        assert_initialized_main_thread!();
        let position = position.into();
        SegmentDoneBuilder::new(position)
    }

    pub fn new_gap<'a>(timestamp: ::ClockTime, duration: ::ClockTime) -> GapBuilder<'a> {
        assert_initialized_main_thread!();
        GapBuilder::new(timestamp, duration)
    }

    pub fn new_qos<'a>(
        type_: ::QOSType,
        proportion: f64,
        diff: i64,
        timestamp: ::ClockTime,
    ) -> QosBuilder<'a> {
        assert_initialized_main_thread!();
        QosBuilder::new(type_, proportion, diff, timestamp)
    }

    pub fn new_seek<'a, V: Into<GenericFormattedValue>>(
        rate: f64,
        flags: ::SeekFlags,
        start_type: ::SeekType,
        start: V,
        stop_type: ::SeekType,
        stop: V,
    ) -> SeekBuilder<'a> {
        assert_initialized_main_thread!();
        let start = start.into();
        let stop = stop.into();
        assert_eq!(start.get_format(), stop.get_format());

        SeekBuilder::new(rate, flags, start_type, start, stop_type, stop)
    }

    pub fn new_navigation<'a>(structure: ::Structure) -> NavigationBuilder<'a> {
        assert_initialized_main_thread!();
        NavigationBuilder::new(structure)
    }

    pub fn new_latency<'a>(latency: ::ClockTime) -> LatencyBuilder<'a> {
        assert_initialized_main_thread!();
        LatencyBuilder::new(latency)
    }

    pub fn new_step<'a, V: Into<GenericFormattedValue>>(
        amount: V,
        rate: f64,
        flush: bool,
        intermediate: bool,
    ) -> StepBuilder<'a> {
        assert_initialized_main_thread!();
        StepBuilder::new(amount.into(), rate, flush, intermediate)
    }

    pub fn new_reconfigure<'a>() -> ReconfigureBuilder<'a> {
        assert_initialized_main_thread!();
        ReconfigureBuilder::new()
    }

    pub fn new_toc_select(uid: &str) -> TocSelectBuilder {
        assert_initialized_main_thread!();
        TocSelectBuilder::new(uid)
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn new_select_streams<'a>(streams: &'a [&'a str]) -> SelectStreamsBuilder<'a> {
        assert_initialized_main_thread!();
        SelectStreamsBuilder::new(streams)
    }

    pub fn new_custom_upstream<'a>(structure: ::Structure) -> CustomUpstreamBuilder<'a> {
        assert_initialized_main_thread!();
        CustomUpstreamBuilder::new(structure)
    }

    pub fn new_custom_downstream<'a>(structure: ::Structure) -> CustomDownstreamBuilder<'a> {
        assert_initialized_main_thread!();
        CustomDownstreamBuilder::new(structure)
    }

    pub fn new_custom_downstream_oob<'a>(structure: ::Structure) -> CustomDownstreamOobBuilder<'a> {
        assert_initialized_main_thread!();
        CustomDownstreamOobBuilder::new(structure)
    }

    pub fn new_custom_downstream_sticky<'a>(
        structure: ::Structure,
    ) -> CustomDownstreamStickyBuilder<'a> {
        assert_initialized_main_thread!();
        CustomDownstreamStickyBuilder::new(structure)
    }

    pub fn new_custom_both<'a>(structure: ::Structure) -> CustomBothBuilder<'a> {
        assert_initialized_main_thread!();
        CustomBothBuilder::new(structure)
    }

    pub fn new_custom_both_oob<'a>(structure: ::Structure) -> CustomBothOobBuilder<'a> {
        assert_initialized_main_thread!();
        CustomBothOobBuilder::new(structure)
    }
}

impl glib::types::StaticType for EventRef {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(ffi::gst_event_get_type()) }
    }
}

impl fmt::Debug for EventRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Event")
            .field("type", &unsafe {
                let type_ = ffi::gst_event_type_get_name((*self.as_ptr()).type_);
                CStr::from_ptr(type_).to_str().unwrap()
            })
            .field("seqnum", &self.get_seqnum())
            .field("structure", &self.get_structure())
            .finish()
    }
}

impl ToOwned for EventRef {
    type Owned = GstRc<EventRef>;

    fn to_owned(&self) -> GstRc<EventRef> {
        unsafe {
            from_glib_full(ffi::gst_mini_object_copy(self.as_ptr() as *const _)
                as *mut _)
        }
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

macro_rules! declare_concrete_event(
    ($name:ident) => {
        pub struct $name<'a>(&'a EventRef);

        impl<'a> Deref for $name<'a> {
            type Target = EventRef;

            fn deref(&self) -> &Self::Target {
                self.0
            }
        }
    }
);

declare_concrete_event!(FlushStart);

declare_concrete_event!(FlushStop);
impl<'a> FlushStop<'a> {
    pub fn get_reset_time(&self) -> bool {
        unsafe {
            let mut reset_time = mem::uninitialized();

            ffi::gst_event_parse_flush_stop(self.as_mut_ptr(), &mut reset_time);

            from_glib(reset_time)
        }
    }
}

declare_concrete_event!(StreamStart);
impl<'a> StreamStart<'a> {
    pub fn get_stream_id(&self) -> &'a str {
        unsafe {
            let mut stream_id = ptr::null();

            ffi::gst_event_parse_stream_start(self.as_mut_ptr(), &mut stream_id);
            CStr::from_ptr(stream_id).to_str().unwrap()
        }
    }

    pub fn get_stream_flags(&self) -> ::StreamFlags {
        unsafe {
            let mut stream_flags = mem::uninitialized();

            ffi::gst_event_parse_stream_flags(self.as_mut_ptr(), &mut stream_flags);

            from_glib(stream_flags)
        }
    }

    pub fn get_group_id(&self) -> GroupId {
        unsafe {
            let mut group_id = mem::uninitialized();

            ffi::gst_event_parse_group_id(self.as_mut_ptr(), &mut group_id);

            from_glib(group_id)
        }
    }
}

declare_concrete_event!(Caps);
impl<'a> Caps<'a> {
    pub fn get_caps(&self) -> &'a ::CapsRef {
        unsafe {
            let mut caps = ptr::null_mut();

            ffi::gst_event_parse_caps(self.as_mut_ptr(), &mut caps);
            ::CapsRef::from_ptr(caps)
        }
    }
}

declare_concrete_event!(Segment);
impl<'a> Segment<'a> {
    pub fn get_segment(&self) -> &'a ::Segment {
        unsafe {
            let mut segment = ptr::null();

            ffi::gst_event_parse_segment(self.as_mut_ptr(), &mut segment);
            &*(segment as *mut ffi::GstSegment as *mut ::Segment)
        }
    }
}

declare_concrete_event!(StreamCollection);
impl<'a> StreamCollection<'a> {
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_stream_collection(&self) -> ::StreamCollection {
        unsafe {
            let mut stream_collection = ptr::null_mut();

            ffi::gst_event_parse_stream_collection(self.as_mut_ptr(), &mut stream_collection);
            from_glib_full(stream_collection)
        }
    }
}

declare_concrete_event!(Tag);
impl<'a> Tag<'a> {
    pub fn get_tag(&self) -> &'a ::TagListRef {
        unsafe {
            let mut tags = ptr::null_mut();

            ffi::gst_event_parse_tag(self.as_mut_ptr(), &mut tags);
            ::TagListRef::from_ptr(tags)
        }
    }
}

declare_concrete_event!(BufferSize);
impl<'a> BufferSize<'a> {
    pub fn get(&self) -> (GenericFormattedValue, GenericFormattedValue, bool) {
        unsafe {
            let mut fmt = mem::uninitialized();
            let mut minsize = mem::uninitialized();
            let mut maxsize = mem::uninitialized();
            let mut async = mem::uninitialized();

            ffi::gst_event_parse_buffer_size(
                self.as_mut_ptr(),
                &mut fmt,
                &mut minsize,
                &mut maxsize,
                &mut async,
            );
            (
                GenericFormattedValue::new(from_glib(fmt), minsize),
                GenericFormattedValue::new(from_glib(fmt), maxsize),
                from_glib(async),
            )
        }
    }
}

declare_concrete_event!(SinkMessage);
impl<'a> SinkMessage<'a> {
    pub fn get_message(&self) -> ::Message {
        unsafe {
            let mut msg = ptr::null_mut();

            ffi::gst_event_parse_sink_message(self.as_mut_ptr(), &mut msg);
            from_glib_full(msg)
        }
    }
}

declare_concrete_event!(StreamGroupDone);
impl<'a> StreamGroupDone<'a> {
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_group_id(&self) -> GroupId {
        unsafe {
            let mut group_id = mem::uninitialized();

            ffi::gst_event_parse_stream_group_done(self.as_mut_ptr(), &mut group_id);

            from_glib(group_id)
        }
    }
}

declare_concrete_event!(Eos);

declare_concrete_event!(Toc);
impl<'a> Toc<'a> {
    pub fn get_toc(&self) -> (&'a ::TocRef, bool) {
        unsafe {
            let mut toc = ptr::null_mut();
            let mut updated = mem::uninitialized();

            ffi::gst_event_parse_toc(self.as_mut_ptr(), &mut toc, &mut updated);
            (::TocRef::from_ptr(toc), from_glib(updated))
        }
    }
}

declare_concrete_event!(Protection);
impl<'a> Protection<'a> {
    pub fn get(&self) -> (&'a str, &'a ::BufferRef, Option<&'a str>) {
        unsafe {
            let mut system_id = ptr::null();
            let mut buffer = ptr::null_mut();
            let mut origin = ptr::null();

            ffi::gst_event_parse_protection(
                self.as_mut_ptr(),
                &mut system_id,
                &mut buffer,
                &mut origin,
            );

            (
                CStr::from_ptr(system_id).to_str().unwrap(),
                ::BufferRef::from_ptr(buffer),
                if origin.is_null() {
                    None
                } else {
                    Some(CStr::from_ptr(origin).to_str().unwrap())
                },
            )
        }
    }
}

declare_concrete_event!(SegmentDone);
impl<'a> SegmentDone<'a> {
    pub fn get(&self) -> GenericFormattedValue {
        unsafe {
            let mut fmt = mem::uninitialized();
            let mut position = mem::uninitialized();

            ffi::gst_event_parse_segment_done(self.as_mut_ptr(), &mut fmt, &mut position);

            GenericFormattedValue::new(from_glib(fmt), position)
        }
    }
}

declare_concrete_event!(Gap);
impl<'a> Gap<'a> {
    pub fn get(&self) -> (::ClockTime, ::ClockTime) {
        unsafe {
            let mut timestamp = mem::uninitialized();
            let mut duration = mem::uninitialized();

            ffi::gst_event_parse_gap(self.as_mut_ptr(), &mut timestamp, &mut duration);

            (from_glib(timestamp), from_glib(duration))
        }
    }
}

declare_concrete_event!(Qos);
impl<'a> Qos<'a> {
    pub fn get(&self) -> (::QOSType, f64, i64, ::ClockTime) {
        unsafe {
            let mut type_ = mem::uninitialized();
            let mut proportion = mem::uninitialized();
            let mut diff = mem::uninitialized();
            let mut timestamp = mem::uninitialized();

            ffi::gst_event_parse_qos(
                self.as_mut_ptr(),
                &mut type_,
                &mut proportion,
                &mut diff,
                &mut timestamp,
            );

            (from_glib(type_), proportion, diff, from_glib(timestamp))
        }
    }
}

declare_concrete_event!(Seek);
impl<'a> Seek<'a> {
    pub fn get(
        &self,
    ) -> (
        f64,
        ::SeekFlags,
        ::SeekType,
        GenericFormattedValue,
        ::SeekType,
        GenericFormattedValue,
    ) {
        unsafe {
            let mut rate = mem::uninitialized();
            let mut fmt = mem::uninitialized();
            let mut flags = mem::uninitialized();
            let mut start_type = mem::uninitialized();
            let mut start = mem::uninitialized();
            let mut stop_type = mem::uninitialized();
            let mut stop = mem::uninitialized();

            ffi::gst_event_parse_seek(
                self.as_mut_ptr(),
                &mut rate,
                &mut fmt,
                &mut flags,
                &mut start_type,
                &mut start,
                &mut stop_type,
                &mut stop,
            );

            (
                rate,
                from_glib(flags),
                from_glib(start_type),
                GenericFormattedValue::new(from_glib(fmt), start),
                from_glib(stop_type),
                GenericFormattedValue::new(from_glib(fmt), stop),
            )
        }
    }
}

declare_concrete_event!(Navigation);

declare_concrete_event!(Latency);
impl<'a> Latency<'a> {
    pub fn get_latency(&self) -> ::ClockTime {
        unsafe {
            let mut latency = mem::uninitialized();

            ffi::gst_event_parse_latency(self.as_mut_ptr(), &mut latency);

            from_glib(latency)
        }
    }
}

declare_concrete_event!(Step);
impl<'a> Step<'a> {
    pub fn get(&self) -> (GenericFormattedValue, f64, bool, bool) {
        unsafe {
            let mut fmt = mem::uninitialized();
            let mut amount = mem::uninitialized();
            let mut rate = mem::uninitialized();
            let mut flush = mem::uninitialized();
            let mut intermediate = mem::uninitialized();

            ffi::gst_event_parse_step(
                self.as_mut_ptr(),
                &mut fmt,
                &mut amount,
                &mut rate,
                &mut flush,
                &mut intermediate,
            );

            (
                GenericFormattedValue::new(from_glib(fmt), amount as i64),
                rate,
                from_glib(flush),
                from_glib(intermediate),
            )
        }
    }
}

declare_concrete_event!(Reconfigure);

declare_concrete_event!(TocSelect);
impl<'a> TocSelect<'a> {
    pub fn get_uid(&self) -> &'a str {
        unsafe {
            let mut uid = ptr::null_mut();

            ffi::gst_event_parse_toc_select(self.as_mut_ptr(), &mut uid);

            CStr::from_ptr(uid).to_str().unwrap()
        }
    }
}

declare_concrete_event!(SelectStreams);
impl<'a> SelectStreams<'a> {
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_streams(&self) -> Vec<String> {
        unsafe {
            let mut streams = ptr::null_mut();

            ffi::gst_event_parse_select_streams(self.as_mut_ptr(), &mut streams);

            FromGlibPtrContainer::from_glib_full(streams)
        }
    }
}

declare_concrete_event!(CustomUpstream);
declare_concrete_event!(CustomDownstream);
declare_concrete_event!(CustomDownstreamOob);
declare_concrete_event!(CustomDownstreamSticky);
declare_concrete_event!(CustomBoth);
declare_concrete_event!(CustomBothOob);

macro_rules! event_builder_generic_impl {
    ($new_fn:expr) => {
        pub fn seqnum(self, seqnum: Seqnum) -> Self {
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

        pub fn build(mut self) -> Event {
            assert_initialized_main_thread!();
            unsafe {
                let event = $new_fn(&mut self);
                if let Some(seqnum) = self.seqnum {
                    ffi::gst_event_set_seqnum(event, seqnum.to_glib());
                }

                if let Some(running_time_offset) = self.running_time_offset {
                    ffi::gst_event_set_running_time_offset(event, running_time_offset);
                }

                if !self.other_fields.is_empty() {
                    let s = StructureRef::from_glib_borrow_mut(
                        ffi::gst_event_writable_structure(event)
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

pub struct FlushStartBuilder<'a> {
    seqnum: Option<Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
}
impl<'a> FlushStartBuilder<'a> {
    fn new() -> Self {
        skip_assert_initialized!();
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
        }
    }

    event_builder_generic_impl!(|_| ffi::gst_event_new_flush_start());
}

pub struct FlushStopBuilder<'a> {
    seqnum: Option<Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    reset_time: bool,
}
impl<'a> FlushStopBuilder<'a> {
    fn new(reset_time: bool) -> Self {
        skip_assert_initialized!();
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
            reset_time: reset_time,
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_flush_stop(s.reset_time.to_glib()));
}

pub struct StreamStartBuilder<'a> {
    seqnum: Option<Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    stream_id: &'a str,
    flags: Option<::StreamFlags>,
    group_id: Option<GroupId>,
}
impl<'a> StreamStartBuilder<'a> {
    fn new(stream_id: &'a str) -> Self {
        skip_assert_initialized!();
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
            stream_id: stream_id,
            flags: None,
            group_id: None,
        }
    }

    pub fn flags(self, flags: ::StreamFlags) -> Self {
        Self {
            flags: Some(flags),
            ..self
        }
    }

    pub fn group_id(self, group_id: GroupId) -> Self {
        Self {
            group_id: Some(group_id),
            ..self
        }
    }

    event_builder_generic_impl!(|s: &Self| {
        let ev = ffi::gst_event_new_stream_start(s.stream_id.to_glib_none().0);
        if let Some(flags) = s.flags {
            ffi::gst_event_set_stream_flags(ev, flags.to_glib());
        }
        if let Some(group_id) = s.group_id {
            ffi::gst_event_set_group_id(ev, group_id.to_glib());
        }
        ev
    });
}

pub struct CapsBuilder<'a> {
    seqnum: Option<Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    caps: &'a ::Caps,
}
impl<'a> CapsBuilder<'a> {
    fn new(caps: &'a ::Caps) -> Self {
        skip_assert_initialized!();
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
            caps: caps,
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_caps(s.caps.as_mut_ptr()));
}

pub struct SegmentBuilder<'a> {
    seqnum: Option<Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    segment: &'a ::Segment,
}
impl<'a> SegmentBuilder<'a> {
    fn new(segment: &'a ::Segment) -> Self {
        skip_assert_initialized!();
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
            segment: segment,
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_segment(s.segment.to_glib_none().0));
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
pub struct StreamCollectionBuilder<'a> {
    seqnum: Option<Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    stream_collection: &'a ::StreamCollection,
}
#[cfg(any(feature = "v1_10", feature = "dox"))]
impl<'a> StreamCollectionBuilder<'a> {
    fn new(stream_collection: &'a ::StreamCollection) -> Self {
        skip_assert_initialized!();
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
            stream_collection: stream_collection,
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_stream_collection(
        s.stream_collection.to_glib_none().0
    ));
}

pub struct TagBuilder<'a> {
    seqnum: Option<Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    tags: Option<::TagList>,
}
impl<'a> TagBuilder<'a> {
    fn new(tags: ::TagList) -> Self {
        skip_assert_initialized!();
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
            tags: Some(tags),
        }
    }

    event_builder_generic_impl!(|s: &mut Self| {
        let tags = s.tags.take().unwrap();
        ffi::gst_event_new_tag(tags.into_ptr())
    });
}

pub struct BufferSizeBuilder<'a> {
    seqnum: Option<Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    minsize: GenericFormattedValue,
    maxsize: GenericFormattedValue,
    async: bool,
}
impl<'a> BufferSizeBuilder<'a> {
    fn new(minsize: GenericFormattedValue, maxsize: GenericFormattedValue, async: bool) -> Self {
        skip_assert_initialized!();
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
            minsize: minsize,
            maxsize: maxsize,
            async: async,
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_buffer_size(
        s.minsize.get_format().to_glib(),
        s.minsize.get_value(),
        s.maxsize.get_value(),
        s.async.to_glib(),
    ));
}

pub struct SinkMessageBuilder<'a> {
    seqnum: Option<Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    name: &'a str,
    msg: &'a ::Message,
}
impl<'a> SinkMessageBuilder<'a> {
    fn new(name: &'a str, msg: &'a ::Message) -> Self {
        skip_assert_initialized!();
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
            name: name,
            msg: msg,
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_sink_message(
        s.name.to_glib_none().0,
        s.msg.as_mut_ptr()
    ));
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
pub struct StreamGroupDoneBuilder<'a> {
    seqnum: Option<Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    group_id: GroupId,
}
#[cfg(any(feature = "v1_10", feature = "dox"))]
impl<'a> StreamGroupDoneBuilder<'a> {
    fn new(group_id: GroupId) -> Self {
        skip_assert_initialized!();
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
            group_id: group_id,
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_stream_group_done(
        s.group_id.to_glib()
    ));
}

pub struct EosBuilder<'a> {
    seqnum: Option<Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
}
impl<'a> EosBuilder<'a> {
    fn new() -> Self {
        skip_assert_initialized!();
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
        }
    }

    event_builder_generic_impl!(|_| ffi::gst_event_new_eos());
}

pub struct TocBuilder<'a> {
    seqnum: Option<Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    toc: &'a ::Toc,
    updated: bool,
}
impl<'a> TocBuilder<'a> {
    fn new(toc: &'a ::Toc, updated: bool) -> Self {
        skip_assert_initialized!();
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
            toc: toc,
            updated: updated,
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_toc(
        s.toc.to_glib_none().0,
        s.updated.to_glib()
    ));
}

pub struct ProtectionBuilder<'a> {
    seqnum: Option<Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    system_id: &'a str,
    data: &'a ::Buffer,
    origin: Option<&'a str>,
}
impl<'a> ProtectionBuilder<'a> {
    fn new(system_id: &'a str, data: &'a ::Buffer) -> Self {
        skip_assert_initialized!();
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
            system_id: system_id,
            data: data,
            origin: None,
        }
    }

    pub fn origin(self, origin: &'a str) -> Self {
        Self {
            origin: Some(origin),
            ..self
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_protection(
        s.system_id.to_glib_none().0,
        s.data.as_mut_ptr(),
        s.origin.to_glib_none().0,
    ));
}

pub struct SegmentDoneBuilder<'a> {
    seqnum: Option<Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    position: GenericFormattedValue,
}
impl<'a> SegmentDoneBuilder<'a> {
    fn new(position: GenericFormattedValue) -> Self {
        skip_assert_initialized!();
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
            position: position,
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_segment_done(
        s.position.get_format().to_glib(),
        s.position.get_value()
    ));
}

pub struct GapBuilder<'a> {
    seqnum: Option<Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    timestamp: ::ClockTime,
    duration: ::ClockTime,
}
impl<'a> GapBuilder<'a> {
    fn new(timestamp: ::ClockTime, duration: ::ClockTime) -> Self {
        skip_assert_initialized!();
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
            timestamp: timestamp,
            duration: duration,
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_gap(
        s.timestamp.to_glib(),
        s.duration.to_glib()
    ));
}

pub struct QosBuilder<'a> {
    seqnum: Option<Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    type_: ::QOSType,
    proportion: f64,
    diff: i64,
    timestamp: ::ClockTime,
}
impl<'a> QosBuilder<'a> {
    fn new(type_: ::QOSType, proportion: f64, diff: i64, timestamp: ::ClockTime) -> Self {
        skip_assert_initialized!();
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
            type_: type_,
            proportion: proportion,
            diff: diff,
            timestamp: timestamp,
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_qos(
        s.type_.to_glib(),
        s.proportion,
        s.diff,
        s.timestamp.to_glib(),
    ));
}

pub struct SeekBuilder<'a> {
    seqnum: Option<Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    rate: f64,
    flags: ::SeekFlags,
    start_type: ::SeekType,
    start: GenericFormattedValue,
    stop_type: ::SeekType,
    stop: GenericFormattedValue,
}
impl<'a> SeekBuilder<'a> {
    fn new(
        rate: f64,
        flags: ::SeekFlags,
        start_type: ::SeekType,
        start: GenericFormattedValue,
        stop_type: ::SeekType,
        stop: GenericFormattedValue,
    ) -> Self {
        skip_assert_initialized!();
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
            rate: rate,
            flags: flags,
            start_type,
            start,
            stop_type,
            stop,
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_seek(
        s.rate,
        s.start.get_format().to_glib(),
        s.flags.to_glib(),
        s.start_type.to_glib(),
        s.start.get_value(),
        s.stop_type.to_glib(),
        s.stop.get_value(),
    ));
}

pub struct NavigationBuilder<'a> {
    seqnum: Option<Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    structure: Option<Structure>,
}
impl<'a> NavigationBuilder<'a> {
    fn new(structure: Structure) -> Self {
        skip_assert_initialized!();
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
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

pub struct LatencyBuilder<'a> {
    seqnum: Option<Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    latency: ::ClockTime,
}
impl<'a> LatencyBuilder<'a> {
    fn new(latency: ::ClockTime) -> Self {
        skip_assert_initialized!();
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
            latency: latency,
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_latency(s.latency.to_glib()));
}

pub struct StepBuilder<'a> {
    seqnum: Option<Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    amount: GenericFormattedValue,
    rate: f64,
    flush: bool,
    intermediate: bool,
}
impl<'a> StepBuilder<'a> {
    fn new(amount: GenericFormattedValue, rate: f64, flush: bool, intermediate: bool) -> Self {
        skip_assert_initialized!();
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
            amount: amount,
            rate: rate,
            flush: flush,
            intermediate: intermediate,
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_step(
        s.amount.get_format().to_glib(),
        s.amount.get_value() as u64,
        s.rate,
        s.flush.to_glib(),
        s.intermediate.to_glib(),
    ));
}

pub struct ReconfigureBuilder<'a> {
    seqnum: Option<Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
}
impl<'a> ReconfigureBuilder<'a> {
    fn new() -> Self {
        skip_assert_initialized!();
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
        }
    }

    event_builder_generic_impl!(|_| ffi::gst_event_new_reconfigure());
}

pub struct TocSelectBuilder<'a> {
    seqnum: Option<Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    uid: &'a str,
}
impl<'a> TocSelectBuilder<'a> {
    fn new(uid: &'a str) -> Self {
        skip_assert_initialized!();
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
            uid: uid,
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_toc_select(s.uid.to_glib_none().0));
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
pub struct SelectStreamsBuilder<'a> {
    seqnum: Option<Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    streams: &'a [&'a str],
}
#[cfg(any(feature = "v1_10", feature = "dox"))]
impl<'a> SelectStreamsBuilder<'a> {
    fn new(streams: &'a [&'a str]) -> Self {
        skip_assert_initialized!();
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
            streams: streams,
        }
    }

    event_builder_generic_impl!(|s: &Self| ffi::gst_event_new_select_streams(
        s.streams.to_glib_full()
    ));
}

pub struct CustomUpstreamBuilder<'a> {
    seqnum: Option<Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    structure: Option<Structure>,
}
impl<'a> CustomUpstreamBuilder<'a> {
    fn new(structure: Structure) -> Self {
        skip_assert_initialized!();
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
            structure: Some(structure),
        }
    }

    event_builder_generic_impl!(|s: &mut Self| {
        let structure = s.structure.take();
        let ev =
            ffi::gst_event_new_custom(ffi::GST_EVENT_CUSTOM_UPSTREAM, structure.to_glib_none().0);
        mem::forget(structure);

        ev
    });
}

pub struct CustomDownstreamBuilder<'a> {
    seqnum: Option<Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    structure: Option<Structure>,
}
impl<'a> CustomDownstreamBuilder<'a> {
    fn new(structure: Structure) -> Self {
        skip_assert_initialized!();
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
            structure: Some(structure),
        }
    }

    event_builder_generic_impl!(|s: &mut Self| {
        let structure = s.structure.take();
        let ev =
            ffi::gst_event_new_custom(ffi::GST_EVENT_CUSTOM_DOWNSTREAM, structure.to_glib_none().0);
        mem::forget(structure);

        ev
    });
}

pub struct CustomDownstreamOobBuilder<'a> {
    seqnum: Option<Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    structure: Option<Structure>,
}
impl<'a> CustomDownstreamOobBuilder<'a> {
    fn new(structure: Structure) -> Self {
        skip_assert_initialized!();
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
            structure: Some(structure),
        }
    }

    event_builder_generic_impl!(|s: &mut Self| {
        let structure = s.structure.take();
        let ev = ffi::gst_event_new_custom(
            ffi::GST_EVENT_CUSTOM_DOWNSTREAM_OOB,
            structure.to_glib_none().0,
        );
        mem::forget(structure);

        ev
    });
}

pub struct CustomDownstreamStickyBuilder<'a> {
    seqnum: Option<Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    structure: Option<Structure>,
}
impl<'a> CustomDownstreamStickyBuilder<'a> {
    fn new(structure: Structure) -> Self {
        skip_assert_initialized!();
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
            structure: Some(structure),
        }
    }

    event_builder_generic_impl!(|s: &mut Self| {
        let structure = s.structure.take();
        let ev = ffi::gst_event_new_custom(
            ffi::GST_EVENT_CUSTOM_DOWNSTREAM_STICKY,
            structure.to_glib_none().0,
        );
        mem::forget(structure);

        ev
    });
}

pub struct CustomBothBuilder<'a> {
    seqnum: Option<Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    structure: Option<Structure>,
}
impl<'a> CustomBothBuilder<'a> {
    fn new(structure: Structure) -> Self {
        skip_assert_initialized!();
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
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

pub struct CustomBothOobBuilder<'a> {
    seqnum: Option<Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    structure: Option<Structure>,
}
impl<'a> CustomBothOobBuilder<'a> {
    fn new(structure: Structure) -> Self {
        skip_assert_initialized!();
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
            structure: Some(structure),
        }
    }

    event_builder_generic_impl!(|s: &mut Self| {
        let structure = s.structure.take();
        let ev =
            ffi::gst_event_new_custom(ffi::GST_EVENT_CUSTOM_BOTH_OOB, structure.to_glib_none().0);
        mem::forget(structure);

        ev
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        ::init().unwrap();

        // Event without arguments
        let flush_start_evt = Event::new_flush_start().build();
        match flush_start_evt.view() {
            EventView::FlushStart(flush_start_evt) => {
                assert!(!flush_start_evt.is_sticky());
                assert!(flush_start_evt.get_structure().is_none());
            },
            _ => panic!("flush_start_evt.view() is not an EventView::FlushStart(_)"),
        }

        let flush_start_evt = Event::new_flush_start()
            .other_fields(&[("extra-field", &true)])
            .build();
        match flush_start_evt.view() {
            EventView::FlushStart(flush_start_evt) => {
                assert!(flush_start_evt.get_structure().is_some());
                if let Some(other_fields) = flush_start_evt.get_structure() {
                    assert!(other_fields.has_field("extra-field"));
                }
            },
            _ => panic!("flush_start_evt.view() is not an EventView::FlushStart(_)"),
        }

        // Event with arguments
        let flush_stop_evt = Event::new_flush_stop(true)
            .other_fields(&[("extra-field", &true)])
            .build();
        match flush_stop_evt.view() {
            EventView::FlushStop(flush_stop_evt) => {
                assert_eq!(flush_stop_evt.get_reset_time(), true);
                assert!(flush_stop_evt.get_structure().is_some());
                if let Some(other_fields) = flush_stop_evt.get_structure() {
                    assert!(other_fields.has_field("extra-field"));
                }
            }
            _ => panic!("flush_stop_evt.view() is not an EventView::FlushStop(_)"),
        }
    }
}
