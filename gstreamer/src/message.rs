// Copyright (C) 2016-2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use Object;
use miniobject::*;
use structure::*;
use TagList;

use std::ptr;
use std::mem;
use std::ffi::CStr;

use glib;
use glib::Cast;
use glib::IsA;
use glib::value::ToValue;
use glib::translate::{from_glib, from_glib_full, from_glib_none, mut_override, ToGlib, ToGlibPtr};

#[repr(C)]
pub struct MessageRef(ffi::GstMessage);

pub type Message = GstRc<MessageRef>;

unsafe impl MiniObject for MessageRef {
    type GstType = ffi::GstMessage;
}

impl MessageRef {
    pub fn get_src(&self) -> Object {
        unsafe { from_glib_none((*self.as_ptr()).src) }
    }

    pub fn get_seqnum(&self) -> u32 {
        unsafe { ffi::gst_message_get_seqnum(self.as_mut_ptr()) }
    }

    pub fn get_structure(&self) -> &StructureRef {
        unsafe {
            let structure = ffi::gst_message_get_structure(self.as_mut_ptr());
            StructureRef::from_glib_borrow(structure)
        }
    }

    pub fn view(&self) -> MessageView {
        let type_ = unsafe { (*self.as_ptr()).type_ };

        match type_ {
            ffi::GST_MESSAGE_EOS => MessageView::Eos(Eos(self)),
            ffi::GST_MESSAGE_ERROR => MessageView::Error(Error(self)),
            ffi::GST_MESSAGE_WARNING => MessageView::Warning(Warning(self)),
            ffi::GST_MESSAGE_INFO => MessageView::Info(Info(self)),
            ffi::GST_MESSAGE_TAG => MessageView::Tag(Tag(self)),
            ffi::GST_MESSAGE_BUFFERING => MessageView::Buffering(Buffering(self)),
            ffi::GST_MESSAGE_STATE_CHANGED => MessageView::StateChanged(StateChanged(self)),
            ffi::GST_MESSAGE_STATE_DIRTY => MessageView::StateDirty(StateDirty(self)),
            ffi::GST_MESSAGE_STEP_DONE => MessageView::StepDone(StepDone(self)),
            ffi::GST_MESSAGE_CLOCK_PROVIDE => MessageView::ClockProvide(ClockProvide(self)),
            ffi::GST_MESSAGE_CLOCK_LOST => MessageView::ClockLost(ClockLost(self)),
            ffi::GST_MESSAGE_NEW_CLOCK => MessageView::NewClock(NewClock(self)),
            ffi::GST_MESSAGE_STRUCTURE_CHANGE => {
                MessageView::StructureChange(StructureChange(self))
            }
            ffi::GST_MESSAGE_STREAM_STATUS => MessageView::StreamStatus(StreamStatus(self)),
            ffi::GST_MESSAGE_APPLICATION => MessageView::Application(Application(self)),
            ffi::GST_MESSAGE_ELEMENT => MessageView::Element(Element(self)),
            ffi::GST_MESSAGE_SEGMENT_START => MessageView::SegmentStart(SegmentStart(self)),
            ffi::GST_MESSAGE_SEGMENT_DONE => MessageView::SegmentDone(SegmentDone(self)),
            ffi::GST_MESSAGE_DURATION_CHANGED => {
                MessageView::DurationChanged(DurationChanged(self))
            }
            ffi::GST_MESSAGE_LATENCY => MessageView::Latency(Latency(self)),
            ffi::GST_MESSAGE_ASYNC_START => MessageView::AsyncStart(AsyncStart(self)),
            ffi::GST_MESSAGE_ASYNC_DONE => MessageView::AsyncDone(AsyncDone(self)),
            ffi::GST_MESSAGE_REQUEST_STATE => MessageView::RequestState(RequestState(self)),
            ffi::GST_MESSAGE_STEP_START => MessageView::StepStart(StepStart(self)),
            ffi::GST_MESSAGE_QOS => MessageView::Qos(Qos(self)),
            ffi::GST_MESSAGE_PROGRESS => MessageView::Progress(Progress(self)),
            ffi::GST_MESSAGE_TOC => MessageView::Toc(Toc(self)),
            ffi::GST_MESSAGE_RESET_TIME => MessageView::ResetTime(ResetTime(self)),
            ffi::GST_MESSAGE_STREAM_START => MessageView::StreamStart(StreamStart(self)),
            ffi::GST_MESSAGE_NEED_CONTEXT => MessageView::NeedContext(NeedContext(self)),
            ffi::GST_MESSAGE_HAVE_CONTEXT => MessageView::HaveContext(HaveContext(self)),
            ffi::GST_MESSAGE_DEVICE_ADDED => MessageView::DeviceAdded(DeviceAdded(self)),
            ffi::GST_MESSAGE_DEVICE_REMOVED => MessageView::DeviceRemoved(DeviceRemoved(self)),
            ffi::GST_MESSAGE_PROPERTY_NOTIFY => MessageView::PropertyNotify(PropertyNotify(self)),
            ffi::GST_MESSAGE_STREAM_COLLECTION => {
                MessageView::StreamCollection(StreamCollection(self))
            }
            ffi::GST_MESSAGE_STREAMS_SELECTED => {
                MessageView::StreamsSelected(StreamsSelected(self))
            }
            _ => MessageView::Other,
        }
    }
}

impl GstRc<MessageRef> {
    pub fn new_eos<'a>() -> EosBuilder<'a> {
        assert_initialized_main_thread!();
        EosBuilder::new()
    }

    pub fn new_error(error: &glib::Error) -> ErrorBuilder {
        assert_initialized_main_thread!();
        ErrorBuilder::new(error)
    }

    pub fn new_warning(error: &glib::Error) -> WarningBuilder {
        assert_initialized_main_thread!();
        WarningBuilder::new(error)
    }

    pub fn new_info(error: &glib::Error) -> InfoBuilder {
        assert_initialized_main_thread!();
        InfoBuilder::new(error)
    }

    pub fn new_tag(tags: &TagList) -> TagBuilder {
        assert_initialized_main_thread!();
        TagBuilder::new(tags)
    }

    pub fn new_buffering<'a>(percent: i32) -> BufferingBuilder<'a> {
        assert_initialized_main_thread!();
        BufferingBuilder::new(percent)
    }

