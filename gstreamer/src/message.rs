// Copyright (C) 2016-2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use Object;
use Element;
use miniobject::*;

use std::ptr;
use std::mem;
use std::ffi::CStr;

use glib;
use glib_ffi;
use glib::translate::{from_glib, from_glib_none, from_glib_full, mut_override, ToGlibPtr, ToGlib};

#[repr(C)]
pub struct MessageImpl(ffi::GstMessage);

pub type Message = GstRc<MessageImpl>;

unsafe impl MiniObject for MessageImpl {
    type GstType = ffi::GstMessage;
}

impl MessageImpl {
    pub fn get_src(&self) -> Object {
        unsafe {
            from_glib_none((*self.as_ptr()).src)
        }
    }

    pub fn get_seqnum(&self) -> u32 {
        unsafe {
            ffi::gst_message_get_seqnum(self.as_mut_ptr())
        }
    }

    // TODO get_structure()

    pub fn view(&self) -> MessageView {
        let type_ = unsafe { (*self.as_ptr()).type_ };

        if type_ == ffi::GST_MESSAGE_EOS {
            MessageView::Eos
        } else if type_ == ffi::GST_MESSAGE_ERROR {
            MessageView::Error(Error(self))
        } else if type_ == ffi::GST_MESSAGE_WARNING {
            MessageView::Warning(Warning(self))
        } else if type_ == ffi::GST_MESSAGE_INFO {
            MessageView::Info(Info(self))
        } else if type_ == ffi::GST_MESSAGE_TAG {
            MessageView::Tag(Tag(self))
        } else if type_ == ffi::GST_MESSAGE_BUFFERING {
            MessageView::Buffering(Buffering(self))
        } else if type_ == ffi::GST_MESSAGE_STATE_CHANGED {
            MessageView::StateChanged(StateChanged(self))
        } else if type_ == ffi::GST_MESSAGE_STATE_DIRTY {
            MessageView::StateDirty
        } else if type_ == ffi::GST_MESSAGE_STEP_DONE {
            MessageView::StepDone(StepDone(self))
        } else if type_ == ffi::GST_MESSAGE_CLOCK_PROVIDE {
            MessageView::ClockProvide(ClockProvide(self))
        } else if type_ == ffi::GST_MESSAGE_CLOCK_LOST {
            MessageView::ClockLost(ClockLost(self))
        } else if type_ == ffi::GST_MESSAGE_NEW_CLOCK {
            MessageView::NewClock(NewClock(self))
        } else if type_ == ffi::GST_MESSAGE_STRUCTURE_CHANGE {
            MessageView::StructureChange(StructureChange(self))
        } else if type_ == ffi::GST_MESSAGE_STREAM_STATUS {
            MessageView::StreamStatus(StreamStatus(self))
        } else if type_ == ffi::GST_MESSAGE_APPLICATION {
            MessageView::Application
        } else if type_ == ffi::GST_MESSAGE_ELEMENT {
            MessageView::Element
        } else if type_ == ffi::GST_MESSAGE_SEGMENT_START {
            MessageView::SegmentStart(SegmentStart(self))
        } else if type_ == ffi::GST_MESSAGE_SEGMENT_DONE {
            MessageView::SegmentDone(SegmentDone(self))
        } else if type_ == ffi::GST_MESSAGE_DURATION_CHANGED {
            MessageView::DurationChanged
        } else if type_ == ffi::GST_MESSAGE_LATENCY {
            MessageView::Latency
        } else if type_ == ffi::GST_MESSAGE_ASYNC_START {
            MessageView::AsyncStart
        } else if type_ == ffi::GST_MESSAGE_ASYNC_DONE {
            MessageView::AsyncDone(AsyncDone(self))
        } else if type_ == ffi::GST_MESSAGE_REQUEST_STATE {
            MessageView::RequestState(RequestState(self))
        } else if type_ == ffi::GST_MESSAGE_STEP_START {
            MessageView::StepStart(StepStart(self))
        } else if type_ == ffi::GST_MESSAGE_QOS {
            MessageView::Qos(Qos(self))
        } else if type_ == ffi::GST_MESSAGE_PROGRESS {
            MessageView::Progress(Progress(self))
        } else if type_ == ffi::GST_MESSAGE_TOC {
            MessageView::Toc(Toc(self))
        } else if type_ == ffi::GST_MESSAGE_RESET_TIME {
            MessageView::ResetTime(ResetTime(self))
        } else if type_ == ffi::GST_MESSAGE_STREAM_START {
            MessageView::StreamStart(StreamStart(self))
        } else if type_ == ffi::GST_MESSAGE_NEED_CONTEXT {
            MessageView::NeedContext(NeedContext(self))
        } else if type_ == ffi::GST_MESSAGE_HAVE_CONTEXT {
            MessageView::HaveContext(HaveContext(self))
        } else if type_ == ffi::GST_MESSAGE_DEVICE_ADDED {
            MessageView::DeviceAdded(DeviceAdded(self))
        } else if type_ == ffi::GST_MESSAGE_DEVICE_REMOVED {
            MessageView::DeviceRemoved(DeviceRemoved(self))
        } else if type_ == ffi::GST_MESSAGE_PROPERTY_NOTIFY {
            MessageView::PropertyNotify(PropertyNotify(self))
        } else if type_ == ffi::GST_MESSAGE_STREAM_COLLECTION {
            MessageView::StreamCollection(StreamCollection(self))
        } else if type_ == ffi::GST_MESSAGE_STREAMS_SELECTED {
            MessageView::StreamsSelected(StreamsSelected(self))
        } else {
            MessageView::Other
        }
    }
}

