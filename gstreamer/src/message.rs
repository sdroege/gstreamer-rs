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
use GstObjectExt;
use Seqnum;
use GroupId;
use GenericFormattedValue;

use std::ptr;
use std::mem;
use std::fmt;
use std::ffi::CStr;
use std::ops::Deref;

use glib;
use glib::Cast;
use glib::IsA;
use glib::value::ToSendValue;
use glib::translate::{from_glib, from_glib_full, from_glib_none, mut_override, ToGlib, ToGlibPtr};

#[repr(C)]
pub struct MessageRef(ffi::GstMessage);

pub type Message = GstRc<MessageRef>;

unsafe impl Sync for MessageRef {}
unsafe impl Send for MessageRef {}

unsafe impl MiniObject for MessageRef {
    type GstType = ffi::GstMessage;
}

impl MessageRef {
    pub fn get_src(&self) -> Option<Object> {
        unsafe { from_glib_none((*self.as_ptr()).src) }
    }

    pub fn get_seqnum(&self) -> Seqnum {
        unsafe { from_glib(ffi::gst_message_get_seqnum(self.as_mut_ptr())) }
    }

    pub fn get_structure(&self) -> Option<&StructureRef> {
        unsafe {
            let structure = ffi::gst_message_get_structure(self.as_mut_ptr());
            if structure.is_null() {
                None
            } else {
                Some(StructureRef::from_glib_borrow(structure))
            }
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

    pub fn new_error<T: MessageErrorDomain>(error: T, message: &str) -> ErrorBuilder<T> {
        assert_initialized_main_thread!();
        ErrorBuilder::new(error, message)
    }

    pub fn new_warning<T: MessageErrorDomain>(error: T, message: &str) -> WarningBuilder<T> {
        assert_initialized_main_thread!();
        WarningBuilder::new(error, message)
    }

    pub fn new_info<T: MessageErrorDomain>(error: T, message: &str) -> InfoBuilder<T> {
        assert_initialized_main_thread!();
        InfoBuilder::new(error, message)
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

    pub fn new_step_done<'a, V: Into<GenericFormattedValue>>(
        amount: V,
        rate: f64,
        flush: bool,
        intermediate: bool,
        duration: V,
        eos: bool,
    ) -> StepDoneBuilder<'a> {
        assert_initialized_main_thread!();
        StepDoneBuilder::new(
            amount.into(),
            rate,
            flush,
            intermediate,
            duration.into(),
            eos,
        )
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

    pub fn new_segment_start<'a, V: Into<GenericFormattedValue>>(
        position: V,
    ) -> SegmentStartBuilder<'a> {
        assert_initialized_main_thread!();
        let position = position.into();
        SegmentStartBuilder::new(position)
    }

    pub fn new_segment_done<'a, V: Into<GenericFormattedValue>>(
        position: V,
    ) -> SegmentDoneBuilder<'a> {
        assert_initialized_main_thread!();
        let position = position.into();
        SegmentDoneBuilder::new(position)
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

    pub fn new_async_done<'a>(running_time: ::ClockTime) -> AsyncDoneBuilder<'a> {
        assert_initialized_main_thread!();
        AsyncDoneBuilder::new(running_time)
    }

    pub fn new_request_state<'a>(state: ::State) -> RequestStateBuilder<'a> {
        assert_initialized_main_thread!();
        RequestStateBuilder::new(state)
    }

    pub fn new_step_start<'a, V: Into<GenericFormattedValue>>(
        active: bool,
        amount: V,
        rate: f64,
        flush: bool,
        intermediate: bool,
    ) -> StepStartBuilder<'a> {
        assert_initialized_main_thread!();
        StepStartBuilder::new(active, amount.into(), rate, flush, intermediate)
    }

    pub fn new_qos_builder<'a>(
        live: bool,
        running_time: ::ClockTime,
        stream_time: ::ClockTime,
        timestamp: ::ClockTime,
        duration: ::ClockTime,
    ) -> QosBuilder<'a> {
        assert_initialized_main_thread!();
        QosBuilder::new(live, running_time, stream_time, timestamp, duration)
    }

    pub fn new_progress<'a>(
        type_: ::ProgressType,
        code: &'a str,
        text: &'a str,
    ) -> ProgressBuilder<'a> {
        assert_initialized_main_thread!();
        ProgressBuilder::new(type_, code, text)
    }

    pub fn new_toc(toc: &::Toc, updated: bool) -> TocBuilder {
        assert_initialized_main_thread!();
        TocBuilder::new(toc, updated)
    }

    pub fn new_reset_time<'a>(running_time: ::ClockTime) -> ResetTimeBuilder<'a> {
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

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn new_property_notify(property_name: &str) -> PropertyNotifyBuilder {
        assert_initialized_main_thread!();
        PropertyNotifyBuilder::new(property_name)
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn new_stream_collection(collection: &::StreamCollection) -> StreamCollectionBuilder {
        assert_initialized_main_thread!();
        StreamCollectionBuilder::new(collection)
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn new_streams_selected(collection: &::StreamCollection) -> StreamsSelectedBuilder {
        assert_initialized_main_thread!();
        StreamsSelectedBuilder::new(collection)
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn new_redirect(location: &str) -> RedirectBuilder {
        assert_initialized_main_thread!();
        RedirectBuilder::new(location)
    }
}

impl glib::types::StaticType for MessageRef {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(ffi::gst_message_get_type()) }
    }
}

impl fmt::Debug for MessageRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Message")
            .field("type", &unsafe {
                let type_ = ffi::gst_message_type_get_name((*self.as_ptr()).type_);
                CStr::from_ptr(type_).to_str().unwrap()
            })
            .field("seqnum", &self.get_seqnum())
            .field("src", &self.get_src().map(|s| s.get_name()))
            .field("structure", &self.get_structure())
            .finish()
    }
}

