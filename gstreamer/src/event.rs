// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use gst_sys;
use structure::*;
use GenericFormattedValue;

use std::cmp;
use std::ffi::CStr;
use std::fmt;
use std::mem;
use std::num::NonZeroU32;
use std::ops::Deref;
use std::ptr;

use glib;
use glib::translate::{from_glib, from_glib_full, from_glib_none, ToGlib, ToGlibPtr};
use glib::value::ToSendValue;

#[cfg(any(feature = "v1_10", feature = "dox"))]
use glib::translate::FromGlibPtrContainer;

use EventType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Seqnum(pub(crate) NonZeroU32);

impl Seqnum {
    pub fn next() -> Self {
        unsafe {
            let v = gst_sys::gst_util_seqnum_next();
            if v == 0 {
                Seqnum::next()
            } else {
                Seqnum(NonZeroU32::new_unchecked(v))
            }
        }
    }
}

impl ToGlib for Seqnum {
    type GlibType = u32;

    fn to_glib(&self) -> u32 {
        self.0.get()
    }
}

impl cmp::PartialOrd for Seqnum {
    fn partial_cmp(&self, other: &Seqnum) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for Seqnum {
    fn cmp(&self, other: &Seqnum) -> cmp::Ordering {
        unsafe {
            let ret = gst_sys::gst_util_seqnum_compare(self.0.get(), other.0.get());
            ret.cmp(&0)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GroupId(pub(crate) NonZeroU32);

impl GroupId {
    pub fn next() -> Self {
        unsafe {
            let v = gst_sys::gst_util_group_id_next();
            if v == 0 {
                GroupId::next()
            } else {
                GroupId(NonZeroU32::new_unchecked(v))
            }
        }
    }
}

impl EventType {
    pub fn is_upstream(self) -> bool {
        (self.to_glib() as u32) & gst_sys::GST_EVENT_TYPE_UPSTREAM != 0
    }

    pub fn is_downstream(self) -> bool {
        (self.to_glib() as u32) & gst_sys::GST_EVENT_TYPE_DOWNSTREAM != 0
    }

    pub fn is_serialized(self) -> bool {
        (self.to_glib() as u32) & gst_sys::GST_EVENT_TYPE_SERIALIZED != 0
    }

    pub fn is_sticky(self) -> bool {
        (self.to_glib() as u32) & gst_sys::GST_EVENT_TYPE_STICKY != 0
    }

    pub fn is_sticky_multi(self) -> bool {
        (self.to_glib() as u32) & gst_sys::GST_EVENT_TYPE_STICKY_MULTI != 0
    }
}

impl PartialOrd for EventType {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        if !self.is_serialized() || !other.is_serialized() {
            return None;
        }

        let v1 = self.to_glib() as u32;
        let v2 = other.to_glib() as u32;

        let stream_start = gst_sys::GST_EVENT_STREAM_START as u32;
        let segment = gst_sys::GST_EVENT_SEGMENT as u32;
        let eos = gst_sys::GST_EVENT_EOS as u32;

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

gst_define_mini_object_wrapper!(Event, EventRef, gst_sys::GstEvent, || {
    gst_sys::gst_event_get_type()
});

impl EventRef {
    pub fn get_seqnum(&self) -> Seqnum {
        unsafe {
            let seqnum = gst_sys::gst_event_get_seqnum(self.as_mut_ptr());
            assert_ne!(seqnum, 0);
            Seqnum(NonZeroU32::new_unchecked(seqnum))
        }
    }

    pub fn get_running_time_offset(&self) -> i64 {
        unsafe { gst_sys::gst_event_get_running_time_offset(self.as_mut_ptr()) }
    }

    pub fn set_running_time_offset(&mut self, offset: i64) {
        unsafe { gst_sys::gst_event_set_running_time_offset(self.as_mut_ptr(), offset) }
    }

    pub fn get_structure(&self) -> Option<&StructureRef> {
        unsafe {
            let structure = gst_sys::gst_event_get_structure(self.as_mut_ptr());
            if structure.is_null() {
                None
            } else {
                Some(StructureRef::from_glib_borrow(structure))
            }
        }
    }

    pub fn structure_mut(&mut self) -> &mut StructureRef {
        unsafe {
            StructureRef::from_glib_borrow_mut(gst_sys::gst_event_writable_structure(
                self.as_mut_ptr(),
            ))
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
            gst_sys::GST_EVENT_FLUSH_START => EventView::FlushStart(FlushStart(self)),
            gst_sys::GST_EVENT_FLUSH_STOP => EventView::FlushStop(FlushStop(self)),
            gst_sys::GST_EVENT_STREAM_START => EventView::StreamStart(StreamStart(self)),
            gst_sys::GST_EVENT_CAPS => EventView::Caps(Caps(self)),
            gst_sys::GST_EVENT_SEGMENT => EventView::Segment(Segment(self)),
            gst_sys::GST_EVENT_STREAM_COLLECTION => {
                EventView::StreamCollection(StreamCollection(self))
            }
            gst_sys::GST_EVENT_TAG => EventView::Tag(Tag(self)),
            gst_sys::GST_EVENT_BUFFERSIZE => EventView::BufferSize(BufferSize(self)),
            gst_sys::GST_EVENT_SINK_MESSAGE => EventView::SinkMessage(SinkMessage(self)),
            gst_sys::GST_EVENT_STREAM_GROUP_DONE => {
                EventView::StreamGroupDone(StreamGroupDone(self))
            }
            gst_sys::GST_EVENT_EOS => EventView::Eos(Eos(self)),
            gst_sys::GST_EVENT_TOC => EventView::Toc(Toc(self)),
            gst_sys::GST_EVENT_PROTECTION => EventView::Protection(Protection(self)),
            gst_sys::GST_EVENT_SEGMENT_DONE => EventView::SegmentDone(SegmentDone(self)),
            gst_sys::GST_EVENT_GAP => EventView::Gap(Gap(self)),
            gst_sys::GST_EVENT_QOS => EventView::Qos(Qos(self)),
            gst_sys::GST_EVENT_SEEK => EventView::Seek(Seek(self)),
            gst_sys::GST_EVENT_NAVIGATION => EventView::Navigation(Navigation(self)),
            gst_sys::GST_EVENT_LATENCY => EventView::Latency(Latency(self)),
            gst_sys::GST_EVENT_STEP => EventView::Step(Step(self)),
            gst_sys::GST_EVENT_RECONFIGURE => EventView::Reconfigure(Reconfigure(self)),
            gst_sys::GST_EVENT_TOC_SELECT => EventView::TocSelect(TocSelect(self)),
            gst_sys::GST_EVENT_SELECT_STREAMS => EventView::SelectStreams(SelectStreams(self)),
            gst_sys::GST_EVENT_CUSTOM_UPSTREAM => EventView::CustomUpstream(CustomUpstream(self)),
            gst_sys::GST_EVENT_CUSTOM_DOWNSTREAM => {
                EventView::CustomDownstream(CustomDownstream(self))
            }
            gst_sys::GST_EVENT_CUSTOM_DOWNSTREAM_OOB => {
                EventView::CustomDownstreamOob(CustomDownstreamOob(self))
            }
            gst_sys::GST_EVENT_CUSTOM_DOWNSTREAM_STICKY => {
                EventView::CustomDownstreamSticky(CustomDownstreamSticky(self))
            }
            gst_sys::GST_EVENT_CUSTOM_BOTH => EventView::CustomBoth(CustomBoth(self)),
            gst_sys::GST_EVENT_CUSTOM_BOTH_OOB => EventView::CustomBothOob(CustomBothOob(self)),
            _ => EventView::Other,
        }
    }
}

impl Event {
    #[deprecated(
        since = "0.16.0",
        note = "use `event::FlushStart::new` or `event::FlushStart::builder` instead"
    )]
    pub fn new_flush_start<'a>() -> FlushStartBuilder<'a> {
        assert_initialized_main_thread!();
        FlushStartBuilder::new()
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `event::FlushStop::new` or `event::FlushStop::builder` instead"
    )]
    pub fn new_flush_stop<'a>(reset_time: bool) -> FlushStopBuilder<'a> {
        assert_initialized_main_thread!();
        FlushStopBuilder::new(reset_time)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `event::StreamStart::new` or `event::StreamStart::builder` instead"
    )]
    pub fn new_stream_start(stream_id: &str) -> StreamStartBuilder {
        assert_initialized_main_thread!();
        StreamStartBuilder::new(stream_id)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `event::Caps::new` or `event::Caps::builder` instead"
    )]
    pub fn new_caps(caps: &::Caps) -> CapsBuilder {
        assert_initialized_main_thread!();
        CapsBuilder::new(caps)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `event::Segment::new` or `event::Segment::builder` instead"
    )]
    pub fn new_segment<F: ::FormattedValue>(segment: &::FormattedSegment<F>) -> SegmentBuilder {
        assert_initialized_main_thread!();
        SegmentBuilder::new(segment.as_ref())
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[deprecated(
        since = "0.16.0",
        note = "use `event::StreamCollection::new` or `event::StreamCollection::builder` instead"
    )]
    pub fn new_stream_collection(
        stream_collection: &::StreamCollection,
    ) -> StreamCollectionBuilder {
        assert_initialized_main_thread!();
        StreamCollectionBuilder::new(stream_collection)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `event::Tag::new` or `event::Tag::builder` instead"
    )]
    pub fn new_tag<'a>(tags: ::TagList) -> TagBuilder<'a> {
        assert_initialized_main_thread!();
        TagBuilder::new(tags)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `event::BufferSize::new` or `event::BufferSize::builder` instead"
    )]
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

    #[deprecated(
        since = "0.16.0",
        note = "use `event::SinkMessage::new` or `event::SinkMessage::builder` instead"
    )]
    pub fn new_sink_message<'a>(name: &'a str, msg: &'a ::Message) -> SinkMessageBuilder<'a> {
        assert_initialized_main_thread!();
        SinkMessageBuilder::new(name, msg)
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[deprecated(
        since = "0.16.0",
        note = "use `event::StreamGroupDone::new` or `event::StreamGroupDone::builder` instead"
    )]
    pub fn new_stream_group_done<'a>(group_id: GroupId) -> StreamGroupDoneBuilder<'a> {
        assert_initialized_main_thread!();
        StreamGroupDoneBuilder::new(group_id)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `event::Eos::new` or `event::Eos::builder` instead"
    )]
    pub fn new_eos<'a>() -> EosBuilder<'a> {
        assert_initialized_main_thread!();
        EosBuilder::new()
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `event::Toc::new` or `event::Toc::builder` instead"
    )]
    pub fn new_toc(toc: &::Toc, updated: bool) -> TocBuilder {
        assert_initialized_main_thread!();
        TocBuilder::new(toc, updated)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `event::Protection::new` or `event::Protection::builder` instead"
    )]
    pub fn new_protection<'a>(system_id: &'a str, data: &'a ::Buffer) -> ProtectionBuilder<'a> {
        assert_initialized_main_thread!();
        ProtectionBuilder::new(system_id, data)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `event::SegmentDone::new` or `event::SegmentDone::builder` instead"
    )]
    pub fn new_segment_done<'a, V: Into<GenericFormattedValue>>(
        position: V,
    ) -> SegmentDoneBuilder<'a> {
        assert_initialized_main_thread!();
        let position = position.into();
        SegmentDoneBuilder::new(position)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `event::Gap::new` or `event::Gap::builder` instead"
    )]
    pub fn new_gap<'a>(timestamp: ::ClockTime, duration: ::ClockTime) -> GapBuilder<'a> {
        assert_initialized_main_thread!();
        GapBuilder::new(timestamp, duration)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `event::Qos::new` or `event::Qos::builder` instead"
    )]
    pub fn new_qos<'a>(
        type_: ::QOSType,
        proportion: f64,
        diff: i64,
        timestamp: ::ClockTime,
    ) -> QosBuilder<'a> {
        assert_initialized_main_thread!();
        QosBuilder::new(type_, proportion, diff, timestamp)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `event::Seek::new` or `event::Seek::builder` instead"
    )]
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

    #[deprecated(
        since = "0.16.0",
        note = "use `event::Navigation::new` or `event::Navigation::builder` instead"
    )]
    pub fn new_navigation<'a>(structure: ::Structure) -> NavigationBuilder<'a> {
        assert_initialized_main_thread!();
        NavigationBuilder::new(structure)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `event::Latency::new` or `event::Latency::builder` instead"
    )]
    pub fn new_latency<'a>(latency: ::ClockTime) -> LatencyBuilder<'a> {
        assert_initialized_main_thread!();
        LatencyBuilder::new(latency)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `event::Step::new` or `event::Step::builder` instead"
    )]
    pub fn new_step<'a, V: Into<GenericFormattedValue>>(
        amount: V,
        rate: f64,
        flush: bool,
        intermediate: bool,
    ) -> StepBuilder<'a> {
        assert_initialized_main_thread!();
        StepBuilder::new(amount.into(), rate, flush, intermediate)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `event::Reconfigure::new` or `event::Reconfigure::builder` instead"
    )]
    pub fn new_reconfigure<'a>() -> ReconfigureBuilder<'a> {
        assert_initialized_main_thread!();
        ReconfigureBuilder::new()
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `event::TocSelect::new` or `event::TocSelect::builder` instead"
    )]
    pub fn new_toc_select(uid: &str) -> TocSelectBuilder {
        assert_initialized_main_thread!();
        TocSelectBuilder::new(uid)
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[deprecated(
        since = "0.16.0",
        note = "use `event::SelectStreams::new` or `event::SelectStreams::builder` instead"
    )]
    pub fn new_select_streams<'a>(streams: &'a [&'a str]) -> SelectStreamsBuilder<'a> {
        assert_initialized_main_thread!();
        SelectStreamsBuilder::new(streams)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `event::CustomUpstream::new` or `event::CustomUpstream::builder` instead"
    )]
    pub fn new_custom_upstream<'a>(structure: ::Structure) -> CustomUpstreamBuilder<'a> {
        assert_initialized_main_thread!();
        CustomUpstreamBuilder::new(structure)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `event::CustomDownstream::new` or `event::CustomDownstream::builder` instead"
    )]
    pub fn new_custom_downstream<'a>(structure: ::Structure) -> CustomDownstreamBuilder<'a> {
        assert_initialized_main_thread!();
        CustomDownstreamBuilder::new(structure)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `event::CustomDownstreamOob::new` or `event::CustomDownstreamOob::builder` instead"
    )]
    pub fn new_custom_downstream_oob<'a>(structure: ::Structure) -> CustomDownstreamOobBuilder<'a> {
        assert_initialized_main_thread!();
        CustomDownstreamOobBuilder::new(structure)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `event::CustomDownstreamSticky::new` or `event::CustomDownstreamSticky::builder` instead"
    )]
    pub fn new_custom_downstream_sticky<'a>(
        structure: ::Structure,
    ) -> CustomDownstreamStickyBuilder<'a> {
        assert_initialized_main_thread!();
        CustomDownstreamStickyBuilder::new(structure)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `event::CustomBoth::new` or `event::CustomBoth::builder` instead"
    )]
    pub fn new_custom_both<'a>(structure: ::Structure) -> CustomBothBuilder<'a> {
        assert_initialized_main_thread!();
        CustomBothBuilder::new(structure)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `event::CustomBothOob::new` or `event::CustomBothOob::builder` instead"
    )]
    pub fn new_custom_both_oob<'a>(structure: ::Structure) -> CustomBothOobBuilder<'a> {
        assert_initialized_main_thread!();
        CustomBothOobBuilder::new(structure)
    }
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        EventRef::fmt(self, f)
    }
}

