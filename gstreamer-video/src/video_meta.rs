// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;
use std::ptr;

use glib::translate::{from_glib, from_glib_none, FromGlib, IntoGlib, ToGlibPtr};
use gst::prelude::*;

#[repr(transparent)]
pub struct VideoMeta(ffi::GstVideoMeta);

unsafe impl Send for VideoMeta {}
unsafe impl Sync for VideoMeta {}

impl VideoMeta {
    pub fn add(
        buffer: &mut gst::BufferRef,
        flags: crate::VideoFrameFlags,
        format: crate::VideoFormat,
        width: u32,
        height: u32,
    ) -> Result<gst::MetaRefMut<Self, gst::meta::Standalone>, glib::BoolError> {
        skip_assert_initialized!();

        if format == crate::VideoFormat::Unknown || format == crate::VideoFormat::Encoded {
            return Err(glib::bool_error!("Unsupported video format {}", format));
        }

        let info = crate::VideoInfo::builder(format, width, height).build()?;

        if !info.is_valid() {
            return Err(glib::bool_error!("Invalid video info"));
        }

        if buffer.size() < info.size() {
            return Err(glib::bool_error!(
                "Buffer smaller than required frame size ({} < {})",
                buffer.size(),
                info.size()
            ));
        }

        unsafe {
            let meta = ffi::gst_buffer_add_video_meta(
                buffer.as_mut_ptr(),
                flags.into_glib(),
                format.into_glib(),
                width,
                height,
            );

            if meta.is_null() {
                return Err(glib::bool_error!("Failed to add video meta"));
            }

            Ok(Self::from_mut_ptr(buffer, meta))
        }
    }

    pub fn add_full<'a>(
        buffer: &'a mut gst::BufferRef,
        flags: crate::VideoFrameFlags,
        format: crate::VideoFormat,
        width: u32,
        height: u32,
        offset: &[usize],
        stride: &[i32],
    ) -> Result<gst::MetaRefMut<'a, Self, gst::meta::Standalone>, glib::BoolError> {
        skip_assert_initialized!();

        if format == crate::VideoFormat::Unknown || format == crate::VideoFormat::Encoded {
            return Err(glib::bool_error!("Unsupported video format {}", format));
        }

        let n_planes = offset.len() as u32;
        let info = crate::VideoInfo::builder(format, width, height)
            .offset(offset)
            .stride(stride)
            .build()?;

        if !info.is_valid() {
            return Err(glib::bool_error!("Invalid video info"));
        }

        if buffer.size() < info.size() {
            return Err(glib::bool_error!(
                "Buffer smaller than required frame size ({} < {})",
                buffer.size(),
                info.size()
            ));
        }

        unsafe {
            let meta = ffi::gst_buffer_add_video_meta_full(
                buffer.as_mut_ptr(),
                flags.into_glib(),
                format.into_glib(),
                width,
                height,
                n_planes,
                offset.as_ptr() as *mut _,
                stride.as_ptr() as *mut _,
            );

            if meta.is_null() {
                return Err(glib::bool_error!("Failed to add video meta"));
            }

            Ok(Self::from_mut_ptr(buffer, meta))
        }
    }

    pub fn flags(&self) -> crate::VideoFrameFlags {
        unsafe { from_glib(self.0.flags) }
    }

    pub fn format(&self) -> crate::VideoFormat {
        unsafe { from_glib(self.0.format) }
    }

    pub fn id(&self) -> i32 {
        self.0.id
    }

    pub fn width(&self) -> u32 {
        self.0.width
    }

    pub fn height(&self) -> u32 {
        self.0.height
    }

    pub fn n_planes(&self) -> u32 {
        self.0.n_planes
    }

    pub fn offset(&self) -> &[usize] {
        &self.0.offset[0..(self.0.n_planes as usize)]
    }

    pub fn stride(&self) -> &[i32] {
        &self.0.stride[0..(self.0.n_planes as usize)]
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    pub fn alignment(&self) -> crate::VideoAlignment {
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
    pub fn plane_size(&self) -> Result<[usize; crate::VIDEO_MAX_PLANES], glib::BoolError> {
        let mut plane_size = [0; crate::VIDEO_MAX_PLANES];

        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_video_meta_get_plane_size(
                    &self.0 as *const _ as usize as *mut _,
                    &mut plane_size,
                ),
                "Failed to get plane size"
            )?;
        }

        Ok(plane_size)
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    pub fn plane_height(&self) -> Result<[u32; crate::VIDEO_MAX_PLANES], glib::BoolError> {
        let mut plane_height = [0; crate::VIDEO_MAX_PLANES];

        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_video_meta_get_plane_height(
                    &self.0 as *const _ as usize as *mut _,
                    &mut plane_height,
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
            glib::result_from_gboolean!(
                ffi::gst_video_meta_set_alignment(&mut self.0, alignment.0),
                "Failed to set alignment on VideoMeta"
            )
        }
    }
}

