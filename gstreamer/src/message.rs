// Copyright (C) 2016-2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use gst_sys;
use structure::*;
use GenericFormattedValue;
use GroupId;
use GstObjectExt;
use MessageType;
use Object;
use Seqnum;
use TagList;

use std::ffi::CStr;
use std::fmt;
use std::mem;
use std::num::NonZeroU32;
use std::ops::Deref;
use std::ptr;

use glib;
use glib::translate::{from_glib, from_glib_full, from_glib_none, mut_override, ToGlib, ToGlibPtr};
use glib::value::ToSendValue;
use glib::Cast;
use glib::IsA;

gst_define_mini_object_wrapper!(Message, MessageRef, gst_sys::GstMessage, || {
    gst_sys::gst_message_get_type()
});

impl MessageRef {
    pub fn get_src(&self) -> Option<Object> {
        unsafe { from_glib_none((*self.as_ptr()).src) }
    }

    pub fn get_seqnum(&self) -> Seqnum {
        unsafe {
            let seqnum = gst_sys::gst_message_get_seqnum(self.as_mut_ptr());

            if seqnum == 0 {
                // seqnum for this message is invalid. This can happen with buggy elements
                // overriding the seqnum with GST_SEQNUM_INVALID instead of the expected seqnum.
                // As a workaround, let's generate an unused valid seqnum.
                let next = Seqnum::next();

                ::gst_warning!(
                    ::CAT_RUST,
                    "get_seqnum detected invalid seqnum, returning next {:?}",
                    next
                );

                return next;
            }

            Seqnum(NonZeroU32::new_unchecked(seqnum))
        }
    }

    pub fn get_structure(&self) -> Option<&StructureRef> {
        unsafe {
            let structure = gst_sys::gst_message_get_structure(self.as_mut_ptr());
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
            gst_sys::GST_MESSAGE_EOS => MessageView::Eos(Eos(self)),
            gst_sys::GST_MESSAGE_ERROR => MessageView::Error(Error(self)),
            gst_sys::GST_MESSAGE_WARNING => MessageView::Warning(Warning(self)),
            gst_sys::GST_MESSAGE_INFO => MessageView::Info(Info(self)),
            gst_sys::GST_MESSAGE_TAG => MessageView::Tag(Tag(self)),
            gst_sys::GST_MESSAGE_BUFFERING => MessageView::Buffering(Buffering(self)),
            gst_sys::GST_MESSAGE_STATE_CHANGED => MessageView::StateChanged(StateChanged(self)),
            gst_sys::GST_MESSAGE_STATE_DIRTY => MessageView::StateDirty(StateDirty(self)),
            gst_sys::GST_MESSAGE_STEP_DONE => MessageView::StepDone(StepDone(self)),
            gst_sys::GST_MESSAGE_CLOCK_PROVIDE => MessageView::ClockProvide(ClockProvide(self)),
            gst_sys::GST_MESSAGE_CLOCK_LOST => MessageView::ClockLost(ClockLost(self)),
            gst_sys::GST_MESSAGE_NEW_CLOCK => MessageView::NewClock(NewClock(self)),
            gst_sys::GST_MESSAGE_STRUCTURE_CHANGE => {
                MessageView::StructureChange(StructureChange(self))
            }
            gst_sys::GST_MESSAGE_STREAM_STATUS => MessageView::StreamStatus(StreamStatus(self)),
            gst_sys::GST_MESSAGE_APPLICATION => MessageView::Application(Application(self)),
            gst_sys::GST_MESSAGE_ELEMENT => MessageView::Element(Element(self)),
            gst_sys::GST_MESSAGE_SEGMENT_START => MessageView::SegmentStart(SegmentStart(self)),
            gst_sys::GST_MESSAGE_SEGMENT_DONE => MessageView::SegmentDone(SegmentDone(self)),
            gst_sys::GST_MESSAGE_DURATION_CHANGED => {
                MessageView::DurationChanged(DurationChanged(self))
            }
            gst_sys::GST_MESSAGE_LATENCY => MessageView::Latency(Latency(self)),
            gst_sys::GST_MESSAGE_ASYNC_START => MessageView::AsyncStart(AsyncStart(self)),
            gst_sys::GST_MESSAGE_ASYNC_DONE => MessageView::AsyncDone(AsyncDone(self)),
            gst_sys::GST_MESSAGE_REQUEST_STATE => MessageView::RequestState(RequestState(self)),
            gst_sys::GST_MESSAGE_STEP_START => MessageView::StepStart(StepStart(self)),
            gst_sys::GST_MESSAGE_QOS => MessageView::Qos(Qos(self)),
            gst_sys::GST_MESSAGE_PROGRESS => MessageView::Progress(Progress(self)),
            gst_sys::GST_MESSAGE_TOC => MessageView::Toc(Toc(self)),
            gst_sys::GST_MESSAGE_RESET_TIME => MessageView::ResetTime(ResetTime(self)),
            gst_sys::GST_MESSAGE_STREAM_START => MessageView::StreamStart(StreamStart(self)),
            gst_sys::GST_MESSAGE_NEED_CONTEXT => MessageView::NeedContext(NeedContext(self)),
            gst_sys::GST_MESSAGE_HAVE_CONTEXT => MessageView::HaveContext(HaveContext(self)),
            gst_sys::GST_MESSAGE_DEVICE_ADDED => MessageView::DeviceAdded(DeviceAdded(self)),
            gst_sys::GST_MESSAGE_DEVICE_REMOVED => MessageView::DeviceRemoved(DeviceRemoved(self)),
            gst_sys::GST_MESSAGE_PROPERTY_NOTIFY => {
                MessageView::PropertyNotify(PropertyNotify(self))
            }
            gst_sys::GST_MESSAGE_STREAM_COLLECTION => {
                MessageView::StreamCollection(StreamCollection(self))
            }
            gst_sys::GST_MESSAGE_STREAMS_SELECTED => {
                MessageView::StreamsSelected(StreamsSelected(self))
            }
            gst_sys::GST_MESSAGE_DEVICE_CHANGED => MessageView::DeviceChanged(DeviceChanged(self)),
            _ => MessageView::Other,
        }
    }

    pub fn get_type(&self) -> MessageType {
        unsafe { from_glib((*self.as_ptr()).type_) }
    }
}