impl fmt::Debug for EventRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Event")
            .field("ptr", unsafe { &self.as_ptr() })
            .field("type", &unsafe {
                let type_ = gst_sys::gst_event_type_get_name((*self.as_ptr()).type_);
                CStr::from_ptr(type_).to_str().unwrap()
            })
            .field("seqnum", &self.get_seqnum())
            .field("structure", &self.get_structure())
            .finish()
    }
}

#[derive(Debug)]
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
        #[derive(Debug)]
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
impl<'a> FlushStart<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Event {
        skip_assert_initialized!();
        Self::builder().build()
    }

    pub fn builder() -> FlushStartBuilder<'a> {
        assert_initialized_main_thread!();
        FlushStartBuilder::new()
    }
}

declare_concrete_event!(FlushStop);
impl<'a> FlushStop<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(reset_time: bool) -> Event {
        skip_assert_initialized!();
        Self::builder(reset_time).build()
    }

    pub fn builder(reset_time: bool) -> FlushStopBuilder<'a> {
        assert_initialized_main_thread!();
        FlushStopBuilder::new(reset_time)
    }

    pub fn get_reset_time(&self) -> bool {
        unsafe {
            let mut reset_time = mem::MaybeUninit::uninit();

            gst_sys::gst_event_parse_flush_stop(self.as_mut_ptr(), reset_time.as_mut_ptr());

            from_glib(reset_time.assume_init())
        }
    }
}

