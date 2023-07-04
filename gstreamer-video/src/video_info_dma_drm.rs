// Take a look at the license at the top of the repository in the LICENSE file.

use std::{fmt, marker::PhantomData, mem, ops, ptr, str};

use glib::translate::*;
use gst::prelude::*;

use crate::{VideoFormat, VideoInfo};

#[doc(alias = "gst_video_dma_drm_fourcc_from_format")]
pub fn dma_drm_fourcc_from_format(v: VideoFormat) -> Result<u32, glib::BoolError> {
    skip_assert_initialized!();
    unsafe {
        let res = ffi::gst_video_dma_drm_fourcc_from_format(v.into_glib());
        if res == 0 {
            Err(glib::bool_error!("Unsupported video format"))
        } else {
            Ok(res)
        }
    }
}

#[doc(alias = "gst_video_dma_drm_fourcc_to_format")]
pub fn dma_drm_fourcc_to_format(v: u32) -> Result<VideoFormat, glib::BoolError> {
    skip_assert_initialized!();
    unsafe {
        let res = ffi::gst_video_dma_drm_fourcc_to_format(v);
        if res == ffi::GST_VIDEO_FORMAT_UNKNOWN {
            Err(glib::bool_error!("Unsupported fourcc"))
        } else {
            Ok(from_glib(res))
        }
    }
}

#[doc(alias = "gst_video_dma_drm_fourcc_to_string")]
pub fn dma_drm_fourcc_to_string(fourcc: u32, modifier: u64) -> glib::GString {
    skip_assert_initialized!();
    unsafe {
        glib::GString::from_glib_full(ffi::gst_video_dma_drm_fourcc_to_string(fourcc, modifier))
    }
}

#[doc(alias = "gst_video_dma_drm_fourcc_from_string")]
pub fn dma_drm_fourcc_from_str(v: &str) -> Result<(u32, u64), glib::BoolError> {
    skip_assert_initialized!();
    unsafe {
        let mut modifier = mem::MaybeUninit::uninit();
        let res =
            ffi::gst_video_dma_drm_fourcc_from_string(v.to_glib_none().0, modifier.as_mut_ptr());
        if res == 0 {
            Err(glib::bool_error!("Can't parse fourcc string"))
        } else {
            Ok((res, modifier.assume_init()))
        }
    }
}

#[doc(alias = "GstVideoInfoDmaDrm")]
#[derive(Clone)]
#[repr(transparent)]
pub struct VideoInfoDmaDrm(pub(crate) ffi::GstVideoInfoDmaDrm);

impl ops::Deref for VideoInfoDmaDrm {
    type Target = VideoInfo;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(&self.0.vinfo as *const ffi::GstVideoInfo as *const VideoInfo) }
    }
}

impl fmt::Debug for VideoInfoDmaDrm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoInfoDmaDrm")
            .field("info", &**self)
            .field("drm_fourcc", &self.0.drm_fourcc)
            .field("drm_modifier", &self.0.drm_modifier)
            .finish()
    }
}

impl VideoInfoDmaDrm {
    pub fn new(info: VideoInfo, fourcc: u32, modifier: u64) -> VideoInfoDmaDrm {
        assert_initialized_main_thread!();

        VideoInfoDmaDrm(ffi::GstVideoInfoDmaDrm {
            vinfo: info.0,
            drm_fourcc: fourcc,
            drm_modifier: modifier,
            _gst_reserved: [0; 20],
        })
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        !self.0.vinfo.finfo.is_null()
            && self.0.vinfo.width > 0
            && self.0.vinfo.height > 0
            && self.0.vinfo.size > 0
    }

    #[doc(alias = "gst_video_info_dma_drm_from_caps")]
    pub fn from_caps(caps: &gst::CapsRef) -> Result<Self, glib::error::BoolError> {
        skip_assert_initialized!();

        unsafe {
            let mut info = mem::MaybeUninit::uninit();
            if from_glib(ffi::gst_video_info_dma_drm_from_caps(
                info.as_mut_ptr(),
                caps.as_ptr(),
            )) {
                Ok(Self(info.assume_init()))
            } else {
                Err(glib::bool_error!(
                    "Failed to create VideoInfoDmaDrm from caps"
                ))
            }
        }
    }

    #[doc(alias = "gst_video_info_dma_drm_to_caps")]
    pub fn to_caps(&self) -> Result<gst::Caps, glib::error::BoolError> {
        unsafe {
            let result = from_glib_full(ffi::gst_video_info_dma_drm_to_caps(mut_override(&self.0)));
            match result {
                Some(c) => Ok(c),
                None => Err(glib::bool_error!(
                    "Failed to create caps from VideoInfoDmaDrm"
                )),
            }
        }
    }

