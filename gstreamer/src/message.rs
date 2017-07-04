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
use glib::translate::{from_glib, from_glib_none, from_glib_full, ToGlibPtr};

#[repr(C)]
pub struct MessageImpl(ffi::GstMessage);

pub type Message = GstRc<MessageImpl>;

unsafe impl MiniObject for MessageImpl {
    type GstType = ffi::GstMessage;
}

// TODO builder pattern for message creation

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

            ffi::gst_message_parse_stream_selection(self.0.as_mut_ptr(), &mut collection);

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
            let n = ffi::gst_message_num_redirect_entries(self.0.as_mut_ptr());

            (0..n).map(|i| {
                let mut location = ptr::null();

                ffi::gst_message_parse_redirect_entry(self.0.as_mut_ptr(), &mut location, ptr::null_mut(), ptr::null_mut());

                CStr::from_ptr(location).to_str().unwrap()
            }).collect()
        }
    }
}

pub struct EosBuilder {
    src: Option<Object>,
    seqnum: Option<u32>,
}

impl EosBuilder {
    pub fn new() -> EosBuilder {
        EosBuilder {
            src: None,
            seqnum: None,
        }
    }

    pub fn src(self, src: Option<Object>) -> EosBuilder {
        EosBuilder {
            src: src,
            .. self
        }
    }

    pub fn seqnum(self, seqnum: u32) -> EosBuilder {
        EosBuilder {
            seqnum: Some(seqnum),
            .. self
        }
    }

    pub fn build(self) -> Message {
        unsafe {
            let msg = ffi::gst_message_new_eos(self.src.to_glib_none().0);
            if let Some(seqnum) = self.seqnum {
                ffi::gst_message_set_seqnum(msg, seqnum);
            }

            from_glib_full(msg)
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