declare_concrete_event!(StreamStart);
impl<'a> StreamStart<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(stream_id: &str) -> Event {
        skip_assert_initialized!();
        Self::builder(stream_id).build()
    }

    pub fn builder(stream_id: &str) -> StreamStartBuilder {
        assert_initialized_main_thread!();
        StreamStartBuilder::new(stream_id)
    }

    pub fn get_stream_id(&self) -> &'a str {
        unsafe {
            let mut stream_id = ptr::null();

            gst_sys::gst_event_parse_stream_start(self.as_mut_ptr(), &mut stream_id);
            CStr::from_ptr(stream_id).to_str().unwrap()
        }
    }

    pub fn get_stream_flags(&self) -> ::StreamFlags {
        unsafe {
            let mut stream_flags = mem::MaybeUninit::uninit();

            gst_sys::gst_event_parse_stream_flags(self.as_mut_ptr(), stream_flags.as_mut_ptr());

            from_glib(stream_flags.assume_init())
        }
    }

    pub fn get_group_id(&self) -> Option<GroupId> {
        unsafe {
            let mut group_id = mem::MaybeUninit::uninit();

            gst_sys::gst_event_parse_group_id(self.as_mut_ptr(), group_id.as_mut_ptr());

            let group_id = group_id.assume_init();
            if group_id == 0 {
                None
            } else {
                Some(GroupId(NonZeroU32::new_unchecked(group_id)))
            }
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_stream(&self) -> Option<::Stream> {
        unsafe {
            let mut stream = ptr::null_mut();
            gst_sys::gst_event_parse_stream(self.as_mut_ptr(), &mut stream);
            from_glib_full(stream)
        }
    }
}

declare_concrete_event!(Caps);
impl<'a> Caps<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(caps: &::Caps) -> Event {
        skip_assert_initialized!();
        Self::builder(caps).build()
    }

    pub fn builder(caps: &::Caps) -> CapsBuilder {
        assert_initialized_main_thread!();
        CapsBuilder::new(caps)
    }

    pub fn get_caps(&self) -> &'a ::CapsRef {
        unsafe {
            let mut caps = ptr::null_mut();

            gst_sys::gst_event_parse_caps(self.as_mut_ptr(), &mut caps);
            ::CapsRef::from_ptr(caps)
        }
    }

    pub fn get_caps_owned(&self) -> ::Caps {
        unsafe { from_glib_none(self.get_caps().as_ptr()) }
    }
}

declare_concrete_event!(Segment);
impl<'a> Segment<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<F: ::FormattedValue>(segment: &::FormattedSegment<F>) -> Event {
        skip_assert_initialized!();
        Self::builder(segment).build()
    }

    pub fn builder<F: ::FormattedValue>(segment: &::FormattedSegment<F>) -> SegmentBuilder {
        assert_initialized_main_thread!();
        SegmentBuilder::new(segment.as_ref())
    }

    pub fn get_segment(&self) -> &'a ::Segment {
        unsafe {
            let mut segment = ptr::null();

            gst_sys::gst_event_parse_segment(self.as_mut_ptr(), &mut segment);
            &*(segment as *mut gst_sys::GstSegment as *mut ::Segment)
        }
    }
}

declare_concrete_event!(StreamCollection);
impl<'a> StreamCollection<'a> {
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(stream_collection: &::StreamCollection) -> Event {
        skip_assert_initialized!();
        Self::builder(stream_collection).build()
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn builder(stream_collection: &::StreamCollection) -> StreamCollectionBuilder {
        assert_initialized_main_thread!();
        StreamCollectionBuilder::new(stream_collection)
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_stream_collection(&self) -> ::StreamCollection {
        unsafe {
            let mut stream_collection = ptr::null_mut();

            gst_sys::gst_event_parse_stream_collection(self.as_mut_ptr(), &mut stream_collection);
            from_glib_full(stream_collection)
        }
    }
}

declare_concrete_event!(Tag);
impl<'a> Tag<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(tags: ::TagList) -> Event {
        skip_assert_initialized!();
        Self::builder(tags).build()
    }

    pub fn builder(tags: ::TagList) -> TagBuilder<'a> {
        assert_initialized_main_thread!();
        TagBuilder::new(tags)
    }

    pub fn get_tag(&self) -> &'a ::TagListRef {
        unsafe {
            let mut tags = ptr::null_mut();

            gst_sys::gst_event_parse_tag(self.as_mut_ptr(), &mut tags);
            ::TagListRef::from_ptr(tags)
        }
    }

    pub fn get_tag_owned(&self) -> ::TagList {
        unsafe { from_glib_none(self.get_tag().as_ptr()) }
    }
}

