// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use std::ptr;

use glib;
use glib::translate::{from_glib, from_glib_none, FromGlib, ToGlib, ToGlibPtr};
use gst;
use gst::prelude::*;
use gst_sys;
use gst_video_sys;

#[repr(transparent)]
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
    ) -> Result<gst::MetaRefMut<Self, gst::meta::Standalone>, glib::BoolError> {
        skip_assert_initialized!();

        if format == ::VideoFormat::Unknown || format == ::VideoFormat::Encoded {
            return Err(glib_bool_error!("Unsupported video format {}", format));
        }

        let info = ::VideoInfo::builder(format, width, height).build()?;

        if !info.is_valid() {
            return Err(glib_bool_error!("Invalid video info"));
        }

        if buffer.get_size() < info.size() {
            return Err(glib_bool_error!(
                "Buffer smaller than required frame size ({} < {})",
                buffer.get_size(),
                info.size()
            ));
        }

        unsafe {
            let meta = gst_video_sys::gst_buffer_add_video_meta(
                buffer.as_mut_ptr(),
                flags.to_glib(),
                format.to_glib(),
                width,
                height,
            );

            if meta.is_null() {
                return Err(glib_bool_error!("Failed to add video meta"));
            }

            Ok(Self::from_mut_ptr(buffer, meta))
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
    ) -> Result<gst::MetaRefMut<'a, Self, gst::meta::Standalone>, glib::BoolError> {
        skip_assert_initialized!();

        if format == ::VideoFormat::Unknown || format == ::VideoFormat::Encoded {
            return Err(glib_bool_error!("Unsupported video format {}", format));
        }

        let n_planes = offset.len() as u32;
        let info = ::VideoInfo::builder(format, width, height)
            .offset(offset)
            .stride(stride)
            .build()?;

        if !info.is_valid() {
            return Err(glib_bool_error!("Invalid video info"));
        }

        if buffer.get_size() < info.size() {
            return Err(glib_bool_error!(
                "Buffer smaller than required frame size ({} < {})",
                buffer.get_size(),
                info.size()
            ));
        }

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

            if meta.is_null() {
                return Err(glib_bool_error!("Failed to add video meta"));
            }

            Ok(Self::from_mut_ptr(buffer, meta))
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

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    pub fn get_alignment(&self) -> crate::VideoAlignment {
        crate::VideoAlignment::new(
            self.0.alignment.padding_top,
            self.0.alignment.padding_bottom,
            self.0.alignment.padding_left,
            self.0.alignment.padding_right,
            &self.0.alignment.stride_align,
        )
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    pub fn get_plane_size(&self) -> Result<[usize; crate::VIDEO_MAX_PLANES], glib::BoolError> {
        let mut plane_size = [0; crate::VIDEO_MAX_PLANES];

        unsafe {
            glib::glib_result_from_gboolean!(
                gst_video_sys::gst_video_meta_get_plane_size(
                    &self.0 as *const _ as usize as *mut _,
                    plane_size.as_mut_ptr(),
                ),
                "Failed to get plane size"
            )?;
        }

        Ok(plane_size)
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    pub fn get_plane_height(&self) -> Result<[u32; crate::VIDEO_MAX_PLANES], glib::BoolError> {
        let mut plane_height = [0; crate::VIDEO_MAX_PLANES];

        unsafe {
            glib::glib_result_from_gboolean!(
                gst_video_sys::gst_video_meta_get_plane_height(
                    &self.0 as *const _ as usize as *mut _,
                    plane_height.as_mut_ptr(),
                ),
                "Failed to get plane height"
            )?;
        }

        Ok(plane_height)
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    pub fn set_alignment(
        &mut self,
        alignment: &crate::VideoAlignment,
    ) -> Result<(), glib::BoolError> {
        unsafe {
            glib::glib_result_from_gboolean!(
                gst_video_sys::gst_video_meta_set_alignment(&mut self.0, alignment.0),
                "Failed to set alignment on VideoMeta"
            )
        }
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

#[repr(transparent)]
pub struct VideoCropMeta(gst_video_sys::GstVideoCropMeta);

unsafe impl Send for VideoCropMeta {}
unsafe impl Sync for VideoCropMeta {}

impl VideoCropMeta {
    pub fn add(
        buffer: &mut gst::BufferRef,
        rect: (u32, u32, u32, u32),
    ) -> gst::MetaRefMut<Self, gst::meta::Standalone> {
        skip_assert_initialized!();
        unsafe {
            let meta = gst_sys::gst_buffer_add_meta(
                buffer.as_mut_ptr(),
                gst_video_sys::gst_video_crop_meta_get_info(),
                ptr::null_mut(),
            ) as *mut gst_video_sys::GstVideoCropMeta;

            {
                let meta = &mut *meta;
                meta.x = rect.0;
                meta.y = rect.1;
                meta.width = rect.2;
                meta.height = rect.3;
            }

            Self::from_mut_ptr(buffer, meta)
        }
    }

    pub fn get_rect(&self) -> (u32, u32, u32, u32) {
        (self.0.x, self.0.y, self.0.width, self.0.height)
    }

    pub fn set_rect(&mut self, rect: (u32, u32, u32, u32)) {
        self.0.x = rect.0;
        self.0.y = rect.1;
        self.0.width = rect.2;
        self.0.height = rect.3;
    }
}

unsafe impl MetaAPI for VideoCropMeta {
    type GstType = gst_video_sys::GstVideoCropMeta;

    fn get_meta_api() -> glib::Type {
        unsafe { from_glib(gst_video_sys::gst_video_crop_meta_api_get_type()) }
    }
}

impl fmt::Debug for VideoCropMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoCropMeta")
            .field("rect", &self.get_rect())
            .finish()
    }
}

#[repr(transparent)]
pub struct VideoRegionOfInterestMeta(gst_video_sys::GstVideoRegionOfInterestMeta);

unsafe impl Send for VideoRegionOfInterestMeta {}
unsafe impl Sync for VideoRegionOfInterestMeta {}

impl VideoRegionOfInterestMeta {
    pub fn add<'a>(
        buffer: &'a mut gst::BufferRef,
        roi_type: &str,
        rect: (u32, u32, u32, u32),
    ) -> gst::MetaRefMut<'a, Self, gst::meta::Standalone> {
        skip_assert_initialized!();
        unsafe {
            let meta = gst_video_sys::gst_buffer_add_video_region_of_interest_meta(
                buffer.as_mut_ptr(),
                roi_type.to_glib_none().0,
                rect.0,
                rect.1,
                rect.2,
                rect.3,
            );

            Self::from_mut_ptr(buffer, meta)
        }
    }

    pub fn get_rect(&self) -> (u32, u32, u32, u32) {
        (self.0.x, self.0.y, self.0.w, self.0.h)
    }

    pub fn get_id(&self) -> i32 {
        self.0.id
    }

    pub fn get_parent_id(&self) -> i32 {
        self.0.parent_id
    }

    pub fn get_roi_type<'a>(&self) -> &'a str {
        glib::Quark::from_glib(self.0.roi_type).to_string()
    }

    #[cfg(feature = "v1_14")]
    pub fn get_params(&self) -> ParamsIter {
        ParamsIter {
            _meta: self,
            list: self.0.params,
        }
    }

    #[cfg(feature = "v1_14")]
    pub fn get_param<'b>(&self, name: &'b str) -> Option<&gst::StructureRef> {
        self.get_params().find(|s| s.get_name() == name)
    }

    pub fn set_rect(&mut self, rect: (u32, u32, u32, u32)) {
        self.0.x = rect.0;
        self.0.y = rect.1;
        self.0.w = rect.2;
        self.0.h = rect.3;
    }

    pub fn set_id(&mut self, id: i32) {
        self.0.id = id
    }

    pub fn set_parent_id(&mut self, id: i32) {
        self.0.parent_id = id
    }

    #[cfg(feature = "v1_14")]
    pub fn add_param(&mut self, s: gst::Structure) {
        unsafe {
            gst_video_sys::gst_video_region_of_interest_meta_add_param(&mut self.0, s.into_ptr());
        }
    }
}