impl ToOwned for MessageRef {
    type Owned = GstRc<MessageRef>;

    fn to_owned(&self) -> GstRc<MessageRef> {
        unsafe {
            from_glib_full(ffi::gst_mini_object_copy(self.as_ptr() as *const _)
                as *mut _)
        }
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

macro_rules! declare_concrete_message(
    ($name:ident) => {
        pub struct $name<'a>(&'a MessageRef);

        impl<'a> Deref for $name<'a> {
            type Target = MessageRef;

            fn deref(&self) -> &Self::Target {
                self.0
            }
        }
    }
);

declare_concrete_message!(Eos);

declare_concrete_message!(Error);
impl<'a> Error<'a> {
    pub fn get_error(&self) -> glib::Error {
        unsafe {
            let mut error = ptr::null_mut();

            ffi::gst_message_parse_error(self.as_mut_ptr(), &mut error, ptr::null_mut());

            from_glib_full(error)
        }
    }

    pub fn get_debug(&self) -> Option<String> {
        unsafe {
            let mut debug = ptr::null_mut();

            ffi::gst_message_parse_error(self.as_mut_ptr(), ptr::null_mut(), &mut debug);

            from_glib_full(debug)
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_details(&self) -> Option<&StructureRef> {
        unsafe {
            let mut details = ptr::null();

            ffi::gst_message_parse_error_details(self.as_mut_ptr(), &mut details);

            if details.is_null() {
                None
            } else {
                Some(StructureRef::from_glib_borrow(details))
            }
        }
    }
}

declare_concrete_message!(Warning);
impl<'a> Warning<'a> {
    pub fn get_error(&self) -> glib::Error {
        unsafe {
            let mut error = ptr::null_mut();

            ffi::gst_message_parse_warning(self.as_mut_ptr(), &mut error, ptr::null_mut());

            from_glib_full(error)
        }
    }

    pub fn get_debug(&self) -> Option<String> {
        unsafe {
            let mut debug = ptr::null_mut();

            ffi::gst_message_parse_warning(self.as_mut_ptr(), ptr::null_mut(), &mut debug);

            from_glib_full(debug)
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_details(&self) -> Option<&StructureRef> {
        unsafe {
            let mut details = ptr::null();

            ffi::gst_message_parse_error_details(self.as_mut_ptr(), &mut details);

            if details.is_null() {
                None
            } else {
                Some(StructureRef::from_glib_borrow(details))
            }
        }
    }
}

declare_concrete_message!(Info);
impl<'a> Info<'a> {
    pub fn get_error(&self) -> glib::Error {
        unsafe {
            let mut error = ptr::null_mut();

            ffi::gst_message_parse_info(self.as_mut_ptr(), &mut error, ptr::null_mut());

            from_glib_full(error)
        }
    }

    pub fn get_debug(&self) -> Option<String> {
        unsafe {
            let mut debug = ptr::null_mut();

            ffi::gst_message_parse_info(self.as_mut_ptr(), ptr::null_mut(), &mut debug);

            from_glib_full(debug)
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_details(&self) -> Option<&StructureRef> {
        unsafe {
            let mut details = ptr::null();

            ffi::gst_message_parse_error_details(self.as_mut_ptr(), &mut details);

            if details.is_null() {
                None
            } else {
                Some(StructureRef::from_glib_borrow(details))
            }
        }
    }
}

declare_concrete_message!(Tag);
impl<'a> Tag<'a> {
    pub fn get_tags(&self) -> TagList {
        unsafe {
            let mut tags = ptr::null_mut();
            ffi::gst_message_parse_tag(self.as_mut_ptr(), &mut tags);
            from_glib_full(tags)
        }
    }
}

declare_concrete_message!(Buffering);
impl<'a> Buffering<'a> {
    pub fn get_percent(&self) -> i32 {
        unsafe {
            let mut p = mem::uninitialized();
            ffi::gst_message_parse_buffering(self.as_mut_ptr(), &mut p);
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
                self.as_mut_ptr(),
                &mut mode,
                &mut avg_in,
                &mut avg_out,
                &mut buffering_left,
            );

            (from_glib(mode), avg_in, avg_out, buffering_left)
        }
    }
}

declare_concrete_message!(StateChanged);
impl<'a> StateChanged<'a> {
    pub fn get_old(&self) -> ::State {
        unsafe {
            let mut state = mem::uninitialized();

            ffi::gst_message_parse_state_changed(
                self.as_mut_ptr(),
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
                self.as_mut_ptr(),
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
                self.as_mut_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
                &mut state,
            );

            from_glib(state)
        }
    }
}

declare_concrete_message!(StateDirty);

declare_concrete_message!(StepDone);
impl<'a> StepDone<'a> {
    pub fn get(
        &self,
    ) -> (
        GenericFormattedValue,
        f64,
        bool,
        bool,
        GenericFormattedValue,
        bool,
    ) {
        unsafe {
            let mut format = mem::uninitialized();
            let mut amount = mem::uninitialized();
            let mut rate = mem::uninitialized();
            let mut flush = mem::uninitialized();
            let mut intermediate = mem::uninitialized();
            let mut duration = mem::uninitialized();
            let mut eos = mem::uninitialized();

            ffi::gst_message_parse_step_done(
                self.as_mut_ptr(),
                &mut format,
                &mut amount,
                &mut rate,
                &mut flush,
                &mut intermediate,
                &mut duration,
                &mut eos,
            );

            (
                GenericFormattedValue::new(from_glib(format), amount as i64),
                rate,
                from_glib(flush),
                from_glib(intermediate),
                GenericFormattedValue::new(from_glib(format), duration as i64),
                from_glib(eos),
            )
        }
    }
}

declare_concrete_message!(ClockProvide);
impl<'a> ClockProvide<'a> {
    pub fn get_clock(&self) -> Option<::Clock> {
        let mut clock = ptr::null_mut();

        unsafe {
            ffi::gst_message_parse_clock_provide(self.as_mut_ptr(), &mut clock, ptr::null_mut());

            from_glib_none(clock)
        }
    }

    pub fn get_ready(&self) -> bool {
        unsafe {
            let mut ready = mem::uninitialized();

            ffi::gst_message_parse_clock_provide(self.as_mut_ptr(), ptr::null_mut(), &mut ready);

            from_glib(ready)
        }
    }
}

declare_concrete_message!(ClockLost);
impl<'a> ClockLost<'a> {
    pub fn get_clock(&self) -> Option<::Clock> {
        let mut clock = ptr::null_mut();

        unsafe {
            ffi::gst_message_parse_clock_lost(self.as_mut_ptr(), &mut clock);

            from_glib_none(clock)
        }
    }
}

declare_concrete_message!(NewClock);
impl<'a> NewClock<'a> {
    pub fn get_clock(&self) -> Option<::Clock> {
        let mut clock = ptr::null_mut();

        unsafe {
            ffi::gst_message_parse_new_clock(self.as_mut_ptr(), &mut clock);

            from_glib_none(clock)
        }
    }
}

declare_concrete_message!(StructureChange);
impl<'a> StructureChange<'a> {
    pub fn get(&self) -> (::StructureChangeType, ::Element, bool) {
        unsafe {
            let mut type_ = mem::uninitialized();
            let mut owner = ptr::null_mut();
            let mut busy = mem::uninitialized();

            ffi::gst_message_parse_structure_change(
                self.as_mut_ptr(),
                &mut type_,
                &mut owner,
                &mut busy,
            );

            (from_glib(type_), from_glib_none(owner), from_glib(busy))
        }
    }
}

declare_concrete_message!(StreamStatus);
impl<'a> StreamStatus<'a> {
    pub fn get(&self) -> (::StreamStatusType, ::Element) {
        unsafe {
            let mut type_ = mem::uninitialized();
            let mut owner = ptr::null_mut();

            ffi::gst_message_parse_stream_status(self.as_mut_ptr(), &mut type_, &mut owner);

            (from_glib(type_), from_glib_none(owner))
        }
    }

    pub fn get_stream_status_object(&self) -> Option<glib::Value> {
        unsafe {
            let value = ffi::gst_message_get_stream_status_object(self.as_mut_ptr());

            from_glib_none(value)
        }
    }
}

declare_concrete_message!(Application);

declare_concrete_message!(Element);

declare_concrete_message!(SegmentStart);
impl<'a> SegmentStart<'a> {
    pub fn get(&self) -> GenericFormattedValue {
        unsafe {
            let mut format = mem::uninitialized();
            let mut position = mem::uninitialized();

            ffi::gst_message_parse_segment_start(self.as_mut_ptr(), &mut format, &mut position);

            GenericFormattedValue::new(from_glib(format), position)
        }
    }
}

declare_concrete_message!(SegmentDone);
impl<'a> SegmentDone<'a> {
    pub fn get(&self) -> GenericFormattedValue {
        unsafe {
            let mut format = mem::uninitialized();
            let mut position = mem::uninitialized();

            ffi::gst_message_parse_segment_done(self.as_mut_ptr(), &mut format, &mut position);

            GenericFormattedValue::new(from_glib(format), position)
        }
    }
}

declare_concrete_message!(DurationChanged);
declare_concrete_message!(Latency);
declare_concrete_message!(AsyncStart);

declare_concrete_message!(AsyncDone);
impl<'a> AsyncDone<'a> {
    pub fn get_running_time(&self) -> ::ClockTime {
        unsafe {
            let mut running_time = mem::uninitialized();

            ffi::gst_message_parse_async_done(self.as_mut_ptr(), &mut running_time);

            from_glib(running_time)
        }
    }
}

declare_concrete_message!(RequestState);
impl<'a> RequestState<'a> {
    pub fn get_requested_state(&self) -> ::State {
        unsafe {
            let mut state = mem::uninitialized();

            ffi::gst_message_parse_request_state(self.as_mut_ptr(), &mut state);

            from_glib(state)
        }
    }
}

declare_concrete_message!(StepStart);
impl<'a> StepStart<'a> {
    pub fn get(&self) -> (bool, GenericFormattedValue, f64, bool, bool) {
        unsafe {
            let mut active = mem::uninitialized();
            let mut format = mem::uninitialized();
            let mut amount = mem::uninitialized();
            let mut rate = mem::uninitialized();
            let mut flush = mem::uninitialized();
            let mut intermediate = mem::uninitialized();

            ffi::gst_message_parse_step_start(
                self.as_mut_ptr(),
                &mut active,
                &mut format,
                &mut amount,
                &mut rate,
                &mut flush,
                &mut intermediate,
            );

            (
                from_glib(active),
                GenericFormattedValue::new(from_glib(format), amount as i64),
                rate,
                from_glib(flush),
                from_glib(intermediate),
            )
        }
    }
}

declare_concrete_message!(Qos);
impl<'a> Qos<'a> {
    pub fn get(&self) -> (bool, ::ClockTime, ::ClockTime, ::ClockTime, ::ClockTime) {
        unsafe {
            let mut live = mem::uninitialized();
            let mut running_time = mem::uninitialized();
            let mut stream_time = mem::uninitialized();
            let mut timestamp = mem::uninitialized();
            let mut duration = mem::uninitialized();

            ffi::gst_message_parse_qos(
                self.as_mut_ptr(),
                &mut live,
                &mut running_time,
                &mut stream_time,
                &mut timestamp,
                &mut duration,
            );

            (
                from_glib(live),
                from_glib(running_time),
                from_glib(stream_time),
                from_glib(timestamp),
                from_glib(duration),
            )
        }
    }

    pub fn get_values(&self) -> (i64, f64, i32) {
        unsafe {
            let mut jitter = mem::uninitialized();
            let mut proportion = mem::uninitialized();
            let mut quality = mem::uninitialized();

            ffi::gst_message_parse_qos_values(
                self.as_mut_ptr(),
                &mut jitter,
                &mut proportion,
                &mut quality,
            );

            (jitter, proportion, quality)
        }
    }

    pub fn get_stats(&self) -> (GenericFormattedValue, GenericFormattedValue) {
        unsafe {
            let mut format = mem::uninitialized();
            let mut processed = mem::uninitialized();
            let mut dropped = mem::uninitialized();

            ffi::gst_message_parse_qos_stats(
                self.as_mut_ptr(),
                &mut format,
                &mut processed,
                &mut dropped,
            );

            (
                GenericFormattedValue::new(from_glib(format), processed as i64),
                GenericFormattedValue::new(from_glib(format), dropped as i64),
            )
        }
    }
}

declare_concrete_message!(Progress);
impl<'a> Progress<'a> {
    pub fn get(&self) -> (::ProgressType, &'a str, &'a str) {
        unsafe {
            let mut type_ = mem::uninitialized();
            let mut code = ptr::null_mut();
            let mut text = ptr::null_mut();

            ffi::gst_message_parse_progress(self.as_mut_ptr(), &mut type_, &mut code, &mut text);

            let code = CStr::from_ptr(code).to_str().unwrap();
            let text = CStr::from_ptr(text).to_str().unwrap();

            (from_glib(type_), code, text)
        }
    }
}

declare_concrete_message!(Toc);
impl<'a> Toc<'a> {
    pub fn get_toc(&self) -> (::Toc, bool) {
        unsafe {
            let mut toc = ptr::null_mut();
            let mut updated = mem::uninitialized();
            ffi::gst_message_parse_toc(self.as_mut_ptr(), &mut toc, &mut updated);
            (from_glib_full(toc), from_glib(updated))
        }
    }
}

declare_concrete_message!(ResetTime);
impl<'a> ResetTime<'a> {
    pub fn get_running_time(&self) -> ::ClockTime {
        unsafe {
            let mut running_time = mem::uninitialized();

            ffi::gst_message_parse_reset_time(self.as_mut_ptr(), &mut running_time);

            from_glib(running_time)
        }
    }
}

declare_concrete_message!(StreamStart);
impl<'a> StreamStart<'a> {
    pub fn get_group_id(&self) -> Option<GroupId> {
        unsafe {
            let mut group_id = mem::uninitialized();

            if from_glib(ffi::gst_message_parse_group_id(
                self.as_mut_ptr(),
                &mut group_id,
            )) {
                Some(from_glib(group_id))
            } else {
                None
            }
        }
    }
}

declare_concrete_message!(NeedContext);
impl<'a> NeedContext<'a> {
    pub fn get_context_type(&self) -> &str {
        unsafe {
            let mut context_type = ptr::null();

            ffi::gst_message_parse_context_type(self.as_mut_ptr(), &mut context_type);

            CStr::from_ptr(context_type).to_str().unwrap()
        }
    }
}

declare_concrete_message!(HaveContext);
impl<'a> HaveContext<'a> {
    pub fn get_context(&self) -> ::Context {
        unsafe {
            let mut context = ptr::null_mut();
            ffi::gst_message_parse_have_context(self.as_mut_ptr(), &mut context);
            from_glib_full(context)
        }
    }
}

declare_concrete_message!(DeviceAdded);
impl<'a> DeviceAdded<'a> {
    pub fn get_device(&self) -> ::Device {
        unsafe {
            let mut device = ptr::null_mut();

            ffi::gst_message_parse_device_added(self.as_mut_ptr(), &mut device);

            from_glib_none(device)
        }
    }
}

declare_concrete_message!(DeviceRemoved);
impl<'a> DeviceRemoved<'a> {
    pub fn get_device(&self) -> ::Device {
        unsafe {
            let mut device = ptr::null_mut();

            ffi::gst_message_parse_device_removed(self.as_mut_ptr(), &mut device);

            from_glib_none(device)
        }
    }
}

declare_concrete_message!(PropertyNotify);
impl<'a> PropertyNotify<'a> {
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get(&self) -> (Object, &str, Option<&'a ::Value>) {
        unsafe {
            let mut object = ptr::null_mut();
            let mut property_name = ptr::null();
            let mut value = ptr::null();

            ffi::gst_message_parse_property_notify(
                self.as_mut_ptr(),
                &mut object,
                &mut property_name,
                &mut value,
            );

            (
                from_glib_none(object),
                CStr::from_ptr(property_name).to_str().unwrap(),
                if value.is_null() {
                    None
                } else {
                    Some(&*(value as *const glib::Value))
                },
            )
        }
    }
}

declare_concrete_message!(StreamCollection);
impl<'a> StreamCollection<'a> {
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_stream_collection(&self) -> ::StreamCollection {
        unsafe {
            let mut collection = ptr::null_mut();

            ffi::gst_message_parse_stream_collection(self.as_mut_ptr(), &mut collection);

            from_glib_full(collection)
        }
    }
}

declare_concrete_message!(StreamsSelected);
impl<'a> StreamsSelected<'a> {
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_stream_collection(&self) -> ::StreamCollection {
        unsafe {
            let mut collection = ptr::null_mut();

            ffi::gst_message_parse_streams_selected(self.as_mut_ptr(), &mut collection);

            from_glib_full(collection)
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_streams(&self) -> Vec<::Stream> {
        unsafe {
            let n = ffi::gst_message_streams_selected_get_size(self.as_mut_ptr());

            (0..n)
                .map(|i| {
                    from_glib_full(ffi::gst_message_streams_selected_get_stream(
                        self.as_mut_ptr(),
                        i,
                    ))
                })
                .collect()
        }
    }
}

declare_concrete_message!(Redirect);
impl<'a> Redirect<'a> {
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_entries(&self) -> Vec<(&str, Option<TagList>, Option<&StructureRef>)> {
        unsafe {
            let n = ffi::gst_message_get_num_redirect_entries(self.as_mut_ptr());

            (0..n)
                .map(|i| {
                    let mut location = ptr::null();
                    let mut tags = ptr::null_mut();
                    let mut structure = ptr::null();

                    ffi::gst_message_parse_redirect_entry(
                        self.as_mut_ptr(),
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
        pub fn src<O: IsA<Object> + Cast + Clone>(self, src: Option<&O>) -> Self {
            Self {
                src: src.map(|o| {
                    let o = (*o).clone();
                    o.upcast::<Object>()
                }),
                .. self
            }
        }

        pub fn seqnum(self, seqnum: Seqnum) -> Self {
            Self {
                seqnum: Some(seqnum),
                .. self
            }
        }

        // Warning: other_fields are ignored with argument-less messages
        // until GStreamer 1.14 is released
        pub fn other_fields(self, other_fields: &[(&'a str, &'a ToSendValue)]) -> Self {
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
                    ffi::gst_message_set_seqnum(msg, seqnum.to_glib());
                }

                if !self.other_fields.is_empty() {
                    // issue with argument-less messages. We need the function
                    // ffi::gst_message_writable_structure to sort this out
                    // and this function will be available in GStreamer 1.14
                    // See https://github.com/sdroege/gstreamer-rs/pull/75
                    // and https://bugzilla.gnome.org/show_bug.cgi?id=792928
                    let structure = ffi::gst_message_get_structure(msg);
                    if !structure.is_null() {
                        let structure = StructureRef::from_glib_borrow_mut(structure as *mut _);

                        for (k, v) in self.other_fields {
                            structure.set_value(k, v.to_send_value());
                        }
                    }
                }

                from_glib_full(msg)
            }
        }
    }
}

pub struct EosBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
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

pub trait MessageErrorDomain: glib::error::ErrorDomain {}

impl MessageErrorDomain for ::CoreError {}
impl MessageErrorDomain for ::ResourceError {}
impl MessageErrorDomain for ::StreamError {}
impl MessageErrorDomain for ::LibraryError {}

pub struct ErrorBuilder<'a, T> {
    src: Option<Object>,
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    error: T,
    message: &'a str,
    debug: Option<&'a str>,
    #[allow(unused)] details: Option<Structure>,
}
impl<'a, T: MessageErrorDomain> ErrorBuilder<'a, T> {
    fn new(error: T, message: &'a str) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            error: error,
            message: message,
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

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn details(self, details: Structure) -> Self {
        Self {
            details: Some(details),
            ..self
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        #[cfg(any(feature = "v1_10", feature = "dox"))]
        {
            let details = match s.details.take() {
                None => ptr::null_mut(),
                Some(details) => details.into_ptr(),
            };

            let error = glib::Error::new(s.error, s.message);

            ffi::gst_message_new_error_with_details(
                src,
                mut_override(error.to_glib_none().0),
                s.debug.to_glib_none().0,
                details,
            )
        }
        #[cfg(not(any(feature = "v1_10", feature = "dox")))]
        {
            let error = glib::Error::new(s.error, s.message);

            ffi::gst_message_new_error(
                src,
                mut_override(error.to_glib_none().0),
                s.debug.to_glib_none().0,
            )
        }
    });
}

pub struct WarningBuilder<'a, T> {
    src: Option<Object>,
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    error: T,
    message: &'a str,
    debug: Option<&'a str>,
    #[allow(unused)] details: Option<Structure>,
}
impl<'a, T: MessageErrorDomain> WarningBuilder<'a, T> {
    fn new(error: T, message: &'a str) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            error: error,
            message: message,
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

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn details(self, details: Structure) -> Self {
        Self {
            details: Some(details),
            ..self
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        #[cfg(any(feature = "v1_10", feature = "dox"))]
        {
            let details = match s.details.take() {
                None => ptr::null_mut(),
                Some(details) => details.into_ptr(),
            };

            let error = glib::Error::new(s.error, s.message);

            ffi::gst_message_new_warning_with_details(
                src,
                mut_override(error.to_glib_none().0),
                s.debug.to_glib_none().0,
                details,
            )
        }
        #[cfg(not(any(feature = "v1_10", feature = "dox")))]
        {
            let error = glib::Error::new(s.error, s.message);

            ffi::gst_message_new_warning(
                src,
                mut_override(error.to_glib_none().0),
                s.debug.to_glib_none().0,
            )
        }
    });
}

pub struct InfoBuilder<'a, T> {
    src: Option<Object>,
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    error: T,
    message: &'a str,
    debug: Option<&'a str>,
    #[allow(unused)] details: Option<Structure>,
}
impl<'a, T: MessageErrorDomain> InfoBuilder<'a, T> {
    fn new(error: T, message: &'a str) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            error: error,
            message: message,
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

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn details(self, details: Structure) -> Self {
        Self {
            details: Some(details),
            ..self
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        #[cfg(any(feature = "v1_10", feature = "dox"))]
        {
            let details = match s.details.take() {
                None => ptr::null_mut(),
                Some(details) => details.into_ptr(),
            };

            let error = glib::Error::new(s.error, s.message);

            ffi::gst_message_new_info_with_details(
                src,
                mut_override(error.to_glib_none().0),
                s.debug.to_glib_none().0,
                details,
            )
        }
        #[cfg(not(any(feature = "v1_10", feature = "dox")))]
        {
            let error = glib::Error::new(s.error, s.message);

            ffi::gst_message_new_info(
                src,
                mut_override(error.to_glib_none().0),
                s.debug.to_glib_none().0,
            )
        }
    });
}

pub struct TagBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
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

    message_builder_generic_impl!(|s: &Self, src| ffi::gst_message_new_tag(
        src,
        s.tags.to_glib_full()
    ));
}

pub struct BufferingBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
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
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
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

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_state_changed(
        src,
        s.old.to_glib(),
        s.new.to_glib(),
        s.pending.to_glib(),
    ));
}

pub struct StateDirtyBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
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
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    amount: GenericFormattedValue,
    rate: f64,
    flush: bool,
    intermediate: bool,
    duration: GenericFormattedValue,
    eos: bool,
}
impl<'a> StepDoneBuilder<'a> {
    fn new(
        amount: GenericFormattedValue,
        rate: f64,
        flush: bool,
        intermediate: bool,
        duration: GenericFormattedValue,
        eos: bool,
    ) -> Self {
        skip_assert_initialized!();
        assert_eq!(amount.get_format(), duration.get_format());
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            amount: amount,
            rate: rate,
            flush: flush,
            intermediate: intermediate,
            duration: duration,
            eos: eos,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_step_done(
        src,
        s.amount.get_format().to_glib(),
        s.amount.get_value() as u64,
        s.rate,
        s.flush.to_glib(),
        s.intermediate.to_glib(),
        s.duration.get_value() as u64,
        s.eos.to_glib(),
    ));
}

pub struct ClockProvideBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
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

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_clock_provide(
        src,
        s.clock.to_glib_none().0,
        s.ready.to_glib()
    ));
}

pub struct ClockLostBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
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

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_clock_lost(
        src,
        s.clock.to_glib_none().0
    ));
}

pub struct NewClockBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
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

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_new_clock(
        src,
        s.clock.to_glib_none().0
    ));
}

