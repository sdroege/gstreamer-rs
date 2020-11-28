use crate::ffi::GstAudioRingBufferSpec;
use glib::translate::*;
use gst::Caps;

use crate::AudioInfo;
use crate::AudioRingBufferFormatType;

use std::fmt;

#[repr(transparent)]
pub struct AudioRingBufferSpec(pub(crate) GstAudioRingBufferSpec);

impl AudioRingBufferSpec {
    pub fn get_type(&self) -> AudioRingBufferFormatType {
        AudioRingBufferFormatType::from_glib(self.0.type_)
    }

    pub fn set_type(&mut self, value: AudioRingBufferFormatType) {
        self.0.type_ = value.to_glib();
    }

    pub fn get_caps(&self) -> Caps {
        unsafe { Caps::from_glib_none(self.0.caps) }
    }

    pub fn get_audio_info(&self) -> AudioInfo {
        unsafe { AudioInfo::from_glib_none(mut_override(&self.0.info)) }
    }

    pub fn get_latency_time(&self) -> u64 {
        self.0.latency_time
    }

    pub fn set_latency_time(&mut self, value: u64) {
        self.0.latency_time = value;
    }

    pub fn get_buffer_time(&self) -> u64 {
        self.0.buffer_time
    }

    pub fn set_buffer_time(&mut self, value: u64) {
        self.0.buffer_time = value;
    }

    pub fn get_segsize(&self) -> i32 {
        self.0.segsize
    }

    pub fn set_segsize(&mut self, value: i32) {
        self.0.segsize = value;
    }

    pub fn get_segtotal(&self) -> i32 {
        self.0.segtotal
    }

    pub fn set_segtotal(&mut self, value: i32) {
        self.0.segtotal = value;
    }

    pub fn get_seglatency(&self) -> i32 {
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
            .field("type", &self.get_type())
            .field("caps", &self.get_caps())
            .field("audio_info", &self.get_audio_info())
            .field("latency_time", &self.get_latency_time())
            .field("buffer_time", &self.get_buffer_time())
            .field("segsize", &self.get_segsize())
            .field("segtotal", &self.get_segtotal())
            .field("seglatency", &self.get_seglatency())
            .finish()
    }
}
