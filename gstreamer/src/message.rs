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

use std::ptr;

use glib;
use glib::translate::{from_glib, from_glib_none, from_glib_full, ToGlibPtr};

#[repr(C)]
pub struct MessageImpl(ffi::GstMessage);

pub type Message = GstRc<MessageImpl>;

unsafe impl MiniObject for MessageImpl {
    type GstType = ffi::GstMessage;
}

impl MessageImpl {
    pub fn new_eos(src: &Object) -> GstRc<Self> {
        unsafe {
            from_glib_full(ffi::gst_message_new_eos(src.to_glib_none().0))
        }
    }

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

    pub fn set_seqnum(&mut self, seqnum: u32) {
        unsafe {
            ffi::gst_message_set_seqnum(self.as_mut_ptr(), seqnum)
        }
    }

    // TODO get_structure()

    pub fn view(&self) -> MessageView {
        let type_ = unsafe { (*self.as_ptr()).type_ };

        if type_ == ffi::GST_MESSAGE_EOS {
            MessageView::Eos(Eos(self))
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
            MessageView::StateDirty(StateDirty(self))
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
            MessageView::Application(Application(self))
        } else if type_ == ffi::GST_MESSAGE_ELEMENT {
            MessageView::Element(Element(self))
        } else if type_ == ffi::GST_MESSAGE_SEGMENT_START {
            MessageView::SegmentStart(SegmentStart(self))
        } else if type_ == ffi::GST_MESSAGE_SEGMENT_DONE {
            MessageView::SegmentDone(SegmentDone(self))
        } else if type_ == ffi::GST_MESSAGE_DURATION_CHANGED {
            MessageView::DurationChanged(DurationChanged(self))
        } else if type_ == ffi::GST_MESSAGE_LATENCY {
            MessageView::Latency(Latency(self))
        } else if type_ == ffi::GST_MESSAGE_ASYNC_START {
            MessageView::AsyncStart(AsyncStart(self))
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
            unimplemented!()
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
    __NonExhaustive,
}

pub struct Eos<'a>(&'a MessageImpl);

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
        let mut p = 0;
        unsafe {
            ffi::gst_message_parse_buffering(self.0.as_mut_ptr(), &mut p);
        }

        p
    }

    pub fn get_buffering_stats(&self) -> (::BufferingMode, i32, i32, i64) {
        let mut mode = ffi::GstBufferingMode::Stream;
        let mut avg_in = 0;
        let mut avg_out = 0;
        let mut buffering_left = 0;

        unsafe {
            ffi::gst_message_parse_buffering_stats(self.0.as_mut_ptr(), &mut mode, &mut avg_in, &mut avg_out, &mut buffering_left);
        }

        (from_glib(mode), avg_in, avg_out, buffering_left)
    }
}

pub struct StateChanged<'a>(&'a MessageImpl);
pub struct StateDirty<'a>(&'a MessageImpl);
pub struct StepDone<'a>(&'a MessageImpl);
pub struct ClockProvide<'a>(&'a MessageImpl);
pub struct ClockLost<'a>(&'a MessageImpl);
pub struct NewClock<'a>(&'a MessageImpl);
pub struct StructureChange<'a>(&'a MessageImpl);
pub struct StreamStatus<'a>(&'a MessageImpl);
pub struct Application<'a>(&'a MessageImpl);
pub struct Element<'a>(&'a MessageImpl);
pub struct SegmentStart<'a>(&'a MessageImpl);
pub struct SegmentDone<'a>(&'a MessageImpl);
pub struct DurationChanged<'a>(&'a MessageImpl);
pub struct Latency<'a>(&'a MessageImpl);
pub struct AsyncStart<'a>(&'a MessageImpl);
pub struct AsyncDone<'a>(&'a MessageImpl);
pub struct RequestState<'a>(&'a MessageImpl);
pub struct StepStart<'a>(&'a MessageImpl);
pub struct Qos<'a>(&'a MessageImpl);
pub struct Progress<'a>(&'a MessageImpl);
pub struct Toc<'a>(&'a MessageImpl);
pub struct ResetTime<'a>(&'a MessageImpl);
pub struct StreamStart<'a>(&'a MessageImpl);
pub struct NeedContext<'a>(&'a MessageImpl);
pub struct HaveContext<'a>(&'a MessageImpl);
pub struct DeviceAdded<'a>(&'a MessageImpl);
pub struct DeviceRemoved<'a>(&'a MessageImpl);
pub struct PropertyNotify<'a>(&'a MessageImpl);
pub struct StreamCollection<'a>(&'a MessageImpl);
pub struct StreamsSelected<'a>(&'a MessageImpl);
pub struct Redirect<'a>(&'a MessageImpl);

impl glib::types::StaticType for GstRc<MessageImpl> {
    fn static_type() -> glib::types::Type {
        unsafe {
            from_glib(ffi::gst_message_get_type())
        }
    }
}