    pub fn new_state_changed<'a>(
        old: ::State,
        new: ::State,
        pending: ::State,
    ) -> StateChangedBuilder<'a> {
        assert_initialized_main_thread!();
        StateChangedBuilder::new(old, new, pending)
    }

    pub fn new_state_dirty<'a>() -> StateDirtyBuilder<'a> {
        assert_initialized_main_thread!();
        StateDirtyBuilder::new()
    }

    pub fn new_step_done<'a>(
        format: ::Format,
        amount: u64,
        rate: f64,
        flush: bool,
        intermediate: bool,
        duration: u64,
        eos: bool,
    ) -> StepDoneBuilder<'a> {
        assert_initialized_main_thread!();
        StepDoneBuilder::new(format, amount, rate, flush, intermediate, duration, eos)
    }

    pub fn new_clock_provide(clock: &::Clock, ready: bool) -> ClockProvideBuilder {
        assert_initialized_main_thread!();
        ClockProvideBuilder::new(clock, ready)
    }

    pub fn new_clock_lost(clock: &::Clock) -> ClockLostBuilder {
        assert_initialized_main_thread!();
        ClockLostBuilder::new(clock)
    }

    pub fn new_new_clock(clock: &::Clock) -> NewClockBuilder {
        assert_initialized_main_thread!();
        NewClockBuilder::new(clock)
    }

    pub fn new_structure_change(
        type_: ::StructureChangeType,
        owner: &::Element,
        busy: bool,
    ) -> StructureChangeBuilder {
        assert_initialized_main_thread!();
        StructureChangeBuilder::new(type_, owner, busy)
    }

    pub fn new_stream_status(type_: ::StreamStatusType, owner: &::Element) -> StreamStatusBuilder {
        assert_initialized_main_thread!();
        StreamStatusBuilder::new(type_, owner)
    }

    pub fn new_application<'a>(structure: ::Structure) -> ApplicationBuilder<'a> {
        assert_initialized_main_thread!();
        ApplicationBuilder::new(structure)
    }

    pub fn new_element<'a>(structure: ::Structure) -> ElementBuilder<'a> {
        assert_initialized_main_thread!();
        ElementBuilder::new(structure)
    }

    pub fn new_segment_start<'a>(format: ::Format, position: i64) -> SegmentStartBuilder<'a> {
        assert_initialized_main_thread!();
        SegmentStartBuilder::new(format, position)
    }

    pub fn new_segment_done<'a>(format: ::Format, position: i64) -> SegmentDoneBuilder<'a> {
        assert_initialized_main_thread!();
        SegmentDoneBuilder::new(format, position)
    }

    pub fn new_duration_changed<'a>() -> DurationChangedBuilder<'a> {
        assert_initialized_main_thread!();
        DurationChangedBuilder::new()
    }

    pub fn new_latency<'a>() -> LatencyBuilder<'a> {
        assert_initialized_main_thread!();
        LatencyBuilder::new()
    }

    pub fn new_async_start<'a>() -> AsyncStartBuilder<'a> {
        assert_initialized_main_thread!();
        AsyncStartBuilder::new()
    }

    pub fn new_async_done<'a>(running_time: u64) -> AsyncDoneBuilder<'a> {
        assert_initialized_main_thread!();
        AsyncDoneBuilder::new(running_time)
    }

    pub fn new_request_state<'a>(state: ::State) -> RequestStateBuilder<'a> {
        assert_initialized_main_thread!();
        RequestStateBuilder::new(state)
    }

    pub fn new_step_start<'a>(
        active: bool,
        format: ::Format,
        amount: u64,
        rate: f64,
        flush: bool,
        intermediate: bool,
    ) -> StepStartBuilder<'a> {
        assert_initialized_main_thread!();
        StepStartBuilder::new(active, format, amount, rate, flush, intermediate)
    }

    pub fn new_qos_builder<'a>(
        live: bool,
        running_time: u64,
        stream_time: u64,
        timestamp: u64,
        duration: u64,
    ) -> QosBuilder<'a> {
        assert_initialized_main_thread!();
        QosBuilder::new(live, running_time, stream_time, timestamp, duration)
    }

    pub fn new_progress<'a>(type_: ::ProgressType) -> ProgressBuilder<'a> {
        assert_initialized_main_thread!();
        ProgressBuilder::new(type_)
    }

    pub fn new_toc(toc: &::Toc, updated: bool) -> TocBuilder {
        assert_initialized_main_thread!();
        TocBuilder::new(toc, updated)
    }

    pub fn new_reset_time<'a>(running_time: u64) -> ResetTimeBuilder<'a> {
        assert_initialized_main_thread!();
        ResetTimeBuilder::new(running_time)
    }

    pub fn new_stream_start<'a>() -> StreamStartBuilder<'a> {
        assert_initialized_main_thread!();
        StreamStartBuilder::new()
    }

    pub fn new_need_context(context_type: &str) -> NeedContextBuilder {
        assert_initialized_main_thread!();
        NeedContextBuilder::new(context_type)
    }

    pub fn new_have_context<'a>(context: ::Context) -> HaveContextBuilder<'a> {
        assert_initialized_main_thread!();
        HaveContextBuilder::new(context)
    }

    pub fn new_device_added(device: &::Device) -> DeviceAddedBuilder {
        assert_initialized_main_thread!();
        DeviceAddedBuilder::new(device)
    }

    pub fn new_device_removed(device: &::Device) -> DeviceRemovedBuilder {
        assert_initialized_main_thread!();
        DeviceRemovedBuilder::new(device)
    }

    #[cfg(feature = "v1_10")]
    pub fn new_property_notify<'a>(
        property_name: &'a str,
        value: &'a glib::Value,
    ) -> PropertyNotifyBuilder<'a> {
        assert_initialized_main_thread!();
        PropertyNotifyBuilder::new(property_name, value)
    }

    #[cfg(feature = "v1_10")]
    pub fn new_stream_collection<'a>(
        collection: &'a ::StreamCollection,
    ) -> StreamCollectionBuilder<'a> {
        assert_initialized_main_thread!();
        StreamCollectionBuilder::new(collection)
    }

    #[cfg(feature = "v1_10")]
    pub fn new_streams_selected<'a>(
        collection: &'a ::StreamCollection,
    ) -> StreamsSelectedBuilder<'a> {
        assert_initialized_main_thread!();
        StreamsSelectedBuilder::new(collection)
    }

    #[cfg(feature = "v1_10")]
    pub fn new_redirect<'a>(
        location: &'a str,
        tag_list: Option<&'a TagList>,
        entry_struct: Option<Structure>,
    ) -> RedirectBuilder<'a> {
        assert_initialized_main_thread!();
        RedirectBuilder::new(location, tag_list, entry_struct)
    }
}

impl glib::types::StaticType for MessageRef {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(ffi::gst_message_get_type()) }
    }
}