#[cfg(feature = "v1_14")]
pub struct ParamsIter<'a> {
    _meta: &'a VideoRegionOfInterestMeta,
    list: *const glib_sys::GList,
}

#[cfg(feature = "v1_14")]
impl<'a> Iterator for ParamsIter<'a> {
    type Item = &'a gst::StructureRef;

    fn next(&mut self) -> Option<&'a gst::StructureRef> {
        if self.list.is_null() {
            return None;
        }

        unsafe {
            let data = (*self.list).data;
            assert!(!data.is_null());
            self.list = (*self.list).next;

            let s = gst::StructureRef::from_glib_borrow(data as *const gst_sys::GstStructure);

            Some(s)
        }
    }
}

unsafe impl MetaAPI for VideoRegionOfInterestMeta {
    type GstType = gst_video_sys::GstVideoRegionOfInterestMeta;

    fn get_meta_api() -> glib::Type {
        unsafe { from_glib(gst_video_sys::gst_video_region_of_interest_meta_api_get_type()) }
    }
}

impl fmt::Debug for VideoRegionOfInterestMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut f = f.debug_struct("VideoRegionOfInterestMeta");

        f.field("roi_type", &self.get_roi_type())
            .field("rect", &self.get_rect())
            .field("id", &self.get_id())
            .field("parent_id", &self.get_parent_id());

        #[cfg(feature = "v1_14")]
        {
            f.field("params", &self.get_params().collect::<Vec<_>>());
        }

        f.finish()
    }
}