declare_concrete_event!(BufferSize);
impl<'a> BufferSize<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<V: Into<GenericFormattedValue>>(minsize: V, maxsize: V, async: bool) -> Event {
        skip_assert_initialized!();
        Self::builder(minsize, maxsize, async).build()
    }

    pub fn builder<V: Into<GenericFormattedValue>>(
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

    pub fn get(&self) -> (GenericFormattedValue, GenericFormattedValue, bool) {
        unsafe {
            let mut fmt = mem::MaybeUninit::uninit();
            let mut minsize = mem::MaybeUninit::uninit();
            let mut maxsize = mem::MaybeUninit::uninit();
            let mut async_ = mem::MaybeUninit::uninit();

            gst_sys::gst_event_parse_buffer_size(
                self.as_mut_ptr(),
                fmt.as_mut_ptr(),
                minsize.as_mut_ptr(),
                maxsize.as_mut_ptr(),
                async_.as_mut_ptr(),
            );
            (
                GenericFormattedValue::new(from_glib(fmt.assume_init()), minsize.assume_init()),
                GenericFormattedValue::new(from_glib(fmt.assume_init()), maxsize.assume_init()),
                from_glib(async_.assume_init()),
            )
        }
    }
}

declare_concrete_event!(SinkMessage);
impl<'a> SinkMessage<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(name: &'a str, msg: &'a ::Message) -> Event {
        skip_assert_initialized!();
        Self::builder(name, msg).build()
    }

    pub fn builder(name: &'a str, msg: &'a ::Message) -> SinkMessageBuilder<'a> {
        assert_initialized_main_thread!();
        SinkMessageBuilder::new(name, msg)
    }

    pub fn get_message(&self) -> ::Message {
        unsafe {
            let mut msg = ptr::null_mut();

            gst_sys::gst_event_parse_sink_message(self.as_mut_ptr(), &mut msg);
            from_glib_full(msg)
        }
    }
}

declare_concrete_event!(StreamGroupDone);
impl<'a> StreamGroupDone<'a> {
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(group_id: GroupId) -> Event {
        skip_assert_initialized!();
        Self::builder(group_id).build()
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn builder(group_id: GroupId) -> StreamGroupDoneBuilder<'a> {
        assert_initialized_main_thread!();
        StreamGroupDoneBuilder::new(group_id)
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_group_id(&self) -> GroupId {
        unsafe {
            let mut group_id = mem::MaybeUninit::uninit();

            gst_sys::gst_event_parse_stream_group_done(self.as_mut_ptr(), group_id.as_mut_ptr());

            let group_id = group_id.assume_init();
            assert_ne!(group_id, 0);
            GroupId(NonZeroU32::new_unchecked(group_id))
        }
    }
}

declare_concrete_event!(Eos);
impl<'a> Eos<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Event {
        skip_assert_initialized!();
        Self::builder().build()
    }

    pub fn builder() -> EosBuilder<'a> {
        assert_initialized_main_thread!();
        EosBuilder::new()
    }
}

declare_concrete_event!(Toc);
impl<'a> Toc<'a> {
    // FIXME could use false for updated as default
    // Even better: use an enum for updated so that it is more explicit than true / false
    #[allow(clippy::new_ret_no_self)]
    pub fn new(toc: &::Toc, updated: bool) -> Event {
        skip_assert_initialized!();
        Self::builder(toc, updated).build()
    }

    pub fn builder(toc: &::Toc, updated: bool) -> TocBuilder {
        assert_initialized_main_thread!();
        TocBuilder::new(toc, updated)
    }

    pub fn get_toc(&self) -> (&'a ::TocRef, bool) {
        unsafe {
            let mut toc = ptr::null_mut();
            let mut updated = mem::MaybeUninit::uninit();

            gst_sys::gst_event_parse_toc(self.as_mut_ptr(), &mut toc, updated.as_mut_ptr());
            (::TocRef::from_ptr(toc), from_glib(updated.assume_init()))
        }
    }

    pub fn get_toc_owned(&self) -> (::Toc, bool) {
        unsafe {
            let (toc, updated) = self.get_toc();
            (from_glib_none(toc.as_ptr()), updated)
        }
    }
}

declare_concrete_event!(Protection);
impl<'a> Protection<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(system_id: &'a str, data: &'a ::Buffer) -> Event {
        skip_assert_initialized!();
        Self::builder(system_id, data).build()
    }

    pub fn builder(system_id: &'a str, data: &'a ::Buffer) -> ProtectionBuilder<'a> {
        assert_initialized_main_thread!();
        ProtectionBuilder::new(system_id, data)
    }

    pub fn get(&self) -> (&'a str, &'a ::BufferRef, Option<&'a str>) {
        unsafe {
            let mut system_id = ptr::null();
            let mut buffer = ptr::null_mut();
            let mut origin = ptr::null();

            gst_sys::gst_event_parse_protection(
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

    pub fn get_owned(&self) -> (&'a str, ::Buffer, Option<&'a str>) {
        unsafe {
            let (system_id, buffer, origin) = self.get();
            (system_id, from_glib_none(buffer.as_ptr()), origin)
        }
    }
}

declare_concrete_event!(SegmentDone);
impl<'a> SegmentDone<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<V: Into<GenericFormattedValue>>(position: V) -> Event {
        skip_assert_initialized!();
        Self::builder(position).build()
    }

    pub fn builder<V: Into<GenericFormattedValue>>(position: V) -> SegmentDoneBuilder<'a> {
        assert_initialized_main_thread!();
        let position = position.into();
        SegmentDoneBuilder::new(position)
    }

    pub fn get(&self) -> GenericFormattedValue {
        unsafe {
            let mut fmt = mem::MaybeUninit::uninit();
            let mut position = mem::MaybeUninit::uninit();

            gst_sys::gst_event_parse_segment_done(
                self.as_mut_ptr(),
                fmt.as_mut_ptr(),
                position.as_mut_ptr(),
            );

            GenericFormattedValue::new(from_glib(fmt.assume_init()), position.assume_init())
        }
    }
}

declare_concrete_event!(Gap);
impl<'a> Gap<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(timestamp: ::ClockTime, duration: ::ClockTime) -> Event {
        skip_assert_initialized!();
        Self::builder(timestamp, duration).build()
    }

    pub fn builder(timestamp: ::ClockTime, duration: ::ClockTime) -> GapBuilder<'a> {
        assert_initialized_main_thread!();
        GapBuilder::new(timestamp, duration)
    }

    pub fn get(&self) -> (::ClockTime, ::ClockTime) {
        unsafe {
            let mut timestamp = mem::MaybeUninit::uninit();
            let mut duration = mem::MaybeUninit::uninit();

            gst_sys::gst_event_parse_gap(
                self.as_mut_ptr(),
                timestamp.as_mut_ptr(),
                duration.as_mut_ptr(),
            );

            (
                from_glib(timestamp.assume_init()),
                from_glib(duration.assume_init()),
            )
        }
    }
}