impl Message {
    #[deprecated(
        since = "0.16.0",
        note = "use `message::Eos::new` or `message::Eos::builder` instead"
    )]
    pub fn new_eos<'a>() -> EosBuilder<'a> {
        assert_initialized_main_thread!();
        EosBuilder::new()
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `message::Error::new` or `message::Error::builder` instead"
    )]
    pub fn new_error<T: MessageErrorDomain>(error: T, message: &str) -> ErrorBuilder<T> {
        assert_initialized_main_thread!();
        ErrorBuilder::new(error, message)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `message::Warning::new` or `message::Warning::builder` instead"
    )]
    pub fn new_warning<T: MessageErrorDomain>(error: T, message: &str) -> WarningBuilder<T> {
        assert_initialized_main_thread!();
        WarningBuilder::new(error, message)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `message::Info::new` or `message::Info::builder` instead"
    )]
    pub fn new_info<T: MessageErrorDomain>(error: T, message: &str) -> InfoBuilder<T> {
        assert_initialized_main_thread!();
        InfoBuilder::new(error, message)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `message::Tag::new` or `message::Tag::builder` instead"
    )]
    pub fn new_tag(tags: &TagList) -> TagBuilder {
        assert_initialized_main_thread!();
        TagBuilder::new(tags)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `message::Buffering::new` or `message::Buffering::builder` instead"
    )]
    pub fn new_buffering<'a>(percent: i32) -> BufferingBuilder<'a> {
        assert_initialized_main_thread!();
        BufferingBuilder::new(percent)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `message::StateChanged::new` or `message::StateChanged::builder` instead"
    )]
    pub fn new_state_changed<'a>(
        old: ::State,
        new: ::State,
        pending: ::State,
    ) -> StateChangedBuilder<'a> {
        assert_initialized_main_thread!();
        StateChangedBuilder::new(old, new, pending)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `message::StateDirty::new` or `message::StateDirty::builder` instead"
    )]
    pub fn new_state_dirty<'a>() -> StateDirtyBuilder<'a> {
        assert_initialized_main_thread!();
        StateDirtyBuilder::new()
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `message::StepDone::new` or `message::StepDone::builder` instead"
    )]
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

    #[deprecated(
        since = "0.16.0",
        note = "use `message::ClockProvide::new` or `message::ClockProvide::builder` instead"
    )]
    pub fn new_clock_provide(clock: &::Clock, ready: bool) -> ClockProvideBuilder {
        assert_initialized_main_thread!();
        ClockProvideBuilder::new(clock, ready)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `message::ClockLost::new` or `message::ClockLost::builder` instead"
    )]
    pub fn new_clock_lost(clock: &::Clock) -> ClockLostBuilder {
        assert_initialized_main_thread!();
        ClockLostBuilder::new(clock)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `message::NewClock::new` or `message::NewClock::builder` instead"
    )]
    pub fn new_new_clock(clock: &::Clock) -> NewClockBuilder {
        assert_initialized_main_thread!();
        NewClockBuilder::new(clock)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `message::StructureChange::new` or `message::StructureChange::builder` instead"
    )]
    pub fn new_structure_change(
        type_: ::StructureChangeType,
        owner: &::Element,
        busy: bool,
    ) -> StructureChangeBuilder {
        assert_initialized_main_thread!();
        StructureChangeBuilder::new(type_, owner, busy)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `message::StreamStatus::new` or `message::StreamStatus::builder` instead"
    )]
    pub fn new_stream_status(type_: ::StreamStatusType, owner: &::Element) -> StreamStatusBuilder {
        assert_initialized_main_thread!();
        StreamStatusBuilder::new(type_, owner)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `message::Application::new` or `message::Application::builder` instead"
    )]
    pub fn new_application<'a>(structure: ::Structure) -> ApplicationBuilder<'a> {
        assert_initialized_main_thread!();
        ApplicationBuilder::new(structure)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `message::Element::new` or `message::Element::builder` instead"
    )]
    pub fn new_element<'a>(structure: ::Structure) -> ElementBuilder<'a> {
        assert_initialized_main_thread!();
        ElementBuilder::new(structure)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `message::SegmentStart::new` or `message::SegmentStart::builder` instead"
    )]
    pub fn new_segment_start<'a, V: Into<GenericFormattedValue>>(
        position: V,
    ) -> SegmentStartBuilder<'a> {
        assert_initialized_main_thread!();
        let position = position.into();
        SegmentStartBuilder::new(position)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `message::SegmentDone::new` or `message::SegmentDone::builder` instead"
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
        note = "use `message::DurationChanged::new` or `message::DurationChanged::builder` instead"
    )]
    pub fn new_duration_changed<'a>() -> DurationChangedBuilder<'a> {
        assert_initialized_main_thread!();
        DurationChangedBuilder::new()
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `message::Latency::new` or `message::Latency::builder` instead"
    )]
    pub fn new_latency<'a>() -> LatencyBuilder<'a> {
        assert_initialized_main_thread!();
        LatencyBuilder::new()
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `message::AsyncStart::new` or `message::AsyncStart::builder` instead"
    )]
    pub fn new_async_start<'a>() -> AsyncStartBuilder<'a> {
        assert_initialized_main_thread!();
        AsyncStartBuilder::new()
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `message::AsyncDone::new` or `message::AsyncDone::builder` instead"
    )]
    pub fn new_async_done<'a>(running_time: ::ClockTime) -> AsyncDoneBuilder<'a> {
        assert_initialized_main_thread!();
        AsyncDoneBuilder::new(running_time)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `message::RequestState::new` or `message::RequestState::builder` instead"
    )]
    pub fn new_request_state<'a>(state: ::State) -> RequestStateBuilder<'a> {
        assert_initialized_main_thread!();
        RequestStateBuilder::new(state)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `message::StepStart::new` or `message::StepStart::builder` instead"
    )]
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

    #[deprecated(
        since = "0.16.0",
        note = "use `message::Qos::new` or `message::Qos::builder` instead"
    )]
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

    #[deprecated(
        since = "0.16.0",
        note = "use `message::Progress::new` or `message::Progress::builder` instead"
    )]
    pub fn new_progress<'a>(
        type_: ::ProgressType,
        code: &'a str,
        text: &'a str,
    ) -> ProgressBuilder<'a> {
        assert_initialized_main_thread!();
        ProgressBuilder::new(type_, code, text)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `message::Toc::new` or `message::Toc::builder` instead"
    )]
    pub fn new_toc(toc: &::Toc, updated: bool) -> TocBuilder {
        assert_initialized_main_thread!();
        TocBuilder::new(toc, updated)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `message::ResetTime::new` or `message::ResetTime::builder` instead"
    )]
    pub fn new_reset_time<'a>(running_time: ::ClockTime) -> ResetTimeBuilder<'a> {
        assert_initialized_main_thread!();
        ResetTimeBuilder::new(running_time)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `message::StreamStart::new` or `message::StreamStart::builder` instead"
    )]
    pub fn new_stream_start<'a>() -> StreamStartBuilder<'a> {
        assert_initialized_main_thread!();
        StreamStartBuilder::new()
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `message::NeedContext::new` or `message::NeedContext::builder` instead"
    )]
    pub fn new_need_context(context_type: &str) -> NeedContextBuilder {
        assert_initialized_main_thread!();
        NeedContextBuilder::new(context_type)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `message::HaveContext::new` or `message::HaveContext::builder` instead"
    )]
    pub fn new_have_context<'a>(context: ::Context) -> HaveContextBuilder<'a> {
        assert_initialized_main_thread!();
        HaveContextBuilder::new(context)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `message::DeviceAdded::new` or `message::DeviceAdded::builder` instead"
    )]
    pub fn new_device_added(device: &::Device) -> DeviceAddedBuilder {
        assert_initialized_main_thread!();
        DeviceAddedBuilder::new(device)
    }

    #[deprecated(
        since = "0.16.0",
        note = "use `message::DeviceRemoved::new` or `message::DeviceRemoved::builder` instead"
    )]
    pub fn new_device_removed(device: &::Device) -> DeviceRemovedBuilder {
        assert_initialized_main_thread!();
        DeviceRemovedBuilder::new(device)
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[deprecated(
        since = "0.16.0",
        note = "use `message::PropertyNotify::new` or `message::PropertyNotify::builder` instead"
    )]
    pub fn new_property_notify(property_name: &str) -> PropertyNotifyBuilder {
        assert_initialized_main_thread!();
        PropertyNotifyBuilder::new(property_name)
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[deprecated(
        since = "0.16.0",
        note = "use `message::StreamCollection::new` or `message::StreamCollection::builder` instead"
    )]
    pub fn new_stream_collection(collection: &::StreamCollection) -> StreamCollectionBuilder {
        assert_initialized_main_thread!();
        StreamCollectionBuilder::new(collection)
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[deprecated(
        since = "0.16.0",
        note = "use `message::StreamsSelected::new` or `message::StreamsSelected::builder` instead"
    )]
    pub fn new_streams_selected(collection: &::StreamCollection) -> StreamsSelectedBuilder {
        assert_initialized_main_thread!();
        StreamsSelectedBuilder::new(collection)
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[deprecated(
        since = "0.16.0",
        note = "use `message::Redirect::new` or `message::Redirect::builder` instead"
    )]
    pub fn new_redirect(location: &str) -> RedirectBuilder {
        assert_initialized_main_thread!();
        RedirectBuilder::new(location)
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[deprecated(
        since = "0.16.0",
        note = "use `message::DeviceChanged::new` or `message::DeviceChanged::builder` instead"
    )]
    pub fn new_device_changed<'a>(
        device: &'a ::Device,
        changed_device: &'a ::Device,
    ) -> DeviceChangedBuilder<'a> {
        assert_initialized_main_thread!();
        DeviceChangedBuilder::new(device, changed_device)
    }
}

impl fmt::Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        MessageRef::fmt(self, f)
    }
}

impl fmt::Debug for MessageRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Don't retrieve `seqnum` using `MessageRef::get_seqnum`
        // because it would generate a new seqnum if a buggy `Element`
        // emitted a `Message` with an invalid `seqnum`.
        // We want to help the user find out there is something wrong here,
        // so they can investigate the origin.
        let seqnum = unsafe { gst_sys::gst_message_get_seqnum(self.as_mut_ptr()) };
        let seqnum = if seqnum != 0 {
            &seqnum as &dyn fmt::Debug
        } else {
            &"INVALID (0)" as &dyn fmt::Debug
        };

        f.debug_struct("Message")
            .field("ptr", unsafe { &self.as_ptr() })
            .field("type", &unsafe {
                let type_ = gst_sys::gst_message_type_get_name((*self.as_ptr()).type_);
                CStr::from_ptr(type_).to_str().unwrap()
            })
            .field("seqnum", seqnum)
            .field("src", &self.get_src().map(|s| s.get_name().to_owned()))
            .field("structure", &self.get_structure())
            .finish()
    }
}

#[derive(Debug)]
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
    DeviceChanged(DeviceChanged<'a>),
    Other,
    __NonExhaustive,
}

macro_rules! declare_concrete_message(
    ($name:ident) => {
        #[derive(Debug)]
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
impl<'a> Eos<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Message {
        skip_assert_initialized!();
        Self::builder().build()
    }

    pub fn builder() -> EosBuilder<'a> {
        assert_initialized_main_thread!();
        EosBuilder::new()
    }
}

