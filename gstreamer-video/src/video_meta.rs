// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

use glib;
use glib::translate::{from_glib, from_glib_none, ToGlib};
use gst;
use gst::prelude::*;
use gst_sys;
use gst_video_sys;

#[repr(C)]
pub struct VideoMeta(gst_video_sys::GstVideoMeta);

unsafe impl Send for VideoMeta {}
unsafe impl Sync for VideoMeta {}

impl VideoMeta {
    pub fn add(
        buffer: &mut gst::BufferRef,
        flags: ::VideoFrameFlags,
        format: ::VideoFormat,
        width: u32,
        height: u32,
    ) -> gst::MetaRefMut<Self, gst::meta::Standalone> {
        skip_assert_initialized!();

        let info = ::VideoInfo::new(format, width, height).build().unwrap();
        assert!(buffer.get_size() >= info.size());

        unsafe {
            let meta = gst_video_sys::gst_buffer_add_video_meta(
                buffer.as_mut_ptr(),
                flags.to_glib(),
                format.to_glib(),
                width,
                height,
            );

            Self::from_mut_ptr(buffer, meta)
        }
    }

    pub fn add_full<'a>(
        buffer: &'a mut gst::BufferRef,
        flags: ::VideoFrameFlags,
        format: ::VideoFormat,
        width: u32,
        height: u32,
        offset: &[usize],
        stride: &[i32],
    ) -> gst::MetaRefMut<'a, Self, gst::meta::Standalone> {
        skip_assert_initialized!();

        let n_planes = offset.len() as u32;
        let info = ::VideoInfo::new(format, width, height)
            .offset(offset)
            .stride(stride)
            .build()
            .expect("Unable to build VideoInfo for given format, offsets and strides");
        assert!(buffer.get_size() >= info.size());

        unsafe {
            let meta = gst_video_sys::gst_buffer_add_video_meta_full(
                buffer.as_mut_ptr(),
                flags.to_glib(),
                format.to_glib(),
                width,
                height,
                n_planes,
                offset.as_ptr() as *mut _,
                stride.as_ptr() as *mut _,
            );

            Self::from_mut_ptr(buffer, meta)
        }
    }

    pub fn get_flags(&self) -> ::VideoFrameFlags {
        from_glib(self.0.flags)
    }

    pub fn get_format(&self) -> ::VideoFormat {
        from_glib(self.0.format)
    }

    pub fn get_id(&self) -> i32 {
        self.0.id
    }

    pub fn get_width(&self) -> u32 {
        self.0.width
    }

    pub fn get_height(&self) -> u32 {
        self.0.height
    }

    pub fn get_n_planes(&self) -> u32 {
        self.0.n_planes
    }

    pub fn get_offset(&self) -> &[usize] {
        &self.0.offset[0..(self.0.n_planes as usize)]
    }

    pub fn get_stride(&self) -> &[i32] {
        &self.0.stride[0..(self.0.n_planes as usize)]
    }
}

unsafe impl MetaAPI for VideoMeta {
    type GstType = gst_video_sys::GstVideoMeta;

    fn get_meta_api() -> glib::Type {
        unsafe { from_glib(gst_video_sys::gst_video_meta_api_get_type()) }
    }
}

impl fmt::Debug for VideoMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoMeta")
            .field("id", &self.get_id())
            .field("flags", &self.get_flags())
            .field("format", &self.get_format())
            .field("width", &self.get_width())
            .field("height", &self.get_height())
            .field("n_planes", &self.get_n_planes())
            .field("offset", &self.get_offset())
            .field("stride", &self.get_stride())
            .finish()
    }
}

#[repr(C)]
pub struct VideoOverlayCompositionMeta(gst_video_sys::GstVideoOverlayCompositionMeta);

unsafe impl Send for VideoOverlayCompositionMeta {}
unsafe impl Sync for VideoOverlayCompositionMeta {}

impl VideoOverlayCompositionMeta {
    pub fn add<'a>(
        buffer: &'a mut gst::BufferRef,
        overlay: &::VideoOverlayComposition,
    ) -> gst::MetaRefMut<'a, Self, gst::meta::Standalone> {
        skip_assert_initialized!();
        unsafe {
            let meta = gst_video_sys::gst_buffer_add_video_overlay_composition_meta(
                buffer.as_mut_ptr(),
                overlay.as_mut_ptr(),
            );

            Self::from_mut_ptr(buffer, meta)
        }
    }

    pub fn get_overlay(&self) -> &::VideoOverlayCompositionRef {
        unsafe { ::VideoOverlayCompositionRef::from_ptr(self.0.overlay) }
    }

    pub fn get_overlay_owned(&self) -> ::VideoOverlayComposition {
        unsafe { from_glib_none(self.get_overlay().as_ptr()) }
    }

    pub fn set_overlay(&mut self, overlay: &::VideoOverlayComposition) {
        #![allow(clippy::cast_ptr_alignment)]
        unsafe {
            gst_sys::gst_mini_object_unref(self.0.overlay as *mut _);
            self.0.overlay = gst_sys::gst_mini_object_ref(overlay.as_mut_ptr() as *mut _) as *mut _;
        }
    }
}