declare_concrete_event!(Qos);
impl<'a> Qos<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(type_: ::QOSType, proportion: f64, diff: i64, timestamp: ::ClockTime) -> Event {
        skip_assert_initialized!();
        Self::builder(type_, proportion, diff, timestamp).build()
    }

    pub fn builder(
        type_: ::QOSType,
        proportion: f64,
        diff: i64,
        timestamp: ::ClockTime,
    ) -> QosBuilder<'a> {
        assert_initialized_main_thread!();
        QosBuilder::new(type_, proportion, diff, timestamp)
    }

    pub fn get(&self) -> (::QOSType, f64, i64, ::ClockTime) {
        unsafe {
            let mut type_ = mem::MaybeUninit::uninit();
            let mut proportion = mem::MaybeUninit::uninit();
            let mut diff = mem::MaybeUninit::uninit();
            let mut timestamp = mem::MaybeUninit::uninit();

            gst_sys::gst_event_parse_qos(
                self.as_mut_ptr(),
                type_.as_mut_ptr(),
                proportion.as_mut_ptr(),
                diff.as_mut_ptr(),
                timestamp.as_mut_ptr(),
            );

            (
                from_glib(type_.assume_init()),
                proportion.assume_init(),
                diff.assume_init(),
                from_glib(timestamp.assume_init()),
            )
        }
    }
}

declare_concrete_event!(Seek);
impl<'a> Seek<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<V: Into<GenericFormattedValue>>(
        rate: f64,
        flags: ::SeekFlags,
        start_type: ::SeekType,
        start: V,
        stop_type: ::SeekType,
        stop: V,
    ) -> Event {
        skip_assert_initialized!();
        Self::builder(rate, flags, start_type, start, stop_type, stop).build()
    }

    pub fn builder<V: Into<GenericFormattedValue>>(
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
            let mut rate = mem::MaybeUninit::uninit();
            let mut fmt = mem::MaybeUninit::uninit();
            let mut flags = mem::MaybeUninit::uninit();
            let mut start_type = mem::MaybeUninit::uninit();
            let mut start = mem::MaybeUninit::uninit();
            let mut stop_type = mem::MaybeUninit::uninit();
            let mut stop = mem::MaybeUninit::uninit();

            gst_sys::gst_event_parse_seek(
                self.as_mut_ptr(),
                rate.as_mut_ptr(),
                fmt.as_mut_ptr(),
                flags.as_mut_ptr(),
                start_type.as_mut_ptr(),
                start.as_mut_ptr(),
                stop_type.as_mut_ptr(),
                stop.as_mut_ptr(),
            );

            (
                rate.assume_init(),
                from_glib(flags.assume_init()),
                from_glib(start_type.assume_init()),
                GenericFormattedValue::new(from_glib(fmt.assume_init()), start.assume_init()),
                from_glib(stop_type.assume_init()),
                GenericFormattedValue::new(from_glib(fmt.assume_init()), stop.assume_init()),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn get_trickmode_interval(&self) -> ::ClockTime {
        unsafe {
            let mut trickmode_interval = mem::MaybeUninit::uninit();

            gst_sys::gst_event_parse_seek_trickmode_interval(
                self.as_mut_ptr(),
                trickmode_interval.as_mut_ptr(),
            );

            from_glib(trickmode_interval.assume_init())
        }
    }
}

declare_concrete_event!(Navigation);
impl<'a> Navigation<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(structure: ::Structure) -> Event {
        skip_assert_initialized!();
        Self::builder(structure).build()
    }

    pub fn builder(structure: ::Structure) -> NavigationBuilder<'a> {
        assert_initialized_main_thread!();
        NavigationBuilder::new(structure)
    }
}

declare_concrete_event!(Latency);
impl<'a> Latency<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(latency: ::ClockTime) -> Event {
        skip_assert_initialized!();
        Self::builder(latency).build()
    }

    pub fn builder(latency: ::ClockTime) -> LatencyBuilder<'a> {
        assert_initialized_main_thread!();
        LatencyBuilder::new(latency)
    }

    pub fn get_latency(&self) -> ::ClockTime {
        unsafe {
            let mut latency = mem::MaybeUninit::uninit();

            gst_sys::gst_event_parse_latency(self.as_mut_ptr(), latency.as_mut_ptr());

            from_glib(latency.assume_init())
        }
    }
}

declare_concrete_event!(Step);
impl<'a> Step<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<V: Into<GenericFormattedValue>>(
        amount: V,
        rate: f64,
        flush: bool,
        intermediate: bool,
    ) -> Event {
        skip_assert_initialized!();
        Self::builder(amount.into(), rate, flush, intermediate).build()
    }

    pub fn builder<V: Into<GenericFormattedValue>>(
        amount: V,
        rate: f64,
        flush: bool,
        intermediate: bool,
    ) -> StepBuilder<'a> {
        assert_initialized_main_thread!();
        StepBuilder::new(amount.into(), rate, flush, intermediate)
    }

    pub fn get(&self) -> (GenericFormattedValue, f64, bool, bool) {
        unsafe {
            let mut fmt = mem::MaybeUninit::uninit();
            let mut amount = mem::MaybeUninit::uninit();
            let mut rate = mem::MaybeUninit::uninit();
            let mut flush = mem::MaybeUninit::uninit();
            let mut intermediate = mem::MaybeUninit::uninit();

            gst_sys::gst_event_parse_step(
                self.as_mut_ptr(),
                fmt.as_mut_ptr(),
                amount.as_mut_ptr(),
                rate.as_mut_ptr(),
                flush.as_mut_ptr(),
                intermediate.as_mut_ptr(),
            );

            (
                GenericFormattedValue::new(
                    from_glib(fmt.assume_init()),
                    amount.assume_init() as i64,
                ),
                rate.assume_init(),
                from_glib(flush.assume_init()),
                from_glib(intermediate.assume_init()),
            )
        }
    }
}

declare_concrete_event!(Reconfigure);
impl<'a> Reconfigure<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Event {
        skip_assert_initialized!();
        Self::builder().build()
    }

    pub fn builder() -> ReconfigureBuilder<'a> {
        assert_initialized_main_thread!();
        ReconfigureBuilder::new()
    }
}

declare_concrete_event!(TocSelect);
impl<'a> TocSelect<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(uid: &str) -> Event {
        skip_assert_initialized!();
        Self::builder(uid).build()
    }

    pub fn builder(uid: &str) -> TocSelectBuilder {
        assert_initialized_main_thread!();
        TocSelectBuilder::new(uid)
    }

    pub fn get_uid(&self) -> &'a str {
        unsafe {
            let mut uid = ptr::null_mut();

            gst_sys::gst_event_parse_toc_select(self.as_mut_ptr(), &mut uid);

            CStr::from_ptr(uid).to_str().unwrap()
        }
    }
}

declare_concrete_event!(SelectStreams);
impl<'a> SelectStreams<'a> {
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(streams: &'a [&'a str]) -> Event {
        skip_assert_initialized!();
        Self::builder(streams).build()
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn builder(streams: &'a [&'a str]) -> SelectStreamsBuilder {
        assert_initialized_main_thread!();
        SelectStreamsBuilder::new(streams)
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_streams(&self) -> Vec<String> {
        unsafe {
            let mut streams = ptr::null_mut();

            gst_sys::gst_event_parse_select_streams(self.as_mut_ptr(), &mut streams);

            FromGlibPtrContainer::from_glib_full(streams)
        }
    }
}