impl glib::types::StaticType for GstRc<MessageImpl> {
    fn static_type() -> glib::types::Type {
        unsafe {
            from_glib(ffi::gst_message_get_type())
        }
    }
}

pub enum MessageView<'a> {
    Eos,
    Error(Error<'a>),
    Warning(Warning<'a>),
    Info(Info<'a>),
    Tag(Tag<'a>),
    Buffering(Buffering<'a>),
    StateChanged(StateChanged<'a>),
    StateDirty,
    StepDone(StepDone<'a>),
    ClockProvide(ClockProvide<'a>),
    ClockLost(ClockLost<'a>),
    NewClock(NewClock<'a>),
    StructureChange(StructureChange<'a>),
    StreamStatus(StreamStatus<'a>),
    Application,
    Element,
    SegmentStart(SegmentStart<'a>),
    SegmentDone(SegmentDone<'a>),
    DurationChanged,
    Latency,
    AsyncStart,
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

pub struct Error<'a>(&'a MessageImpl);
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

    // TODO get_details()
}

pub struct Warning<'a>(&'a MessageImpl);
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

    // TODO get_details()
}

pub struct Info<'a>(&'a MessageImpl);
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

    // TODO get_details()
}

pub struct Tag<'a>(&'a MessageImpl);
impl<'a> Tag<'a> {
    // TODO: get_tags()
}

pub struct Buffering<'a>(&'a MessageImpl);
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

            ffi::gst_message_parse_buffering_stats(self.0.as_mut_ptr(), &mut mode, &mut avg_in, &mut avg_out, &mut buffering_left);

            (from_glib(mode), avg_in, avg_out, buffering_left)
        }
    }
}

pub struct StateChanged<'a>(&'a MessageImpl);
impl<'a> StateChanged<'a> {
    pub fn get_old(&self) -> ::State {
        unsafe {
            let mut state = mem::uninitialized();

            ffi::gst_message_parse_state_changed(self.0.as_mut_ptr(), &mut state, ptr::null_mut(), ptr::null_mut());

            from_glib(state)
        }
    }

    pub fn get_current(&self) -> ::State {
        unsafe {
            let mut state = mem::uninitialized();

            ffi::gst_message_parse_state_changed(self.0.as_mut_ptr(), ptr::null_mut(), &mut state, ptr::null_mut());

            from_glib(state)
        }
    }

    pub fn get_pending(&self) -> ::State {
        unsafe {
            let mut state = mem::uninitialized();

            ffi::gst_message_parse_state_changed(self.0.as_mut_ptr(), ptr::null_mut(), ptr::null_mut(), &mut state);

            from_glib(state)
        }
    }
}

pub struct StepDone<'a>(&'a MessageImpl);
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

            ffi::gst_message_parse_step_done(self.0.as_mut_ptr(), &mut format, &mut amount, &mut rate, &mut flush, &mut intermediate, &mut duration, &mut eos);

            (from_glib(format), amount, rate, from_glib(flush), from_glib(intermediate), duration, from_glib(eos))
        }
    }
}


pub struct ClockProvide<'a>(&'a MessageImpl);
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

pub struct ClockLost<'a>(&'a MessageImpl);
impl<'a> ClockLost<'a> {
    pub fn get_clock(&self) -> Option<::Clock> {
        let mut clock = ptr::null_mut();

        unsafe {
            ffi::gst_message_parse_clock_lost(self.0.as_mut_ptr(), &mut clock);

            from_glib_none(clock)
        }
    }
}

pub struct NewClock<'a>(&'a MessageImpl);
impl<'a> NewClock<'a> {
    pub fn get_clock(&self) -> Option<::Clock> {
        let mut clock = ptr::null_mut();

        unsafe {
            ffi::gst_message_parse_new_clock(self.0.as_mut_ptr(), &mut clock);

            from_glib_none(clock)
        }
    }
}

