// Take a look at the license at the top of the repository in the LICENSE file.

use std::{ffi::CStr, marker::PhantomData, mem};

use glib::translate::*;
use once_cell::sync::Lazy;

pub static BUFFER_POOL_OPTION_VIDEO_AFFINE_TRANSFORMATION_META: Lazy<&'static str> =
    Lazy::new(|| unsafe {
        CStr::from_ptr(ffi::GST_BUFFER_POOL_OPTION_VIDEO_AFFINE_TRANSFORMATION_META)
            .to_str()
            .unwrap()
    });
pub static BUFFER_POOL_OPTION_VIDEO_ALIGNMENT: Lazy<&'static str> = Lazy::new(|| unsafe {
    CStr::from_ptr(ffi::GST_BUFFER_POOL_OPTION_VIDEO_ALIGNMENT)
        .to_str()
        .unwrap()
});
pub static BUFFER_POOL_OPTION_VIDEO_GL_TEXTURE_UPLOAD_META: Lazy<&'static str> =
    Lazy::new(|| unsafe {
        CStr::from_ptr(ffi::GST_BUFFER_POOL_OPTION_VIDEO_GL_TEXTURE_UPLOAD_META)
            .to_str()
            .unwrap()
    });
pub static BUFFER_POOL_OPTION_VIDEO_META: Lazy<&'static str> = Lazy::new(|| unsafe {
    CStr::from_ptr(ffi::GST_BUFFER_POOL_OPTION_VIDEO_META)
        .to_str()
        .unwrap()
});

#[derive(Debug, Clone)]
#[doc(alias = "GstVideoAlignment")]
pub struct VideoAlignment(pub(crate) ffi::GstVideoAlignment);

impl VideoAlignment {
    #[doc(alias = "get_padding_top")]
    pub fn padding_top(&self) -> u32 {
        self.0.padding_top
    }
    #[doc(alias = "get_padding_bottom")]
    pub fn padding_bottom(&self) -> u32 {
        self.0.padding_bottom
    }
    #[doc(alias = "get_padding_left")]
    pub fn padding_left(&self) -> u32 {
        self.0.padding_left
    }
    #[doc(alias = "get_padding_right")]
    pub fn padding_right(&self) -> u32 {
        self.0.padding_right
    }
    #[doc(alias = "get_stride_align")]
    pub fn stride_align(&self) -> &[u32; ffi::GST_VIDEO_MAX_PLANES as usize] {
        &self.0.stride_align
    }

    pub fn new(
        padding_top: u32,
        padding_bottom: u32,
        padding_left: u32,
        padding_right: u32,
        stride_align: &[u32; ffi::GST_VIDEO_MAX_PLANES as usize],
    ) -> Self {
        assert_initialized_main_thread!();

        let videoalignment = ffi::GstVideoAlignment {
            padding_top,
            padding_bottom,
            padding_left,
            padding_right,
            stride_align: *stride_align,
        };

        Self(videoalignment)
    }
}

impl PartialEq for VideoAlignment {
    fn eq(&self, other: &Self) -> bool {
        self.padding_top() == other.padding_top()
            && self.padding_bottom() == other.padding_bottom()
            && self.padding_left() == other.padding_left()
            && self.padding_right() == other.padding_right()
            && self.stride_align() == other.stride_align()
    }
}

impl Eq for VideoAlignment {}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::GstVideoAlignment> for VideoAlignment {
    type Storage = PhantomData<&'a Self>;

    fn to_glib_none(&'a self) -> Stash<*const ffi::GstVideoAlignment, Self> {
        Stash(&self.0, PhantomData)
    }
}

pub trait VideoBufferPoolConfig {
    #[doc(alias = "get_video_alignment")]
    fn video_alignment(&self) -> Option<VideoAlignment>;

    fn set_video_alignment(&mut self, align: &VideoAlignment);
}

impl VideoBufferPoolConfig for gst::BufferPoolConfigRef {
    #[doc(alias = "gst_buffer_pool_config_get_video_alignment")]
    fn video_alignment(&self) -> Option<VideoAlignment> {
        unsafe {
            let mut alignment = mem::MaybeUninit::zeroed();
            let ret = from_glib(ffi::gst_buffer_pool_config_get_video_alignment(
                self.as_ref().as_mut_ptr(),
                alignment.as_mut_ptr(),
            ));
            if ret {
                Some(VideoAlignment(alignment.assume_init()))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gst_buffer_pool_config_set_video_alignment")]
    fn set_video_alignment(&mut self, align: &VideoAlignment) {
        unsafe {
            ffi::gst_buffer_pool_config_set_video_alignment(
                self.as_mut().as_mut_ptr(),
                &align.0 as *const _ as *mut _,
            )
        }
    }
}