    #[doc(alias = "gst_video_info_dma_drm_from_video_info")]
    pub fn from_video_info(
        video_info: &crate::VideoInfo,
        modifier: u64,
    ) -> Result<Self, glib::error::BoolError> {
        skip_assert_initialized!();

        unsafe {
            let mut info = mem::MaybeUninit::uninit();
            if from_glib(ffi::gst_video_info_dma_drm_from_video_info(
                info.as_mut_ptr(),
                video_info.to_glib_none().0,
                modifier,
            )) {
                Ok(Self(info.assume_init()))
            } else {
                Err(glib::bool_error!(
                    "Failed to create VideoInfoDmaDrm from VideoInfo"
                ))
            }
        }
    }

    #[doc(alias = "gst_video_info_dma_drm_to_video_info")]
    pub fn to_video_info(&self) -> Result<crate::VideoInfo, glib::error::BoolError> {
        unsafe {
            let mut video_info = mem::MaybeUninit::uninit();
            if from_glib(ffi::gst_video_info_dma_drm_to_video_info(
                mut_override(&self.0),
                video_info.as_mut_ptr(),
            )) {
                Ok(crate::VideoInfo(video_info.assume_init()))
            } else {
                Err(glib::bool_error!(
                    "Failed to create VideoInfo from VideoInfoDmaDrm"
                ))
            }
        }
    }

    #[inline]
    pub fn fourcc(&self) -> u32 {
        self.0.drm_fourcc
    }

    #[inline]
    pub fn modifier(&self) -> u64 {
        self.0.drm_modifier
    }
}

impl PartialEq for VideoInfoDmaDrm {
    #[doc(alias = "gst_video_info_is_equal")]
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            from_glib(ffi::gst_video_info_is_equal(&self.0.vinfo, &other.0.vinfo))
                && self.0.drm_fourcc == other.0.drm_fourcc
                && self.0.drm_modifier == other.0.drm_modifier
        }
    }
}

impl Eq for VideoInfoDmaDrm {}

unsafe impl Send for VideoInfoDmaDrm {}
unsafe impl Sync for VideoInfoDmaDrm {}

impl glib::types::StaticType for VideoInfoDmaDrm {
    #[inline]
    fn static_type() -> glib::types::Type {
        unsafe { glib::translate::from_glib(ffi::gst_video_info_dma_drm_get_type()) }
    }
}

impl glib::value::ValueType for VideoInfoDmaDrm {
    type Type = Self;
}

#[doc(hidden)]
unsafe impl<'a> glib::value::FromValue<'a> for VideoInfoDmaDrm {
    type Checker = glib::value::GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib_none(glib::gobject_ffi::g_value_get_boxed(value.to_glib_none().0)
            as *mut ffi::GstVideoInfoDmaDrm)
    }
}

#[doc(hidden)]
impl glib::value::ToValue for VideoInfoDmaDrm {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                self.to_glib_none().0 as *mut _,
            )
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[doc(hidden)]
impl glib::value::ToValueOptional for VideoInfoDmaDrm {
    fn to_value_optional(s: Option<&Self>) -> glib::Value {
        skip_assert_initialized!();
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                s.to_glib_none().0 as *mut _,
            )
        }
        value
    }
}

#[doc(hidden)]
impl From<VideoInfoDmaDrm> for glib::Value {
    fn from(v: VideoInfoDmaDrm) -> glib::Value {
        skip_assert_initialized!();
        glib::value::ToValue::to_value(&v)
    }
}

#[doc(hidden)]
impl glib::translate::Uninitialized for VideoInfoDmaDrm {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

#[doc(hidden)]
impl glib::translate::GlibPtrDefault for VideoInfoDmaDrm {
    type GlibType = *mut ffi::GstVideoInfoDmaDrm;
}

#[doc(hidden)]
impl<'a> glib::translate::ToGlibPtr<'a, *const ffi::GstVideoInfoDmaDrm> for VideoInfoDmaDrm {
    type Storage = PhantomData<&'a Self>;

    #[inline]
    fn to_glib_none(&'a self) -> glib::translate::Stash<'a, *const ffi::GstVideoInfoDmaDrm, Self> {
        glib::translate::Stash(&self.0, PhantomData)
    }

    fn to_glib_full(&self) -> *const ffi::GstVideoInfoDmaDrm {
        unimplemented!()
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrNone<*const ffi::GstVideoInfoDmaDrm> for VideoInfoDmaDrm {
    #[inline]
    unsafe fn from_glib_none(ptr: *const ffi::GstVideoInfoDmaDrm) -> Self {
        Self(ptr::read(ptr))
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrNone<*mut ffi::GstVideoInfoDmaDrm> for VideoInfoDmaDrm {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::GstVideoInfoDmaDrm) -> Self {
        Self(ptr::read(ptr))
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrFull<*mut ffi::GstVideoInfoDmaDrm> for VideoInfoDmaDrm {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::GstVideoInfoDmaDrm) -> Self {
        let info = from_glib_none(ptr);
        glib::ffi::g_free(ptr as *mut _);
        info
    }
}