pub struct StructureChange<'a>(&'a MessageImpl);
impl<'a> StructureChange<'a> {
    pub fn get(&self) -> (::StructureChangeType, Option<Element>, bool) {
        unsafe {
            let mut type_ = mem::uninitialized();
            let mut owner = ptr::null_mut();
            let mut busy = mem::uninitialized();

            ffi::gst_message_parse_structure_change(self.0.as_mut_ptr(), &mut type_, &mut owner, &mut busy);

            (from_glib(type_), from_glib_none(owner), from_glib(busy))
        }
    }
}

pub struct StreamStatus<'a>(&'a MessageImpl);
impl<'a> StreamStatus<'a> {
    pub fn get(&self) -> (::StreamStatusType, Option<Element>) {
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

pub struct SegmentStart<'a>(&'a MessageImpl);
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

pub struct SegmentDone<'a>(&'a MessageImpl);
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

pub struct AsyncDone<'a>(&'a MessageImpl);
impl<'a> AsyncDone<'a> {
    pub fn get_running_time(&self) -> u64 {
        unsafe {
            let mut running_time = mem::uninitialized();

            ffi::gst_message_parse_async_done(self.0.as_mut_ptr(), &mut running_time);

            running_time
        }
    }
}

pub struct RequestState<'a>(&'a MessageImpl);
impl<'a> RequestState<'a> {
    pub fn get_requested_state(&self) -> ::State {
        unsafe {
            let mut state = mem::uninitialized();

            ffi::gst_message_parse_request_state(self.0.as_mut_ptr(), &mut state);

            from_glib(state)
        }
    }
}

pub struct StepStart<'a>(&'a MessageImpl);
impl<'a> StepStart<'a> {
    pub fn get(&self) -> (bool, ::Format, u64, f64, bool, bool) {
        unsafe {
            let mut active = mem::uninitialized();
            let mut format = mem::uninitialized();
            let mut amount = mem::uninitialized();
            let mut rate = mem::uninitialized();
            let mut flush = mem::uninitialized();
            let mut intermediate = mem::uninitialized();

            ffi::gst_message_parse_step_start(self.0.as_mut_ptr(), &mut active, &mut format, &mut amount, &mut rate, &mut flush, &mut intermediate);

            (from_glib(active), from_glib(format), amount, rate, from_glib(flush), from_glib(intermediate))
        }
    }
}

pub struct Qos<'a>(&'a MessageImpl);
impl<'a> Qos<'a> {
    pub fn get(&self) -> (bool, u64, u64, u64, u64) {
        unsafe {
            let mut live = mem::uninitialized();
            let mut running_time = mem::uninitialized();
            let mut stream_time = mem::uninitialized();
            let mut timestamp = mem::uninitialized();
            let mut duration = mem::uninitialized();

            ffi::gst_message_parse_qos(self.0.as_mut_ptr(), &mut live, &mut running_time, &mut stream_time, &mut timestamp, &mut duration);

            (from_glib(live), running_time, stream_time, timestamp, duration)
        }
    }

    pub fn get_values(&self) -> (i64, f64, i32) {
        unsafe {
            let mut jitter = mem::uninitialized();
            let mut proportion = mem::uninitialized();
            let mut quality = mem::uninitialized();

            ffi::gst_message_parse_qos_values(self.0.as_mut_ptr(), &mut jitter, &mut proportion, &mut quality);

            (jitter, proportion, quality)
        }
    }

    pub fn get_stats(&self) -> (::Format, u64, u64) {
        unsafe {
            let mut format = mem::uninitialized();
            let mut processed = mem::uninitialized();
            let mut dropped = mem::uninitialized();

            ffi::gst_message_parse_qos_stats(self.0.as_mut_ptr(), &mut format, &mut processed, &mut dropped);

            (from_glib(format), processed, dropped)
        }
    }
}

pub struct Progress<'a>(&'a MessageImpl);
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

pub struct Toc<'a>(&'a MessageImpl);
impl<'a> Toc<'a> {
    // TODO get_toc()
}

pub struct ResetTime<'a>(&'a MessageImpl);
impl<'a> ResetTime<'a> {
    pub fn get_running_time(&self) -> u64 {
        unsafe {
            let mut running_time = mem::uninitialized();

            ffi::gst_message_parse_reset_time(self.0.as_mut_ptr(), &mut running_time);

            running_time
        }
    }
}

pub struct StreamStart<'a>(&'a MessageImpl);
impl<'a> StreamStart<'a> {
    pub fn get_group_id(&self) -> Option<u32> {
        unsafe {
            let mut group_id = mem::uninitialized();

            if from_glib(ffi::gst_message_parse_group_id(self.0.as_mut_ptr(), &mut group_id)) {
                Some(group_id)
            } else {
                None
            }
        }
    }
}