declare_concrete_event!(CustomUpstream);
impl<'a> CustomUpstream<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(structure: ::Structure) -> Event {
        skip_assert_initialized!();
        Self::builder(structure).build()
    }

    pub fn builder(structure: ::Structure) -> CustomUpstreamBuilder<'a> {
        assert_initialized_main_thread!();
        CustomUpstreamBuilder::new(structure)
    }
}

declare_concrete_event!(CustomDownstream);
impl<'a> CustomDownstream<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(structure: ::Structure) -> Event {
        skip_assert_initialized!();
        Self::builder(structure).build()
    }

    pub fn builder(structure: ::Structure) -> CustomDownstreamBuilder<'a> {
        assert_initialized_main_thread!();
        CustomDownstreamBuilder::new(structure)
    }
}

declare_concrete_event!(CustomDownstreamOob);
impl<'a> CustomDownstreamOob<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(structure: ::Structure) -> Event {
        skip_assert_initialized!();
        Self::builder(structure).build()
    }

    pub fn builder(structure: ::Structure) -> CustomDownstreamOobBuilder<'a> {
        assert_initialized_main_thread!();
        CustomDownstreamOobBuilder::new(structure)
    }
}

declare_concrete_event!(CustomDownstreamSticky);
impl<'a> CustomDownstreamSticky<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(structure: ::Structure) -> Event {
        skip_assert_initialized!();
        Self::builder(structure).build()
    }

    pub fn builder(structure: ::Structure) -> CustomDownstreamStickyBuilder<'a> {
        assert_initialized_main_thread!();
        CustomDownstreamStickyBuilder::new(structure)
    }
}

declare_concrete_event!(CustomBoth);
impl<'a> CustomBoth<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(structure: ::Structure) -> Event {
        skip_assert_initialized!();
        Self::builder(structure).build()
    }

    pub fn builder(structure: ::Structure) -> CustomBothBuilder<'a> {
        assert_initialized_main_thread!();
        CustomBothBuilder::new(structure)
    }
}

declare_concrete_event!(CustomBothOob);
impl<'a> CustomBothOob<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(structure: ::Structure) -> Event {
        skip_assert_initialized!();
        Self::builder(structure).build()
    }

    pub fn builder(structure: ::Structure) -> CustomBothOobBuilder<'a> {
        assert_initialized_main_thread!();
        CustomBothOobBuilder::new(structure)
    }
}

struct EventBuilder<'a> {
    seqnum: Option<Seqnum>,
    running_time_offset: Option<i64>,
    other_fields: Vec<(&'a str, &'a dyn ToSendValue)>,
}

impl<'a> EventBuilder<'a> {
    fn new() -> Self {
        Self {
            seqnum: None,
            running_time_offset: None,
            other_fields: Vec::new(),
        }
    }

    fn seqnum(self, seqnum: Seqnum) -> Self {
        Self {
            seqnum: Some(seqnum),
            ..self
        }
    }

    fn running_time_offset(self, running_time_offset: i64) -> Self {
        Self {
            running_time_offset: Some(running_time_offset),
            ..self
        }
    }

    fn other_fields(self, other_fields: &[(&'a str, &'a dyn ToSendValue)]) -> Self {
        Self {
            other_fields: self
                .other_fields
                .iter()
                .cloned()
                .chain(other_fields.iter().cloned())
                .collect(),
            ..self
        }
    }
}

macro_rules! event_builder_generic_impl {
    ($new_fn:expr) => {
        #[allow(clippy::needless_update)]
        pub fn seqnum(self, seqnum: Seqnum) -> Self {
            Self {
                builder: self.builder.seqnum(seqnum),
                ..self
            }
        }

        #[allow(clippy::needless_update)]
        pub fn running_time_offset(self, running_time_offset: i64) -> Self {
            Self {
                builder: self.builder.running_time_offset(running_time_offset),
                ..self
            }
        }

        #[allow(clippy::needless_update)]
        pub fn other_fields(self, other_fields: &[(&'a str, &'a dyn ToSendValue)]) -> Self {
            Self {
                builder: self.builder.other_fields(other_fields),
                ..self
            }
        }

        pub fn build(mut self) -> Event {
            assert_initialized_main_thread!();
            unsafe {
                let event = $new_fn(&mut self);
                if let Some(seqnum) = self.builder.seqnum {
                    gst_sys::gst_event_set_seqnum(event, seqnum.0.get());
                }

                if let Some(running_time_offset) = self.builder.running_time_offset {
                    gst_sys::gst_event_set_running_time_offset(event, running_time_offset);
                }

                if !self.builder.other_fields.is_empty() {
                    let s = StructureRef::from_glib_borrow_mut(
                        gst_sys::gst_event_writable_structure(event),
                    );

                    for (k, v) in self.builder.other_fields {
                        s.set_value(k, v.to_send_value());
                    }
                }

                from_glib_full(event)
            }
        }
    };
}

pub struct FlushStartBuilder<'a> {
    builder: EventBuilder<'a>,
}

impl<'a> FlushStartBuilder<'a> {
    fn new() -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
        }
    }

    event_builder_generic_impl!(|_| gst_sys::gst_event_new_flush_start());
}

pub struct FlushStopBuilder<'a> {
    builder: EventBuilder<'a>,
    reset_time: bool,
}
impl<'a> FlushStopBuilder<'a> {
    fn new(reset_time: bool) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            reset_time,
        }
    }

    event_builder_generic_impl!(|s: &Self| gst_sys::gst_event_new_flush_stop(
        s.reset_time.to_glib()
    ));
}

pub struct StreamStartBuilder<'a> {
    builder: EventBuilder<'a>,
    stream_id: &'a str,
    flags: Option<::StreamFlags>,
    group_id: Option<GroupId>,
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    stream: Option<::Stream>,
}

impl<'a> StreamStartBuilder<'a> {
    fn new(stream_id: &'a str) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            stream_id,
            flags: None,
            group_id: None,
            #[cfg(any(feature = "v1_10", feature = "dox"))]
            stream: None,
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

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn stream(self, stream: ::Stream) -> Self {
        Self {
            stream: Some(stream),
            ..self
        }
    }

    event_builder_generic_impl!(|s: &Self| {
        let ev = gst_sys::gst_event_new_stream_start(s.stream_id.to_glib_none().0);
        if let Some(flags) = s.flags {
            gst_sys::gst_event_set_stream_flags(ev, flags.to_glib());
        }
        if let Some(group_id) = s.group_id {
            gst_sys::gst_event_set_group_id(ev, group_id.0.get());
        }

        #[cfg(any(feature = "v1_10", feature = "dox"))]
        {
            if let Some(ref stream) = s.stream {
                gst_sys::gst_event_set_stream(ev, stream.to_glib_none().0);
            }
        }

        ev
    });
}

pub struct CapsBuilder<'a> {
    builder: EventBuilder<'a>,
    caps: &'a ::Caps,
}

impl<'a> CapsBuilder<'a> {
    fn new(caps: &'a ::Caps) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            caps,
        }
    }

    event_builder_generic_impl!(|s: &Self| gst_sys::gst_event_new_caps(s.caps.as_mut_ptr()));
}

