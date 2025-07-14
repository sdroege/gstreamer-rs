// Copyright (C) 2025 Sebastian Dröge <sebastian@centricular.com>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at
// <https://mozilla.org/MPL/2.0/>.
//
// SPDX-License-Identifier: MPL-2.0

use crate::ffi;
use glib::translate::*;
use gst::prelude::*;

use std::{fmt, slice, sync::LazyLock};

#[doc(alias = "GST_CAPS_FEATURE_META_GST_ANALYTICS_BATCH_META")]
pub const CAPS_FEATURE_META_ANALYTICS_BATCH_META: &glib::GStr = unsafe {
    glib::GStr::from_utf8_with_nul_unchecked(ffi::GST_CAPS_FEATURE_META_GST_ANALYTICS_BATCH_META)
};
pub static CAPS_FEATURES_META_ANALYTICS_BATCH_META: LazyLock<gst::CapsFeatures> =
    LazyLock::new(|| gst::CapsFeatures::new([CAPS_FEATURE_META_ANALYTICS_BATCH_META]));

#[repr(transparent)]
#[doc(alias = "GstAnalyticsBatchMeta")]
pub struct AnalyticsBatchMeta(ffi::GstAnalyticsBatchMeta);

unsafe impl Send for AnalyticsBatchMeta {}
unsafe impl Sync for AnalyticsBatchMeta {}

impl AnalyticsBatchMeta {
    #[doc(alias = "gst_buffer_add_analytics_batch_meta")]
    pub fn add<'a>(
        buffer: &'a mut gst::BufferRef,
    ) -> gst::MetaRefMut<'a, Self, gst::meta::Standalone> {
        skip_assert_initialized!();

        unsafe {
            let meta = ffi::gst_buffer_add_analytics_batch_meta(buffer.as_mut_ptr());

            Self::from_mut_ptr(buffer, meta)
        }
    }

    pub fn streams(&self) -> &[AnalyticsBatchStream] {
        unsafe {
            if self.0.streams.is_null() {
                &[]
            } else {
                slice::from_raw_parts(self.0.streams as *const _, self.0.n_streams)
            }
        }
    }

    pub fn streams_mut(&mut self) -> &mut [AnalyticsBatchStream] {
        unsafe {
            if self.0.streams.is_null() {
                &mut []
            } else {
                slice::from_raw_parts_mut(self.0.streams as *mut _, self.0.n_streams)
            }
        }
    }
}

unsafe impl gst::MetaAPI for AnalyticsBatchMeta {
    type GstType = ffi::GstAnalyticsBatchMeta;

    #[doc(alias = "gst_analytics_batch_meta_api_get_type")]
    #[inline]
    fn meta_api() -> glib::Type {
        unsafe { from_glib(ffi::gst_analytics_batch_meta_api_get_type()) }
    }
}

impl fmt::Debug for AnalyticsBatchMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AnalyticsBatchMeta")
            .field("streams", &self.streams())
            .finish()
    }
}

#[repr(transparent)]
#[doc(alias = "GstAnalyticsBatchStream")]
pub struct AnalyticsBatchStream(ffi::GstAnalyticsBatchStream);

impl AnalyticsBatchStream {
    pub fn index(&self) -> u32 {
        self.0.index
    }

    pub fn buffers(&self) -> &[AnalyticsBatchBuffer] {
        unsafe {
            if self.0.buffers.is_null() {
                &[]
            } else {
                slice::from_raw_parts(self.0.buffers as *const _, self.0.n_buffers)
            }
        }
    }

    pub fn buffers_mut(&mut self) -> &mut [AnalyticsBatchBuffer] {
        unsafe {
            if self.0.buffers.is_null() {
                &mut []
            } else {
                slice::from_raw_parts_mut(self.0.buffers as *mut _, self.0.n_buffers)
            }
        }
    }
}

impl fmt::Debug for AnalyticsBatchStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AnalyticsBatchStream")
            .field("index", &self.index())
            .field("buffers", &self.buffers())
            .finish()
    }
}