pub enum MessageView<'a> {
    Eos(Eos<'a>),
    Error(Error<'a>),
    Warning(Warning<'a>),
    Info(Info<'a>),
    Tag(Tag<'a>),
    Buffering(Buffering<'a>),
    StateChanged(StateChanged<'a>),
    StateDirty(StateDirty<'a>),
    StepDone(StepDone<'a>),
    ClockProvide(ClockProvide<'a>),
    ClockLost(ClockLost<'a>),
    NewClock(NewClock<'a>),
    StructureChange(StructureChange<'a>),
    StreamStatus(StreamStatus<'a>),
    Application(Application<'a>),
    Element(Element<'a>),
    SegmentStart(SegmentStart<'a>),
    SegmentDone(SegmentDone<'a>),
    DurationChanged(DurationChanged<'a>),
    Latency(Latency<'a>),
    AsyncStart(AsyncStart<'a>),
    AsyncDone(AsyncDone<'a>),
    RequestState(RequestState<'a>),
    StepStart(StepStart<'a>),
    Qos(Qos<'a>),
    Progress(Progress<'a>),
    Toc(Toc<'a>),
    ResetTime(ResetTime<'a>),
    StreamStart(StreamStart<'a>),
    NeedContext(NeedContext<'a>),
    HaveContext(HaveContext<'a>),
    DeviceAdded(DeviceAdded<'a>),
    DeviceRemoved(DeviceRemoved<'a>),
    PropertyNotify(PropertyNotify<'a>),
    StreamCollection(StreamCollection<'a>),
    StreamsSelected(StreamsSelected<'a>),
    Redirect(Redirect<'a>),
    Other,
    __NonExhaustive,
}

pub struct Eos<'a>(&'a MessageRef);

pub struct Error<'a>(&'a MessageRef);
impl<'a> Error<'a> {
    pub fn get_error(&self) -> glib::Error {
        unsafe {
            let mut error = ptr::null_mut();

            ffi::gst_message_parse_error(self.0.as_mut_ptr(), &mut error, ptr::null_mut());

            from_glib_full(error)
        }
    }

    pub fn get_debug(&self) -> Option<String> {
        unsafe {
            let mut debug = ptr::null_mut();

            ffi::gst_message_parse_error(self.0.as_mut_ptr(), ptr::null_mut(), &mut debug);

            from_glib_full(debug)
        }
    }

    #[cfg(feature = "v1_10")]
    pub fn get_details(&self) -> Option<&StructureRef> {
        unsafe {
            let mut details = ptr::null();

            ffi::gst_message_parse_error_details(self.0.as_mut_ptr(), &mut details);

            if details.is_null() {
                None
            } else {
                Some(StructureRef::from_glib_borrow(details))
            }
        }
    }
}

pub struct Warning<'a>(&'a MessageRef);
impl<'a> Warning<'a> {
    pub fn get_error(&self) -> glib::Error {
        unsafe {
            let mut error = ptr::null_mut();

            ffi::gst_message_parse_warning(self.0.as_mut_ptr(), &mut error, ptr::null_mut());

            from_glib_full(error)
        }
    }

    pub fn get_debug(&self) -> Option<String> {
        unsafe {
            let mut debug = ptr::null_mut();

            ffi::gst_message_parse_warning(self.0.as_mut_ptr(), ptr::null_mut(), &mut debug);

            from_glib_full(debug)
        }
    }

    #[cfg(feature = "v1_10")]
    pub fn get_details(&self) -> Option<&StructureRef> {
        unsafe {
            let mut details = ptr::null();

            ffi::gst_message_parse_error_details(self.0.as_mut_ptr(), &mut details);

            if details.is_null() {
                None
            } else {
                Some(StructureRef::from_glib_borrow(details))
            }
        }
    }
}

pub struct Info<'a>(&'a MessageRef);
impl<'a> Info<'a> {
    pub fn get_error(&self) -> glib::Error {
        unsafe {
            let mut error = ptr::null_mut();

            ffi::gst_message_parse_info(self.0.as_mut_ptr(), &mut error, ptr::null_mut());

            from_glib_full(error)
        }
    }

    pub fn get_debug(&self) -> Option<String> {
        unsafe {
            let mut debug = ptr::null_mut();

            ffi::gst_message_parse_info(self.0.as_mut_ptr(), ptr::null_mut(), &mut debug);

            from_glib_full(debug)
        }
    }

    #[cfg(feature = "v1_10")]
    pub fn get_details(&self) -> Option<&StructureRef> {
        unsafe {
            let mut details = ptr::null();

            ffi::gst_message_parse_error_details(self.0.as_mut_ptr(), &mut details);

            if details.is_null() {
                None
            } else {
                Some(StructureRef::from_glib_borrow(details))
            }
        }
    }
}

pub struct Tag<'a>(&'a MessageRef);
impl<'a> Tag<'a> {
    pub fn get_tags(&self) -> TagList {
        unsafe {
            let mut tags = ptr::null_mut();
            ffi::gst_message_parse_tag(self.0.as_mut_ptr(), &mut tags);
            from_glib_full(tags)
        }
    }
}

pub struct Buffering<'a>(&'a MessageRef);
impl<'a> Buffering<'a> {
    pub fn get_percent(&self) -> i32 {
        unsafe {
            let mut p = mem::uninitialized();
            ffi::gst_message_parse_buffering(self.0.as_mut_ptr(), &mut p);
            p
        }
    }

    pub fn get_buffering_stats(&self) -> (::BufferingMode, i32, i32, i64) {
        unsafe {
            let mut mode = mem::uninitialized();
            let mut avg_in = mem::uninitialized();
            let mut avg_out = mem::uninitialized();
            let mut buffering_left = mem::uninitialized();

            ffi::gst_message_parse_buffering_stats(
                self.0.as_mut_ptr(),
                &mut mode,
                &mut avg_in,
                &mut avg_out,
                &mut buffering_left,
            );

            (from_glib(mode), avg_in, avg_out, buffering_left)
        }
    }
}

pub struct StateChanged<'a>(&'a MessageRef);
impl<'a> StateChanged<'a> {
    pub fn get_old(&self) -> ::State {
        unsafe {
            let mut state = mem::uninitialized();

            ffi::gst_message_parse_state_changed(
                self.0.as_mut_ptr(),
                &mut state,
                ptr::null_mut(),
                ptr::null_mut(),
            );

            from_glib(state)
        }
    }

    pub fn get_current(&self) -> ::State {
        unsafe {
            let mut state = mem::uninitialized();

            ffi::gst_message_parse_state_changed(
                self.0.as_mut_ptr(),
                ptr::null_mut(),
                &mut state,
                ptr::null_mut(),
            );

            from_glib(state)
        }
    }

    pub fn get_pending(&self) -> ::State {
        unsafe {
            let mut state = mem::uninitialized();

            ffi::gst_message_parse_state_changed(
                self.0.as_mut_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
                &mut state,
            );

            from_glib(state)
        }
    }
}

pub struct StateDirty<'a>(&'a MessageRef);

pub struct StepDone<'a>(&'a MessageRef);
impl<'a> StepDone<'a> {
    pub fn get(&self) -> (::Format, u64, f64, bool, bool, u64, bool) {
        unsafe {
            let mut format = mem::uninitialized();
            let mut amount = mem::uninitialized();
            let mut rate = mem::uninitialized();
            let mut flush = mem::uninitialized();
            let mut intermediate = mem::uninitialized();
            let mut duration = mem::uninitialized();
            let mut eos = mem::uninitialized();

            ffi::gst_message_parse_step_done(
                self.0.as_mut_ptr(),
                &mut format,
                &mut amount,
                &mut rate,
                &mut flush,
                &mut intermediate,
                &mut duration,
                &mut eos,
            );

            (
                from_glib(format),
                amount,
                rate,
                from_glib(flush),
                from_glib(intermediate),
                duration,
                from_glib(eos),
            )
        }
    }
}


pub struct ClockProvide<'a>(&'a MessageRef);
impl<'a> ClockProvide<'a> {
    pub fn get_clock(&self) -> Option<::Clock> {
        let mut clock = ptr::null_mut();

        unsafe {
            ffi::gst_message_parse_clock_provide(self.0.as_mut_ptr(), &mut clock, ptr::null_mut());

            from_glib_none(clock)
        }
    }

    pub fn get_ready(&self) -> bool {
        unsafe {
            let mut ready = mem::uninitialized();

            ffi::gst_message_parse_clock_provide(self.0.as_mut_ptr(), ptr::null_mut(), &mut ready);

            from_glib(ready)
        }
    }
}

pub struct ClockLost<'a>(&'a MessageRef);
impl<'a> ClockLost<'a> {
    pub fn get_clock(&self) -> Option<::Clock> {
        let mut clock = ptr::null_mut();

        unsafe {
            ffi::gst_message_parse_clock_lost(self.0.as_mut_ptr(), &mut clock);

            from_glib_none(clock)
        }
    }
}

pub struct NewClock<'a>(&'a MessageRef);
impl<'a> NewClock<'a> {
    pub fn get_clock(&self) -> Option<::Clock> {
        let mut clock = ptr::null_mut();

        unsafe {
            ffi::gst_message_parse_new_clock(self.0.as_mut_ptr(), &mut clock);

            from_glib_none(clock)
        }
    }
}

pub struct StructureChange<'a>(&'a MessageRef);
impl<'a> StructureChange<'a> {
    pub fn get(&self) -> (::StructureChangeType, Option<::Element>, bool) {
        unsafe {
            let mut type_ = mem::uninitialized();
            let mut owner = ptr::null_mut();
            let mut busy = mem::uninitialized();

            ffi::gst_message_parse_structure_change(
                self.0.as_mut_ptr(),
                &mut type_,
                &mut owner,
                &mut busy,
            );

            (from_glib(type_), from_glib_none(owner), from_glib(busy))
        }
    }
}

pub struct StreamStatus<'a>(&'a MessageRef);
impl<'a> StreamStatus<'a> {
    pub fn get(&self) -> (::StreamStatusType, Option<::Element>) {
        unsafe {
            let mut type_ = mem::uninitialized();
            let mut owner = ptr::null_mut();

            ffi::gst_message_parse_stream_status(self.0.as_mut_ptr(), &mut type_, &mut owner);

            (from_glib(type_), from_glib_none(owner))
        }
    }

    pub fn get_stream_status_object(&self) -> Option<glib::Value> {
        unsafe {
            let value = ffi::gst_message_get_stream_status_object(self.0.as_mut_ptr());

            from_glib_none(value)
        }
    }
}

pub struct Application<'a>(&'a MessageRef);

pub struct Element<'a>(&'a MessageRef);

pub struct SegmentStart<'a>(&'a MessageRef);
impl<'a> SegmentStart<'a> {
    pub fn get(&self) -> (::Format, i64) {
        unsafe {
            let mut format = mem::uninitialized();
            let mut position = mem::uninitialized();

            ffi::gst_message_parse_segment_start(self.0.as_mut_ptr(), &mut format, &mut position);

            (from_glib(format), position)
        }
    }
}

pub struct SegmentDone<'a>(&'a MessageRef);
impl<'a> SegmentDone<'a> {
    pub fn get(&self) -> (::Format, i64) {
        unsafe {
            let mut format = mem::uninitialized();
            let mut position = mem::uninitialized();

            ffi::gst_message_parse_segment_done(self.0.as_mut_ptr(), &mut format, &mut position);

            (from_glib(format), position)
        }
    }
}

pub struct DurationChanged<'a>(&'a MessageRef);

pub struct Latency<'a>(&'a MessageRef);

pub struct AsyncStart<'a>(&'a MessageRef);

pub struct AsyncDone<'a>(&'a MessageRef);
impl<'a> AsyncDone<'a> {
    pub fn get_running_time(&self) -> u64 {
        unsafe {
            let mut running_time = mem::uninitialized();

            ffi::gst_message_parse_async_done(self.0.as_mut_ptr(), &mut running_time);

            running_time
        }
    }
}

pub struct RequestState<'a>(&'a MessageRef);
impl<'a> RequestState<'a> {
    pub fn get_requested_state(&self) -> ::State {
        unsafe {
            let mut state = mem::uninitialized();

            ffi::gst_message_parse_request_state(self.0.as_mut_ptr(), &mut state);

            from_glib(state)
        }
    }
}

pub struct StepStart<'a>(&'a MessageRef);
impl<'a> StepStart<'a> {
    pub fn get(&self) -> (bool, ::Format, u64, f64, bool, bool) {
        unsafe {
            let mut active = mem::uninitialized();
            let mut format = mem::uninitialized();
            let mut amount = mem::uninitialized();
            let mut rate = mem::uninitialized();
            let mut flush = mem::uninitialized();
            let mut intermediate = mem::uninitialized();

            ffi::gst_message_parse_step_start(
                self.0.as_mut_ptr(),
                &mut active,
                &mut format,
                &mut amount,
                &mut rate,
                &mut flush,
                &mut intermediate,
            );

            (
                from_glib(active),
                from_glib(format),
                amount,
                rate,
                from_glib(flush),
                from_glib(intermediate),
            )
        }
    }
}

pub struct Qos<'a>(&'a MessageRef);
impl<'a> Qos<'a> {
    pub fn get(&self) -> (bool, u64, u64, u64, u64) {
        unsafe {
            let mut live = mem::uninitialized();
            let mut running_time = mem::uninitialized();
            let mut stream_time = mem::uninitialized();
            let mut timestamp = mem::uninitialized();
            let mut duration = mem::uninitialized();

            ffi::gst_message_parse_qos(
                self.0.as_mut_ptr(),
                &mut live,
                &mut running_time,
                &mut stream_time,
                &mut timestamp,
                &mut duration,
            );

            (
                from_glib(live),
                running_time,
                stream_time,
                timestamp,
                duration,
            )
        }
    }

    pub fn get_values(&self) -> (i64, f64, i32) {
        unsafe {
            let mut jitter = mem::uninitialized();
            let mut proportion = mem::uninitialized();
            let mut quality = mem::uninitialized();

            ffi::gst_message_parse_qos_values(
                self.0.as_mut_ptr(),
                &mut jitter,
                &mut proportion,
                &mut quality,
            );

            (jitter, proportion, quality)
        }
    }

    pub fn get_stats(&self) -> (::Format, u64, u64) {
        unsafe {
            let mut format = mem::uninitialized();
            let mut processed = mem::uninitialized();
            let mut dropped = mem::uninitialized();

            ffi::gst_message_parse_qos_stats(
                self.0.as_mut_ptr(),
                &mut format,
                &mut processed,
                &mut dropped,
            );

            (from_glib(format), processed, dropped)
        }
    }
}

pub struct Progress<'a>(&'a MessageRef);
impl<'a> Progress<'a> {
    pub fn get(&self) -> (::ProgressType, Option<&'a str>, Option<&'a str>) {
        unsafe {
            let mut type_ = mem::uninitialized();
            let mut code = ptr::null_mut();
            let mut text = ptr::null_mut();

            ffi::gst_message_parse_progress(self.0.as_mut_ptr(), &mut type_, &mut code, &mut text);

            let code = if code.is_null() {
                None
            } else {
                Some(CStr::from_ptr(code).to_str().unwrap())
            };

            let text = if text.is_null() {
                None
            } else {
                Some(CStr::from_ptr(text).to_str().unwrap())
            };

            (from_glib(type_), code, text)
        }
    }
}

pub struct Toc<'a>(&'a MessageRef);
impl<'a> Toc<'a> {
    pub fn get_toc(&self) -> (::Toc, bool) {
        unsafe {
            let mut toc = ptr::null_mut();
            let mut updated = mem::uninitialized();
            ffi::gst_message_parse_toc(self.0.as_mut_ptr(), &mut toc, &mut updated);
            (from_glib_full(toc), from_glib(updated))
        }
    }
}

pub struct ResetTime<'a>(&'a MessageRef);
impl<'a> ResetTime<'a> {
    pub fn get_running_time(&self) -> u64 {
        unsafe {
            let mut running_time = mem::uninitialized();

            ffi::gst_message_parse_reset_time(self.0.as_mut_ptr(), &mut running_time);

            running_time
        }
    }
}

pub struct StreamStart<'a>(&'a MessageRef);
impl<'a> StreamStart<'a> {
    pub fn get_group_id(&self) -> Option<u32> {
        unsafe {
            let mut group_id = mem::uninitialized();

            if from_glib(ffi::gst_message_parse_group_id(
                self.0.as_mut_ptr(),
                &mut group_id,
            )) {
                Some(group_id)
            } else {
                None
            }
        }
    }
}

pub struct NeedContext<'a>(&'a MessageRef);
impl<'a> NeedContext<'a> {
    pub fn get_context_type(&self) -> Option<&str> {
        unsafe {
            let mut context_type = ptr::null();

            if from_glib(ffi::gst_message_parse_context_type(
                self.0.as_mut_ptr(),
                &mut context_type,
            )) && !context_type.is_null()
            {
                Some(CStr::from_ptr(context_type).to_str().unwrap())
            } else {
                None
            }
        }
    }
}

pub struct HaveContext<'a>(&'a MessageRef);
impl<'a> HaveContext<'a> {
    pub fn get_context(&self) -> ::Context {
        unsafe {
            let mut context = ptr::null_mut();
            ffi::gst_message_parse_have_context(self.0.as_mut_ptr(), &mut context);
            from_glib_full(context)
        }
    }
}

pub struct DeviceAdded<'a>(&'a MessageRef);
impl<'a> DeviceAdded<'a> {
    pub fn get_device(&self) -> ::Device {
        unsafe {
            let mut device = ptr::null_mut();

            ffi::gst_message_parse_device_added(self.0.as_mut_ptr(), &mut device);

            from_glib_none(device)
        }
    }
}

pub struct DeviceRemoved<'a>(&'a MessageRef);
impl<'a> DeviceRemoved<'a> {
    pub fn get_device(&self) -> ::Device {
        unsafe {
            let mut device = ptr::null_mut();

            ffi::gst_message_parse_device_removed(self.0.as_mut_ptr(), &mut device);

            from_glib_none(device)
        }
    }
}

pub struct PropertyNotify<'a>(&'a MessageRef);
impl<'a> PropertyNotify<'a> {
    #[cfg(feature = "v1_10")]
    pub fn get(&self) -> (Object, &str, ::Value) {
        unsafe {
            let mut object = ptr::null_mut();
            let mut property_name = ptr::null();
            let mut value = ptr::null();

            ffi::gst_message_parse_property_notify(
                self.0.as_mut_ptr(),
                &mut object,
                &mut property_name,
                &mut value,
            );

            (
                from_glib_none(object),
                CStr::from_ptr(property_name).to_str().unwrap(),
                from_glib_none(value),
            )
        }
    }
}

pub struct StreamCollection<'a>(&'a MessageRef);
impl<'a> StreamCollection<'a> {
    #[cfg(feature = "v1_10")]
    pub fn get_stream_collection(&self) -> ::StreamCollection {
        unsafe {
            let mut collection = ptr::null_mut();

            ffi::gst_message_parse_stream_collection(self.0.as_mut_ptr(), &mut collection);

            from_glib_full(collection)
        }
    }
}
pub struct StreamsSelected<'a>(&'a MessageRef);
impl<'a> StreamsSelected<'a> {
    #[cfg(feature = "v1_10")]
    pub fn get_stream_collection(&self) -> ::StreamCollection {
        unsafe {
            let mut collection = ptr::null_mut();

            ffi::gst_message_parse_streams_selected(self.0.as_mut_ptr(), &mut collection);

            from_glib_full(collection)
        }
    }

