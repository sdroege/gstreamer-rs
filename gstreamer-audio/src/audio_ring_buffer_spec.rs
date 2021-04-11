// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ffi::GstAudioRingBufferSpec;
use glib::translate::*;
use gst::Caps;

use crate::AudioInfo;
use crate::AudioRingBufferFormatType;

use std::fmt;

#[repr(transparent)]
pub struct AudioRingBufferSpec(pub(crate) GstAudioRingBufferSpec);

impl AudioRingBufferSpec {
    pub fn type_(&self) -> AudioRingBufferFormatType {
        unsafe { AudioRingBufferFormatType::from_glib(self.0.type_) }
    }

    pub fn set_type(&mut self, value: AudioRingBufferFormatType) {
        self.0.type_ = value.to_glib();
    }

    pub fn caps(&self) -> Caps {
        unsafe { Caps::from_glib_none(self.0.caps) }
    }

    pub fn audio_info(&self) -> AudioInfo {
        unsafe { AudioInfo::from_glib_none(mut_override(&self.0.info)) }
    }

    pub fn latency_time(&self) -> u64 {
        self.0.latency_time
    }

    pub fn set_latency_time(&mut self, value: u64) {
        self.0.latency_time = value;
    }

    pub fn buffer_time(&self) -> u64 {
        self.0.buffer_time
    }

    pub fn set_buffer_time(&mut self, value: u64) {
        self.0.buffer_time = value;
    }

    pub fn segsize(&self) -> i32 {
        self.0.segsize
    }

    pub fn set_segsize(&mut self, value: i32) {
        self.0.segsize = value;
    }

    pub fn segtotal(&self) -> i32 {
        self.0.segtotal
    }

    pub fn set_segtotal(&mut self, value: i32) {
        self.0.segtotal = value;
    }

    pub fn seglatency(&self) -> i32 {
        self.0.seglatency
    }

    pub fn set_seglatency(&mut self, value: i32) {
        self.0.seglatency = value;
    }
}

impl Clone for AudioRingBufferSpec {
    fn clone(&self) -> Self {
        unsafe {
            let spec = self.0;
            gst::ffi::gst_mini_object_ref(spec.caps as *mut gst::ffi::GstMiniObject);

            AudioRingBufferSpec(spec)
        }
    }
}

impl Drop for AudioRingBufferSpec {
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