unsafe impl MetaAPI for VideoMeta {
    type GstType = ffi::GstVideoMeta;

    fn meta_api() -> glib::Type {
        unsafe { from_glib(ffi::gst_video_meta_api_get_type()) }
    }
}

impl fmt::Debug for VideoMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoMeta")
            .field("id", &self.id())
            .field("flags", &self.flags())
            .field("format", &self.format())
            .field("width", &self.width())
            .field("height", &self.height())
            .field("n_planes", &self.n_planes())
            .field("offset", &self.offset())
            .field("stride", &self.stride())
            .finish()
    }
}

#[repr(transparent)]
pub struct VideoCropMeta(ffi::GstVideoCropMeta);

unsafe impl Send for VideoCropMeta {}
unsafe impl Sync for VideoCropMeta {}

impl VideoCropMeta {
    pub fn add(
        buffer: &mut gst::BufferRef,
        rect: (u32, u32, u32, u32),
    ) -> gst::MetaRefMut<Self, gst::meta::Standalone> {
        skip_assert_initialized!();
        unsafe {
            let meta = gst::ffi::gst_buffer_add_meta(
                buffer.as_mut_ptr(),
                ffi::gst_video_crop_meta_get_info(),
                ptr::null_mut(),
            ) as *mut ffi::GstVideoCropMeta;

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

    pub fn rect(&self) -> (u32, u32, u32, u32) {
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
    type GstType = ffi::GstVideoCropMeta;

    fn meta_api() -> glib::Type {
        unsafe { from_glib(ffi::gst_video_crop_meta_api_get_type()) }
    }
}

impl fmt::Debug for VideoCropMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoCropMeta")
            .field("rect", &self.rect())
            .finish()
    }
}

#[repr(transparent)]
pub struct VideoRegionOfInterestMeta(ffi::GstVideoRegionOfInterestMeta);

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
            let meta = ffi::gst_buffer_add_video_region_of_interest_meta(
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

    pub fn rect(&self) -> (u32, u32, u32, u32) {
        (self.0.x, self.0.y, self.0.w, self.0.h)
    }

    pub fn id(&self) -> i32 {
        self.0.id
    }

    pub fn parent_id(&self) -> i32 {
        self.0.parent_id
    }

    pub fn roi_type<'a>(&self) -> &'a str {
        unsafe { glib::Quark::from_glib(self.0.roi_type).to_string() }
    }

    #[cfg(feature = "v1_14")]
    pub fn params(&self) -> ParamsIter {
        ParamsIter {
            _meta: self,
            list: self.0.params,
        }
    }

    #[cfg(feature = "v1_14")]
    pub fn param<'b>(&self, name: &'b str) -> Option<&gst::StructureRef> {
        self.params().find(|s| s.name() == name)
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
            ffi::gst_video_region_of_interest_meta_add_param(&mut self.0, s.into_ptr());
        }
    }
}

#[cfg(feature = "v1_14")]
pub struct ParamsIter<'a> {
    _meta: &'a VideoRegionOfInterestMeta,
    list: *const glib::ffi::GList,
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

            let s = gst::StructureRef::from_glib_borrow(data as *const gst::ffi::GstStructure);

            Some(s)
        }
    }
}

unsafe impl MetaAPI for VideoRegionOfInterestMeta {
    type GstType = ffi::GstVideoRegionOfInterestMeta;

    fn meta_api() -> glib::Type {
        unsafe { from_glib(ffi::gst_video_region_of_interest_meta_api_get_type()) }
    }
}

impl fmt::Debug for VideoRegionOfInterestMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut f = f.debug_struct("VideoRegionOfInterestMeta");

        f.field("roi_type", &self.roi_type())
            .field("rect", &self.rect())
            .field("id", &self.id())
            .field("parent_id", &self.parent_id());

        #[cfg(feature = "v1_14")]
        {
            f.field("params", &self.params().collect::<Vec<_>>());
        }

        f.finish()
    }
}