declare_concrete_message!(Error);
impl<'a> Error<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: MessageErrorDomain>(error: T, message: &str) -> Message {
        skip_assert_initialized!();
        Self::builder(error, message).build()
    }

    pub fn builder<T: MessageErrorDomain>(error: T, message: &str) -> ErrorBuilder<T> {
        assert_initialized_main_thread!();
        ErrorBuilder::new(error, message)
    }

    pub fn get_error(&self) -> glib::Error {
        unsafe {
            let mut error = ptr::null_mut();

            gst_sys::gst_message_parse_error(self.as_mut_ptr(), &mut error, ptr::null_mut());

            from_glib_full(error)
        }
    }

    pub fn get_debug(&self) -> Option<String> {
        unsafe {
            let mut debug = ptr::null_mut();

            gst_sys::gst_message_parse_error(self.as_mut_ptr(), ptr::null_mut(), &mut debug);

            from_glib_full(debug)
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_details(&self) -> Option<&StructureRef> {
        unsafe {
            let mut details = ptr::null();

            gst_sys::gst_message_parse_error_details(self.as_mut_ptr(), &mut details);

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
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: MessageErrorDomain>(error: T, message: &str) -> Message {
        skip_assert_initialized!();
        Self::builder(error, message).build()
    }

    pub fn builder<T: MessageErrorDomain>(error: T, message: &str) -> WarningBuilder<T> {
        assert_initialized_main_thread!();
        WarningBuilder::new(error, message)
    }

    pub fn get_error(&self) -> glib::Error {
        unsafe {
            let mut error = ptr::null_mut();

            gst_sys::gst_message_parse_warning(self.as_mut_ptr(), &mut error, ptr::null_mut());

            from_glib_full(error)
        }
    }

    pub fn get_debug(&self) -> Option<String> {
        unsafe {
            let mut debug = ptr::null_mut();

            gst_sys::gst_message_parse_warning(self.as_mut_ptr(), ptr::null_mut(), &mut debug);

            from_glib_full(debug)
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_details(&self) -> Option<&StructureRef> {
        unsafe {
            let mut details = ptr::null();

            gst_sys::gst_message_parse_error_details(self.as_mut_ptr(), &mut details);

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
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: MessageErrorDomain>(error: T, message: &str) -> Message {
        skip_assert_initialized!();
        Self::builder(error, message).build()
    }

    pub fn builder<T: MessageErrorDomain>(error: T, message: &str) -> InfoBuilder<T> {
        assert_initialized_main_thread!();
        InfoBuilder::new(error, message)
    }

    pub fn get_error(&self) -> glib::Error {
        unsafe {
            let mut error = ptr::null_mut();

            gst_sys::gst_message_parse_info(self.as_mut_ptr(), &mut error, ptr::null_mut());

            from_glib_full(error)
        }
    }

    pub fn get_debug(&self) -> Option<String> {
        unsafe {
            let mut debug = ptr::null_mut();

            gst_sys::gst_message_parse_info(self.as_mut_ptr(), ptr::null_mut(), &mut debug);

            from_glib_full(debug)
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_details(&self) -> Option<&StructureRef> {
        unsafe {
            let mut details = ptr::null();

            gst_sys::gst_message_parse_error_details(self.as_mut_ptr(), &mut details);

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
    #[allow(clippy::new_ret_no_self)]
    pub fn new(tags: &TagList) -> Message {
        skip_assert_initialized!();
        Self::builder(tags).build()
    }

    pub fn builder(tags: &TagList) -> TagBuilder {
        assert_initialized_main_thread!();
        TagBuilder::new(tags)
    }

    pub fn get_tags(&self) -> TagList {
        unsafe {
            let mut tags = ptr::null_mut();
            gst_sys::gst_message_parse_tag(self.as_mut_ptr(), &mut tags);
            from_glib_full(tags)
        }
    }
}

declare_concrete_message!(Buffering);
impl<'a> Buffering<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(percent: i32) -> Message {
        skip_assert_initialized!();
        Self::builder(percent).build()
    }

    pub fn builder(percent: i32) -> BufferingBuilder<'a> {
        assert_initialized_main_thread!();
        BufferingBuilder::new(percent)
    }

    pub fn get_percent(&self) -> i32 {
        unsafe {
            let mut p = mem::MaybeUninit::uninit();
            gst_sys::gst_message_parse_buffering(self.as_mut_ptr(), p.as_mut_ptr());
            p.assume_init()
        }
    }

    pub fn get_buffering_stats(&self) -> (::BufferingMode, i32, i32, i64) {
        unsafe {
            let mut mode = mem::MaybeUninit::uninit();
            let mut avg_in = mem::MaybeUninit::uninit();
            let mut avg_out = mem::MaybeUninit::uninit();
            let mut buffering_left = mem::MaybeUninit::uninit();

            gst_sys::gst_message_parse_buffering_stats(
                self.as_mut_ptr(),
                mode.as_mut_ptr(),
                avg_in.as_mut_ptr(),
                avg_out.as_mut_ptr(),
                buffering_left.as_mut_ptr(),
            );

            (
                from_glib(mode.assume_init()),
                avg_in.assume_init(),
                avg_out.assume_init(),
                buffering_left.assume_init(),
            )
        }
    }
}

declare_concrete_message!(StateChanged);
impl<'a> StateChanged<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(old: ::State, new: ::State, pending: ::State) -> Message {
        skip_assert_initialized!();
        Self::builder(old, new, pending).build()
    }

    pub fn builder(old: ::State, new: ::State, pending: ::State) -> StateChangedBuilder<'a> {
        assert_initialized_main_thread!();
        StateChangedBuilder::new(old, new, pending)
    }

    pub fn get_old(&self) -> ::State {
        unsafe {
            let mut state = mem::MaybeUninit::uninit();

            gst_sys::gst_message_parse_state_changed(
                self.as_mut_ptr(),
                state.as_mut_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
            );

            from_glib(state.assume_init())
        }
    }

    pub fn get_current(&self) -> ::State {
        unsafe {
            let mut state = mem::MaybeUninit::uninit();

            gst_sys::gst_message_parse_state_changed(
                self.as_mut_ptr(),
                ptr::null_mut(),
                state.as_mut_ptr(),
                ptr::null_mut(),
            );

            from_glib(state.assume_init())
        }
    }

    pub fn get_pending(&self) -> ::State {
        unsafe {
            let mut state = mem::MaybeUninit::uninit();

            gst_sys::gst_message_parse_state_changed(
                self.as_mut_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
                state.as_mut_ptr(),
            );

            from_glib(state.assume_init())
        }
    }
}

declare_concrete_message!(StateDirty);
impl<'a> StateDirty<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Message {
        skip_assert_initialized!();
        Self::builder().build()
    }

    pub fn builder() -> StateDirtyBuilder<'a> {
        assert_initialized_main_thread!();
        StateDirtyBuilder::new()
    }
}

declare_concrete_message!(StepDone);
impl<'a> StepDone<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<V: Into<GenericFormattedValue>>(
        amount: V,
        rate: f64,
        flush: bool,
        intermediate: bool,
        duration: V,
        eos: bool,
    ) -> Message {
        skip_assert_initialized!();
        Self::builder(
            amount.into(),
            rate,
            flush,
            intermediate,
            duration.into(),
            eos,
        )
        .build()
    }

    pub fn builder<V: Into<GenericFormattedValue>>(
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
            let mut format = mem::MaybeUninit::uninit();
            let mut amount = mem::MaybeUninit::uninit();
            let mut rate = mem::MaybeUninit::uninit();
            let mut flush = mem::MaybeUninit::uninit();
            let mut intermediate = mem::MaybeUninit::uninit();
            let mut duration = mem::MaybeUninit::uninit();
            let mut eos = mem::MaybeUninit::uninit();

            gst_sys::gst_message_parse_step_done(
                self.as_mut_ptr(),
                format.as_mut_ptr(),
                amount.as_mut_ptr(),
                rate.as_mut_ptr(),
                flush.as_mut_ptr(),
                intermediate.as_mut_ptr(),
                duration.as_mut_ptr(),
                eos.as_mut_ptr(),
            );

            (
                GenericFormattedValue::new(
                    from_glib(format.assume_init()),
                    amount.assume_init() as i64,
                ),
                rate.assume_init(),
                from_glib(flush.assume_init()),
                from_glib(intermediate.assume_init()),
                GenericFormattedValue::new(
                    from_glib(format.assume_init()),
                    duration.assume_init() as i64,
                ),
                from_glib(eos.assume_init()),
            )
        }
    }
}

declare_concrete_message!(ClockProvide);
impl<'a> ClockProvide<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(clock: &::Clock, ready: bool) -> Message {
        skip_assert_initialized!();
        Self::builder(clock, ready).build()
    }

    pub fn builder(clock: &::Clock, ready: bool) -> ClockProvideBuilder {
        assert_initialized_main_thread!();
        ClockProvideBuilder::new(clock, ready)
    }

    pub fn get_clock(&self) -> Option<::Clock> {
        let mut clock = ptr::null_mut();

        unsafe {
            gst_sys::gst_message_parse_clock_provide(
                self.as_mut_ptr(),
                &mut clock,
                ptr::null_mut(),
            );

            from_glib_none(clock)
        }
    }

    pub fn get_ready(&self) -> bool {
        unsafe {
            let mut ready = mem::MaybeUninit::uninit();

            gst_sys::gst_message_parse_clock_provide(
                self.as_mut_ptr(),
                ptr::null_mut(),
                ready.as_mut_ptr(),
            );

            from_glib(ready.assume_init())
        }
    }
}