pub struct StructureChangeBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
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

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_structure_change(
        src,
        s.type_.to_glib(),
        s.owner.to_glib_none().0,
        s.busy.to_glib(),
    ));
}

pub struct StreamStatusBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    type_: ::StreamStatusType,
    owner: &'a ::Element,
    status_object: Option<&'a glib::ToSendValue>,
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

    pub fn status_object(self, status_object: &'a glib::ToSendValue) -> Self {
        Self {
            status_object: Some(status_object),
            ..self
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        let msg =
            ffi::gst_message_new_stream_status(src, s.type_.to_glib(), s.owner.to_glib_none().0);
        if let Some(status_object) = s.status_object {
            ffi::gst_message_set_stream_status_object(
                msg,
                status_object.to_send_value().to_glib_none().0,
            );
        }
        msg
    });
}

pub struct ApplicationBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
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

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_application(
        src,
        s.structure.take().unwrap().into_ptr()
    ));
}

pub struct ElementBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
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

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_element(
        src,
        s.structure.take().unwrap().into_ptr()
    ));
}

pub struct SegmentStartBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    position: GenericFormattedValue,
}
impl<'a> SegmentStartBuilder<'a> {
    fn new(position: GenericFormattedValue) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            position: position,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_segment_start(
        src,
        s.position.get_format().to_glib(),
        s.position.get_value(),
    ));
}

