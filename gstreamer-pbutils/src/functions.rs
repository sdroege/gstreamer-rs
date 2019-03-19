// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib;
use glib::translate::*;
use gst;
use gst::MiniObject;
use gst_pbutils_sys;
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
        glib_result_from_gboolean!(
            gst_pbutils_sys::gst_pb_utils_add_codec_description_to_tag_list(
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
        glib_result_from_gboolean!(
            gst_pbutils_sys::gst_pb_utils_add_codec_description_to_tag_list(
                taglist.as_mut_ptr(),
                ptr::null_mut(),
                caps.as_ptr(),
            ),
            "Failed to find codec description",
        )
    }
}

pub fn pb_utils_get_encoder_description(caps: &gst::CapsRef) -> Option<String> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(gst_pbutils_sys::gst_pb_utils_get_encoder_description(
            caps.as_ptr(),
        ))
    }
}

pub fn pb_utils_get_decoder_description(caps: &gst::CapsRef) -> Option<String> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(gst_pbutils_sys::gst_pb_utils_get_decoder_description(
            caps.as_ptr(),
        ))
    }
}

pub fn pb_utils_get_codec_description(caps: &gst::CapsRef) -> Option<String> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(gst_pbutils_sys::gst_pb_utils_get_codec_description(
            caps.as_ptr(),
        ))
    }
}