pub struct NeedContext<'a>(&'a MessageImpl);
impl<'a> NeedContext<'a> {
    pub fn get_context_type(&self) -> Option<&str> {
        unsafe {
            let mut context_type = ptr::null();

            if from_glib(ffi::gst_message_parse_context_type(self.0.as_mut_ptr(), &mut context_type)) && !context_type.is_null() {
                Some(CStr::from_ptr(context_type).to_str().unwrap())
            } else {
                None
            }
        }
    }
}

pub struct HaveContext<'a>(&'a MessageImpl);
impl<'a> HaveContext<'a> {
    // TODO: get_context()
}

pub struct DeviceAdded<'a>(&'a MessageImpl);
impl<'a> DeviceAdded<'a> {
    pub fn get_device(&self) -> ::Device {
        unsafe {
            let mut device = ptr::null_mut();

            ffi::gst_message_parse_device_added(self.0.as_mut_ptr(), &mut device);

            from_glib_none(device)
        }
    }
}

pub struct DeviceRemoved<'a>(&'a MessageImpl);
impl<'a> DeviceRemoved<'a> {
    pub fn get_device(&self) -> ::Device {
        unsafe {
            let mut device = ptr::null_mut();

            ffi::gst_message_parse_device_removed(self.0.as_mut_ptr(), &mut device);

            from_glib_none(device)
        }
    }
}

pub struct PropertyNotify<'a>(&'a MessageImpl);
impl<'a> PropertyNotify<'a> {
    #[cfg(feature = "v1_10")]
    pub fn get(&self) -> (Object, &str, ::Value) {
        unsafe {
            let mut object = ptr::null_mut();
            let mut property_name = ptr::null();
            let mut value = ptr::null();

            ffi::gst_message_parse_property_notify(self.0.as_mut_ptr(), &mut object, &mut property_name, &mut value);

            (from_glib_none(object), CStr::from_ptr(property_name).to_str().unwrap(), from_glib_none(value))
        }
    }
}

pub struct StreamCollection<'a>(&'a MessageImpl);
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
pub struct StreamsSelected<'a>(&'a MessageImpl);
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

            (0..n).map(|i| from_glib_full(ffi::gst_message_streams_selected_get_stream(self.0.as_mut_ptr(), i))).collect()
        }
    }
}

pub struct Redirect<'a>(&'a MessageImpl);
impl<'a> StreamsSelected<'a> {
    // TODO: tags, structure
    #[cfg(feature = "v1_10")]
    pub fn get_entries(&self) -> Vec<&str> {
        unsafe {
            let n = ffi::gst_message_get_num_redirect_entries(self.0.as_mut_ptr());

            (0..n).map(|i| {
                let mut location = ptr::null();

                ffi::gst_message_parse_redirect_entry(self.0.as_mut_ptr(), i, &mut location, ptr::null_mut(), ptr::null_mut());

                CStr::from_ptr(location).to_str().unwrap()
            }).collect()
        }
    }
}

macro_rules! message_builder_generic_impl {
    ($new_fn:expr) => {
        pub fn src(self, src: Option<&'a Object>) -> Self {
            Self {
                src: src,
                .. self
            }
        }

        pub fn seqnum(self, seqnum: u32) -> Self {
            Self {
                seqnum: Some(seqnum),
                .. self
            }
        }

        pub fn build(mut self) -> Message {
            unsafe {
                let src = self.src.to_glib_none().0;
                let msg = $new_fn(&mut self, src);
                if let Some(seqnum) = self.seqnum {
                    ffi::gst_message_set_seqnum(msg, seqnum);
                }

                from_glib_full(msg)
            }
        }
    }
}

pub struct EosBuilder<'a> {
    src: Option<&'a Object>,
    seqnum: Option<u32>,
}
impl<'a> EosBuilder<'a> {
    pub fn new() -> Self {
        Self {
            src: None,
            seqnum: None,
        }
    }

    message_builder_generic_impl!(|_, src| ffi::gst_message_new_eos(src));
}

pub struct ErrorBuilder<'a> {
    src: Option<&'a Object>,
    seqnum: Option<u32>,
    error: &'a glib::Error,
    debug: Option<&'a str>,
}
impl<'a> ErrorBuilder<'a> {
    pub fn new(error: &'a glib::Error) -> Self {
        Self {
            src: None,
            seqnum: None,
            error: error,
            debug: None,
        }
    }

    pub fn debug(self, debug: &'a str) -> Self {
        Self {
            debug: Some(debug),
            .. self
        }
    }

    // TODO details

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_error(src, mut_override(s.error.to_glib_none().0), s.debug.to_glib_none().0));
}

pub struct WarningBuilder<'a> {
    src: Option<&'a Object>,
    seqnum: Option<u32>,
    error: &'a glib::Error,
    debug: Option<&'a str>,
}
impl<'a> WarningBuilder<'a> {
    pub fn new(error: &'a glib::Error) -> Self {
        Self {
            src: None,
            seqnum: None,
            error: error,
            debug: None,
        }
    }