#[repr(transparent)]
pub struct VideoAffineTransformationMeta(ffi::GstVideoAffineTransformationMeta);

unsafe impl Send for VideoAffineTransformationMeta {}
unsafe impl Sync for VideoAffineTransformationMeta {}

impl VideoAffineTransformationMeta {
    pub fn add<'a>(
        buffer: &'a mut gst::BufferRef,
        matrix: Option<&[f32; 16]>,
    ) -> gst::MetaRefMut<'a, Self, gst::meta::Standalone> {
        skip_assert_initialized!();
        unsafe {
            let meta = gst::ffi::gst_buffer_add_meta(
                buffer.as_mut_ptr(),
                ffi::gst_video_affine_transformation_meta_get_info(),
                ptr::null_mut(),
            ) as *mut ffi::GstVideoAffineTransformationMeta;

            if let Some(matrix) = matrix {
                let meta = &mut *meta;
                meta.matrix.copy_from_slice(matrix);
            }

            Self::from_mut_ptr(buffer, meta)
        }
    }

    pub fn matrix(&self) -> &[f32; 16] {
        &self.0.matrix
    }

    pub fn set_matrix(&mut self, matrix: &[f32; 16]) {
        self.0.matrix.copy_from_slice(matrix);
    }

    pub fn apply_matrix(&mut self, matrix: &[f32; 16]) {
        unsafe {
            ffi::gst_video_affine_transformation_meta_apply_matrix(&mut self.0, matrix);
        }
    }
}

unsafe impl MetaAPI for VideoAffineTransformationMeta {
    type GstType = ffi::GstVideoAffineTransformationMeta;

    fn meta_api() -> glib::Type {
        unsafe { from_glib(ffi::gst_video_affine_transformation_meta_api_get_type()) }
    }
}

impl fmt::Debug for VideoAffineTransformationMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoAffineTransformationMeta")
            .field("matrix", &self.matrix())
            .finish()
    }
}

#[repr(transparent)]
pub struct VideoOverlayCompositionMeta(ffi::GstVideoOverlayCompositionMeta);

unsafe impl Send for VideoOverlayCompositionMeta {}
unsafe impl Sync for VideoOverlayCompositionMeta {}

impl VideoOverlayCompositionMeta {
    pub fn add<'a>(
        buffer: &'a mut gst::BufferRef,
        overlay: &crate::VideoOverlayComposition,
    ) -> gst::MetaRefMut<'a, Self, gst::meta::Standalone> {
        skip_assert_initialized!();
        unsafe {
            let meta = ffi::gst_buffer_add_video_overlay_composition_meta(
                buffer.as_mut_ptr(),
                overlay.as_mut_ptr(),
            );

            Self::from_mut_ptr(buffer, meta)
        }
    }

    pub fn overlay(&self) -> &crate::VideoOverlayCompositionRef {
        unsafe { crate::VideoOverlayCompositionRef::from_ptr(self.0.overlay) }
    }

    pub fn overlay_owned(&self) -> crate::VideoOverlayComposition {
        unsafe { from_glib_none(self.overlay().as_ptr()) }
    }

    pub fn set_overlay(&mut self, overlay: &crate::VideoOverlayComposition) {
        #![allow(clippy::cast_ptr_alignment)]
        unsafe {
            gst::ffi::gst_mini_object_unref(self.0.overlay as *mut _);
            self.0.overlay =
                gst::ffi::gst_mini_object_ref(overlay.as_mut_ptr() as *mut _) as *mut _;
        }
    }
}

unsafe impl MetaAPI for VideoOverlayCompositionMeta {
    type GstType = ffi::GstVideoOverlayCompositionMeta;

    fn meta_api() -> glib::Type {
        unsafe { from_glib(ffi::gst_video_overlay_composition_meta_api_get_type()) }
    }
}