pub struct SegmentBuilder<'a> {
    builder: EventBuilder<'a>,
    segment: &'a ::Segment,
}

impl<'a> SegmentBuilder<'a> {
    fn new(segment: &'a ::Segment) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            segment,
        }
    }

    event_builder_generic_impl!(|s: &Self| gst_sys::gst_event_new_segment(
        s.segment.to_glib_none().0
    ));
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
pub struct StreamCollectionBuilder<'a> {
    builder: EventBuilder<'a>,
    stream_collection: &'a ::StreamCollection,
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
impl<'a> StreamCollectionBuilder<'a> {
    fn new(stream_collection: &'a ::StreamCollection) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            stream_collection,
        }
    }

    event_builder_generic_impl!(|s: &Self| gst_sys::gst_event_new_stream_collection(
        s.stream_collection.to_glib_none().0
    ));
}

pub struct TagBuilder<'a> {
    builder: EventBuilder<'a>,
    tags: Option<::TagList>,
}

impl<'a> TagBuilder<'a> {
    fn new(tags: ::TagList) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            tags: Some(tags),
        }
    }

    event_builder_generic_impl!(|s: &mut Self| {
        let tags = s.tags.take().unwrap();
        gst_sys::gst_event_new_tag(tags.into_ptr())
    });
}

pub struct BufferSizeBuilder<'a> {
    builder: EventBuilder<'a>,
    minsize: GenericFormattedValue,
    maxsize: GenericFormattedValue,
    async: bool,
}

impl<'a> BufferSizeBuilder<'a> {
    fn new(minsize: GenericFormattedValue, maxsize: GenericFormattedValue, async: bool) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            minsize,
            maxsize,
            async,
        }
    }

    event_builder_generic_impl!(|s: &Self| gst_sys::gst_event_new_buffer_size(
        s.minsize.get_format().to_glib(),
        s.minsize.get_value(),
        s.maxsize.get_value(),
        s.async.to_glib(),
    ));
}

pub struct SinkMessageBuilder<'a> {
    builder: EventBuilder<'a>,
    name: &'a str,
    msg: &'a ::Message,
}

impl<'a> SinkMessageBuilder<'a> {
    fn new(name: &'a str, msg: &'a ::Message) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            name,
            msg,
        }
    }

    event_builder_generic_impl!(|s: &Self| gst_sys::gst_event_new_sink_message(
        s.name.to_glib_none().0,
        s.msg.as_mut_ptr()
    ));
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
pub struct StreamGroupDoneBuilder<'a> {
    builder: EventBuilder<'a>,
    group_id: GroupId,
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
impl<'a> StreamGroupDoneBuilder<'a> {
    fn new(group_id: GroupId) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            group_id,
        }
    }

    event_builder_generic_impl!(|s: &Self| gst_sys::gst_event_new_stream_group_done(
        s.group_id.0.get()
    ));
}

pub struct EosBuilder<'a> {
    builder: EventBuilder<'a>,
}

impl<'a> EosBuilder<'a> {
    fn new() -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
        }
    }

    event_builder_generic_impl!(|_| gst_sys::gst_event_new_eos());
}

pub struct TocBuilder<'a> {
    builder: EventBuilder<'a>,
    toc: &'a ::Toc,
    updated: bool,
}

impl<'a> TocBuilder<'a> {
    fn new(toc: &'a ::Toc, updated: bool) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            toc,
            updated,
        }
    }

    event_builder_generic_impl!(|s: &Self| gst_sys::gst_event_new_toc(
        s.toc.to_glib_none().0,
        s.updated.to_glib()
    ));
}

pub struct ProtectionBuilder<'a> {
    builder: EventBuilder<'a>,
    system_id: &'a str,
    data: &'a ::Buffer,
    origin: Option<&'a str>,
}

impl<'a> ProtectionBuilder<'a> {
    fn new(system_id: &'a str, data: &'a ::Buffer) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            system_id,
            data,
            origin: None,
        }
    }

    pub fn origin(self, origin: &'a str) -> Self {
        Self {
            origin: Some(origin),
            ..self
        }
    }

    event_builder_generic_impl!(|s: &Self| gst_sys::gst_event_new_protection(
        s.system_id.to_glib_none().0,
        s.data.as_mut_ptr(),
        s.origin.to_glib_none().0,
    ));
}

pub struct SegmentDoneBuilder<'a> {
    builder: EventBuilder<'a>,
    position: GenericFormattedValue,
}

impl<'a> SegmentDoneBuilder<'a> {
    fn new(position: GenericFormattedValue) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            position,
        }
    }

    event_builder_generic_impl!(|s: &Self| gst_sys::gst_event_new_segment_done(
        s.position.get_format().to_glib(),
        s.position.get_value()
    ));
}

pub struct GapBuilder<'a> {
    builder: EventBuilder<'a>,
    timestamp: ::ClockTime,
    duration: ::ClockTime,
}

impl<'a> GapBuilder<'a> {
    fn new(timestamp: ::ClockTime, duration: ::ClockTime) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            timestamp,
            duration,
        }
    }

    event_builder_generic_impl!(|s: &Self| gst_sys::gst_event_new_gap(
        s.timestamp.to_glib(),
        s.duration.to_glib()
    ));
}

pub struct QosBuilder<'a> {
    builder: EventBuilder<'a>,
    type_: ::QOSType,
    proportion: f64,
    diff: i64,
    timestamp: ::ClockTime,
}

impl<'a> QosBuilder<'a> {
    fn new(type_: ::QOSType, proportion: f64, diff: i64, timestamp: ::ClockTime) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            type_,
            proportion,
            diff,
            timestamp,
        }
    }

    event_builder_generic_impl!(|s: &Self| gst_sys::gst_event_new_qos(
        s.type_.to_glib(),
        s.proportion,
        s.diff,
        s.timestamp.to_glib(),
    ));
}

pub struct SeekBuilder<'a> {
    builder: EventBuilder<'a>,
    rate: f64,
    flags: ::SeekFlags,
    start_type: ::SeekType,
    start: GenericFormattedValue,
    stop_type: ::SeekType,
    stop: GenericFormattedValue,
    #[allow(unused)]
    trickmode_interval: Option<::ClockTime>,
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
            builder: EventBuilder::new(),
            rate,
            flags,
            start_type,
            start,
            stop_type,
            stop,
            trickmode_interval: None,
        }
    }

    event_builder_generic_impl!(|s: &Self| {
        #[allow(clippy::let_and_return)]
        {
            let ev = gst_sys::gst_event_new_seek(
                s.rate,
                s.start.get_format().to_glib(),
                s.flags.to_glib(),
                s.start_type.to_glib(),
                s.start.get_value(),
                s.stop_type.to_glib(),
                s.stop.get_value(),
            );

            #[cfg(any(feature = "v1_16", feature = "dox"))]
            {
                if let Some(trickmode_interval) = s.trickmode_interval {
                    gst_sys::gst_event_set_seek_trickmode_interval(
                        ev,
                        trickmode_interval.to_glib(),
                    );
                }
            }

            ev
        }
    });
}