    pub fn debug(self, debug: &'a str) -> Self {
        Self {
            debug: Some(debug),
            .. self
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_warning(src, mut_override(s.error.to_glib_none().0), s.debug.to_glib_none().0));
}

pub struct InfoBuilder<'a> {
    src: Option<&'a Object>,
    seqnum: Option<u32>,
    error: &'a glib::Error,
    debug: Option<&'a str>,
}
impl<'a> InfoBuilder<'a> {
    pub fn new(error: &'a glib::Error) -> Self {
        Self {
            src: None,
            seqnum: None,
            error: error,
            debug: None,
        }
    }

    pub fn debug(self, debug: &'a str) -> Self {
        Self {
            debug: Some(debug),
            .. self
        }
    }

    // TODO details

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_warning(src, mut_override(s.error.to_glib_none().0), s.debug.to_glib_none().0));
}

pub struct TagBuilder<'a> {
    src: Option<&'a Object>,
    seqnum: Option<u32>,
    tags: (),
    // TODO tags
}
impl<'a> TagBuilder<'a> {
    pub fn new(tags: /*Tags*/ ()) -> Self {
        Self {
            src: None,
            seqnum: None,
            tags: tags,
        }
    }

    message_builder_generic_impl!(|_, src| ffi::gst_message_new_tag(src, /*s.tags.to_glib_full().0*/ ptr::null_mut()));
}

pub struct BufferingBuilder<'a> {
    src: Option<&'a Object>,
    seqnum: Option<u32>,
    percent: i32,
    stats: Option<(::BufferingMode, i32, i32, i64)>,
}
impl<'a> BufferingBuilder<'a> {
    pub fn new(percent: i32) -> Self {
        Self {
            src: None,
            seqnum: None,
            percent: percent,
            stats: None,
        }
    }

    pub fn stats(self, mode: ::BufferingMode, avg_in: i32, avg_out: i32, buffering_left: i64) -> Self {
        Self {
            stats: Some((mode, avg_in, avg_out, buffering_left)),
            .. self
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        let msg = ffi::gst_message_new_buffering(src, s.percent);

        if let Some((mode, avg_in, avg_out, buffering_left)) = s.stats {
            ffi::gst_message_set_buffering_stats(msg, mode.to_glib(), avg_in, avg_out, buffering_left);
        }

        msg
    });
}

pub struct StateChangedBuilder<'a> {
    src: Option<&'a Object>,
    seqnum: Option<u32>,
    old: ::State,
    new: ::State,
    pending: ::State,
}
impl<'a> StateChangedBuilder<'a> {
    pub fn new(old: ::State, new: ::State, pending: ::State) -> Self {
        Self {
            src: None,
            seqnum: None,
            old: old,
            new: new,
            pending: pending,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_state_changed(src, s.old.to_glib(), s.new.to_glib(), s.pending.to_glib()));
}

pub struct StateDirtyBuilder<'a> {
    src: Option<&'a Object>,
    seqnum: Option<u32>,
}
impl<'a> StateDirtyBuilder<'a> {
    pub fn new() -> Self {
        Self {
            src: None,
            seqnum: None,
        }
    }

    message_builder_generic_impl!(|_, src| ffi::gst_message_new_state_dirty(src));
}

