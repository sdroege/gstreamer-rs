// Take a look at the license at the top of the repository in the LICENSE file.

use std::ffi::CStr;
use std::mem;

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
pub struct VideoAlignment(pub(crate) ffi::GstVideoAlignment);

impl VideoAlignment {
    pub fn get_padding_top(&self) -> u32 {
        self.0.padding_top
    }
    pub fn get_padding_bottom(&self) -> u32 {
        self.0.padding_bottom
    }
    pub fn get_padding_left(&self) -> u32 {
        self.0.padding_left
    }
    pub fn get_padding_right(&self) -> u32 {
        self.0.padding_right
    }
    pub fn get_stride_align(&self) -> &[u32; ffi::GST_VIDEO_MAX_PLANES as usize] {
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

        VideoAlignment(videoalignment)
    }
}

impl PartialEq for VideoAlignment {
    fn eq(&self, other: &VideoAlignment) -> bool {
        self.get_padding_top() == other.get_padding_top()
            && self.get_padding_bottom() == other.get_padding_bottom()
            && self.get_padding_left() == other.get_padding_left()
            && self.get_padding_right() == other.get_padding_right()
            && self.get_stride_align() == other.get_stride_align()
    }
}

impl Eq for VideoAlignment {}

pub trait VideoBufferPoolConfig {
    fn get_video_alignment(&self) -> Option<VideoAlignment>;

    fn set_video_alignment(&mut self, align: &VideoAlignment);
}

impl VideoBufferPoolConfig for gst::BufferPoolConfig {
    fn get_video_alignment(&self) -> Option<VideoAlignment> {
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

    fn set_video_alignment(&mut self, align: &VideoAlignment) {
        unsafe {
            ffi::gst_buffer_pool_config_set_video_alignment(
                self.as_mut().as_mut_ptr(),
                &align.0 as *const _ as *mut _,
            )
        }
    }
}