#[repr(transparent)]
pub struct VideoAffineTransformationMeta(gst_video_sys::GstVideoAffineTransformationMeta);

unsafe impl Send for VideoAffineTransformationMeta {}
unsafe impl Sync for VideoAffineTransformationMeta {}

impl VideoAffineTransformationMeta {
    pub fn add<'a>(
        buffer: &'a mut gst::BufferRef,
        matrix: Option<&[f32; 16]>,
    ) -> gst::MetaRefMut<'a, Self, gst::meta::Standalone> {
        skip_assert_initialized!();
        unsafe {
            let meta = gst_sys::gst_buffer_add_meta(
                buffer.as_mut_ptr(),
                gst_video_sys::gst_video_affine_transformation_meta_get_info(),
                ptr::null_mut(),
            ) as *mut gst_video_sys::GstVideoAffineTransformationMeta;

            if let Some(matrix) = matrix {
                let meta = &mut *meta;
                meta.matrix.copy_from_slice(matrix);
            }

            Self::from_mut_ptr(buffer, meta)
        }
    }

    pub fn get_matrix(&self) -> &[f32; 16] {
        &self.0.matrix
    }

    pub fn set_matrix(&mut self, matrix: &[f32; 16]) {
        self.0.matrix.copy_from_slice(matrix);
    }

    pub fn apply_matrix(&mut self, matrix: &[f32; 16]) {
        unsafe {
            gst_video_sys::gst_video_affine_transformation_meta_apply_matrix(&mut self.0, matrix);
        }
    }
}

unsafe impl MetaAPI for VideoAffineTransformationMeta {
    type GstType = gst_video_sys::GstVideoAffineTransformationMeta;

    fn get_meta_api() -> glib::Type {
        unsafe { from_glib(gst_video_sys::gst_video_affine_transformation_meta_api_get_type()) }
    }
}

impl fmt::Debug for VideoAffineTransformationMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoAffineTransformationMeta")
            .field("matrix", &self.get_matrix())
            .finish()
    }
}