pub struct StepDoneBuilder<'a> {
    src: Option<&'a Object>,
    seqnum: Option<u32>,
    format: ::Format,
    amount: u64,
    rate: f64,
    flush: bool,
    intermediate: bool,
    duration: u64,
    eos: bool,
}
impl<'a> StepDoneBuilder<'a> {
    pub fn new(format: ::Format, amount: u64, rate: f64, flush: bool, intermediate: bool, duration: u64, eos: bool) -> Self {
        Self {
            src: None,
            seqnum: None,
            format: format,
            amount: amount,
            rate: rate,
            flush: flush,
            intermediate: intermediate,
            duration: duration,
            eos: eos,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_step_done(src, s.format.to_glib(), s.amount, s.rate, s.flush.to_glib(), s.intermediate.to_glib(), s.duration, s.eos.to_glib()));
}

pub struct ClockProvideBuilder<'a> {
    src: Option<&'a Object>,
    seqnum: Option<u32>,
    clock: &'a ::Clock,
    ready: bool,
}
impl<'a> ClockProvideBuilder<'a> {
    pub fn new(clock: &'a ::Clock, ready: bool) -> Self {
        Self {
            src: None,
            seqnum: None,
            clock: clock,
            ready: ready,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_clock_provide(src, s.clock.to_glib_none().0, s.ready.to_glib()));
}

pub struct ClockLostBuilder<'a> {
    src: Option<&'a Object>,
    seqnum: Option<u32>,
    clock: &'a ::Clock,
}
impl<'a> ClockLostBuilder<'a> {
    pub fn new(clock: &'a ::Clock) -> Self {
        Self {
            src: None,
            seqnum: None,
            clock: clock,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_clock_lost(src, s.clock.to_glib_none().0));
}

pub struct NewClockBuilder<'a> {
    src: Option<&'a Object>,
    seqnum: Option<u32>,
    clock: &'a ::Clock,
}
impl<'a> NewClockBuilder<'a> {
    pub fn new(clock: &'a ::Clock) -> Self {
        Self {
            src: None,
            seqnum: None,
            clock: clock,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_new_clock(src, s.clock.to_glib_none().0));
}

pub struct StructureChangeBuilder<'a> {
    src: Option<&'a Object>,
    seqnum: Option<u32>,
    type_: ::StructureChangeType,
    owner: &'a ::Element,
    busy: bool,
}
impl<'a> StructureChangeBuilder<'a> {
    pub fn new(type_: ::StructureChangeType, owner: &'a ::Element, busy: bool) -> Self {
        Self {
            src: None,
            seqnum: None,
            type_: type_,
            owner: owner,
            busy: busy,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_structure_change(src, s.type_.to_glib(), s.owner.to_glib_none().0, s.busy.to_glib()));
}

pub struct StreamStatusBuilder<'a> {
    src: Option<&'a Object>,
    seqnum: Option<u32>,
    type_: ::StreamStatusType,
    owner: &'a ::Element,
    status_object: Option<&'a glib::Value>,
}
impl<'a> StreamStatusBuilder<'a> {
    pub fn new(type_: ::StreamStatusType, owner: &'a ::Element) -> Self {
        Self {
            src: None,
            seqnum: None,
            type_: type_,
            owner: owner,
            status_object: None,
        }
    }

    pub fn status_object(self, status_object: &'a glib::Value) -> Self {
        Self {
            status_object: Some(status_object),
            .. self
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        let msg = ffi::gst_message_new_stream_status(src, s.type_.to_glib(), s.owner.to_glib_none().0);
        if let Some(status_object) = s.status_object {
            ffi::gst_message_set_stream_status_object(msg, status_object.to_glib_none().0);
        }
        msg
    });
}

pub struct ApplicationBuilder<'a> {
    src: Option<&'a Object>,
    seqnum: Option<u32>,
    structure: Option<::Structure>,
}
impl<'a> ApplicationBuilder<'a> {
    pub fn new(structure: ::Structure) -> Self {
        Self {
            src: None,
            seqnum: None,
            structure: Some(structure),
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_application(src, s.structure.take().unwrap().into_ptr()));
}

pub struct ElementBuilder<'a> {
    src: Option<&'a Object>,
    seqnum: Option<u32>,
    structure: Option<::Structure>,
}
impl<'a> ElementBuilder<'a> {
    pub fn new(structure: ::Structure) -> Self {
        Self {
            src: None,
            seqnum: None,
            structure: Some(structure),
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_element(src, s.structure.take().unwrap().into_ptr()));
}

pub struct SegmentStartBuilder<'a> {
    src: Option<&'a Object>,
    seqnum: Option<u32>,
    format: ::Format,
    position: i64,
}
impl<'a> SegmentStartBuilder<'a> {
    pub fn new(format: ::Format, position: i64) -> Self {
        Self {
            src: None,
            seqnum: None,
            format: format,
            position: position,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_segment_start(src, s.format.to_glib(), s.position));
}

pub struct SegmentDoneBuilder<'a> {
    src: Option<&'a Object>,
    seqnum: Option<u32>,
    format: ::Format,
    position: i64,
}
impl<'a> SegmentDoneBuilder<'a> {
    pub fn new(format: ::Format, position: i64) -> Self {
        Self {
            src: None,
            seqnum: None,
            format: format,
            position: position,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_segment_done(src, s.format.to_glib(), s.position));
}

pub struct DurationChangedBuilder<'a> {
    src: Option<&'a Object>,
    seqnum: Option<u32>,
}
impl<'a> DurationChangedBuilder<'a> {
    pub fn new() -> Self {
        Self {
            src: None,
            seqnum: None,
        }
    }

    message_builder_generic_impl!(|_, src| ffi::gst_message_new_duration_changed(src));
}

pub struct LatencyBuilder<'a> {
    src: Option<&'a Object>,
    seqnum: Option<u32>,
}
impl<'a> LatencyBuilder<'a> {
    pub fn new() -> Self {
        Self {
            src: None,
            seqnum: None,
        }
    }

    message_builder_generic_impl!(|_, src| ffi::gst_message_new_latency(src));
}