    #[cfg(feature = "v1_10")]
    pub fn get_streams(&self) -> Vec<::Stream> {
        unsafe {
            let n = ffi::gst_message_streams_selected_get_size(self.0.as_mut_ptr());

            (0..n)
                .map(|i| {
                    from_glib_full(ffi::gst_message_streams_selected_get_stream(
                        self.0.as_mut_ptr(),
                        i,
                    ))
                })
                .collect()
        }
    }
}

pub struct Redirect<'a>(&'a MessageRef);
impl<'a> Redirect<'a> {
    #[cfg(feature = "v1_10")]
    pub fn get_entries(&self) -> Vec<(&str, Option<TagList>, Option<&StructureRef>)> {
        unsafe {
            let n = ffi::gst_message_get_num_redirect_entries(self.0.as_mut_ptr());

            (0..n)
                .map(|i| {
                    let mut location = ptr::null();
                    let mut tags = ptr::null_mut();
                    let mut structure = ptr::null();

                    ffi::gst_message_parse_redirect_entry(
                        self.0.as_mut_ptr(),
                        i,
                        &mut location,
                        &mut tags,
                        &mut structure,
                    );

                    let structure = if structure.is_null() {
                        None
                    } else {
                        Some(StructureRef::from_glib_borrow(structure))
                    };

                    (
                        CStr::from_ptr(location).to_str().unwrap(),
                        from_glib_none(tags),
                        structure,
                    )
                })
                .collect()
        }
    }
}