declare_concrete_message!(ClockLost);
impl<'a> ClockLost<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(clock: &::Clock) -> Message {
        skip_assert_initialized!();
        Self::builder(clock).build()
    }

    pub fn builder(clock: &::Clock) -> ClockLostBuilder {
        assert_initialized_main_thread!();
        ClockLostBuilder::new(clock)
    }

    pub fn get_clock(&self) -> Option<::Clock> {
        let mut clock = ptr::null_mut();

        unsafe {
            gst_sys::gst_message_parse_clock_lost(self.as_mut_ptr(), &mut clock);

            from_glib_none(clock)
        }
    }
}

declare_concrete_message!(NewClock);
impl<'a> NewClock<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(clock: &::Clock) -> Message {
        skip_assert_initialized!();
        Self::builder(clock).build()
    }

    pub fn builder(clock: &::Clock) -> NewClockBuilder {
        assert_initialized_main_thread!();
        NewClockBuilder::new(clock)
    }

    pub fn get_clock(&self) -> Option<::Clock> {
        let mut clock = ptr::null_mut();

        unsafe {
            gst_sys::gst_message_parse_new_clock(self.as_mut_ptr(), &mut clock);

            from_glib_none(clock)
        }
    }
}

declare_concrete_message!(StructureChange);
impl<'a> StructureChange<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(type_: ::StructureChangeType, owner: &::Element, busy: bool) -> Message {
        skip_assert_initialized!();
        Self::builder(type_, owner, busy).build()
    }

    pub fn builder(
        type_: ::StructureChangeType,
        owner: &::Element,
        busy: bool,
    ) -> StructureChangeBuilder {
        assert_initialized_main_thread!();
        StructureChangeBuilder::new(type_, owner, busy)
    }

    pub fn get(&self) -> (::StructureChangeType, ::Element, bool) {
        unsafe {
            let mut type_ = mem::MaybeUninit::uninit();
            let mut owner = ptr::null_mut();
            let mut busy = mem::MaybeUninit::uninit();

            gst_sys::gst_message_parse_structure_change(
                self.as_mut_ptr(),
                type_.as_mut_ptr(),
                &mut owner,
                busy.as_mut_ptr(),
            );

            (
                from_glib(type_.assume_init()),
                from_glib_none(owner),
                from_glib(busy.assume_init()),
            )
        }
    }
}

declare_concrete_message!(StreamStatus);
impl<'a> StreamStatus<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(type_: ::StreamStatusType, owner: &::Element) -> Message {
        skip_assert_initialized!();
        Self::builder(type_, owner).build()
    }

    pub fn builder(type_: ::StreamStatusType, owner: &::Element) -> StreamStatusBuilder {
        assert_initialized_main_thread!();
        StreamStatusBuilder::new(type_, owner)
    }

    pub fn get(&self) -> (::StreamStatusType, ::Element) {
        unsafe {
            let mut type_ = mem::MaybeUninit::uninit();
            let mut owner = ptr::null_mut();

            gst_sys::gst_message_parse_stream_status(
                self.as_mut_ptr(),
                type_.as_mut_ptr(),
                &mut owner,
            );

            (from_glib(type_.assume_init()), from_glib_none(owner))
        }
    }

    pub fn get_stream_status_object(&self) -> Option<glib::Value> {
        unsafe {
            let value = gst_sys::gst_message_get_stream_status_object(self.as_mut_ptr());

            from_glib_none(value)
        }
    }
}

declare_concrete_message!(Application);
impl<'a> Application<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(structure: ::Structure) -> Message {
        skip_assert_initialized!();
        Self::builder(structure).build()
    }

    pub fn builder(structure: ::Structure) -> ApplicationBuilder<'a> {
        assert_initialized_main_thread!();
        ApplicationBuilder::new(structure)
    }
}

declare_concrete_message!(Element);
impl<'a> Element<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(structure: ::Structure) -> Message {
        skip_assert_initialized!();
        Self::builder(structure).build()
    }

    pub fn builder(structure: ::Structure) -> ElementBuilder<'a> {
        assert_initialized_main_thread!();
        ElementBuilder::new(structure)
    }
}

declare_concrete_message!(SegmentStart);
impl<'a> SegmentStart<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<V: Into<GenericFormattedValue>>(position: V) -> Message {
        skip_assert_initialized!();
        Self::builder(position).build()
    }

    pub fn builder<V: Into<GenericFormattedValue>>(position: V) -> SegmentStartBuilder<'a> {
        assert_initialized_main_thread!();
        let position = position.into();
        SegmentStartBuilder::new(position)
    }

    pub fn get(&self) -> GenericFormattedValue {
        unsafe {
            let mut format = mem::MaybeUninit::uninit();
            let mut position = mem::MaybeUninit::uninit();

            gst_sys::gst_message_parse_segment_start(
                self.as_mut_ptr(),
                format.as_mut_ptr(),
                position.as_mut_ptr(),
            );

            GenericFormattedValue::new(from_glib(format.assume_init()), position.assume_init())
        }
    }
}

declare_concrete_message!(SegmentDone);
impl<'a> SegmentDone<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<V: Into<GenericFormattedValue>>(position: V) -> Message {
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
            let mut format = mem::MaybeUninit::uninit();
            let mut position = mem::MaybeUninit::uninit();

            gst_sys::gst_message_parse_segment_done(
                self.as_mut_ptr(),
                format.as_mut_ptr(),
                position.as_mut_ptr(),
            );

            GenericFormattedValue::new(from_glib(format.assume_init()), position.assume_init())
        }
    }
}

declare_concrete_message!(DurationChanged);
impl<'a> DurationChanged<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Message {
        skip_assert_initialized!();
        Self::builder().build()
    }

    pub fn builder() -> DurationChangedBuilder<'a> {
        assert_initialized_main_thread!();
        DurationChangedBuilder::new()
    }
}

declare_concrete_message!(Latency);
impl<'a> Latency<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Message {
        skip_assert_initialized!();
        Self::builder().build()
    }

    pub fn builder() -> LatencyBuilder<'a> {
        assert_initialized_main_thread!();
        LatencyBuilder::new()
    }
}

declare_concrete_message!(AsyncStart);
impl<'a> AsyncStart<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Message {
        skip_assert_initialized!();
        Self::builder().build()
    }

    pub fn builder() -> AsyncStartBuilder<'a> {
        assert_initialized_main_thread!();
        AsyncStartBuilder::new()
    }
}

declare_concrete_message!(AsyncDone);
impl<'a> AsyncDone<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(running_time: ::ClockTime) -> Message {
        skip_assert_initialized!();
        Self::builder(running_time).build()
    }

    pub fn builder(running_time: ::ClockTime) -> AsyncDoneBuilder<'a> {
        assert_initialized_main_thread!();
        AsyncDoneBuilder::new(running_time)
    }

    pub fn get_running_time(&self) -> ::ClockTime {
        unsafe {
            let mut running_time = mem::MaybeUninit::uninit();

            gst_sys::gst_message_parse_async_done(self.as_mut_ptr(), running_time.as_mut_ptr());

            from_glib(running_time.assume_init())
        }
    }
}

declare_concrete_message!(RequestState);
impl<'a> RequestState<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(state: ::State) -> Message {
        skip_assert_initialized!();
        Self::builder(state).build()
    }

    pub fn builder(state: ::State) -> RequestStateBuilder<'a> {
        assert_initialized_main_thread!();
        RequestStateBuilder::new(state)
    }

    pub fn get_requested_state(&self) -> ::State {
        unsafe {
            let mut state = mem::MaybeUninit::uninit();

            gst_sys::gst_message_parse_request_state(self.as_mut_ptr(), state.as_mut_ptr());

            from_glib(state.assume_init())
        }
    }
}

declare_concrete_message!(StepStart);
impl<'a> StepStart<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<V: Into<GenericFormattedValue>>(
        active: bool,
        amount: V,
        rate: f64,
        flush: bool,
        intermediate: bool,
    ) -> Message {
        skip_assert_initialized!();
        Self::builder(active, amount.into(), rate, flush, intermediate).build()
    }

    pub fn builder<V: Into<GenericFormattedValue>>(
        active: bool,
        amount: V,
        rate: f64,
        flush: bool,
        intermediate: bool,
    ) -> StepStartBuilder<'a> {
        assert_initialized_main_thread!();
        StepStartBuilder::new(active, amount.into(), rate, flush, intermediate)
    }

    pub fn get(&self) -> (bool, GenericFormattedValue, f64, bool, bool) {
        unsafe {
            let mut active = mem::MaybeUninit::uninit();
            let mut format = mem::MaybeUninit::uninit();
            let mut amount = mem::MaybeUninit::uninit();
            let mut rate = mem::MaybeUninit::uninit();
            let mut flush = mem::MaybeUninit::uninit();
            let mut intermediate = mem::MaybeUninit::uninit();

            gst_sys::gst_message_parse_step_start(
                self.as_mut_ptr(),
                active.as_mut_ptr(),
                format.as_mut_ptr(),
                amount.as_mut_ptr(),
                rate.as_mut_ptr(),
                flush.as_mut_ptr(),
                intermediate.as_mut_ptr(),
            );

            (
                from_glib(active.assume_init()),
                GenericFormattedValue::new(
                    from_glib(format.assume_init()),
                    amount.assume_init() as i64,
                ),
                rate.assume_init(),
                from_glib(flush.assume_init()),
                from_glib(intermediate.assume_init()),
            )
        }
    }
}