pub struct SegmentDoneBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    position: GenericFormattedValue,
}
impl<'a> SegmentDoneBuilder<'a> {
    fn new(position: GenericFormattedValue) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            position: position,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_segment_done(
        src,
        s.position.get_format().to_glib(),
        s.position.get_value(),
    ));
}

pub struct DurationChangedBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
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
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
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
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
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
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    running_time: ::ClockTime,
}
impl<'a> AsyncDoneBuilder<'a> {
    fn new(running_time: ::ClockTime) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            running_time: running_time,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_async_done(
        src,
        s.running_time.to_glib()
    ));
}

pub struct RequestStateBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
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

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_request_state(
        src,
        s.state.to_glib()
    ));
}

pub struct StepStartBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    active: bool,
    amount: GenericFormattedValue,
    rate: f64,
    flush: bool,
    intermediate: bool,
}
impl<'a> StepStartBuilder<'a> {
    fn new(
        active: bool,
        amount: GenericFormattedValue,
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
            amount: amount,
            rate: rate,
            flush: flush,
            intermediate: intermediate,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_step_start(
        src,
        s.active.to_glib(),
        s.amount.get_format().to_glib(),
        s.amount.get_value() as u64,
        s.rate,
        s.flush.to_glib(),
        s.intermediate.to_glib(),
    ));
}