pub struct AsyncStartBuilder<'a> {
    src: Option<&'a Object>,
    seqnum: Option<u32>,
}
impl<'a> AsyncStartBuilder<'a> {
    pub fn new() -> Self {
        Self {
            src: None,
            seqnum: None,
        }
    }

    message_builder_generic_impl!(|_, src| ffi::gst_message_new_async_start(src));
}

pub struct AsyncDoneBuilder<'a> {
    src: Option<&'a Object>,
    seqnum: Option<u32>,
    running_time: u64,
}
impl<'a> AsyncDoneBuilder<'a> {
    pub fn new(running_time: u64) -> Self {
        Self {
            src: None,
            seqnum: None,
            running_time: running_time,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_async_done(src, s.running_time));
}

pub struct RequestStateBuilder<'a> {
    src: Option<&'a Object>,
    seqnum: Option<u32>,
    state: ::State,
}
impl<'a> RequestStateBuilder<'a> {
    pub fn new(state: ::State) -> Self {
        Self {
            src: None,
            seqnum: None,
            state: state,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_request_state(src, s.state.to_glib()));
}

pub struct StepStartBuilder<'a> {
    src: Option<&'a Object>,
    seqnum: Option<u32>,
    active: bool,
    format: ::Format,
    amount: u64,
    rate: f64,
    flush: bool,
    intermediate: bool,
}
impl<'a> StepStartBuilder<'a> {
    pub fn new(active: bool, format: ::Format, amount: u64, rate: f64, flush: bool, intermediate: bool) -> Self {
        Self {
            src: None,
            seqnum: None,
            active: active,
            format: format,
            amount: amount,
            rate: rate,
            flush: flush,
            intermediate: intermediate,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_step_start(src, s.active.to_glib(), s.format.to_glib(), s.amount, s.rate, s.flush.to_glib(), s.intermediate.to_glib()));
}

pub struct QosBuilder<'a> {
    src: Option<&'a Object>,
    seqnum: Option<u32>,
    live: bool,
    running_time: u64,
    stream_time: u64,
    timestamp: u64,
    duration: u64,
    values: Option<(i64, f64, i32)>,
    stats: Option<(::Format, u64, u64)>,
}
impl<'a> QosBuilder<'a> {
    pub fn new(live: bool, running_time: u64, stream_time: u64, timestamp: u64, duration: u64) -> Self {
        Self {
            src: None,
            seqnum: None,
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
            .. self
        }
    }