declare_concrete_message!(Qos);
impl<'a> Qos<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        live: bool,
        running_time: ::ClockTime,
        stream_time: ::ClockTime,
        timestamp: ::ClockTime,
        duration: ::ClockTime,
    ) -> Message {
        skip_assert_initialized!();
        Self::builder(live, running_time, stream_time, timestamp, duration).build()
    }

    pub fn builder(
        live: bool,
        running_time: ::ClockTime,
        stream_time: ::ClockTime,
        timestamp: ::ClockTime,
        duration: ::ClockTime,
    ) -> QosBuilder<'a> {
        assert_initialized_main_thread!();
        QosBuilder::new(live, running_time, stream_time, timestamp, duration)
    }

    pub fn get(&self) -> (bool, ::ClockTime, ::ClockTime, ::ClockTime, ::ClockTime) {
        unsafe {
            let mut live = mem::MaybeUninit::uninit();
            let mut running_time = mem::MaybeUninit::uninit();
            let mut stream_time = mem::MaybeUninit::uninit();
            let mut timestamp = mem::MaybeUninit::uninit();
            let mut duration = mem::MaybeUninit::uninit();

            gst_sys::gst_message_parse_qos(
                self.as_mut_ptr(),
                live.as_mut_ptr(),
                running_time.as_mut_ptr(),
                stream_time.as_mut_ptr(),
                timestamp.as_mut_ptr(),
                duration.as_mut_ptr(),
            );

            (
                from_glib(live.assume_init()),
                from_glib(running_time.assume_init()),
                from_glib(stream_time.assume_init()),
                from_glib(timestamp.assume_init()),
                from_glib(duration.assume_init()),
            )
        }
    }

    pub fn get_values(&self) -> (i64, f64, i32) {
        unsafe {
            let mut jitter = mem::MaybeUninit::uninit();
            let mut proportion = mem::MaybeUninit::uninit();
            let mut quality = mem::MaybeUninit::uninit();

            gst_sys::gst_message_parse_qos_values(
                self.as_mut_ptr(),
                jitter.as_mut_ptr(),
                proportion.as_mut_ptr(),
                quality.as_mut_ptr(),
            );

            (
                jitter.assume_init(),
                proportion.assume_init(),
                quality.assume_init(),
            )
        }
    }

    pub fn get_stats(&self) -> (GenericFormattedValue, GenericFormattedValue) {
        unsafe {
            let mut format = mem::MaybeUninit::uninit();
            let mut processed = mem::MaybeUninit::uninit();
            let mut dropped = mem::MaybeUninit::uninit();

            gst_sys::gst_message_parse_qos_stats(
                self.as_mut_ptr(),
                format.as_mut_ptr(),
                processed.as_mut_ptr(),
                dropped.as_mut_ptr(),
            );

            (
                GenericFormattedValue::new(
                    from_glib(format.assume_init()),
                    processed.assume_init() as i64,
                ),
                GenericFormattedValue::new(
                    from_glib(format.assume_init()),
                    dropped.assume_init() as i64,
                ),
            )
        }
    }
}

declare_concrete_message!(Progress);
impl<'a> Progress<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(type_: ::ProgressType, code: &'a str, text: &'a str) -> Message {
        skip_assert_initialized!();
        Self::builder(type_, code, text).build()
    }

    pub fn builder(type_: ::ProgressType, code: &'a str, text: &'a str) -> ProgressBuilder<'a> {
        assert_initialized_main_thread!();
        ProgressBuilder::new(type_, code, text)
    }

    pub fn get(&self) -> (::ProgressType, &'a str, &'a str) {
        unsafe {
            let mut type_ = mem::MaybeUninit::uninit();
            let mut code = ptr::null_mut();
            let mut text = ptr::null_mut();

            gst_sys::gst_message_parse_progress(
                self.as_mut_ptr(),
                type_.as_mut_ptr(),
                &mut code,
                &mut text,
            );

            let code = CStr::from_ptr(code).to_str().unwrap();
            let text = CStr::from_ptr(text).to_str().unwrap();

            (from_glib(type_.assume_init()), code, text)
        }
    }
}

declare_concrete_message!(Toc);
impl<'a> Toc<'a> {
    // FIXME could use false for updated as default
    // Even better: use an enum for updated so that it is more explicit than true / false
    #[allow(clippy::new_ret_no_self)]
    pub fn new(toc: &::Toc, updated: bool) -> Message {
        skip_assert_initialized!();
        Self::builder(toc, updated).build()
    }

    pub fn builder(toc: &::Toc, updated: bool) -> TocBuilder {
        assert_initialized_main_thread!();
        TocBuilder::new(toc, updated)
    }

    pub fn get_toc(&self) -> (::Toc, bool) {
        unsafe {
            let mut toc = ptr::null_mut();
            let mut updated = mem::MaybeUninit::uninit();
            gst_sys::gst_message_parse_toc(self.as_mut_ptr(), &mut toc, updated.as_mut_ptr());
            (from_glib_full(toc), from_glib(updated.assume_init()))
        }
    }
}

declare_concrete_message!(ResetTime);
impl<'a> ResetTime<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(running_time: ::ClockTime) -> Message {
        skip_assert_initialized!();
        Self::builder(running_time).build()
    }

    pub fn builder(running_time: ::ClockTime) -> ResetTimeBuilder<'a> {
        assert_initialized_main_thread!();
        ResetTimeBuilder::new(running_time)
    }

    pub fn get_running_time(&self) -> ::ClockTime {
        unsafe {
            let mut running_time = mem::MaybeUninit::uninit();

            gst_sys::gst_message_parse_reset_time(self.as_mut_ptr(), running_time.as_mut_ptr());

            from_glib(running_time.assume_init())
        }
    }
}

declare_concrete_message!(StreamStart);
impl<'a> StreamStart<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Message {
        skip_assert_initialized!();
        Self::builder().build()
    }

    pub fn builder() -> StreamStartBuilder<'a> {
        assert_initialized_main_thread!();
        StreamStartBuilder::new()
    }

    pub fn get_group_id(&self) -> Option<GroupId> {
        unsafe {
            let mut group_id = mem::MaybeUninit::uninit();

            if from_glib(gst_sys::gst_message_parse_group_id(
                self.as_mut_ptr(),
                group_id.as_mut_ptr(),
            )) {
                let group_id = group_id.assume_init();
                if group_id == 0 {
                    None
                } else {
                    Some(GroupId(NonZeroU32::new_unchecked(group_id)))
                }
            } else {
                None
            }
        }
    }
}

declare_concrete_message!(NeedContext);
impl<'a> NeedContext<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(context_type: &str) -> Message {
        skip_assert_initialized!();
        Self::builder(context_type).build()
    }

    pub fn builder(context_type: &str) -> NeedContextBuilder {
        assert_initialized_main_thread!();
        NeedContextBuilder::new(context_type)
    }

    pub fn get_context_type(&self) -> &str {
        unsafe {
            let mut context_type = ptr::null();

            gst_sys::gst_message_parse_context_type(self.as_mut_ptr(), &mut context_type);

            CStr::from_ptr(context_type).to_str().unwrap()
        }
    }
}

declare_concrete_message!(HaveContext);
impl<'a> HaveContext<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(context: ::Context) -> Message {
        skip_assert_initialized!();
        Self::builder(context).build()
    }

    pub fn builder(context: ::Context) -> HaveContextBuilder<'a> {
        assert_initialized_main_thread!();
        HaveContextBuilder::new(context)
    }

    pub fn get_context(&self) -> ::Context {
        unsafe {
            let mut context = ptr::null_mut();
            gst_sys::gst_message_parse_have_context(self.as_mut_ptr(), &mut context);
            from_glib_full(context)
        }
    }
}

declare_concrete_message!(DeviceAdded);
impl<'a> DeviceAdded<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(device: &::Device) -> Message {
        skip_assert_initialized!();
        Self::builder(device).build()
    }

    pub fn builder(device: &::Device) -> DeviceAddedBuilder {
        assert_initialized_main_thread!();
        DeviceAddedBuilder::new(device)
    }

    pub fn get_device(&self) -> ::Device {
        unsafe {
            let mut device = ptr::null_mut();

            gst_sys::gst_message_parse_device_added(self.as_mut_ptr(), &mut device);

            from_glib_full(device)
        }
    }
}