pub struct NavigationBuilder<'a> {
    builder: EventBuilder<'a>,
    structure: Option<Structure>,
}

impl<'a> NavigationBuilder<'a> {
    fn new(structure: Structure) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            structure: Some(structure),
        }
    }

    event_builder_generic_impl!(|s: &mut Self| {
        let structure = s.structure.take().unwrap();
        gst_sys::gst_event_new_navigation(structure.into_ptr())
    });
}

pub struct LatencyBuilder<'a> {
    builder: EventBuilder<'a>,
    latency: ::ClockTime,
}

impl<'a> LatencyBuilder<'a> {
    fn new(latency: ::ClockTime) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            latency,
        }
    }

    event_builder_generic_impl!(|s: &Self| gst_sys::gst_event_new_latency(s.latency.to_glib()));
}

pub struct StepBuilder<'a> {
    builder: EventBuilder<'a>,
    amount: GenericFormattedValue,
    rate: f64,
    flush: bool,
    intermediate: bool,
}

impl<'a> StepBuilder<'a> {
    fn new(amount: GenericFormattedValue, rate: f64, flush: bool, intermediate: bool) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            amount,
            rate,
            flush,
            intermediate,
        }
    }

    event_builder_generic_impl!(|s: &Self| gst_sys::gst_event_new_step(
        s.amount.get_format().to_glib(),
        s.amount.get_value() as u64,
        s.rate,
        s.flush.to_glib(),
        s.intermediate.to_glib(),
    ));
}

pub struct ReconfigureBuilder<'a> {
    builder: EventBuilder<'a>,
}

impl<'a> ReconfigureBuilder<'a> {
    fn new() -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
        }
    }

    event_builder_generic_impl!(|_| gst_sys::gst_event_new_reconfigure());
}

pub struct TocSelectBuilder<'a> {
    builder: EventBuilder<'a>,
    uid: &'a str,
}

impl<'a> TocSelectBuilder<'a> {
    fn new(uid: &'a str) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            uid,
        }
    }

    event_builder_generic_impl!(|s: &Self| gst_sys::gst_event_new_toc_select(
        s.uid.to_glib_none().0
    ));
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
pub struct SelectStreamsBuilder<'a> {
    builder: EventBuilder<'a>,
    streams: &'a [&'a str],
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
impl<'a> SelectStreamsBuilder<'a> {
    fn new(streams: &'a [&'a str]) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            streams,
        }
    }

    event_builder_generic_impl!(|s: &Self| gst_sys::gst_event_new_select_streams(
        s.streams.to_glib_full()
    ));
}

pub struct CustomUpstreamBuilder<'a> {
    builder: EventBuilder<'a>,
    structure: Option<Structure>,
}

impl<'a> CustomUpstreamBuilder<'a> {
    fn new(structure: Structure) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            structure: Some(structure),
        }
    }

    event_builder_generic_impl!(|s: &mut Self| {
        let structure = s.structure.take().unwrap();
        gst_sys::gst_event_new_custom(gst_sys::GST_EVENT_CUSTOM_UPSTREAM, structure.into_ptr())
    });
}

pub struct CustomDownstreamBuilder<'a> {
    builder: EventBuilder<'a>,
    structure: Option<Structure>,
}

impl<'a> CustomDownstreamBuilder<'a> {
    fn new(structure: Structure) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            structure: Some(structure),
        }
    }

    event_builder_generic_impl!(|s: &mut Self| {
        let structure = s.structure.take().unwrap();
        gst_sys::gst_event_new_custom(gst_sys::GST_EVENT_CUSTOM_DOWNSTREAM, structure.into_ptr())
    });
}

pub struct CustomDownstreamOobBuilder<'a> {
    builder: EventBuilder<'a>,
    structure: Option<Structure>,
}

impl<'a> CustomDownstreamOobBuilder<'a> {
    fn new(structure: Structure) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            structure: Some(structure),
        }
    }

    event_builder_generic_impl!(|s: &mut Self| {
        let structure = s.structure.take().unwrap();
        gst_sys::gst_event_new_custom(
            gst_sys::GST_EVENT_CUSTOM_DOWNSTREAM_OOB,
            structure.into_ptr(),
        )
    });
}

pub struct CustomDownstreamStickyBuilder<'a> {
    builder: EventBuilder<'a>,
    structure: Option<Structure>,
}

impl<'a> CustomDownstreamStickyBuilder<'a> {
    fn new(structure: Structure) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            structure: Some(structure),
        }
    }

    event_builder_generic_impl!(|s: &mut Self| {
        let structure = s.structure.take().unwrap();
        gst_sys::gst_event_new_custom(
            gst_sys::GST_EVENT_CUSTOM_DOWNSTREAM_STICKY,
            structure.into_ptr(),
        )
    });
}

pub struct CustomBothBuilder<'a> {
    builder: EventBuilder<'a>,
    structure: Option<Structure>,
}

impl<'a> CustomBothBuilder<'a> {
    fn new(structure: Structure) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            structure: Some(structure),
        }
    }

    event_builder_generic_impl!(|s: &mut Self| {
        let structure = s.structure.take().unwrap();
        gst_sys::gst_event_new_custom(gst_sys::GST_EVENT_CUSTOM_BOTH, structure.into_ptr())
    });
}

pub struct CustomBothOobBuilder<'a> {
    builder: EventBuilder<'a>,
    structure: Option<Structure>,
}

impl<'a> CustomBothOobBuilder<'a> {
    fn new(structure: Structure) -> Self {
        skip_assert_initialized!();
        Self {
            builder: EventBuilder::new(),
            structure: Some(structure),
        }
    }

    event_builder_generic_impl!(|s: &mut Self| {
        let structure = s.structure.take().unwrap();
        gst_sys::gst_event_new_custom(gst_sys::GST_EVENT_CUSTOM_BOTH_OOB, structure.into_ptr())
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        ::init().unwrap();

        // Event without arguments
        let flush_start_evt = FlushStart::new();
        match flush_start_evt.view() {
            EventView::FlushStart(flush_start_evt) => {
                assert!(!flush_start_evt.is_sticky());
                assert!(flush_start_evt.get_structure().is_none());
            }
            _ => panic!("flush_start_evt.view() is not an EventView::FlushStart(_)"),
        }

        let flush_start_evt = FlushStart::builder()
            .other_fields(&[("extra-field", &true)])
            .build();
        match flush_start_evt.view() {
            EventView::FlushStart(flush_start_evt) => {
                assert!(flush_start_evt.get_structure().is_some());
                if let Some(other_fields) = flush_start_evt.get_structure() {
                    assert!(other_fields.has_field("extra-field"));
                }
            }
            _ => panic!("flush_start_evt.view() is not an EventView::FlushStart(_)"),
        }

        // Event with arguments
        let flush_stop_evt = FlushStop::builder(true)
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

    #[test]
    fn test_get_structure_mut() {
        ::init().unwrap();

        let mut flush_start_evt = FlushStart::new();

        {
            let flush_start_evt = flush_start_evt.get_mut().unwrap();
            let structure = flush_start_evt.structure_mut();
            structure.set("test", &42u32);
        }

        let structure = flush_start_evt.get_structure().unwrap();
        assert_eq!(structure.get_some("test"), Ok(42u32));
    }
}
