// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use std::ptr;

pub unsafe trait CodecTag<'a>: gst::Tag<'a, TagType = &'a str> {}

unsafe impl<'a> CodecTag<'a> for gst::tags::ContainerFormat {}
unsafe impl<'a> CodecTag<'a> for gst::tags::AudioCodec {}
unsafe impl<'a> CodecTag<'a> for gst::tags::VideoCodec {}
unsafe impl<'a> CodecTag<'a> for gst::tags::SubtitleCodec {}
unsafe impl<'a> CodecTag<'a> for gst::tags::Codec {}

pub fn pb_utils_add_codec_description_to_tag_list_for_tag<'a, T: CodecTag<'a>>(
    taglist: &mut gst::TagListRef,
    caps: &gst::CapsRef,
) -> Result<(), glib::BoolError> {
    assert_initialized_main_thread!();
    let codec_tag = T::tag_name();
    unsafe {
        glib::glib_result_from_gboolean!(
            ffi::gst_pb_utils_add_codec_description_to_tag_list(
                taglist.as_mut_ptr(),
                codec_tag.to_glib_none().0,
                caps.as_ptr(),
            ),
            "Failed to find codec description",
        )
    }
}

pub fn pb_utils_add_codec_description_to_tag_list(
    taglist: &mut gst::TagListRef,
    caps: &gst::CapsRef,
) -> Result<(), glib::BoolError> {
    assert_initialized_main_thread!();
    unsafe {
        glib::glib_result_from_gboolean!(
            ffi::gst_pb_utils_add_codec_description_to_tag_list(
                taglist.as_mut_ptr(),
                ptr::null_mut(),
                caps.as_ptr(),
            ),
            "Failed to find codec description",
        )
    }
}

pub fn pb_utils_get_encoder_description(
    caps: &gst::CapsRef,
) -> Result<glib::GString, glib::error::BoolError> {
    assert_initialized_main_thread!();
    unsafe {
        match from_glib_full(ffi::gst_pb_utils_get_encoder_description(caps.as_ptr())) {
            Some(s) => Ok(s),
            None => Err(glib::glib_bool_error!("Failed to get encoder description")),
        }
    }
}

pub fn pb_utils_get_decoder_description(
    caps: &gst::CapsRef,
) -> Result<glib::GString, glib::error::BoolError> {
    assert_initialized_main_thread!();
    unsafe {
        match from_glib_full(ffi::gst_pb_utils_get_decoder_description(caps.as_ptr())) {
            Some(s) => Ok(s),
            None => Err(glib::glib_bool_error!("Failed to get decoder description")),
        }
    }
}

pub fn pb_utils_get_codec_description(
    caps: &gst::CapsRef,
) -> Result<glib::GString, glib::error::BoolError> {
    assert_initialized_main_thread!();
    unsafe {
        match from_glib_full(ffi::gst_pb_utils_get_codec_description(caps.as_ptr())) {
            Some(s) => Ok(s),
            None => Err(glib::glib_bool_error!("Failed to get codec description")),
        }
    }
}