pub struct QosBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    live: bool,
    running_time: ::ClockTime,
    stream_time: ::ClockTime,
    timestamp: ::ClockTime,
    duration: ::ClockTime,
    values: Option<(i64, f64, i32)>,
    stats: Option<(GenericFormattedValue, GenericFormattedValue)>,
}
impl<'a> QosBuilder<'a> {
    fn new(
        live: bool,
        running_time: ::ClockTime,
        stream_time: ::ClockTime,
        timestamp: ::ClockTime,
        duration: ::ClockTime,
    ) -> Self {
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

    pub fn stats<V: Into<GenericFormattedValue>>(self, processed: V, dropped: V) -> Self {
        let processed = processed.into();
        let dropped = dropped.into();
        assert_eq!(processed.get_format(), dropped.get_format());
        Self {
            stats: Some((processed, dropped)),
            ..self
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        let msg = ffi::gst_message_new_qos(
            src,
            s.live.to_glib(),
            s.running_time.to_glib(),
            s.stream_time.to_glib(),
            s.timestamp.to_glib(),
            s.duration.to_glib(),
        );
        if let Some((jitter, proportion, quality)) = s.values {
            ffi::gst_message_set_qos_values(msg, jitter, proportion, quality);
        }
        if let Some((processed, dropped)) = s.stats {
            ffi::gst_message_set_qos_stats(
                msg,
                processed.get_format().to_glib(),
                processed.get_value() as u64,
                dropped.get_value() as u64,
            );
        }
        msg
    });
}

pub struct ProgressBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    type_: ::ProgressType,
    code: &'a str,
    text: &'a str,
}
impl<'a> ProgressBuilder<'a> {
    fn new(type_: ::ProgressType, code: &'a str, text: &'a str) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            type_: type_,
            code: code,
            text: text,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_progress(
        src,
        s.type_.to_glib(),
        s.code.to_glib_none().0,
        s.text.to_glib_none().0,
    ));
}