    pub fn stats(self, format: ::Format, processed: u64, dropped: u64) -> Self {
        Self {
            stats: Some((format, processed, dropped)),
            .. self
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        let msg = ffi::gst_message_new_qos(src, s.live.to_glib(), s.running_time, s.stream_time, s.timestamp, s.duration);
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
    src: Option<&'a Object>,
    seqnum: Option<u32>,
    type_: ::ProgressType,
    code: Option<&'a str>,
    text: Option<&'a str>,
}
impl<'a> ProgressBuilder<'a> {
    pub fn new(type_: ::ProgressType) -> Self {
        Self {
            src: None,
            seqnum: None,
            type_: type_,
            code: None,
            text: None,
        }
    }

    pub fn code(self, code: &'a str) -> Self {
        Self {
            code: Some(code),
            .. self
        }
    }

    pub fn text(self, text: &'a str) -> Self {
        Self {
            text: Some(text),
            .. self
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_progress(src, s.type_.to_glib(), s.code.to_glib_none().0, s.text.to_glib_none().0));
}

// TODO Toc
pub struct TocBuilder<'a> {
    src: Option<&'a Object>,
    seqnum: Option<u32>,
    toc: (),
    updated: bool,
}
impl<'a> TocBuilder<'a> {
    pub fn new(toc: () /* &'a Toc */, updated: bool) -> Self {
        Self {
            src: None,
            seqnum: None,
            toc: toc,
            updated: updated,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_toc(src, ptr::null_mut() /*s.structure.to_glib_full()*/, s.updated.to_glib()));
}

pub struct ResetTimeBuilder<'a> {
    src: Option<&'a Object>,
    seqnum: Option<u32>,
    running_time: u64,
}
impl<'a> ResetTimeBuilder<'a> {
    pub fn new(running_time: u64) -> Self {
        Self {
            src: None,
            seqnum: None,
            running_time: running_time,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_reset_time(src, s.running_time));
}

pub struct StreamStartBuilder<'a> {
    src: Option<&'a Object>,
    seqnum: Option<u32>,
    group_id: Option<u32>,
}
impl<'a> StreamStartBuilder<'a> {
    pub fn new() -> Self {
        Self {
            src: None,
            seqnum: None,
            group_id: None,
        }
    }

    pub fn group_id(self, group_id: u32) -> Self {
        Self {
            group_id: Some(group_id),
            .. self
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
    src: Option<&'a Object>,
    seqnum: Option<u32>,
    context_type: &'a str,
}
impl<'a> NeedContextBuilder<'a> {
    pub fn new(context_type: &'a str) -> Self {
        Self {
            src: None,
            seqnum: None,
            context_type: context_type,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_need_context(src, s.context_type.to_glib_none().0));
}

// TODO Context
pub struct HaveContextBuilder<'a> {
    src: Option<&'a Object>,
    seqnum: Option<u32>,
    context: (),
}
impl<'a> HaveContextBuilder<'a> {
    pub fn new(context: () /* ::Context */) -> Self {
        Self {
            src: None,
            seqnum: None,
            context: (),
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_have_context(src, ptr::null_mut() /*s.context.to_glib_full().0*/));
}

pub struct DeviceAddedBuilder<'a> {
    src: Option<&'a Object>,
    seqnum: Option<u32>,
    device: &'a ::Device,
}
impl<'a> DeviceAddedBuilder<'a> {
    pub fn new(device: &'a ::Device) -> Self {
        Self {
            src: None,
            seqnum: None,
            device: device,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_device_added(src, s.device.to_glib_none().0));
}

pub struct DeviceRemovedBuilder<'a> {
    src: Option<&'a Object>,
    seqnum: Option<u32>,
    device: &'a ::Device,
}
impl<'a> DeviceRemovedBuilder<'a> {
    pub fn new(device: &'a ::Device) -> Self {
        Self {
            src: None,
            seqnum: None,
            device: device,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_device_removed(src, s.device.to_glib_none().0));
}

pub struct PropertyNotifyBuilder<'a> {
    src: Option<&'a Object>,
    seqnum: Option<u32>,
    property_name: &'a str,
    value: &'a glib::Value,
}
#[cfg(feature = "v1_10")]
impl<'a> PropertyNotifyBuilder<'a> {
    pub fn new(property_name: &'a str, value: &'a glib::Value) -> Self {
        Self {
            src: None,
            seqnum: None,
            property_name: property_name,
            value: value,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_property_notify(src, s.property_name.to_glib_none().0, mut_override(s.value.to_glib_none().0)));
}

pub struct StreamCollectionBuilder<'a> {
    src: Option<&'a Object>,
    seqnum: Option<u32>,
    #[cfg(feature = "v1_10")]
    collection: &'a ::StreamCollection,
}
#[cfg(feature = "v1_10")]
impl<'a> StreamCollectionBuilder<'a> {
    pub fn new(collection: &'a ::StreamCollection) -> Self {
        Self {
            src: None,
            seqnum: None,
            collection: collection,
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| ffi::gst_message_new_stream_collection(src, s.collection.to_glib_none().0));
}

pub struct StreamsSelectedBuilder<'a> {
    src: Option<&'a Object>,
    seqnum: Option<u32>,
    #[cfg(feature = "v1_10")]
    collection: &'a ::StreamCollection,
    #[cfg(feature = "v1_10")]
    streams: Option<&'a[&'a ::Stream]>,
}
#[cfg(feature = "v1_10")]
impl<'a> StreamsSelectedBuilder<'a> {
    pub fn new(collection: &'a ::StreamCollection) -> Self {
        Self {
            src: None,
            seqnum: None,
            collection: collection,
            streams: None,
        }
    }

    pub fn streams(self, streams: &'a[&'a ::Stream]) -> Self {
        Self {
            streams: Some(streams),
            .. self
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

// TODO TagList, Structure
pub struct RedirectBuilder<'a> {
    src: Option<&'a Object>,
    seqnum: Option<u32>,
    location: &'a str,
    tag_list: Option<()>,
    entry_struct: Option<()>,
    entries: Option<&'a[(&'a str, (&'a ()), (&'a ()))]>,
}
#[cfg(feature = "v1_10")]
impl<'a> RedirectBuilder<'a> {
    pub fn new(location: &'a str, tag_list: Option<()>, entry_struct: Option<()>) -> Self {
        Self {
            src: None,
            seqnum: None,
            location: location,
            tag_list: tag_list,
            entry_struct: entry_struct,
            entries: None,
        }
    }

    pub fn entries(self, entries: &'a[(&'a str, (&'a ()), (&'a ()))]) -> Self {
        Self {
            entries: Some(entries),
            .. self
        }
    }

    message_builder_generic_impl!(|s: &mut Self, src| {
        let msg = ffi::gst_message_new_redirect(src, s.location.to_glib_none().0, ptr::null_mut(), ptr::null_mut());
        if let Some(entries) = s.entries {
            for &(location, tag_list, entry_struct) in entries {
                ffi::gst_message_add_redirect_entry(msg, location.to_glib_none().0, ptr::null_mut(), ptr::null_mut());
            }
        }
        msg
    });
}

