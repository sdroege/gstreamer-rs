// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use gst_audio_sys;
use gst_sys;

use glib;
use glib::translate::{from_glib, Borrowed, FromGlibPtrNone, ToGlibPtr};
use gst;

use std::fmt;
use std::marker::PhantomData;
use std::mem;
use std::ops;
use std::ptr;
use std::slice;

pub enum Readable {}
pub enum Writable {}

pub struct AudioBuffer<T> {
    // Has to be boxed because it contains self-references
    audio_buffer: Box<gst_audio_sys::GstAudioBuffer>,
    buffer: Option<gst::Buffer>,
    info: ::AudioInfo,
    phantom: PhantomData<T>,
}

unsafe impl<T> Send for AudioBuffer<T> {}
unsafe impl<T> Sync for AudioBuffer<T> {}

impl<T> fmt::Debug for AudioBuffer<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AudioBuffer")
            .field("audio_buffer", &self.audio_buffer)
            .field("buffer", &self.buffer)
            .field("info", &self.info)
            .field("phantom", &self.phantom)
            .finish()
    }
}

impl<T> AudioBuffer<T> {
    pub fn info(&self) -> &::AudioInfo {
        &self.info
    }

    pub fn into_buffer(mut self) -> gst::Buffer {
        self.buffer.take().unwrap()
    }

    pub fn format(&self) -> ::AudioFormat {
        self.info().format()
    }

    pub fn format_info(&self) -> ::AudioFormatInfo {
        self.info().format_info()
    }

    pub fn channels(&self) -> u32 {
        self.info().channels()
    }

    pub fn rate(&self) -> u32 {
        self.info().rate()
    }

    pub fn layout(&self) -> ::AudioLayout {
        self.info().layout()
    }

    pub fn width(&self) -> u32 {
        self.info().width()
    }

    pub fn depth(&self) -> u32 {
        self.info().depth()
    }

    pub fn sample_stride(&self) -> u32 {
        self.info().width() / 8
    }

    pub fn bps(&self) -> u32 {
        self.info().bps()
    }

    pub fn bpf(&self) -> u32 {
        self.info().bpf()
    }

    pub fn n_samples(&self) -> usize {
        self.audio_buffer.n_samples
    }

    pub fn n_planes(&self) -> u32 {
        self.audio_buffer.n_planes as u32
    }

    pub fn plane_size(&self) -> usize {
        (self.n_samples() as usize * self.sample_stride() as usize * self.channels() as usize)
            / self.n_planes() as usize
    }

    pub fn buffer(&self) -> &gst::BufferRef {
        unsafe { gst::BufferRef::from_ptr(self.audio_buffer.buffer) }
    }

    pub fn plane_data(&self, plane: u32) -> Result<&[u8], glib::BoolError> {
        if plane >= self.n_planes() {
            return Err(glib_bool_error!("Plane index higher than number of planes"));
        }

        unsafe {
            Ok(slice::from_raw_parts(
                (*self.audio_buffer.planes.add(plane as usize)) as *const u8,
                self.plane_size(),
            ))
        }
    }

    pub fn as_audio_buffer_ref(&self) -> AudioBufferRef<&gst::BufferRef> {
        let info = self.info.clone();
        AudioBufferRef {
            audio_buffer: AudioBufferPtr::Borrowed(ptr::NonNull::from(&*self.audio_buffer)),
            buffer: Some(self.buffer()),
            info,
            unmap: false,
        }
    }

    pub fn as_ptr(&self) -> *const gst_audio_sys::GstAudioBuffer {
        &*self.audio_buffer
    }
}

impl<T> Drop for AudioBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            gst_audio_sys::gst_audio_buffer_unmap(&mut *self.audio_buffer);
        }
    }
}

impl AudioBuffer<Readable> {
    pub fn from_buffer_readable(
        buffer: gst::Buffer,
        info: &::AudioInfo,
    ) -> Result<AudioBuffer<Readable>, gst::Buffer> {
        skip_assert_initialized!();

        assert!(info.is_valid());

        unsafe {
            let mut audio_buffer = Box::new(mem::MaybeUninit::zeroed().assume_init());
            let res: bool = from_glib(gst_audio_sys::gst_audio_buffer_map(
                &mut *audio_buffer,
                info.to_glib_none().0 as *mut _,
                buffer.to_glib_none().0,
                gst_sys::GST_MAP_READ,
            ));

            if !res {
                Err(buffer)
            } else {
                let info = ::AudioInfo::from_glib_none(
                    &audio_buffer.info as *const _ as *mut gst_audio_sys::GstAudioInfo,
                );
                Ok(AudioBuffer {
                    audio_buffer,
                    buffer: Some(buffer),
                    info,
                    phantom: PhantomData,
                })
            }
        }
    }
}