#[repr(transparent)]
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
#[repr(transparent)]
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
#[repr(transparent)]
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
#[repr(transparent)]
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
                ::VideoFrameFlags::empty(),
                ::VideoFormat::Argb,
                320,
                240,
            )
            .unwrap();
            assert_eq!(meta.get_id(), 0);
            assert_eq!(meta.get_flags(), ::VideoFrameFlags::empty());
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
            assert_eq!(meta.get_flags(), ::VideoFrameFlags::empty());
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
                ::VideoFrameFlags::empty(),
                ::VideoFormat::Argb,
                320,
                240,
                &[0],
                &[320 * 4],
            )
            .unwrap();
            assert_eq!(meta.get_id(), 0);
            assert_eq!(meta.get_flags(), ::VideoFrameFlags::empty());
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
            assert_eq!(meta.get_flags(), ::VideoFrameFlags::empty());
            assert_eq!(meta.get_format(), ::VideoFormat::Argb);
            assert_eq!(meta.get_width(), 320);
            assert_eq!(meta.get_height(), 240);
            assert_eq!(meta.get_n_planes(), 1);
            assert_eq!(meta.get_offset(), &[0]);
            assert_eq!(meta.get_stride(), &[320 * 4]);
        }
    }

    #[test]
    #[cfg(feature = "v1_18")]
    fn test_vide_meta_alignment() {
        gst::init().unwrap();

        let mut buffer = gst::Buffer::with_size(115200).unwrap();
        let meta = VideoMeta::add(
            buffer.get_mut().unwrap(),
            crate::VideoFrameFlags::empty(),
            crate::VideoFormat::Nv12,
            320,
            240,
        )
        .unwrap();

        let alig = meta.get_alignment();
        assert_eq!(alig, crate::VideoAlignment::new(0, 0, 0, 0, &[0, 0, 0, 0]));

        assert_eq!(meta.get_plane_size().unwrap(), [76800, 38400, 0, 0]);
        assert_eq!(meta.get_plane_height().unwrap(), [240, 120, 0, 0]);

        /* horizontal padding */
        let mut info = crate::VideoInfo::builder(crate::VideoFormat::Nv12, 320, 240)
            .build()
            .expect("Failed to create VideoInfo");
        let mut alig = crate::VideoAlignment::new(0, 0, 2, 6, &[0, 0, 0, 0]);
        assert!(info.align(&mut alig));

        let mut meta = VideoMeta::add_full(
            buffer.get_mut().unwrap(),
            crate::VideoFrameFlags::empty(),
            crate::VideoFormat::Nv12,
            info.width(),
            info.height(),
            info.offset(),
            info.stride(),
        )
        .unwrap();
        meta.set_alignment(&alig).unwrap();

        let alig = meta.get_alignment();
        assert_eq!(alig, crate::VideoAlignment::new(0, 0, 2, 6, &[0, 0, 0, 0]));

        assert_eq!(meta.get_plane_size().unwrap(), [78720, 39360, 0, 0]);
        assert_eq!(meta.get_plane_height().unwrap(), [240, 120, 0, 0]);

        /* vertical alignment */
        let mut info = crate::VideoInfo::builder(crate::VideoFormat::Nv12, 320, 240)
            .build()
            .expect("Failed to create VideoInfo");
        let mut alig = crate::VideoAlignment::new(2, 6, 0, 0, &[0, 0, 0, 0]);
        assert!(info.align(&mut alig));

        let mut meta = VideoMeta::add_full(
            buffer.get_mut().unwrap(),
            crate::VideoFrameFlags::empty(),
            crate::VideoFormat::Nv12,
            info.width(),
            info.height(),
            info.offset(),
            info.stride(),
        )
        .unwrap();
        meta.set_alignment(&alig).unwrap();

        let alig = meta.get_alignment();
        assert_eq!(alig, crate::VideoAlignment::new(2, 6, 0, 0, &[0, 0, 0, 0]));

        assert_eq!(meta.get_plane_size().unwrap(), [79360, 39680, 0, 0]);
        assert_eq!(meta.get_plane_height().unwrap(), [248, 124, 0, 0]);
    }
}