unsafe impl MetaAPI for VideoOverlayCompositionMeta {
    type GstType = gst_video_sys::GstVideoOverlayCompositionMeta;

    fn get_meta_api() -> glib::Type {
        unsafe { from_glib(gst_video_sys::gst_video_overlay_composition_meta_api_get_type()) }
    }
}

impl fmt::Debug for VideoOverlayCompositionMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoOverlayCompositionMeta")
            .field("overlay", &self.get_overlay())
            .finish()
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[repr(C)]
pub struct VideoCaptionMeta(gst_video_sys::GstVideoCaptionMeta);

#[cfg(any(feature = "v1_16", feature = "dox"))]
unsafe impl Send for VideoCaptionMeta {}
#[cfg(any(feature = "v1_16", feature = "dox"))]
unsafe impl Sync for VideoCaptionMeta {}

#[cfg(any(feature = "v1_16", feature = "dox"))]
impl VideoCaptionMeta {
    pub fn add<'a>(
        buffer: &'a mut gst::BufferRef,
        caption_type: ::VideoCaptionType,
        data: &[u8],
    ) -> gst::MetaRefMut<'a, Self, gst::meta::Standalone> {
        skip_assert_initialized!();
        assert!(!data.is_empty());
        unsafe {
            let meta = gst_video_sys::gst_buffer_add_video_caption_meta(
                buffer.as_mut_ptr(),
                caption_type.to_glib(),
                data.as_ptr(),
                data.len(),
            );

            Self::from_mut_ptr(buffer, meta)
        }
    }

    pub fn get_caption_type(&self) -> ::VideoCaptionType {
        from_glib(self.0.caption_type)
    }

    pub fn get_data(&self) -> &[u8] {
        unsafe {
            use std::slice;

            slice::from_raw_parts(self.0.data, self.0.size)
        }
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
unsafe impl MetaAPI for VideoCaptionMeta {
    type GstType = gst_video_sys::GstVideoCaptionMeta;

    fn get_meta_api() -> glib::Type {
        unsafe { from_glib(gst_video_sys::gst_video_caption_meta_api_get_type()) }
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
impl fmt::Debug for VideoCaptionMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoCaptionMeta")
            .field("caption_type", &self.get_caption_type())
            .field("data", &self.get_data())
            .finish()
    }
}

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[repr(C)]
pub struct VideoAFDMeta(gst_video_sys::GstVideoAFDMeta);

#[cfg(any(feature = "v1_18", feature = "dox"))]
unsafe impl Send for VideoAFDMeta {}
#[cfg(any(feature = "v1_18", feature = "dox"))]
unsafe impl Sync for VideoAFDMeta {}

#[cfg(any(feature = "v1_18", feature = "dox"))]
impl VideoAFDMeta {
    pub fn add(
        buffer: &mut gst::BufferRef,
        field: u8,
        spec: ::VideoAFDSpec,
        afd: ::VideoAFDValue,
    ) -> gst::MetaRefMut<Self, gst::meta::Standalone> {
        skip_assert_initialized!();

        unsafe {
            let meta = gst_video_sys::gst_buffer_add_video_afd_meta(
                buffer.as_mut_ptr(),
                field,
                spec.to_glib(),
                afd.to_glib(),
            );

            Self::from_mut_ptr(buffer, meta)
        }
    }

    pub fn get_field(&self) -> u8 {
        self.0.field
    }

    pub fn get_spec(&self) -> ::VideoAFDSpec {
        from_glib(self.0.spec)
    }

    pub fn get_afd(&self) -> ::VideoAFDValue {
        from_glib(self.0.afd)
    }
}

#[cfg(any(feature = "v1_18", feature = "dox"))]
unsafe impl MetaAPI for VideoAFDMeta {
    type GstType = gst_video_sys::GstVideoAFDMeta;

    fn get_meta_api() -> glib::Type {
        unsafe { from_glib(gst_video_sys::gst_video_afd_meta_api_get_type()) }
    }
}

#[cfg(any(feature = "v1_18", feature = "dox"))]
impl fmt::Debug for VideoAFDMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoAFDMeta")
            .field("field", &self.get_field())
            .field("spec", &self.get_spec())
            .field("afd", &self.get_afd())
            .finish()
    }
}

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[repr(C)]
pub struct VideoBarMeta(gst_video_sys::GstVideoBarMeta);