impl AudioBuffer<Writable> {
    pub fn from_buffer_writable(
        buffer: gst::Buffer,
        info: &::AudioInfo,
    ) -> Result<AudioBuffer<Writable>, gst::Buffer> {
        skip_assert_initialized!();

        assert!(info.is_valid());

        unsafe {
            let mut audio_buffer = Box::new(mem::MaybeUninit::zeroed().assume_init());
            let res: bool = from_glib(gst_audio_sys::gst_audio_buffer_map(
                &mut *audio_buffer,
                info.to_glib_none().0 as *mut _,
                buffer.to_glib_none().0,
                gst_sys::GST_MAP_READ | gst_sys::GST_MAP_WRITE,
            ));

            if !res {
                Err(buffer)
            } else {
                let info = ::AudioInfo::from_glib_none(
                    &audio_buffer.info as *const _ as *mut gst_audio_sys::GstAudioInfo,
                );
                Ok(AudioBuffer {
                    audio_buffer,
                    buffer: Some(buffer),
                    info,
                    phantom: PhantomData,
                })
            }
        }
    }

    pub fn buffer_mut(&mut self) -> &mut gst::BufferRef {
        unsafe { gst::BufferRef::from_mut_ptr(self.audio_buffer.buffer) }
    }

    pub fn plane_data_mut(&mut self, plane: u32) -> Result<&mut [u8], glib::BoolError> {
        if plane >= self.n_planes() {
            return Err(glib_bool_error!("Plane index higher than number of planes"));
        }

        unsafe {
            Ok(slice::from_raw_parts_mut(
                (*self.audio_buffer.planes.add(plane as usize)) as *mut u8,
                self.plane_size(),
            ))
        }
    }

    pub fn as_mut_audio_buffer_ref(&mut self) -> AudioBufferRef<&mut gst::BufferRef> {
        let info = self.info.clone();
        AudioBufferRef {
            audio_buffer: AudioBufferPtr::Borrowed(ptr::NonNull::from(&mut *self.audio_buffer)),
            buffer: Some(self.buffer_mut()),
            info,
            unmap: false,
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut gst_audio_sys::GstAudioBuffer {
        &mut *self.audio_buffer
    }
}

#[derive(Debug)]
enum AudioBufferPtr {
    Owned(Box<gst_audio_sys::GstAudioBuffer>),
    Borrowed(ptr::NonNull<gst_audio_sys::GstAudioBuffer>),
}

impl ops::Deref for AudioBufferPtr {
    type Target = gst_audio_sys::GstAudioBuffer;

    fn deref(&self) -> &Self::Target {
        match self {
            AudioBufferPtr::Owned(ref b) => &*b,
            AudioBufferPtr::Borrowed(ref b) => unsafe { b.as_ref() },
        }
    }
}

impl ops::DerefMut for AudioBufferPtr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            AudioBufferPtr::Owned(ref mut b) => &mut *b,
            AudioBufferPtr::Borrowed(ref mut b) => unsafe { b.as_mut() },
        }
    }
}

#[derive(Debug)]
pub struct AudioBufferRef<T> {
    // Has to be boxed because it contains self-references
    audio_buffer: AudioBufferPtr,
    buffer: Option<T>,
    info: ::AudioInfo,
    unmap: bool,
}

impl<T> AudioBufferRef<T> {
    pub fn info(&self) -> &::AudioInfo {
        &self.info
    }

    pub fn format(&self) -> ::AudioFormat {
        self.info().format()
    }

    pub fn format_info(&self) -> ::AudioFormatInfo {
        self.info().format_info()
    }

    pub fn channels(&self) -> u32 {
        self.info().channels()
    }

    pub fn rate(&self) -> u32 {
        self.info().rate()
    }

    pub fn layout(&self) -> ::AudioLayout {
        self.info().layout()
    }

    pub fn width(&self) -> u32 {
        self.info().width()
    }

    pub fn depth(&self) -> u32 {
        self.info().depth()
    }

