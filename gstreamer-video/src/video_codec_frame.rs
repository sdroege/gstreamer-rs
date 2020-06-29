// Copyright (C) 2017 Thibault Saunier <tsaunier@gnome.org>
// Copyright (C) 2019 Guillaume Desmottes <guillaume.desmottes@collabora.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::translate::*;
use gst;
use gst_video_sys;
use std::fmt;
use std::mem;
use utils::HasStreamLock;
use VideoCodecFrameFlags;

pub struct VideoCodecFrame<'a> {
    frame: *mut gst_video_sys::GstVideoCodecFrame,
    /* GstVideoCodecFrame API isn't safe so protect the frame using the
     * element (decoder or encoder) stream lock */
    element: &'a dyn HasStreamLock,
}

#[doc(hidden)]
impl<'a> ::glib::translate::ToGlibPtr<'a, *mut gst_video_sys::GstVideoCodecFrame>
    for VideoCodecFrame<'a>
{
    type Storage = &'a Self;

    fn to_glib_none(
        &'a self,
    ) -> ::glib::translate::Stash<'a, *mut gst_video_sys::GstVideoCodecFrame, Self> {
        Stash(self.frame, self)
    }

    fn to_glib_full(&self) -> *mut gst_video_sys::GstVideoCodecFrame {
        unimplemented!()
    }
}

impl<'a> fmt::Debug for VideoCodecFrame<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut b = f.debug_struct("VideoCodecFrame");

        b.field("flags", &self.get_flags())
            .field("system_frame_number", &self.get_system_frame_number())
            .field("decode_frame_number", &self.get_decode_frame_number())
            .field(
                "presentation_frame_number",
                &self.get_presentation_frame_number(),
            )
            .field("dts", &self.get_dts())
            .field("pts", &self.get_pts())
            .field("duration", &self.get_duration())
            .field("distance_from_sync", &self.get_distance_from_sync())
            .field("input_buffer", &self.get_input_buffer())
            .field("output_buffer", &self.get_output_buffer())
            .field("deadline", &self.get_deadline());

        b.finish()
    }
}

impl<'a> VideoCodecFrame<'a> {
    // Take ownership of @frame
    pub(crate) unsafe fn new<T: HasStreamLock>(
        frame: *mut gst_video_sys::GstVideoCodecFrame,
        element: &'a T,
    ) -> Self {
        skip_assert_initialized!();
        let stream_lock = element.get_stream_lock();
        glib_sys::g_rec_mutex_lock(stream_lock);
        Self { frame, element }
    }

    pub fn get_flags(&self) -> VideoCodecFrameFlags {
        let flags = unsafe { (*self.to_glib_none().0).flags };
        VideoCodecFrameFlags::from_bits_truncate(flags)
    }

    pub fn set_flags(&mut self, flags: VideoCodecFrameFlags) {
        unsafe { (*self.to_glib_none().0).flags |= flags.bits() }
    }

    pub fn unset_flags(&mut self, flags: VideoCodecFrameFlags) {
        unsafe { (*self.to_glib_none().0).flags &= !flags.bits() }
    }

    pub fn get_system_frame_number(&self) -> u32 {
        unsafe { (*self.to_glib_none().0).system_frame_number }
    }

    pub fn get_decode_frame_number(&self) -> u32 {
        unsafe { (*self.to_glib_none().0).decode_frame_number }
    }

    pub fn get_presentation_frame_number(&self) -> u32 {
        unsafe { (*self.to_glib_none().0).presentation_frame_number }
    }

    pub fn get_dts(&self) -> gst::ClockTime {
        unsafe { from_glib((*self.to_glib_none().0).dts) }
    }

    pub fn set_dts(&mut self, dts: gst::ClockTime) {
        unsafe {
            (*self.to_glib_none().0).dts = dts.to_glib();
        }
    }

    pub fn get_pts(&self) -> gst::ClockTime {
        unsafe { from_glib((*self.to_glib_none().0).pts) }
    }

    pub fn set_pts(&mut self, pts: gst::ClockTime) {
        unsafe {
            (*self.to_glib_none().0).pts = pts.to_glib();
        }
    }

    pub fn get_duration(&self) -> gst::ClockTime {
        unsafe { from_glib((*self.to_glib_none().0).duration) }
    }

    pub fn set_duration(&mut self, duration: gst::ClockTime) {
        unsafe {
            (*self.to_glib_none().0).duration = duration.to_glib();
        }
    }

    pub fn get_distance_from_sync(&self) -> i32 {
        unsafe { (*self.to_glib_none().0).distance_from_sync }
    }

    pub fn get_input_buffer(&self) -> Option<&gst::BufferRef> {
        unsafe {
            let ptr = (*self.to_glib_none().0).input_buffer;
            if ptr.is_null() {
                None
            } else {
                Some(gst::BufferRef::from_ptr(ptr))
            }
        }
    }

    pub fn get_output_buffer(&self) -> Option<&gst::BufferRef> {
        unsafe {
            let ptr = (*self.to_glib_none().0).output_buffer;
            if ptr.is_null() {
                None
            } else {
                Some(gst::BufferRef::from_ptr(ptr))
            }
        }
    }

    pub fn get_output_buffer_mut(&mut self) -> Option<&mut gst::BufferRef> {
        unsafe {
            let ptr = (*self.to_glib_none().0).output_buffer;
            if ptr.is_null() {
                None
            } else {
                let writable: bool = from_glib(gst_sys::gst_mini_object_is_writable(
                    ptr as *const gst_sys::GstMiniObject,
                ));
                assert!(writable);

                Some(gst::BufferRef::from_mut_ptr(ptr))
            }
        }
    }

    pub fn set_output_buffer(&mut self, output_buffer: gst::Buffer) {
        unsafe {
            let prev = (*self.to_glib_none().0).output_buffer;

            if !prev.is_null() {
                gst_sys::gst_mini_object_unref(prev as *mut gst_sys::GstMiniObject);
            }

            let ptr = output_buffer.into_ptr();
            let writable: bool = from_glib(gst_sys::gst_mini_object_is_writable(
                ptr as *const gst_sys::GstMiniObject,
            ));
            assert!(writable);

            (*self.to_glib_none().0).output_buffer = ptr;
        }
    }

    pub fn get_deadline(&self) -> gst::ClockTime {
        unsafe { from_glib((*self.to_glib_none().0).deadline) }
    }

    #[doc(hidden)]
    pub unsafe fn into_ptr(self) -> *mut gst_video_sys::GstVideoCodecFrame {
        let stream_lock = self.element.get_stream_lock();
        glib_sys::g_rec_mutex_unlock(stream_lock);

        let s = mem::ManuallyDrop::new(self);
        s.to_glib_none().0
    }
}

impl<'a> Drop for VideoCodecFrame<'a> {
    fn drop(&mut self) {
        unsafe {
            let stream_lock = self.element.get_stream_lock();
            glib_sys::g_rec_mutex_unlock(stream_lock);

            gst_video_sys::gst_video_codec_frame_unref(self.frame);
        }
    }
}