#[repr(transparent)]
#[doc(alias = "GstAnalyticsBatchBuffer")]
pub struct AnalyticsBatchBuffer(ffi::GstAnalyticsBatchBuffer);

impl AnalyticsBatchBuffer {
    pub fn sticky_events(&self) -> &[gst::Event] {
        unsafe {
            if self.0.sticky_events.is_null() {
                &[]
            } else {
                slice::from_raw_parts(self.0.sticky_events as *const _, self.0.n_sticky_events)
            }
        }
    }

    pub fn serialized_events(&self) -> &[gst::Event] {
        unsafe {
            if self.0.serialized_events.is_null() {
                &[]
            } else {
                slice::from_raw_parts(
                    self.0.serialized_events as *const _,
                    self.0.n_serialized_events,
                )
            }
        }
    }

    pub fn buffer(&self) -> Option<&gst::BufferRef> {
        unsafe {
            if self.0.buffer.is_null() {
                None
            } else {
                Some(gst::BufferRef::from_ptr(self.0.buffer))
            }
        }
    }

    pub fn buffer_owned(&self) -> Option<gst::Buffer> {
        unsafe {
            if self.0.buffer.is_null() {
                None
            } else {
                Some(gst::Buffer::from_glib_none(self.0.buffer))
            }
        }
    }

    pub fn buffer_mut(&mut self) -> Option<&mut gst::BufferRef> {
        unsafe {
            if self.0.serialized_events.is_null() {
                None
            } else {
                self.0.buffer = gst::ffi::gst_mini_object_make_writable(
                    self.0.buffer as *mut gst::ffi::GstMiniObject,
                ) as *mut gst::ffi::GstBuffer;
                Some(gst::BufferRef::from_mut_ptr(self.0.buffer))
            }
        }
    }

    pub fn buffer_list(&self) -> Option<&gst::BufferListRef> {
        unsafe {
            if self.0.buffer_list.is_null() {
                None
            } else {
                Some(gst::BufferListRef::from_ptr(self.0.buffer_list))
            }
        }
    }

    pub fn buffer_list_owned(&self) -> Option<gst::BufferList> {
        unsafe {
            if self.0.buffer_list.is_null() {
                None
            } else {
                Some(gst::BufferList::from_glib_none(self.0.buffer_list))
            }
        }
    }

    pub fn buffer_list_mut(&mut self) -> Option<&mut gst::BufferListRef> {
        unsafe {
            if self.0.serialized_events.is_null() {
                None
            } else {
                self.0.buffer_list = gst::ffi::gst_mini_object_make_writable(
                    self.0.buffer_list as *mut gst::ffi::GstMiniObject,
                ) as *mut gst::ffi::GstBufferList;
                Some(gst::BufferListRef::from_mut_ptr(self.0.buffer_list))
            }
        }
    }

    #[doc(alias = "gst_analytics_batch_buffer_get_caps")]
    pub fn caps(&self) -> Option<gst::Caps> {
        unsafe {
            from_glib_none(ffi::gst_analytics_batch_buffer_get_caps(mut_override(
                &self.0,
            )))
        }
    }

    #[doc(alias = "gst_analytics_batch_buffer_get_segment")]
    pub fn segment(&self) -> Option<gst::Segment> {
        unsafe {
            from_glib_none(ffi::gst_analytics_batch_buffer_get_segment(mut_override(
                &self.0,
            )))
        }
    }

    #[doc(alias = "gst_analytics_batch_buffer_get_stream_id")]
    pub fn stream_id(&self) -> Option<&glib::GStr> {
        unsafe {
            let res = ffi::gst_analytics_batch_buffer_get_stream_id(mut_override(&self.0));

            if res.is_null() {
                None
            } else {
                Some(glib::GStr::from_ptr(res))
            }
        }
    }
}

impl fmt::Debug for AnalyticsBatchBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AnalyticsBatchBuffer")
            .field("sticky_events", &self.sticky_events())
            .field("serialized_events", &self.serialized_events())
            .field("buffer", &self.buffer())
            .finish()
    }
}