pub struct TocBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
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

    message_builder_generic_impl!(|s: &Self, src| ffi::gst_message_new_toc(
        src,
        s.toc.to_glib_none().0,
        s.updated.to_glib()
    ));
}

pub struct ResetTimeBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    running_time: ::ClockTime,
}
impl<'a> ResetTimeBuilder<'a> {
    fn new(running_time: ::ClockTime) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            running_time: running_time,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_reset_time(
        src,
        s.running_time.to_glib()
    ));
}

pub struct StreamStartBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    group_id: Option<GroupId>,
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

    pub fn group_id(self, group_id: GroupId) -> Self {
        Self {
            group_id: Some(group_id),
            ..self
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        let msg = ffi::gst_message_new_stream_start(src);
        if let Some(group_id) = s.group_id {
            ffi::gst_message_set_group_id(msg, group_id.to_glib());
        }
        msg
    });
}

pub struct NeedContextBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
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

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_need_context(
        src,
        s.context_type.to_glib_none().0
    ));
}

pub struct HaveContextBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
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
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
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

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_device_added(
        src,
        s.device.to_glib_none().0
    ));
}

pub struct DeviceRemovedBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
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

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_device_removed(
        src,
        s.device.to_glib_none().0
    ));
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
pub struct PropertyNotifyBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    property_name: &'a str,
    value: Option<&'a glib::ToSendValue>,
}
#[cfg(any(feature = "v1_10", feature = "dox"))]
impl<'a> PropertyNotifyBuilder<'a> {
    fn new(property_name: &'a str) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            property_name: property_name,
            value: None,
        }
    }

    pub fn value(self, value: &'a glib::ToSendValue) -> Self {
        Self {
            value: Some(value),
            ..self
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        let val = s.value.map(|v| v.to_send_value());
        ffi::gst_message_new_property_notify(
            src,
            s.property_name.to_glib_none().0,
            mut_override(
                val.as_ref()
                    .map(|v| v.to_glib_none().0)
                    .unwrap_or(ptr::null()),
            ),
        )
    });
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
pub struct StreamCollectionBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    collection: &'a ::StreamCollection,
}
#[cfg(any(feature = "v1_10", feature = "dox"))]
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

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_stream_collection(
        src,
        s.collection.to_glib_none().0
    ));
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
pub struct StreamsSelectedBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    #[cfg(any(feature = "v1_10", feature = "dox"))] collection: &'a ::StreamCollection,
    #[cfg(any(feature = "v1_10", feature = "dox"))] streams: Option<&'a [&'a ::Stream]>,
}
#[cfg(any(feature = "v1_10", feature = "dox"))]
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

