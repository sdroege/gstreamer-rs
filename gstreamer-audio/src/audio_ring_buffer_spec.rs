// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

use glib::translate::*;
use gst::Caps;

use crate::{ffi::GstAudioRingBufferSpec, AudioInfo, AudioRingBufferFormatType};

#[repr(transparent)]
pub struct AudioRingBufferSpec(pub(crate) GstAudioRingBufferSpec);

impl AudioRingBufferSpec {
    #[doc(alias = "get_type")]
    #[inline]
    pub fn type_(&self) -> AudioRingBufferFormatType {
        unsafe { AudioRingBufferFormatType::from_glib(self.0.type_) }
    }

    #[inline]
    pub fn set_type(&mut self, value: AudioRingBufferFormatType) {
        self.0.type_ = value.into_glib();
    }

    #[doc(alias = "get_caps")]
    #[inline]
    pub fn caps(&self) -> Caps {
        unsafe { Caps::from_glib_none(self.0.caps) }
    }

    #[doc(alias = "get_audio_info")]
    #[inline]
    pub fn audio_info(&self) -> AudioInfo {
        unsafe { AudioInfo::from_glib_none(&self.0.info as *const ffi::GstAudioInfo) }
    }

    #[doc(alias = "get_latency_time")]
    #[inline]
    pub fn latency_time(&self) -> u64 {
        self.0.latency_time
    }

    #[inline]
    pub fn set_latency_time(&mut self, value: u64) {
        self.0.latency_time = value;
    }

    #[doc(alias = "get_buffer_time")]
    #[inline]
    pub fn buffer_time(&self) -> u64 {
        self.0.buffer_time
    }

    #[inline]
    pub fn set_buffer_time(&mut self, value: u64) {
        self.0.buffer_time = value;
    }

    #[doc(alias = "get_segsize")]
    #[inline]
    pub fn segsize(&self) -> i32 {
        self.0.segsize
    }

    #[inline]
    pub fn set_segsize(&mut self, value: i32) {
        self.0.segsize = value;
    }

    #[doc(alias = "get_segtotal")]
    #[inline]
    pub fn segtotal(&self) -> i32 {
        self.0.segtotal
    }

    #[inline]
    pub fn set_segtotal(&mut self, value: i32) {
        self.0.segtotal = value;
    }

    #[doc(alias = "get_seglatency")]
    #[inline]
    pub fn seglatency(&self) -> i32 {
        self.0.seglatency
    }

    #[inline]
    pub fn set_seglatency(&mut self, value: i32) {
        self.0.seglatency = value;
    }
}

impl Clone for AudioRingBufferSpec {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            let spec = self.0;
            gst::ffi::gst_mini_object_ref(spec.caps as *mut gst::ffi::GstMiniObject);

            Self(spec)
        }
    }
}

impl Drop for AudioRingBufferSpec {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            gst::ffi::gst_mini_object_unref(self.0.caps as *mut gst::ffi::GstMiniObject);
        }
    }
}

unsafe impl Send for AudioRingBufferSpec {}
unsafe impl Sync for AudioRingBufferSpec {}

impl fmt::Debug for AudioRingBufferSpec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AudioRingBufferSpec")
            .field("type", &self.type_())
            .field("caps", &self.caps())
            .field("audio_info", &self.audio_info())
            .field("latency_time", &self.latency_time())
            .field("buffer_time", &self.buffer_time())
            .field("segsize", &self.segsize())
            .field("segtotal", &self.segtotal())
            .field("seglatency", &self.seglatency())
            .finish()
    }
}