#[cfg(any(feature = "v1_18", feature = "dox"))]
unsafe impl Send for VideoBarMeta {}
#[cfg(any(feature = "v1_18", feature = "dox"))]
unsafe impl Sync for VideoBarMeta {}

#[cfg(any(feature = "v1_18", feature = "dox"))]
impl VideoBarMeta {
    pub fn add(
        buffer: &mut gst::BufferRef,
        field: u8,
        is_letterbox: bool,
        bar_data1: u32,
        bar_data2: u32,
    ) -> gst::MetaRefMut<Self, gst::meta::Standalone> {
        skip_assert_initialized!();

        unsafe {
            let meta = gst_video_sys::gst_buffer_add_video_bar_meta(
                buffer.as_mut_ptr(),
                field,
                is_letterbox.to_glib(),
                bar_data1,
                bar_data2,
            );

            Self::from_mut_ptr(buffer, meta)
        }
    }

    pub fn get_field(&self) -> u8 {
        self.0.field
    }

    pub fn is_letterbox(&self) -> bool {
        from_glib(self.0.is_letterbox)
    }

    pub fn get_bar_data1(&self) -> u32 {
        self.0.bar_data1
    }

    pub fn get_bar_data2(&self) -> u32 {
        self.0.bar_data2
    }
}

#[cfg(any(feature = "v1_18", feature = "dox"))]
unsafe impl MetaAPI for VideoBarMeta {
    type GstType = gst_video_sys::GstVideoBarMeta;

    fn get_meta_api() -> glib::Type {
        unsafe { from_glib(gst_video_sys::gst_video_bar_meta_api_get_type()) }
    }
}

#[cfg(any(feature = "v1_18", feature = "dox"))]
impl fmt::Debug for VideoBarMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoBarMeta")
            .field("field", &self.get_field())
            .field("is_letterbox", &self.is_letterbox())
            .field("bar_data1", &self.get_bar_data1())
            .field("bar_data2", &self.get_bar_data2())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_get_meta() {
        gst::init().unwrap();

        let mut buffer = gst::Buffer::with_size(320 * 240 * 4).unwrap();
        {
            let meta = VideoMeta::add(
                buffer.get_mut().unwrap(),
                ::VideoFrameFlags::NONE,
                ::VideoFormat::Argb,
                320,
                240,
            );
            assert_eq!(meta.get_id(), 0);
            assert_eq!(meta.get_flags(), ::VideoFrameFlags::NONE);
            assert_eq!(meta.get_format(), ::VideoFormat::Argb);
            assert_eq!(meta.get_width(), 320);
            assert_eq!(meta.get_height(), 240);
            assert_eq!(meta.get_n_planes(), 1);
            assert_eq!(meta.get_offset(), &[0]);
            assert_eq!(meta.get_stride(), &[320 * 4]);
        }

        {
            let meta = buffer.get_meta::<VideoMeta>().unwrap();
            assert_eq!(meta.get_id(), 0);
            assert_eq!(meta.get_flags(), ::VideoFrameFlags::NONE);
            assert_eq!(meta.get_format(), ::VideoFormat::Argb);
            assert_eq!(meta.get_width(), 320);
            assert_eq!(meta.get_height(), 240);
            assert_eq!(meta.get_n_planes(), 1);
            assert_eq!(meta.get_offset(), &[0]);
            assert_eq!(meta.get_stride(), &[320 * 4]);
        }
    }

    #[test]
    fn test_add_full_get_meta() {
        gst::init().unwrap();

        let mut buffer = gst::Buffer::with_size(320 * 240 * 4).unwrap();
        {
            let meta = VideoMeta::add_full(
                buffer.get_mut().unwrap(),
                ::VideoFrameFlags::NONE,
                ::VideoFormat::Argb,
                320,
                240,
                &[0],
                &[320 * 4],
            );
            assert_eq!(meta.get_id(), 0);
            assert_eq!(meta.get_flags(), ::VideoFrameFlags::NONE);
            assert_eq!(meta.get_format(), ::VideoFormat::Argb);
            assert_eq!(meta.get_width(), 320);
            assert_eq!(meta.get_height(), 240);
            assert_eq!(meta.get_n_planes(), 1);
            assert_eq!(meta.get_offset(), &[0]);
            assert_eq!(meta.get_stride(), &[320 * 4]);
        }

        {
            let meta = buffer.get_meta::<VideoMeta>().unwrap();
            assert_eq!(meta.get_id(), 0);
            assert_eq!(meta.get_flags(), ::VideoFrameFlags::NONE);
            assert_eq!(meta.get_format(), ::VideoFormat::Argb);
            assert_eq!(meta.get_width(), 320);
            assert_eq!(meta.get_height(), 240);
            assert_eq!(meta.get_n_planes(), 1);
            assert_eq!(meta.get_offset(), &[0]);
            assert_eq!(meta.get_stride(), &[320 * 4]);
        }
    }
}