#[cfg(any(feature = "v1_10", feature = "dox"))]
pub struct RedirectBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<Seqnum>,
    other_fields: Vec<(&'a str, &'a ToSendValue)>,
    location: &'a str,
    tag_list: Option<&'a TagList>,
    entry_struct: Option<Structure>,
    #[cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
    entries: Option<&'a [(&'a str, Option<&'a TagList>, Option<&'a Structure>)]>,
}
#[cfg(any(feature = "v1_10", feature = "dox"))]
impl<'a> RedirectBuilder<'a> {
    fn new(location: &'a str) -> Self {
        skip_assert_initialized!();
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
            location: location,
            tag_list: None,
            entry_struct: None,
            entries: None,
        }
    }

    pub fn tag_list(self, tag_list: &'a TagList) -> Self {
        Self {
            tag_list: Some(tag_list),
            ..self
        }
    }

    pub fn entry_struct(self, entry_struct: Structure) -> Self {
        Self {
            entry_struct: Some(entry_struct),
            ..self
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        ::init().unwrap();

        // Message without arguments
        let eos_msg = Message::new_eos()
            .seqnum(Seqnum(1))
            .build();
        match eos_msg.view() {
            MessageView::Eos(eos_msg) => {
                assert_eq!(eos_msg.get_seqnum(), Seqnum(1));
                assert!(eos_msg.get_structure().is_none());
            },
            _ => panic!("eos_msg.view() is not a MessageView::Eos(_)"),
        }

        // Note: can't define other_fields for argument-less messages before GStreamer 1.14

        // Message with arguments
        let buffering_msg = Message::new_buffering(42)
            .other_fields(&[("extra-field", &true)])
            .build();
        match buffering_msg.view() {
            MessageView::Buffering(buffering_msg) => {
                assert_eq!(buffering_msg.get_percent(), 42);
                assert!(buffering_msg.get_structure().is_some());
                assert!(buffering_msg.get_structure().unwrap().has_field("extra-field"));
            }
            _ => panic!("buffering_msg.view() is not a MessageView::Buffering(_)"),
        }
    }
}
