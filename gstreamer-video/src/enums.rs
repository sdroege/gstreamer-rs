// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
use glib::translate::*;

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
use crate::VideoCaptionType;
#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
use crate::VideoOrientationMethod;

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
impl VideoCaptionType {
    #[doc(alias = "gst_video_caption_type_from_caps")]
    pub fn from_caps(caps: &gst::CapsRef) -> VideoCaptionType {
        assert_initialized_main_thread!();
        unsafe { from_glib(ffi::gst_video_caption_type_from_caps(caps.as_ptr())) }
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
impl VideoOrientationMethod {
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_video_orientation_from_tag")]
    pub fn from_tag(taglist: &gst::TagListRef) -> Option<VideoOrientationMethod> {
        assert_initialized_main_thread!();

        unsafe {
            use std::mem;

            let mut method = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_video_orientation_from_tag(
                mut_override(taglist.as_ptr()),
                method.as_mut_ptr(),
            ));
            if ret {
                Some(from_glib(method.assume_init()))
            } else {
                None
            }
        }
    }
}