    pub fn sample_stride(&self) -> u32 {
        self.info().width() / 8
    }

    pub fn bps(&self) -> u32 {
        self.info().bps()
    }

    pub fn bpf(&self) -> u32 {
        self.info().bpf()
    }

    pub fn n_samples(&self) -> usize {
        self.audio_buffer.n_samples
    }

    pub fn n_planes(&self) -> u32 {
        self.audio_buffer.n_planes as u32
    }

    pub fn plane_size(&self) -> usize {
        (self.n_samples() as usize * self.sample_stride() as usize * self.channels() as usize)
            / self.n_planes() as usize
    }

    pub fn plane_data(&self, plane: u32) -> Result<&[u8], glib::BoolError> {
        if plane >= self.n_planes() {
            return Err(glib_bool_error!("Plane index higher than number of planes"));
        }

        unsafe {
            Ok(slice::from_raw_parts(
                (*self.audio_buffer.planes.add(plane as usize)) as *const u8,
                self.plane_size(),
            ))
        }
    }

    pub fn as_ptr(&self) -> *const gst_audio_sys::GstAudioBuffer {
        &*self.audio_buffer
    }
}

impl<'a> AudioBufferRef<&'a gst::BufferRef> {
    pub unsafe fn from_glib_borrow(
        audio_buffer: *const gst_audio_sys::GstAudioBuffer,
    ) -> Borrowed<Self> {
        assert!(!audio_buffer.is_null());

        let info = ::AudioInfo::from_glib_none(
            &(*audio_buffer).info as *const _ as *mut gst_audio_sys::GstAudioInfo,
        );
        let buffer = gst::BufferRef::from_ptr((*audio_buffer).buffer);
        Borrowed::new(AudioBufferRef {
            audio_buffer: AudioBufferPtr::Borrowed(ptr::NonNull::new_unchecked(
                audio_buffer as *mut _,
            )),
            buffer: Some(buffer),
            info,
            unmap: false,
        })
    }

    pub fn from_buffer_ref_readable<'b>(
        buffer: &'a gst::BufferRef,
        info: &'b ::AudioInfo,
    ) -> Result<AudioBufferRef<&'a gst::BufferRef>, glib::BoolError> {
        skip_assert_initialized!();

        assert!(info.is_valid());

        unsafe {
            let mut audio_buffer = Box::new(mem::MaybeUninit::zeroed().assume_init());
            let res: bool = from_glib(gst_audio_sys::gst_audio_buffer_map(
                &mut *audio_buffer,
                info.to_glib_none().0 as *mut _,
                buffer.as_mut_ptr(),
                gst_sys::GST_MAP_READ,
            ));

            if !res {
                Err(glib_bool_error!("Failed to map AudioBuffer"))
            } else {
                let info = ::AudioInfo::from_glib_none(
                    &audio_buffer.info as *const _ as *mut gst_audio_sys::GstAudioInfo,
                );
                Ok(AudioBufferRef {
                    audio_buffer: AudioBufferPtr::Owned(audio_buffer),
                    buffer: Some(buffer),
                    info,
                    unmap: true,
                })
            }
        }
    }

    pub fn buffer(&self) -> &gst::BufferRef {
        self.buffer.as_ref().unwrap()
    }
}

impl<'a> AudioBufferRef<&'a mut gst::BufferRef> {
    pub unsafe fn from_glib_borrow_mut(
        audio_buffer: *mut gst_audio_sys::GstAudioBuffer,
    ) -> Borrowed<Self> {
        assert!(!audio_buffer.is_null());

        let info = ::AudioInfo::from_glib_none(
            &(*audio_buffer).info as *const _ as *mut gst_audio_sys::GstAudioInfo,
        );
        let buffer = gst::BufferRef::from_mut_ptr((*audio_buffer).buffer);
        Borrowed::new(AudioBufferRef {
            audio_buffer: AudioBufferPtr::Borrowed(ptr::NonNull::new_unchecked(audio_buffer)),
            buffer: Some(buffer),
            info,
            unmap: false,
        })
    }

