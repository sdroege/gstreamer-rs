use glib::translate::*;
use gst::Caps;
use gst_audio_sys::GstAudioRingBufferSpec;

use std::mem;
use std::ptr;

use AudioInfo;
use AudioRingBufferFormatType;

pub struct AudioRingBufferSpec {
    inner: GstAudioRingBufferSpec,
    caps: Caps,
    info: AudioInfo,
}

impl AudioRingBufferSpec {
    pub(crate) unsafe fn copy_into(&self, into: *mut GstAudioRingBufferSpec) {
        (*into).type_ = self.inner.type_;
        (*into).latency_time = self.inner.latency_time;
        (*into).buffer_time = self.inner.buffer_time;
        (*into).segsize = self.inner.segsize;
        (*into).segtotal = self.inner.segtotal;
        (*into).seglatency = self.inner.seglatency;
    }

    pub fn get_type(&self) -> AudioRingBufferFormatType {
        AudioRingBufferFormatType::from_glib(self.inner.type_)
    }

    pub fn set_type(&mut self, value: AudioRingBufferFormatType) {
        self.inner.type_ = value.to_glib();
    }

    pub fn get_caps(&self) -> &Caps {
        &self.caps
    }

    pub fn get_audio_info(&self) -> &AudioInfo {
        &self.info
    }

    pub fn get_latency_time(&self) -> u64 {
        self.inner.latency_time
    }

    pub fn set_latency_time(&mut self, value: u64) {
        self.inner.latency_time = value;
    }

    pub fn get_buffer_time(&self) -> u64 {
        self.inner.buffer_time
    }

    pub fn set_buffer_time(&mut self, value: u64) {
        self.inner.buffer_time = value;
    }

    pub fn get_segsize(&self) -> i32 {
        self.inner.segsize
    }

    pub fn set_segsize(&mut self, value: i32) {
        self.inner.segsize = value;
    }

    pub fn get_segtotal(&self) -> i32 {
        self.inner.segtotal
    }

    pub fn set_segtotal(&mut self, value: i32) {
        self.inner.segtotal = value;
    }

    pub fn get_segelatency(&self) -> i32 {
        self.inner.seglatency
    }

    pub fn set_seglatency(&mut self, value: i32) {
        self.inner.seglatency = value;
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrNone<*mut GstAudioRingBufferSpec> for AudioRingBufferSpec {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut GstAudioRingBufferSpec) -> Self {
        AudioRingBufferSpec {
            inner: ptr::read(ptr),
            caps: Caps::from_glib_none((*ptr).caps),
            info: AudioInfo::from_glib_none(&mut (*ptr).info),
        }
    }
}

impl<'a> ToGlibPtr<'a, *mut GstAudioRingBufferSpec> for AudioRingBufferSpec {
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> Stash<'a, *mut GstAudioRingBufferSpec, Self> {
        unsafe {
            Stash(
                mem::transmute::<*const GstAudioRingBufferSpec, *mut GstAudioRingBufferSpec>(
                    &self.inner,
                ),
                self,
            )
        }
    }

    fn to_glib_full(&self) -> *mut GstAudioRingBufferSpec {
        unimplemented!();
    }
}

impl<'a> ToGlibPtrMut<'a, *mut GstAudioRingBufferSpec> for AudioRingBufferSpec {
    type Storage = &'a mut Self;

    fn to_glib_none_mut(&'a mut self) -> StashMut<*mut GstAudioRingBufferSpec, Self> {
        StashMut(&mut self.inner, self)
    }
}