macro_rules! message_builder_generic_impl {
    ($new_fn:expr) => {
        pub fn src<T: IsA<Object> + Cast + Clone>(self, src: Option<&T>) -> Self {
            Self {
                src: src.map(|o| {
                    let o = (*o).clone();
                    o.upcast::<Object>()
                }),
                .. self
            }
        }

        pub fn seqnum(self, seqnum: u32) -> Self {
            Self {
                seqnum: Some(seqnum),
                .. self
            }
        }

        pub fn other_fields(self, other_fields: &[(&'a str, &'a ToValue)]) -> Self {
            Self {
                other_fields: self.other_fields.iter().cloned()
                    .chain(other_fields.iter().cloned())
                    .collect(),
                .. self
            }
        }

        pub fn build(mut self) -> Message {
            assert_initialized_main_thread!();
            unsafe {
                let src = self.src.to_glib_none().0;
                let msg = $new_fn(&mut self, src);
                if let Some(seqnum) = self.seqnum {
                    ffi::gst_message_set_seqnum(msg, seqnum);
                }

                {
                    let s = StructureRef::from_glib_borrow_mut(
                        ffi::gst_message_get_structure(msg) as *mut _
                    );

                    for (k, v) in self.other_fields {
                        s.set_value(k, v.to_value());
                    }
                }

                from_glib_full(msg)
            }
        }
    }
}

