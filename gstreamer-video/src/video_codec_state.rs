// Copyright (C) 2017 Thibault Saunier <tsaunier@gnome.org>
// Copyright (C) 2019 Guillaume Desmottes <guillaume.desmottes@collabora.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::translate::*;
use gst_sys;
use gst_video_sys;
use std::fmt;
use std::marker::PhantomData;
use std::ptr;
use utils::HasStreamLock;

use gst;

use video_info::VideoInfo;

pub trait VideoCodecStateContext<'a> {
    fn get_element(&self) -> Option<&'a dyn HasStreamLock>;
    fn get_element_as_ptr(&self) -> *const gst_sys::GstElement;
}

pub struct InNegotiation<'a> {
    /* GstVideoCodecState API isn't safe so protect the state using the
     * element (decoder or encoder) stream lock */
    element: &'a dyn HasStreamLock,
}
pub struct Readable {}

impl<'a> VideoCodecStateContext<'a> for InNegotiation<'a> {
    fn get_element(&self) -> Option<&'a dyn HasStreamLock> {
        Some(self.element)
    }

    fn get_element_as_ptr(&self) -> *const gst_sys::GstElement {
        self.element.get_element_as_ptr()
    }
}

impl<'a> VideoCodecStateContext<'a> for Readable {
    fn get_element(&self) -> Option<&'a dyn HasStreamLock> {
        None
    }

    fn get_element_as_ptr(&self) -> *const gst_sys::GstElement {
        ptr::null()
    }
}

pub struct VideoCodecState<'a, T: VideoCodecStateContext<'a>> {
    state: *mut gst_video_sys::GstVideoCodecState,
    pub(crate) context: T,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: VideoCodecStateContext<'a>> fmt::Debug for VideoCodecState<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("VideoCodecState")
            .field("info", &self.get_info())
            .field("caps", &self.get_caps())
            .field("codec_data", &self.get_codec_data())
            .field("allocation_caps", &self.get_allocation_caps())
            .finish()
    }
}

impl<'a> VideoCodecState<'a, Readable> {
    // Take ownership of @state
    pub(crate) unsafe fn new(state: *mut gst_video_sys::GstVideoCodecState) -> Self {
        skip_assert_initialized!();
        Self {
            state,
            context: Readable {},
            phantom: PhantomData,
        }
    }
}

impl<'a> VideoCodecState<'a, InNegotiation<'a>> {
    // Take ownership of @state
    pub(crate) unsafe fn new<T: HasStreamLock>(
        state: *mut gst_video_sys::GstVideoCodecState,
        element: &'a T,
    ) -> Self {
        skip_assert_initialized!();
        let stream_lock = element.get_stream_lock();
        glib_sys::g_rec_mutex_lock(stream_lock);
        Self {
            state,
            context: InNegotiation { element },
            phantom: PhantomData,
        }
    }
}

impl<'a, T: VideoCodecStateContext<'a>> VideoCodecState<'a, T> {
    pub fn get_info(&self) -> VideoInfo {
        unsafe {
            let ptr = &((*self.as_mut_ptr()).info) as *const _ as usize as *mut _;
            VideoInfo::from_glib_none(ptr)
        }
    }

    pub fn get_caps(&self) -> Option<&gst::CapsRef> {
        unsafe {
            let ptr = (*self.as_mut_ptr()).caps;

            if ptr.is_null() {
                None
            } else {
                Some(gst::CapsRef::from_ptr(ptr))
            }
        }
    }

    pub fn get_codec_data(&self) -> Option<&gst::BufferRef> {
        unsafe {
            let ptr = (*self.as_mut_ptr()).codec_data;

            if ptr.is_null() {
                None
            } else {
                Some(gst::BufferRef::from_ptr(ptr))
            }
        }
    }

    pub fn get_allocation_caps(&self) -> Option<&gst::CapsRef> {
        unsafe {
            let ptr = (*self.as_mut_ptr()).allocation_caps;

            if ptr.is_null() {
                None
            } else {
                Some(gst::CapsRef::from_ptr(ptr))
            }
        }
    }
    #[doc(hidden)]
    pub fn as_mut_ptr(&self) -> *mut gst_video_sys::GstVideoCodecState {
        self.state
    }
}

impl<'a, T: VideoCodecStateContext<'a>> Drop for VideoCodecState<'a, T> {
    fn drop(&mut self) {
        unsafe {
            if let Some(element) = self.context.get_element() {
                let stream_lock = element.get_stream_lock();
                glib_sys::g_rec_mutex_unlock(stream_lock);
            }
            gst_video_sys::gst_video_codec_state_unref(self.state);
        }
    }
}

impl<'a> VideoCodecState<'a, InNegotiation<'a>> {
    pub fn set_info(&mut self, info: VideoInfo) {
        unsafe {
            ptr::write(&mut (*self.as_mut_ptr()).info, *(info.to_glib_none().0));
        }
    }

    pub fn set_caps(&mut self, caps: &gst::Caps) {
        unsafe {
            let prev = (*self.as_mut_ptr()).caps;

            if !prev.is_null() {
                gst_sys::gst_mini_object_unref(prev as *mut gst_sys::GstMiniObject)
            }

            ptr::write(
                &mut (*self.as_mut_ptr()).caps,
                gst_sys::gst_mini_object_ref(caps.as_mut_ptr() as *mut _) as *mut _,
            );
        }
    }

    pub fn set_codec_data(&mut self, codec_data: &gst::Buffer) {
        unsafe {
            let prev = (*self.as_mut_ptr()).codec_data;

            if !prev.is_null() {
                gst_sys::gst_mini_object_unref(prev as *mut gst_sys::GstMiniObject)
            }

            ptr::write(
                &mut (*self.as_mut_ptr()).codec_data,
                gst_sys::gst_mini_object_ref(codec_data.as_mut_ptr() as *mut _) as *mut _,
            );
        }
    }

    pub fn set_allocation_caps(&mut self, allocation_caps: &gst::Caps) {
        unsafe {
            let prev = (*self.as_mut_ptr()).allocation_caps;

            if !prev.is_null() {
                gst_sys::gst_mini_object_unref(prev as *mut gst_sys::GstMiniObject)
            }

            ptr::write(
                &mut (*self.as_mut_ptr()).allocation_caps,
                gst_sys::gst_mini_object_ref(allocation_caps.as_mut_ptr() as *mut _) as *mut _,
            );
        }
    }
}

impl<'a> Clone for VideoCodecState<'a, Readable> {
    fn clone(&self) -> Self {
        unsafe {
            let state = gst_video_sys::gst_video_codec_state_ref(self.state);
            Self::new(state)
        }
    }
}

unsafe impl<'a> Send for VideoCodecState<'a, Readable> {}
unsafe impl<'a> Sync for VideoCodecState<'a, Readable> {}