impl fmt::Debug for VideoOverlayCompositionMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoOverlayCompositionMeta")
            .field("overlay", &self.overlay())
            .finish()
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
#[repr(transparent)]
pub struct VideoCaptionMeta(ffi::GstVideoCaptionMeta);

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
unsafe impl Send for VideoCaptionMeta {}
#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
unsafe impl Sync for VideoCaptionMeta {}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
impl VideoCaptionMeta {
    pub fn add<'a>(
        buffer: &'a mut gst::BufferRef,
        caption_type: crate::VideoCaptionType,
        data: &[u8],
    ) -> gst::MetaRefMut<'a, Self, gst::meta::Standalone> {
        skip_assert_initialized!();
        assert!(!data.is_empty());
        unsafe {
            let meta = ffi::gst_buffer_add_video_caption_meta(
                buffer.as_mut_ptr(),
                caption_type.into_glib(),
                data.as_ptr(),
                data.len(),
            );

            Self::from_mut_ptr(buffer, meta)
        }
    }

    pub fn caption_type(&self) -> crate::VideoCaptionType {
        unsafe { from_glib(self.0.caption_type) }
    }

    pub fn data(&self) -> &[u8] {
        unsafe {
            use std::slice;

            slice::from_raw_parts(self.0.data, self.0.size)
        }
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
unsafe impl MetaAPI for VideoCaptionMeta {
    type GstType = ffi::GstVideoCaptionMeta;

    fn meta_api() -> glib::Type {
        unsafe { from_glib(ffi::gst_video_caption_meta_api_get_type()) }
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
impl fmt::Debug for VideoCaptionMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoCaptionMeta")
            .field("caption_type", &self.caption_type())
            .field("data", &self.data())
            .finish()
    }
}

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
#[repr(transparent)]
pub struct VideoAFDMeta(ffi::GstVideoAFDMeta);

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
unsafe impl Send for VideoAFDMeta {}
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
unsafe impl Sync for VideoAFDMeta {}

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
impl VideoAFDMeta {
    pub fn add(
        buffer: &mut gst::BufferRef,
        field: u8,
        spec: crate::VideoAFDSpec,
        afd: crate::VideoAFDValue,
    ) -> gst::MetaRefMut<Self, gst::meta::Standalone> {
        skip_assert_initialized!();

        unsafe {
            let meta = ffi::gst_buffer_add_video_afd_meta(
                buffer.as_mut_ptr(),
                field,
                spec.into_glib(),
                afd.into_glib(),
            );

            Self::from_mut_ptr(buffer, meta)
        }
    }

    pub fn field(&self) -> u8 {
        self.0.field
    }

    pub fn spec(&self) -> crate::VideoAFDSpec {
        unsafe { from_glib(self.0.spec) }
    }

    pub fn afd(&self) -> crate::VideoAFDValue {
        unsafe { from_glib(self.0.afd) }
    }
}

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
unsafe impl MetaAPI for VideoAFDMeta {
    type GstType = ffi::GstVideoAFDMeta;

    fn meta_api() -> glib::Type {
        unsafe { from_glib(ffi::gst_video_afd_meta_api_get_type()) }
    }
}

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
impl fmt::Debug for VideoAFDMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoAFDMeta")
            .field("field", &self.field())
            .field("spec", &self.spec())
            .field("afd", &self.afd())
            .finish()
    }
}

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
#[repr(transparent)]
pub struct VideoBarMeta(ffi::GstVideoBarMeta);

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
unsafe impl Send for VideoBarMeta {}
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
unsafe impl Sync for VideoBarMeta {}

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
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
            let meta = ffi::gst_buffer_add_video_bar_meta(
                buffer.as_mut_ptr(),
                field,
                is_letterbox.into_glib(),
                bar_data1,
                bar_data2,
            );

            Self::from_mut_ptr(buffer, meta)
        }
    }

    pub fn field(&self) -> u8 {
        self.0.field
    }

    pub fn is_letterbox(&self) -> bool {
        unsafe { from_glib(self.0.is_letterbox) }
    }

    pub fn bar_data1(&self) -> u32 {
        self.0.bar_data1
    }

    pub fn bar_data2(&self) -> u32 {
        self.0.bar_data2
    }
}

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
unsafe impl MetaAPI for VideoBarMeta {
    type GstType = ffi::GstVideoBarMeta;