pub struct EosBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
}
impl<'a> EosBuilder<'a> {
    fn new() -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
        }
    }

    message_builder_generic_impl!(|_, src| ffi::gst_message_new_eos(src));
}

pub struct ErrorBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
    error: &'a glib::Error,
    debug: Option<&'a str>,
    #[allow(unused)] details: Option<Structure>,
}
impl<'a> ErrorBuilder<'a> {
    fn new(error: &'a glib::Error) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            error: error,
            debug: None,
            details: None,
        }
    }

    pub fn debug(self, debug: &'a str) -> Self {
        Self {
            debug: Some(debug),
            ..self
        }
    }

    #[cfg(feature = "v1_10")]
    pub fn details(self, details: Structure) -> Self {
        Self {
            details: Some(details),
            ..self
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        #[cfg(feature = "v1_10")]
        {
            let details = match s.details.take() {
                None => ptr::null_mut(),
                Some(details) => details.into_ptr(),
            };

            ffi::gst_message_new_error_with_details(
                src,
                mut_override(s.error.to_glib_none().0),
                s.debug.to_glib_none().0,
                details,
            )
        }
        #[cfg(not(feature = "v1_10"))]
        {
            ffi::gst_message_new_error(
                src,
                mut_override(s.error.to_glib_none().0),
                s.debug.to_glib_none().0,
            )
        }
    });
}

pub struct WarningBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
    error: &'a glib::Error,
    debug: Option<&'a str>,
    #[allow(unused)] details: Option<Structure>,
}
impl<'a> WarningBuilder<'a> {
    fn new(error: &'a glib::Error) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            error: error,
            debug: None,
            details: None,
        }
    }

    pub fn debug(self, debug: &'a str) -> Self {
        Self {
            debug: Some(debug),
            ..self
        }
    }

    #[cfg(feature = "v1_10")]
    pub fn details(self, details: Structure) -> Self {
        Self {
            details: Some(details),
            ..self
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        #[cfg(feature = "v1_10")]
        {
            let details = match s.details.take() {
                None => ptr::null_mut(),
                Some(details) => details.into_ptr(),
            };

            ffi::gst_message_new_warning_with_details(
                src,
                mut_override(s.error.to_glib_none().0),
                s.debug.to_glib_none().0,
                details,
            )
        }
        #[cfg(not(feature = "v1_10"))]
        {
            ffi::gst_message_new_warning(
                src,
                mut_override(s.error.to_glib_none().0),
                s.debug.to_glib_none().0,
            )
        }
    });
}

pub struct InfoBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
    error: &'a glib::Error,
    debug: Option<&'a str>,
    #[allow(unused)] details: Option<Structure>,
}
impl<'a> InfoBuilder<'a> {
    fn new(error: &'a glib::Error) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            error: error,
            debug: None,
            details: None,
        }
    }

    pub fn debug(self, debug: &'a str) -> Self {
        Self {
            debug: Some(debug),
            ..self
        }
    }

    #[cfg(feature = "v1_10")]
    pub fn details(self, details: Structure) -> Self {
        Self {
            details: Some(details),
            ..self
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        #[cfg(feature = "v1_10")]
        {
            let details = match s.details.take() {
                None => ptr::null_mut(),
                Some(details) => details.into_ptr(),
            };

            ffi::gst_message_new_info_with_details(
                src,
                mut_override(s.error.to_glib_none().0),
                s.debug.to_glib_none().0,
                details,
            )
        }
        #[cfg(not(feature = "v1_10"))]
        {
            ffi::gst_message_new_info(
                src,
                mut_override(s.error.to_glib_none().0),
                s.debug.to_glib_none().0,
            )
        }
    });
}

pub struct TagBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
    tags: &'a TagList,
}
impl<'a> TagBuilder<'a> {
    fn new(tags: &'a TagList) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            tags: tags,
        }
    }

    message_builder_generic_impl!(|s: &Self, src| {
        ffi::gst_message_new_tag(src, s.tags.to_glib_full())
    });
}

pub struct BufferingBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
    percent: i32,
    stats: Option<(::BufferingMode, i32, i32, i64)>,
}
impl<'a> BufferingBuilder<'a> {
    fn new(percent: i32) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            percent: percent,
            stats: None,
        }
    }

    pub fn stats(
        self,
        mode: ::BufferingMode,
        avg_in: i32,
        avg_out: i32,
        buffering_left: i64,
    ) -> Self {
        skip_assert_initialized!();
        Self {
            stats: Some((mode, avg_in, avg_out, buffering_left)),
            ..self
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        let msg = ffi::gst_message_new_buffering(src, s.percent);

        if let Some((mode, avg_in, avg_out, buffering_left)) = s.stats {
            ffi::gst_message_set_buffering_stats(
                msg,
                mode.to_glib(),
                avg_in,
                avg_out,
                buffering_left,
            );
        }

        msg
    });
}

pub struct StateChangedBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
    old: ::State,
    new: ::State,
    pending: ::State,
}
impl<'a> StateChangedBuilder<'a> {
    fn new(old: ::State, new: ::State, pending: ::State) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            old: old,
            new: new,
            pending: pending,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        ffi::gst_message_new_state_changed(
            src,
            s.old.to_glib(),
            s.new.to_glib(),
            s.pending.to_glib(),
        )
    });
}

pub struct StateDirtyBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
}
impl<'a> StateDirtyBuilder<'a> {
    fn new() -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
        }
    }

    message_builder_generic_impl!(|_, src| ffi::gst_message_new_state_dirty(src));
}