declare_concrete_message!(DeviceRemoved);
impl<'a> DeviceRemoved<'a> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(device: &::Device) -> Message {
        skip_assert_initialized!();
        Self::builder(device).build()
    }

    pub fn builder(device: &::Device) -> DeviceRemovedBuilder {
        assert_initialized_main_thread!();
        DeviceRemovedBuilder::new(device)
    }

    pub fn get_device(&self) -> ::Device {
        unsafe {
            let mut device = ptr::null_mut();

            gst_sys::gst_message_parse_device_removed(self.as_mut_ptr(), &mut device);

            from_glib_full(device)
        }
    }
}

declare_concrete_message!(PropertyNotify);
impl<'a> PropertyNotify<'a> {
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(property_name: &str) -> Message {
        skip_assert_initialized!();
        Self::builder(property_name).build()
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn builder(property_name: &str) -> PropertyNotifyBuilder {
        assert_initialized_main_thread!();
        PropertyNotifyBuilder::new(property_name)
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get(&self) -> (Object, &str, Option<&'a glib::Value>) {
        unsafe {
            let mut object = ptr::null_mut();
            let mut property_name = ptr::null();
            let mut value = ptr::null();

            gst_sys::gst_message_parse_property_notify(
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
    #[allow(clippy::new_ret_no_self)]
    pub fn new(collection: &::StreamCollection) -> Message {
        skip_assert_initialized!();
        Self::builder(collection).build()
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn builder(collection: &::StreamCollection) -> StreamCollectionBuilder {
        assert_initialized_main_thread!();
        StreamCollectionBuilder::new(collection)
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_stream_collection(&self) -> ::StreamCollection {
        unsafe {
            let mut collection = ptr::null_mut();

            gst_sys::gst_message_parse_stream_collection(self.as_mut_ptr(), &mut collection);

            from_glib_full(collection)
        }
    }
}

declare_concrete_message!(StreamsSelected);
impl<'a> StreamsSelected<'a> {
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(collection: &::StreamCollection) -> Message {
        skip_assert_initialized!();
        Self::builder(collection).build()
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn builder(collection: &::StreamCollection) -> StreamsSelectedBuilder {
        assert_initialized_main_thread!();
        StreamsSelectedBuilder::new(collection)
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_stream_collection(&self) -> ::StreamCollection {
        unsafe {
            let mut collection = ptr::null_mut();

            gst_sys::gst_message_parse_streams_selected(self.as_mut_ptr(), &mut collection);

            from_glib_full(collection)
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_streams(&self) -> Vec<::Stream> {
        unsafe {
            let n = gst_sys::gst_message_streams_selected_get_size(self.as_mut_ptr());

            (0..n)
                .map(|i| {
                    from_glib_full(gst_sys::gst_message_streams_selected_get_stream(
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
    #[allow(clippy::new_ret_no_self)]
    pub fn new(location: &str) -> Message {
        skip_assert_initialized!();
        Self::builder(location).build()
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn builder(location: &str) -> RedirectBuilder {
        assert_initialized_main_thread!();
        RedirectBuilder::new(location)
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_entries(&self) -> Vec<(&str, Option<TagList>, Option<&StructureRef>)> {
        unsafe {
            let n = gst_sys::gst_message_get_num_redirect_entries(self.as_mut_ptr());

            (0..n)
                .map(|i| {
                    let mut location = ptr::null();
                    let mut tags = ptr::null_mut();
                    let mut structure = ptr::null();

                    gst_sys::gst_message_parse_redirect_entry(
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

declare_concrete_message!(DeviceChanged);
impl<'a> DeviceChanged<'a> {
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[allow(clippy::new_ret_no_self)]
    pub fn new(device: &'a ::Device, changed_device: &'a ::Device) -> Message {
        skip_assert_initialized!();
        Self::builder(device, changed_device).build()
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn builder(device: &'a ::Device, changed_device: &'a ::Device) -> DeviceChangedBuilder<'a> {
        assert_initialized_main_thread!();
        DeviceChangedBuilder::new(device, changed_device)
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn get_device_changed(&self) -> (::Device, ::Device) {
        unsafe {
            let mut device = ptr::null_mut();
            let mut changed_device = ptr::null_mut();

            gst_sys::gst_message_parse_device_changed(
                self.as_mut_ptr(),
                &mut device,
                &mut changed_device,
            );

            (from_glib_full(device), from_glib_full(changed_device))
        }
    }
}

struct MessageBuilder<'a> {
    src: Option<Object>,
    seqnum: Option<Seqnum>,
    #[allow(unused)]
    other_fields: Vec<(&'a str, &'a dyn ToSendValue)>,
}

impl<'a> MessageBuilder<'a> {
    fn new() -> Self {
        Self {
            src: None,
            seqnum: None,
            other_fields: Vec::new(),
        }
    }

    pub fn src<O: IsA<Object> + Cast + Clone>(self, src: &O) -> Self {
        Self {
            src: Some(src.clone().upcast::<Object>()),
            ..self
        }
    }

    fn seqnum(self, seqnum: Seqnum) -> Self {
        Self {
            seqnum: Some(seqnum),
            ..self
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
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

macro_rules! message_builder_generic_impl {
    ($new_fn:expr) => {
        #[allow(clippy::needless_update)]
        pub fn src<O: IsA<Object> + Cast + Clone>(self, src: &O) -> Self {
            Self {
                builder: self.builder.src(src),
                ..self
            }
        }

        #[allow(clippy::needless_update)]
        pub fn seqnum(self, seqnum: Seqnum) -> Self {
            Self {
                builder: self.builder.seqnum(seqnum),
                ..self
            }
        }

        #[cfg(any(feature = "v1_14", feature = "dox"))]
        #[allow(clippy::needless_update)]
        pub fn other_fields(self, other_fields: &[(&'a str, &'a dyn ToSendValue)]) -> Self {
            Self {
                builder: self.builder.other_fields(other_fields),
                ..self
            }
        }

        pub fn build(mut self) -> Message {
            assert_initialized_main_thread!();
            unsafe {
                let src = self.builder.src.to_glib_none().0;
                let msg = $new_fn(&mut self, src);
                if let Some(seqnum) = self.builder.seqnum {
                    gst_sys::gst_message_set_seqnum(msg, seqnum.0.get());
                }

                #[cfg(any(feature = "v1_14", feature = "dox"))]
                {
                    if !self.builder.other_fields.is_empty() {
                        let structure = gst_sys::gst_message_writable_structure(msg);

                        if !structure.is_null() {
                            let structure = StructureRef::from_glib_borrow_mut(structure as *mut _);

                            for (k, v) in self.builder.other_fields {
                                structure.set_value(k, v.to_send_value());
                            }
                        }
                    }
                }

                from_glib_full(msg)
            }
        }
    };
}

pub struct EosBuilder<'a> {
    builder: MessageBuilder<'a>,
}

impl<'a> EosBuilder<'a> {
    fn new() -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
        }
    }

    message_builder_generic_impl!(|_, src| gst_sys::gst_message_new_eos(src));
}

pub trait MessageErrorDomain: glib::error::ErrorDomain {}

impl MessageErrorDomain for ::CoreError {}
impl MessageErrorDomain for ::ResourceError {}
impl MessageErrorDomain for ::StreamError {}
impl MessageErrorDomain for ::LibraryError {}

pub struct ErrorBuilder<'a, T> {
    builder: MessageBuilder<'a>,
    error: T,
    message: &'a str,
    debug: Option<&'a str>,
    #[allow(unused)]
    details: Option<Structure>,
}

impl<'a, T: MessageErrorDomain> ErrorBuilder<'a, T> {
    fn new(error: T, message: &'a str) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            error,
            message,
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

            gst_sys::gst_message_new_error_with_details(
                src,
                mut_override(error.to_glib_none().0),
                s.debug.to_glib_none().0,
                details,
            )
        }
        #[cfg(not(any(feature = "v1_10", feature = "dox")))]
        {
            let error = glib::Error::new(s.error, s.message);

            gst_sys::gst_message_new_error(
                src,
                mut_override(error.to_glib_none().0),
                s.debug.to_glib_none().0,
            )
        }
    });
}

pub struct WarningBuilder<'a, T> {
    builder: MessageBuilder<'a>,
    error: T,
    message: &'a str,
    debug: Option<&'a str>,
    #[allow(unused)]
    details: Option<Structure>,
}

impl<'a, T: MessageErrorDomain> WarningBuilder<'a, T> {
    fn new(error: T, message: &'a str) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            error,
            message,
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

            gst_sys::gst_message_new_warning_with_details(
                src,
                mut_override(error.to_glib_none().0),
                s.debug.to_glib_none().0,
                details,
            )
        }
        #[cfg(not(any(feature = "v1_10", feature = "dox")))]
        {
            let error = glib::Error::new(s.error, s.message);

            gst_sys::gst_message_new_warning(
                src,
                mut_override(error.to_glib_none().0),
                s.debug.to_glib_none().0,
            )
        }
    });
}

pub struct InfoBuilder<'a, T> {
    builder: MessageBuilder<'a>,
    error: T,
    message: &'a str,
    debug: Option<&'a str>,
    #[allow(unused)]
    details: Option<Structure>,
}

impl<'a, T: MessageErrorDomain> InfoBuilder<'a, T> {
    fn new(error: T, message: &'a str) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            error,
            message,
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

            gst_sys::gst_message_new_info_with_details(
                src,
                mut_override(error.to_glib_none().0),
                s.debug.to_glib_none().0,
                details,
            )
        }
        #[cfg(not(any(feature = "v1_10", feature = "dox")))]
        {
            let error = glib::Error::new(s.error, s.message);

            gst_sys::gst_message_new_info(
                src,
                mut_override(error.to_glib_none().0),
                s.debug.to_glib_none().0,
            )
        }
    });
}

pub struct TagBuilder<'a> {
    builder: MessageBuilder<'a>,
    tags: &'a TagList,
}

impl<'a> TagBuilder<'a> {
    fn new(tags: &'a TagList) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            tags,
        }
    }

    message_builder_generic_impl!(|s: &Self, src| gst_sys::gst_message_new_tag(
        src,
        s.tags.to_glib_full()
    ));
}

pub struct BufferingBuilder<'a> {
    builder: MessageBuilder<'a>,
    percent: i32,
    stats: Option<(::BufferingMode, i32, i32, i64)>,
}

impl<'a> BufferingBuilder<'a> {
    fn new(percent: i32) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            percent,
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
        let msg = gst_sys::gst_message_new_buffering(src, s.percent);

        if let Some((mode, avg_in, avg_out, buffering_left)) = s.stats {
            gst_sys::gst_message_set_buffering_stats(
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
    builder: MessageBuilder<'a>,
    old: ::State,
    new: ::State,
    pending: ::State,
}

impl<'a> StateChangedBuilder<'a> {
    fn new(old: ::State, new: ::State, pending: ::State) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            old,
            new,
            pending,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| gst_sys::gst_message_new_state_changed(
        src,
        s.old.to_glib(),
        s.new.to_glib(),
        s.pending.to_glib(),
    ));
}

pub struct StateDirtyBuilder<'a> {
    builder: MessageBuilder<'a>,
}

impl<'a> StateDirtyBuilder<'a> {
    fn new() -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
        }
    }

    message_builder_generic_impl!(|_, src| gst_sys::gst_message_new_state_dirty(src));
}

pub struct StepDoneBuilder<'a> {
    builder: MessageBuilder<'a>,
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
            builder: MessageBuilder::new(),
            amount,
            rate,
            flush,
            intermediate,
            duration,
            eos,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| gst_sys::gst_message_new_step_done(
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
    builder: MessageBuilder<'a>,
    clock: &'a ::Clock,
    ready: bool,
}

impl<'a> ClockProvideBuilder<'a> {
    fn new(clock: &'a ::Clock, ready: bool) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            clock,
            ready,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| gst_sys::gst_message_new_clock_provide(
        src,
        s.clock.to_glib_none().0,
        s.ready.to_glib()
    ));
}

pub struct ClockLostBuilder<'a> {
    builder: MessageBuilder<'a>,
    clock: &'a ::Clock,
}

impl<'a> ClockLostBuilder<'a> {
    fn new(clock: &'a ::Clock) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            clock,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| gst_sys::gst_message_new_clock_lost(
        src,
        s.clock.to_glib_none().0
    ));
}

pub struct NewClockBuilder<'a> {
    builder: MessageBuilder<'a>,
    clock: &'a ::Clock,
}

impl<'a> NewClockBuilder<'a> {
    fn new(clock: &'a ::Clock) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            clock,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| gst_sys::gst_message_new_new_clock(
        src,
        s.clock.to_glib_none().0
    ));
}

pub struct StructureChangeBuilder<'a> {
    builder: MessageBuilder<'a>,
    type_: ::StructureChangeType,
    owner: &'a ::Element,
    busy: bool,
}

impl<'a> StructureChangeBuilder<'a> {
    fn new(type_: ::StructureChangeType, owner: &'a ::Element, busy: bool) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            type_,
            owner,
            busy,
        }
    }

    message_builder_generic_impl!(
        |s: &mut Self, src| gst_sys::gst_message_new_structure_change(
            src,
            s.type_.to_glib(),
            s.owner.to_glib_none().0,
            s.busy.to_glib(),
        )
    );
}

pub struct StreamStatusBuilder<'a> {
    builder: MessageBuilder<'a>,
    type_: ::StreamStatusType,
    owner: &'a ::Element,
    status_object: Option<&'a dyn glib::ToSendValue>,
}

impl<'a> StreamStatusBuilder<'a> {
    fn new(type_: ::StreamStatusType, owner: &'a ::Element) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            type_,
            owner,
            status_object: None,
        }
    }

    pub fn status_object(self, status_object: &'a dyn glib::ToSendValue) -> Self {
        Self {
            status_object: Some(status_object),
            ..self
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        let msg = gst_sys::gst_message_new_stream_status(
            src,
            s.type_.to_glib(),
            s.owner.to_glib_none().0,
        );
        if let Some(status_object) = s.status_object {
            gst_sys::gst_message_set_stream_status_object(
                msg,
                status_object.to_send_value().to_glib_none().0,
            );
        }
        msg
    });
}

pub struct ApplicationBuilder<'a> {
    builder: MessageBuilder<'a>,
    structure: Option<::Structure>,
}

impl<'a> ApplicationBuilder<'a> {
    fn new(structure: ::Structure) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            structure: Some(structure),
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| gst_sys::gst_message_new_application(
        src,
        s.structure.take().unwrap().into_ptr()
    ));
}

pub struct ElementBuilder<'a> {
    builder: MessageBuilder<'a>,
    structure: Option<::Structure>,
}