    fn meta_api() -> glib::Type {
        unsafe { from_glib(ffi::gst_video_bar_meta_api_get_type()) }
    }
}

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
impl fmt::Debug for VideoBarMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoBarMeta")
            .field("field", &self.field())
            .field("is_letterbox", &self.is_letterbox())
            .field("bar_data1", &self.bar_data1())
            .field("bar_data2", &self.bar_data2())
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
                crate::VideoFrameFlags::empty(),
                crate::VideoFormat::Argb,
                320,
                240,
            )
            .unwrap();
            assert_eq!(meta.id(), 0);
            assert_eq!(meta.flags(), crate::VideoFrameFlags::empty());
            assert_eq!(meta.format(), crate::VideoFormat::Argb);
            assert_eq!(meta.width(), 320);
            assert_eq!(meta.height(), 240);
            assert_eq!(meta.n_planes(), 1);
            assert_eq!(meta.offset(), &[0]);
            assert_eq!(meta.stride(), &[320 * 4]);
        }

        {
            let meta = buffer.meta::<VideoMeta>().unwrap();
            assert_eq!(meta.id(), 0);
            assert_eq!(meta.flags(), crate::VideoFrameFlags::empty());
            assert_eq!(meta.format(), crate::VideoFormat::Argb);
            assert_eq!(meta.width(), 320);
            assert_eq!(meta.height(), 240);
            assert_eq!(meta.n_planes(), 1);
            assert_eq!(meta.offset(), &[0]);
            assert_eq!(meta.stride(), &[320 * 4]);
        }
    }

    #[test]
    fn test_add_full_get_meta() {
        gst::init().unwrap();

        let mut buffer = gst::Buffer::with_size(320 * 240 * 4).unwrap();
        {
            let meta = VideoMeta::add_full(
                buffer.get_mut().unwrap(),
                crate::VideoFrameFlags::empty(),
                crate::VideoFormat::Argb,
                320,
                240,
                &[0],
                &[320 * 4],
            )
            .unwrap();
            assert_eq!(meta.id(), 0);
            assert_eq!(meta.flags(), crate::VideoFrameFlags::empty());
            assert_eq!(meta.format(), crate::VideoFormat::Argb);
            assert_eq!(meta.width(), 320);
            assert_eq!(meta.height(), 240);
            assert_eq!(meta.n_planes(), 1);
            assert_eq!(meta.offset(), &[0]);
            assert_eq!(meta.stride(), &[320 * 4]);
        }

        {
            let meta = buffer.meta::<VideoMeta>().unwrap();
            assert_eq!(meta.id(), 0);
            assert_eq!(meta.flags(), crate::VideoFrameFlags::empty());
            assert_eq!(meta.format(), crate::VideoFormat::Argb);
            assert_eq!(meta.width(), 320);
            assert_eq!(meta.height(), 240);
            assert_eq!(meta.n_planes(), 1);
            assert_eq!(meta.offset(), &[0]);
            assert_eq!(meta.stride(), &[320 * 4]);
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

        let alig = meta.alignment();
        assert_eq!(alig, crate::VideoAlignment::new(0, 0, 0, 0, &[0, 0, 0, 0]));

        assert_eq!(meta.plane_size().unwrap(), [76800, 38400, 0, 0]);
        assert_eq!(meta.plane_height().unwrap(), [240, 120, 0, 0]);

        /* horizontal padding */
        let mut info = crate::VideoInfo::builder(crate::VideoFormat::Nv12, 320, 240)
            .build()
            .expect("Failed to create VideoInfo");
        let mut alig = crate::VideoAlignment::new(0, 0, 2, 6, &[0, 0, 0, 0]);
        info.align(&mut alig).unwrap();

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

        let alig = meta.alignment();
        assert_eq!(alig, crate::VideoAlignment::new(0, 0, 2, 6, &[0, 0, 0, 0]));

        assert_eq!(meta.plane_size().unwrap(), [78720, 39360, 0, 0]);
        assert_eq!(meta.plane_height().unwrap(), [240, 120, 0, 0]);

        /* vertical alignment */
        let mut info = crate::VideoInfo::builder(crate::VideoFormat::Nv12, 320, 240)
            .build()
            .expect("Failed to create VideoInfo");
        let mut alig = crate::VideoAlignment::new(2, 6, 0, 0, &[0, 0, 0, 0]);
        info.align(&mut alig).unwrap();

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

        let alig = meta.alignment();
        assert_eq!(alig, crate::VideoAlignment::new(2, 6, 0, 0, &[0, 0, 0, 0]));

        assert_eq!(meta.plane_size().unwrap(), [79360, 39680, 0, 0]);
        assert_eq!(meta.plane_height().unwrap(), [248, 124, 0, 0]);
    }
}