pub struct StepDoneBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
    format: ::Format,
    amount: u64,
    rate: f64,
    flush: bool,
    intermediate: bool,
    duration: u64,
    eos: bool,
}
impl<'a> StepDoneBuilder<'a> {
    fn new(
        format: ::Format,
        amount: u64,
        rate: f64,
        flush: bool,
        intermediate: bool,
        duration: u64,
        eos: bool,
    ) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            format: format,
            amount: amount,
            rate: rate,
            flush: flush,
            intermediate: intermediate,
            duration: duration,
            eos: eos,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        ffi::gst_message_new_step_done(
            src,
            s.format.to_glib(),
            s.amount,
            s.rate,
            s.flush.to_glib(),
            s.intermediate.to_glib(),
            s.duration,
            s.eos.to_glib(),
        )
    });
}

pub struct ClockProvideBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
    clock: &'a ::Clock,
    ready: bool,
}
impl<'a> ClockProvideBuilder<'a> {
    fn new(clock: &'a ::Clock, ready: bool) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            clock: clock,
            ready: ready,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        ffi::gst_message_new_clock_provide(src, s.clock.to_glib_none().0, s.ready.to_glib())
    });
}

pub struct ClockLostBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
    clock: &'a ::Clock,
}
impl<'a> ClockLostBuilder<'a> {
    fn new(clock: &'a ::Clock) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            clock: clock,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        ffi::gst_message_new_clock_lost(src, s.clock.to_glib_none().0)
    });
}

pub struct NewClockBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
    clock: &'a ::Clock,
}
impl<'a> NewClockBuilder<'a> {
    fn new(clock: &'a ::Clock) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            clock: clock,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        ffi::gst_message_new_new_clock(src, s.clock.to_glib_none().0)
    });
}

pub struct StructureChangeBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
    type_: ::StructureChangeType,
    owner: &'a ::Element,
    busy: bool,
}
impl<'a> StructureChangeBuilder<'a> {
    fn new(type_: ::StructureChangeType, owner: &'a ::Element, busy: bool) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            type_: type_,
            owner: owner,
            busy: busy,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        ffi::gst_message_new_structure_change(
            src,
            s.type_.to_glib(),
            s.owner.to_glib_none().0,
            s.busy.to_glib(),
        )
    });
}

pub struct StreamStatusBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
    type_: ::StreamStatusType,
    owner: &'a ::Element,
    status_object: Option<&'a glib::Value>,
}
impl<'a> StreamStatusBuilder<'a> {
    fn new(type_: ::StreamStatusType, owner: &'a ::Element) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            type_: type_,
            owner: owner,
            status_object: None,
        }
    }

    pub fn status_object(self, status_object: &'a glib::Value) -> Self {
        Self {
            status_object: Some(status_object),
            ..self
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        let msg =
            ffi::gst_message_new_stream_status(src, s.type_.to_glib(), s.owner.to_glib_none().0);
        if let Some(status_object) = s.status_object {
            ffi::gst_message_set_stream_status_object(msg, status_object.to_glib_none().0);
        }
        msg
    });
}

pub struct ApplicationBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
    structure: Option<::Structure>,
}
impl<'a> ApplicationBuilder<'a> {
    fn new(structure: ::Structure) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            structure: Some(structure),
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        ffi::gst_message_new_application(src, s.structure.take().unwrap().into_ptr())
    });
}

pub struct ElementBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
    structure: Option<::Structure>,
}
impl<'a> ElementBuilder<'a> {
    fn new(structure: ::Structure) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            structure: Some(structure),
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        ffi::gst_message_new_element(src, s.structure.take().unwrap().into_ptr())
    });
}

pub struct SegmentStartBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
    format: ::Format,
    position: i64,
}
impl<'a> SegmentStartBuilder<'a> {
    fn new(format: ::Format, position: i64) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            format: format,
            position: position,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        ffi::gst_message_new_segment_start(src, s.format.to_glib(), s.position)
    });
}

pub struct SegmentDoneBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
    format: ::Format,
    position: i64,
}
impl<'a> SegmentDoneBuilder<'a> {
    fn new(format: ::Format, position: i64) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            format: format,
            position: position,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        ffi::gst_message_new_segment_done(src, s.format.to_glib(), s.position)
    });
}

pub struct DurationChangedBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
}
impl<'a> DurationChangedBuilder<'a> {
    fn new() -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
        }
    }

    message_builder_generic_impl!(|_, src| ffi::gst_message_new_duration_changed(src));
}

pub struct LatencyBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
}
impl<'a> LatencyBuilder<'a> {
    fn new() -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
        }
    }

    message_builder_generic_impl!(|_, src| ffi::gst_message_new_latency(src));
}

pub struct AsyncStartBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
}
impl<'a> AsyncStartBuilder<'a> {
    fn new() -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
        }
    }

    message_builder_generic_impl!(|_, src| ffi::gst_message_new_async_start(src));
}

pub struct AsyncDoneBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
    running_time: u64,
}
impl<'a> AsyncDoneBuilder<'a> {
    fn new(running_time: u64) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            running_time: running_time,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        ffi::gst_message_new_async_done(src, s.running_time)
    });
}

pub struct RequestStateBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
    state: ::State,
}
impl<'a> RequestStateBuilder<'a> {
    fn new(state: ::State) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            state: state,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        ffi::gst_message_new_request_state(src, s.state.to_glib())
    });
}

pub struct StepStartBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
    active: bool,
    format: ::Format,
    amount: u64,
    rate: f64,
    flush: bool,
    intermediate: bool,
}
impl<'a> StepStartBuilder<'a> {
    fn new(
        active: bool,
        format: ::Format,
        amount: u64,
        rate: f64,
        flush: bool,
        intermediate: bool,
    ) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            active: active,
            format: format,
            amount: amount,
            rate: rate,
            flush: flush,
            intermediate: intermediate,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        ffi::gst_message_new_step_start(
            src,
            s.active.to_glib(),
            s.format.to_glib(),
            s.amount,
            s.rate,
            s.flush.to_glib(),
            s.intermediate.to_glib(),
        )
    });
}

pub struct QosBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
    live: bool,
    running_time: u64,
    stream_time: u64,
    timestamp: u64,
    duration: u64,
    values: Option<(i64, f64, i32)>,
    stats: Option<(::Format, u64, u64)>,
}
impl<'a> QosBuilder<'a> {
    fn new(live: bool, running_time: u64, stream_time: u64, timestamp: u64, duration: u64) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            live: live,
            running_time: running_time,
            stream_time: stream_time,
            timestamp: timestamp,
            duration: duration,
            values: None,
            stats: None,
        }
    }

    pub fn values(self, jitter: i64, proportion: f64, quality: i32) -> Self {
        Self {
            values: Some((jitter, proportion, quality)),
            ..self
        }
    }

    pub fn stats(self, format: ::Format, processed: u64, dropped: u64) -> Self {
        Self {
            stats: Some((format, processed, dropped)),
            ..self
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        let msg = ffi::gst_message_new_qos(
            src,
            s.live.to_glib(),
            s.running_time,
            s.stream_time,
            s.timestamp,
            s.duration,
        );
        if let Some((jitter, proportion, quality)) = s.values {
            ffi::gst_message_set_qos_values(msg, jitter, proportion, quality);
        }
        if let Some((format, processed, dropped)) = s.stats {
            ffi::gst_message_set_qos_stats(msg, format.to_glib(), processed, dropped);
        }
        msg
    });
}