    pub fn from_buffer_ref_writable<'b>(
        buffer: &'a mut gst::BufferRef,
        info: &'b ::AudioInfo,
    ) -> Result<AudioBufferRef<&'a mut gst::BufferRef>, glib::BoolError> {
        skip_assert_initialized!();

        assert!(info.is_valid());

        unsafe {
            let mut audio_buffer = Box::new(mem::MaybeUninit::zeroed().assume_init());
            let res: bool = from_glib(gst_audio_sys::gst_audio_buffer_map(
                &mut *audio_buffer,
                info.to_glib_none().0 as *mut _,
                buffer.as_mut_ptr(),
                gst_sys::GST_MAP_READ | gst_sys::GST_MAP_WRITE,
            ));

            if !res {
                Err(glib_bool_error!("Failed to map AudioBuffer"))
            } else {
                let info = ::AudioInfo::from_glib_none(
                    &audio_buffer.info as *const _ as *mut gst_audio_sys::GstAudioInfo,
                );
                Ok(AudioBufferRef {
                    audio_buffer: AudioBufferPtr::Owned(audio_buffer),
                    buffer: Some(buffer),
                    info,
                    unmap: true,
                })
            }
        }
    }

    pub fn buffer_mut(&mut self) -> &mut gst::BufferRef {
        self.buffer.as_mut().unwrap()
    }

    pub fn plane_data_mut(&mut self, plane: u32) -> Result<&mut [u8], glib::BoolError> {
        if plane >= self.n_planes() {
            return Err(glib_bool_error!("Plane index higher than number of planes"));
        }

        unsafe {
            Ok(slice::from_raw_parts_mut(
                (*self.audio_buffer.planes.add(plane as usize)) as *mut u8,
                self.plane_size(),
            ))
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut gst_audio_sys::GstAudioBuffer {
        &mut *self.audio_buffer
    }
}

impl<'a> ops::Deref for AudioBufferRef<&'a mut gst::BufferRef> {
    type Target = AudioBufferRef<&'a gst::BufferRef>;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(self as *const AudioBufferRef<&'a mut gst::BufferRef>
                as *const AudioBufferRef<&'a gst::BufferRef>)
        }
    }
}

unsafe impl<T> Send for AudioBufferRef<T> {}
unsafe impl<T> Sync for AudioBufferRef<T> {}