impl<'a> ElementBuilder<'a> {
    fn new(structure: ::Structure) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            structure: Some(structure),
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| gst_sys::gst_message_new_element(
        src,
        s.structure.take().unwrap().into_ptr()
    ));
}

pub struct SegmentStartBuilder<'a> {
    builder: MessageBuilder<'a>,
    position: GenericFormattedValue,
}

impl<'a> SegmentStartBuilder<'a> {
    fn new(position: GenericFormattedValue) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            position,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| gst_sys::gst_message_new_segment_start(
        src,
        s.position.get_format().to_glib(),
        s.position.get_value(),
    ));
}

pub struct SegmentDoneBuilder<'a> {
    builder: MessageBuilder<'a>,
    position: GenericFormattedValue,
}

impl<'a> SegmentDoneBuilder<'a> {
    fn new(position: GenericFormattedValue) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            position,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| gst_sys::gst_message_new_segment_done(
        src,
        s.position.get_format().to_glib(),
        s.position.get_value(),
    ));
}

pub struct DurationChangedBuilder<'a> {
    builder: MessageBuilder<'a>,
}

impl<'a> DurationChangedBuilder<'a> {
    fn new() -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
        }
    }

    message_builder_generic_impl!(|_, src| gst_sys::gst_message_new_duration_changed(src));
}

pub struct LatencyBuilder<'a> {
    builder: MessageBuilder<'a>,
}

impl<'a> LatencyBuilder<'a> {
    fn new() -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
        }
    }

    message_builder_generic_impl!(|_, src| gst_sys::gst_message_new_latency(src));
}

pub struct AsyncStartBuilder<'a> {
    builder: MessageBuilder<'a>,
}

impl<'a> AsyncStartBuilder<'a> {
    fn new() -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
        }
    }

    message_builder_generic_impl!(|_, src| gst_sys::gst_message_new_async_start(src));
}

pub struct AsyncDoneBuilder<'a> {
    builder: MessageBuilder<'a>,
    running_time: ::ClockTime,
}

impl<'a> AsyncDoneBuilder<'a> {
    fn new(running_time: ::ClockTime) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            running_time,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| gst_sys::gst_message_new_async_done(
        src,
        s.running_time.to_glib()
    ));
}

pub struct RequestStateBuilder<'a> {
    builder: MessageBuilder<'a>,
    state: ::State,
}

impl<'a> RequestStateBuilder<'a> {
    fn new(state: ::State) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            state,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| gst_sys::gst_message_new_request_state(
        src,
        s.state.to_glib()
    ));
}

pub struct StepStartBuilder<'a> {
    builder: MessageBuilder<'a>,
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
            builder: MessageBuilder::new(),
            active,
            amount,
            rate,
            flush,
            intermediate,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| gst_sys::gst_message_new_step_start(
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
    builder: MessageBuilder<'a>,
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
            builder: MessageBuilder::new(),
            live,
            running_time,
            stream_time,
            timestamp,
            duration,
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
        let msg = gst_sys::gst_message_new_qos(
            src,
            s.live.to_glib(),
            s.running_time.to_glib(),
            s.stream_time.to_glib(),
            s.timestamp.to_glib(),
            s.duration.to_glib(),
        );
        if let Some((jitter, proportion, quality)) = s.values {
            gst_sys::gst_message_set_qos_values(msg, jitter, proportion, quality);
        }
        if let Some((processed, dropped)) = s.stats {
            gst_sys::gst_message_set_qos_stats(
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
    builder: MessageBuilder<'a>,
    type_: ::ProgressType,
    code: &'a str,
    text: &'a str,
}

impl<'a> ProgressBuilder<'a> {
    fn new(type_: ::ProgressType, code: &'a str, text: &'a str) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            type_,
            code,
            text,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| gst_sys::gst_message_new_progress(
        src,
        s.type_.to_glib(),
        s.code.to_glib_none().0,
        s.text.to_glib_none().0,
    ));
}

pub struct TocBuilder<'a> {
    builder: MessageBuilder<'a>,
    toc: &'a ::Toc,
    updated: bool,
}