pub struct ProgressBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
    type_: ::ProgressType,
    code: Option<&'a str>,
    text: Option<&'a str>,
}
impl<'a> ProgressBuilder<'a> {
    fn new(type_: ::ProgressType) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            type_: type_,
            code: None,
            text: None,
        }
    }

    pub fn code(self, code: &'a str) -> Self {
        Self {
            code: Some(code),
            ..self
        }
    }

    pub fn text(self, text: &'a str) -> Self {
        Self {
            text: Some(text),
            ..self
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        ffi::gst_message_new_progress(
            src,
            s.type_.to_glib(),
            s.code.to_glib_none().0,
            s.text.to_glib_none().0,
        )
    });
}

pub struct TocBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
    toc: &'a ::Toc,
    updated: bool,
}
impl<'a> TocBuilder<'a> {
    fn new(toc: &'a ::Toc, updated: bool) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            toc: toc,
            updated: updated,
        }
    }

    message_builder_generic_impl!(|s: &Self, src| {
        ffi::gst_message_new_toc(src, s.toc.to_glib_none().0, s.updated.to_glib())
    });
}

pub struct ResetTimeBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
    running_time: u64,
}
impl<'a> ResetTimeBuilder<'a> {
    fn new(running_time: u64) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            running_time: running_time,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        ffi::gst_message_new_reset_time(src, s.running_time)
    });
}

pub struct StreamStartBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
    group_id: Option<u32>,
}
impl<'a> StreamStartBuilder<'a> {
    fn new() -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            group_id: None,
        }
    }

    pub fn group_id(self, group_id: u32) -> Self {
        Self {
            group_id: Some(group_id),
            ..self
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        let msg = ffi::gst_message_new_stream_start(src);
        if let Some(group_id) = s.group_id {
            ffi::gst_message_set_group_id(msg, group_id);
        }
        msg
    });
}

pub struct NeedContextBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
    context_type: &'a str,
}
impl<'a> NeedContextBuilder<'a> {
    fn new(context_type: &'a str) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            context_type: context_type,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        ffi::gst_message_new_need_context(src, s.context_type.to_glib_none().0)
    });
}

pub struct HaveContextBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
    context: Option<::Context>,
}
impl<'a> HaveContextBuilder<'a> {
    fn new(context: ::Context) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            context: Some(context),
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        let context = s.context.take().unwrap();
        ffi::gst_message_new_have_context(src, context.into_ptr())
    });
}

pub struct DeviceAddedBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
    device: &'a ::Device,
}
impl<'a> DeviceAddedBuilder<'a> {
    fn new(device: &'a ::Device) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            device: device,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        ffi::gst_message_new_device_added(src, s.device.to_glib_none().0)
    });
}

pub struct DeviceRemovedBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
    device: &'a ::Device,
}
impl<'a> DeviceRemovedBuilder<'a> {
    fn new(device: &'a ::Device) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            device: device,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        ffi::gst_message_new_device_removed(src, s.device.to_glib_none().0)
    });
}

#[cfg(feature = "v1_10")]
pub struct PropertyNotifyBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
    property_name: &'a str,
    value: &'a glib::Value,
}
#[cfg(feature = "v1_10")]
impl<'a> PropertyNotifyBuilder<'a> {
    fn new(property_name: &'a str, value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            property_name: property_name,
            value: value,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        ffi::gst_message_new_property_notify(
            src,
            s.property_name.to_glib_none().0,
            mut_override(s.value.to_glib_none().0),
        )
    });
}

#[cfg(feature = "v1_10")]
pub struct StreamCollectionBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
    collection: &'a ::StreamCollection,
}
#[cfg(feature = "v1_10")]
impl<'a> StreamCollectionBuilder<'a> {
    fn new(collection: &'a ::StreamCollection) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            collection: collection,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        ffi::gst_message_new_stream_collection(src, s.collection.to_glib_none().0)
    });
}

#[cfg(feature = "v1_10")]
pub struct StreamsSelectedBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
    #[cfg(feature = "v1_10")] collection: &'a ::StreamCollection,
    #[cfg(feature = "v1_10")] streams: Option<&'a [&'a ::Stream]>,
}
#[cfg(feature = "v1_10")]
impl<'a> StreamsSelectedBuilder<'a> {
    fn new(collection: &'a ::StreamCollection) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            collection: collection,
            streams: None,
        }
    }

    pub fn streams(self, streams: &'a [&'a ::Stream]) -> Self {
        Self {
            streams: Some(streams),
            ..self
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        let msg = ffi::gst_message_new_streams_selected(src, s.collection.to_glib_none().0);
        if let Some(streams) = s.streams {
            for stream in streams {
                ffi::gst_message_streams_selected_add(msg, stream.to_glib_none().0);
            }
        }
        msg
    });
}

#[cfg(feature = "v1_10")]
pub struct RedirectBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<u32>,
    other_fields: Vec<(&'a str, &'a ToValue)>,
    location: &'a str,
    tag_list: Option<&'a TagList>,
    entry_struct: Option<Structure>,
    #[cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
    entries:
        Option<&'a [(&'a str, Option<&'a TagList>, Option<&'a Structure>)]>,
}
#[cfg(feature = "v1_10")]
impl<'a> RedirectBuilder<'a> {
    fn new(
        location: &'a str,
        tag_list: Option<&'a TagList>,
        entry_struct: Option<Structure>,
    ) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            location: location,
            tag_list: tag_list,
            entry_struct: entry_struct,
            entries: None,
        }
    }

    pub fn entries(
        self,
        entries: &'a [(&'a str, Option<&'a TagList>, Option<&'a Structure>)],
    ) -> Self {
        skip_assert_initialized!();
        Self {
            entries: Some(entries),
            ..self
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        let entry_struct = s.entry_struct.take();
        let entry_struct_ptr = if let Some(entry_struct) = entry_struct {
            entry_struct.into_ptr()
        } else {
            ptr::null_mut()
        };

        let msg = ffi::gst_message_new_redirect(
            src,
            s.location.to_glib_none().0,
            s.tag_list.to_glib_full(),
            entry_struct_ptr,
        );
        if let Some(entries) = s.entries {
            for &(location, tag_list, entry_struct) in entries {
                let entry_struct = entry_struct.cloned();
                let entry_struct_ptr = if let Some(entry_struct) = entry_struct {
                    entry_struct.into_ptr()
                } else {
                    ptr::null_mut()
                };
                ffi::gst_message_add_redirect_entry(
                    msg,
                    location.to_glib_none().0,
                    tag_list.to_glib_full(),
                    entry_struct_ptr,
                );
            }
        }
        msg
    });
}