impl<T> Drop for AudioBufferRef<T> {
    fn drop(&mut self) {
        unsafe {
            if self.unmap {
                gst_audio_sys::gst_audio_buffer_unmap(&mut *self.audio_buffer);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gst;

    #[test]
    fn test_map_read() {
        gst::init().unwrap();

        let info = ::AudioInfo::builder(::AUDIO_FORMAT_S16, 48000, 2)
            .build()
            .unwrap();
        let buffer = gst::Buffer::with_size(info.rate() as usize * info.bpf() as usize).unwrap();
        let buffer = AudioBuffer::from_buffer_readable(buffer, &info).unwrap();

        assert!(buffer.plane_data(0).is_ok());
        assert_eq!(buffer.plane_data(0).unwrap().len(), 2 * 2 * 48000);
        assert!(buffer.plane_data(1).is_err());
        assert!(buffer.info() == &info);

        {
            let buffer = buffer.as_audio_buffer_ref();

            assert!(buffer.plane_data(0).is_ok());
            assert_eq!(buffer.plane_data(0).unwrap().len(), 2 * 2 * 48000);
            assert!(buffer.plane_data(1).is_err());
            assert!(buffer.info() == &info);
        }

        assert!(buffer.plane_data(0).is_ok());
        assert_eq!(buffer.plane_data(0).unwrap().len(), 2 * 2 * 48000);
        assert!(buffer.plane_data(1).is_err());
        assert!(buffer.info() == &info);
    }

    #[test]
    fn test_map_read_planar() {
        gst::init().unwrap();

        let info = ::AudioInfo::builder(::AUDIO_FORMAT_S16, 48000, 2)
            .layout(::AudioLayout::NonInterleaved)
            .build()
            .unwrap();
        let mut buffer =
            gst::Buffer::with_size(info.rate() as usize * info.bpf() as usize).unwrap();
        {
            let buffer = buffer.get_mut().unwrap();
            ::AudioMeta::add(buffer, &info, 48000, &[]).unwrap();
        }
        let buffer = AudioBuffer::from_buffer_readable(buffer, &info).unwrap();

        assert!(buffer.plane_data(0).is_ok());
        assert_eq!(buffer.plane_data(0).unwrap().len(), 2 * 48000);
        assert!(buffer.plane_data(1).is_ok());
        assert_eq!(buffer.plane_data(1).unwrap().len(), 2 * 48000);
        assert!(buffer.info() == &info);

        {
            let buffer = buffer.as_audio_buffer_ref();

            assert!(buffer.plane_data(0).is_ok());
            assert_eq!(buffer.plane_data(0).unwrap().len(), 2 * 48000);
            assert!(buffer.plane_data(1).is_ok());
            assert_eq!(buffer.plane_data(1).unwrap().len(), 2 * 48000);
            assert!(buffer.info() == &info);
        }

        assert!(buffer.plane_data(0).is_ok());
        assert_eq!(buffer.plane_data(0).unwrap().len(), 2 * 48000);
        assert!(buffer.plane_data(1).is_ok());
        assert_eq!(buffer.plane_data(1).unwrap().len(), 2 * 48000);
        assert!(buffer.info() == &info);
    }

    #[test]
    fn test_map_write() {
        gst::init().unwrap();

        let info = ::AudioInfo::builder(::AUDIO_FORMAT_S16, 48000, 2)
            .build()
            .unwrap();
        let buffer = gst::Buffer::with_size(info.rate() as usize * info.bpf() as usize).unwrap();
        let mut buffer = AudioBuffer::from_buffer_writable(buffer, &info).unwrap();

        assert!(buffer.plane_data_mut(0).is_ok());
        assert_eq!(buffer.plane_data_mut(0).unwrap().len(), 2 * 2 * 48000);
        assert!(buffer.plane_data_mut(1).is_err());
        assert!(buffer.info() == &info);

        {
            let mut buffer = buffer.as_mut_audio_buffer_ref();

            assert!(buffer.plane_data_mut(0).is_ok());
            assert_eq!(buffer.plane_data_mut(0).unwrap().len(), 2 * 2 * 48000);
            assert!(buffer.plane_data_mut(1).is_err());
            assert!(buffer.info() == &info);
        }

        assert!(buffer.plane_data_mut(0).is_ok());
        assert_eq!(buffer.plane_data_mut(0).unwrap().len(), 2 * 2 * 48000);
        assert!(buffer.plane_data_mut(1).is_err());
        assert!(buffer.info() == &info);
    }

    #[test]
    fn test_map_write_planar() {
        gst::init().unwrap();

        let info = ::AudioInfo::builder(::AUDIO_FORMAT_S16, 48000, 2)
            .layout(::AudioLayout::NonInterleaved)
            .build()
            .unwrap();
        let mut buffer =
            gst::Buffer::with_size(info.rate() as usize * info.bpf() as usize).unwrap();
        {
            let buffer = buffer.get_mut().unwrap();
            ::AudioMeta::add(buffer, &info, 48000, &[]).unwrap();
        }
        let mut buffer = AudioBuffer::from_buffer_writable(buffer, &info).unwrap();

        assert!(buffer.plane_data_mut(0).is_ok());
        assert_eq!(buffer.plane_data_mut(0).unwrap().len(), 2 * 48000);
        assert!(buffer.plane_data_mut(1).is_ok());
        assert_eq!(buffer.plane_data_mut(1).unwrap().len(), 2 * 48000);
        assert!(buffer.info() == &info);

        {
            let mut buffer = buffer.as_mut_audio_buffer_ref();

            assert!(buffer.plane_data_mut(0).is_ok());
            assert_eq!(buffer.plane_data_mut(0).unwrap().len(), 2 * 48000);
            assert!(buffer.plane_data_mut(1).is_ok());
            assert_eq!(buffer.plane_data_mut(1).unwrap().len(), 2 * 48000);
            assert!(buffer.info() == &info);
        }

        assert!(buffer.plane_data_mut(0).is_ok());
        assert_eq!(buffer.plane_data_mut(0).unwrap().len(), 2 * 48000);
        assert!(buffer.plane_data_mut(1).is_ok());
        assert_eq!(buffer.plane_data_mut(1).unwrap().len(), 2 * 48000);
        assert!(buffer.info() == &info);
    }

    #[test]
    fn test_map_ref_read() {
        gst::init().unwrap();

        let info = ::AudioInfo::builder(::AUDIO_FORMAT_S16, 48000, 2)
            .build()
            .unwrap();
        let buffer = gst::Buffer::with_size(info.rate() as usize * info.bpf() as usize).unwrap();
        let buffer = AudioBufferRef::from_buffer_ref_readable(&buffer, &info).unwrap();

        assert!(buffer.plane_data(0).is_ok());
        assert_eq!(buffer.plane_data(0).unwrap().len(), 2 * 2 * 48000);
        assert!(buffer.plane_data(1).is_err());
        assert!(buffer.info() == &info);

        assert!(buffer.plane_data(0).is_ok());
        assert_eq!(buffer.plane_data(0).unwrap().len(), 2 * 2 * 48000);
        assert!(buffer.plane_data(1).is_err());
        assert!(buffer.info() == &info);
    }

    #[test]
    fn test_map_ref_read_planar() {
        gst::init().unwrap();

        let info = ::AudioInfo::builder(::AUDIO_FORMAT_S16, 48000, 2)
            .layout(::AudioLayout::NonInterleaved)
            .build()
            .unwrap();
        let mut buffer =
            gst::Buffer::with_size(info.rate() as usize * info.bpf() as usize).unwrap();
        {
            let buffer = buffer.get_mut().unwrap();
            ::AudioMeta::add(buffer, &info, 48000, &[]).unwrap();
        }
        let buffer = AudioBufferRef::from_buffer_ref_readable(&buffer, &info).unwrap();

        assert!(buffer.plane_data(0).is_ok());
        assert_eq!(buffer.plane_data(0).unwrap().len(), 2 * 48000);
        assert!(buffer.plane_data(1).is_ok());
        assert_eq!(buffer.plane_data(1).unwrap().len(), 2 * 48000);
        assert!(buffer.info() == &info);

        assert!(buffer.plane_data(0).is_ok());
        assert_eq!(buffer.plane_data(0).unwrap().len(), 2 * 48000);
        assert!(buffer.plane_data(1).is_ok());
        assert_eq!(buffer.plane_data(1).unwrap().len(), 2 * 48000);
        assert!(buffer.info() == &info);
    }

    #[test]
    fn test_map_ref_write() {
        gst::init().unwrap();

        let info = ::AudioInfo::builder(::AUDIO_FORMAT_S16, 48000, 2)
            .build()
            .unwrap();
        let mut buffer =
            gst::Buffer::with_size(info.rate() as usize * info.bpf() as usize).unwrap();

        {
            let buffer = buffer.get_mut().unwrap();
            let mut buffer = AudioBufferRef::from_buffer_ref_writable(buffer, &info).unwrap();

            assert!(buffer.plane_data_mut(0).is_ok());
            assert_eq!(buffer.plane_data_mut(0).unwrap().len(), 2 * 2 * 48000);
            assert!(buffer.plane_data_mut(1).is_err());
            assert!(buffer.info() == &info);

            assert!(buffer.plane_data_mut(0).is_ok());
            assert_eq!(buffer.plane_data_mut(0).unwrap().len(), 2 * 2 * 48000);
            assert!(buffer.plane_data_mut(1).is_err());
            assert!(buffer.info() == &info);
        }
    }

    #[test]
    fn test_map_ref_write_planar() {
        gst::init().unwrap();

        let info = ::AudioInfo::builder(::AUDIO_FORMAT_S16, 48000, 2)
            .layout(::AudioLayout::NonInterleaved)
            .build()
            .unwrap();
        let mut buffer =
            gst::Buffer::with_size(info.rate() as usize * info.bpf() as usize).unwrap();
        {
            let buffer = buffer.get_mut().unwrap();
            ::AudioMeta::add(buffer, &info, 48000, &[]).unwrap();
        }

        {
            let buffer = buffer.get_mut().unwrap();
            let mut buffer = AudioBufferRef::from_buffer_ref_writable(buffer, &info).unwrap();

            assert!(buffer.plane_data_mut(0).is_ok());
            assert_eq!(buffer.plane_data_mut(0).unwrap().len(), 2 * 48000);
            assert!(buffer.plane_data_mut(1).is_ok());
            assert_eq!(buffer.plane_data_mut(1).unwrap().len(), 2 * 48000);
            assert!(buffer.info() == &info);

            assert!(buffer.plane_data_mut(0).is_ok());
            assert_eq!(buffer.plane_data_mut(0).unwrap().len(), 2 * 48000);
            assert!(buffer.plane_data_mut(1).is_ok());
            assert_eq!(buffer.plane_data_mut(1).unwrap().len(), 2 * 48000);
            assert!(buffer.info() == &info);
        }
    }
}