impl<'a> TocBuilder<'a> {
    fn new(toc: &'a ::Toc, updated: bool) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            toc,
            updated,
        }
    }

    message_builder_generic_impl!(|s: &Self, src| gst_sys::gst_message_new_toc(
        src,
        s.toc.to_glib_none().0,
        s.updated.to_glib()
    ));
}

pub struct ResetTimeBuilder<'a> {
    builder: MessageBuilder<'a>,
    running_time: ::ClockTime,
}

impl<'a> ResetTimeBuilder<'a> {
    fn new(running_time: ::ClockTime) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            running_time,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| gst_sys::gst_message_new_reset_time(
        src,
        s.running_time.to_glib()
    ));
}

pub struct StreamStartBuilder<'a> {
    builder: MessageBuilder<'a>,
    group_id: Option<GroupId>,
}

impl<'a> StreamStartBuilder<'a> {
    fn new() -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
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
        let msg = gst_sys::gst_message_new_stream_start(src);
        if let Some(group_id) = s.group_id {
            gst_sys::gst_message_set_group_id(msg, group_id.0.get());
        }
        msg
    });
}

pub struct NeedContextBuilder<'a> {
    builder: MessageBuilder<'a>,
    context_type: &'a str,
}

impl<'a> NeedContextBuilder<'a> {
    fn new(context_type: &'a str) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            context_type,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| gst_sys::gst_message_new_need_context(
        src,
        s.context_type.to_glib_none().0
    ));
}

pub struct HaveContextBuilder<'a> {
    builder: MessageBuilder<'a>,
    context: Option<::Context>,
}

impl<'a> HaveContextBuilder<'a> {
    fn new(context: ::Context) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            context: Some(context),
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        let context = s.context.take().unwrap();
        gst_sys::gst_message_new_have_context(src, context.into_ptr())
    });
}

pub struct DeviceAddedBuilder<'a> {
    builder: MessageBuilder<'a>,
    device: &'a ::Device,
}

impl<'a> DeviceAddedBuilder<'a> {
    fn new(device: &'a ::Device) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            device,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| gst_sys::gst_message_new_device_added(
        src,
        s.device.to_glib_none().0
    ));
}

pub struct DeviceRemovedBuilder<'a> {
    builder: MessageBuilder<'a>,
    device: &'a ::Device,
}

impl<'a> DeviceRemovedBuilder<'a> {
    fn new(device: &'a ::Device) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            device,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| gst_sys::gst_message_new_device_removed(
        src,
        s.device.to_glib_none().0
    ));
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
pub struct PropertyNotifyBuilder<'a> {
    builder: MessageBuilder<'a>,
    property_name: &'a str,
    value: Option<&'a dyn glib::ToSendValue>,
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
impl<'a> PropertyNotifyBuilder<'a> {
    fn new(property_name: &'a str) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            property_name,
            value: None,
        }
    }

    pub fn value(self, value: &'a dyn glib::ToSendValue) -> Self {
        Self {
            value: Some(value),
            ..self
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        let val = s.value.map(|v| v.to_send_value());
        gst_sys::gst_message_new_property_notify(
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
    builder: MessageBuilder<'a>,
    collection: &'a ::StreamCollection,
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
impl<'a> StreamCollectionBuilder<'a> {
    fn new(collection: &'a ::StreamCollection) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            collection,
        }
    }

    message_builder_generic_impl!(
        |s: &mut Self, src| gst_sys::gst_message_new_stream_collection(
            src,
            s.collection.to_glib_none().0
        )
    );
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
pub struct StreamsSelectedBuilder<'a> {
    builder: MessageBuilder<'a>,
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    collection: &'a ::StreamCollection,
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    streams: Option<&'a [&'a ::Stream]>,
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
impl<'a> StreamsSelectedBuilder<'a> {
    fn new(collection: &'a ::StreamCollection) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            collection,
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
        let msg = gst_sys::gst_message_new_streams_selected(src, s.collection.to_glib_none().0);
        if let Some(streams) = s.streams {
            for stream in streams {
                gst_sys::gst_message_streams_selected_add(msg, stream.to_glib_none().0);
            }
        }
        msg
    });
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
pub struct RedirectBuilder<'a> {
    builder: MessageBuilder<'a>,
    location: &'a str,
    tag_list: Option<&'a TagList>,
    entry_struct: Option<Structure>,
    #[allow(clippy::type_complexity)]
    entries: Option<&'a [(&'a str, Option<&'a TagList>, Option<&'a Structure>)]>,
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
impl<'a> RedirectBuilder<'a> {
    fn new(location: &'a str) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            location,
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

        let msg = gst_sys::gst_message_new_redirect(
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
                gst_sys::gst_message_add_redirect_entry(
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

#[cfg(any(feature = "v1_16", feature = "dox"))]
pub struct DeviceChangedBuilder<'a> {
    builder: MessageBuilder<'a>,
    device: &'a ::Device,
    changed_device: &'a ::Device,
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
impl<'a> DeviceChangedBuilder<'a> {
    fn new(device: &'a ::Device, changed_device: &'a ::Device) -> Self {
        skip_assert_initialized!();
        Self {
            builder: MessageBuilder::new(),
            device,
            changed_device,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| gst_sys::gst_message_new_device_changed(
        src,
        s.device.to_glib_none().0,
        s.changed_device.to_glib_none().0,
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        ::init().unwrap();

        // Message without arguments
        let seqnum = Seqnum::next();
        let eos_msg = Eos::builder().seqnum(seqnum).build();
        match eos_msg.view() {
            MessageView::Eos(eos_msg) => {
                assert_eq!(eos_msg.get_seqnum(), seqnum);
                assert!(eos_msg.get_structure().is_none());
            }
            _ => panic!("eos_msg.view() is not a MessageView::Eos(_)"),
        }

        // Message with arguments
        let buffering_msg = Buffering::new(42);
        match buffering_msg.view() {
            MessageView::Buffering(buffering_msg) => {
                assert_eq!(buffering_msg.get_percent(), 42);
            }
            _ => panic!("buffering_msg.view() is not a MessageView::Buffering(_)"),
        }
    }

    #[cfg(feature = "v1_14")]
    #[test]
    fn test_other_fields() {
        ::init().unwrap();

        let seqnum = Seqnum::next();
        let eos_msg = Eos::builder()
            .other_fields(&[("extra-field", &true)])
            .seqnum(seqnum)
            .build();
        match eos_msg.view() {
            MessageView::Eos(eos_msg) => {
                assert_eq!(eos_msg.get_seqnum(), seqnum);
                if let Some(other_fields) = eos_msg.get_structure() {
                    assert!(other_fields.has_field("extra-field"));
                }
            }
            _ => panic!("eos_msg.view() is not a MessageView::Eos(_)"),
        }

        let buffering_msg = Buffering::builder(42)
            .other_fields(&[("extra-field", &true)])
            .build();
        match buffering_msg.view() {
            MessageView::Buffering(buffering_msg) => {
                assert_eq!(buffering_msg.get_percent(), 42);
                if let Some(other_fields) = buffering_msg.get_structure() {
                    assert!(other_fields.has_field("extra-field"));
                }
            }
            _ => panic!("buffering_msg.view() is not a MessageView::Buffering(_)"),
        }
    }

    #[test]
    fn test_get_seqnum_valid() {
        ::init().unwrap();

        let msg = StreamStart::new();
        let seqnum = Seqnum(
            NonZeroU32::new(unsafe { gst_sys::gst_message_get_seqnum(msg.as_mut_ptr()) }).unwrap(),
        );

        match msg.view() {
            MessageView::StreamStart(stream_start) => assert_eq!(seqnum, stream_start.get_seqnum()),
            _ => panic!(),
        }
    }

    #[test]
    fn test_get_seqnum_invalid() {
        ::init().unwrap();

        let msg = StreamStart::new();
        let seqnum_init = msg.get_seqnum();

        // Invalid the seqnum
        unsafe {
            (*msg.as_mut_ptr()).seqnum = gst_sys::GST_SEQNUM_INVALID as u32;
            assert_eq!(0, (*msg.as_ptr()).seqnum);
        };

        match msg.view() {
            MessageView::StreamStart(stream_start) => {
                // get_seqnum is expected to return a new Seqnum,
                // further in the sequence than the last known seqnum.
                assert!(seqnum_init < stream_start.get_seqnum());
            }
            _ => panic!(),
        }
    }
}
