// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use std::{mem, ptr};

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
        glib::result_from_gboolean!(
            ffi::gst_pb_utils_add_codec_description_to_tag_list(
                taglist.as_mut_ptr(),
                codec_tag.to_glib_none().0,
                caps.as_ptr(),
            ),
            "Failed to find codec description",
        )
    }
}

#[doc(alias = "gst_pb_utils_add_codec_description_to_tag_list")]
pub fn pb_utils_add_codec_description_to_tag_list(
    taglist: &mut gst::TagListRef,
    caps: &gst::CapsRef,
) -> Result<(), glib::BoolError> {
    assert_initialized_main_thread!();
    unsafe {
        glib::result_from_gboolean!(
            ffi::gst_pb_utils_add_codec_description_to_tag_list(
                taglist.as_mut_ptr(),
                ptr::null_mut(),
                caps.as_ptr(),
            ),
            "Failed to find codec description",
        )
    }
}

#[doc(alias = "gst_pb_utils_get_encoder_description")]
pub fn pb_utils_get_encoder_description(caps: &gst::CapsRef) -> glib::GString {
    assert_initialized_main_thread!();
    unsafe { from_glib_full(ffi::gst_pb_utils_get_encoder_description(caps.as_ptr())) }
}

#[doc(alias = "gst_pb_utils_get_decoder_description")]
pub fn pb_utils_get_decoder_description(caps: &gst::CapsRef) -> glib::GString {
    assert_initialized_main_thread!();
    unsafe { from_glib_full(ffi::gst_pb_utils_get_decoder_description(caps.as_ptr())) }
}

#[doc(alias = "gst_pb_utils_get_codec_description")]
pub fn pb_utils_get_codec_description(caps: &gst::CapsRef) -> glib::GString {
    assert_initialized_main_thread!();
    unsafe { from_glib_full(ffi::gst_pb_utils_get_codec_description(caps.as_ptr())) }
}

#[doc(alias = "gst_codec_utils_aac_caps_set_level_and_profile")]
pub fn codec_utils_aac_caps_set_level_and_profile(
    caps: &mut gst::CapsRef,
    audio_config: &[u8],
) -> Result<(), glib::BoolError> {
    assert_initialized_main_thread!();

    assert_eq!(caps.size(), 1);

    let s = caps.structure(0).unwrap();
    assert_eq!(s.name(), "audio/mpeg");
    assert!(s
        .get::<i32>("mpegversion")
        .map_or(false, |v| v == 2 || v == 4));

    let len = audio_config.len() as u32;
    unsafe {
        let res: bool = from_glib(ffi::gst_codec_utils_aac_caps_set_level_and_profile(
            caps.as_mut_ptr(),
            audio_config.to_glib_none().0,
            len,
        ));

        if res {
            Ok(())
        } else {
            Err(glib::bool_error!("Failed to set AAC level/profile to caps"))
        }
    }
}

#[doc(alias = "gst_codec_utils_h264_caps_set_level_and_profile")]
pub fn codec_utils_h264_caps_set_level_and_profile(
    caps: &mut gst::CapsRef,
    sps: &[u8],
) -> Result<(), glib::BoolError> {
    assert_initialized_main_thread!();

    assert_eq!(caps.size(), 1);

    let s = caps.structure(0).unwrap();
    assert_eq!(s.name(), "video/x-h264");

    let len = sps.len() as u32;
    unsafe {
        let res: bool = from_glib(ffi::gst_codec_utils_h264_caps_set_level_and_profile(
            caps.as_mut_ptr(),
            sps.to_glib_none().0,
            len,
        ));

        if res {
            Ok(())
        } else {
            Err(glib::bool_error!(
                "Failed to set H264 level/profile to caps"
            ))
        }
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
#[doc(alias = "gst_codec_utils_h264_get_profile_flags_level")]
pub fn codec_utils_h264_get_profile_flags_level(
    codec_data: &[u8],
) -> Result<(u8, u8, u8), glib::BoolError> {
    assert_initialized_main_thread!();
    let len = codec_data.len() as u32;
    unsafe {
        let mut profile = mem::MaybeUninit::uninit();
        let mut flags = mem::MaybeUninit::uninit();
        let mut level = mem::MaybeUninit::uninit();
        glib::result_from_gboolean!(
            ffi::gst_codec_utils_h264_get_profile_flags_level(
                codec_data.to_glib_none().0,
                len,
                profile.as_mut_ptr(),
                flags.as_mut_ptr(),
                level.as_mut_ptr()
            ),
            "Failed to get H264 profile, flags and level"
        )?;
        let profile = profile.assume_init();
        let flags = flags.assume_init();
        let level = level.assume_init();
        Ok((profile, flags, level))
    }
}

#[doc(alias = "gst_codec_utils_h265_caps_set_level_tier_and_profile")]
pub fn codec_utils_h265_caps_set_level_tier_and_profile(
    caps: &mut gst::CapsRef,
    profile_tier_level: &[u8],
) -> Result<(), glib::BoolError> {
    assert_initialized_main_thread!();

    assert_eq!(caps.size(), 1);

    let s = caps.structure(0).unwrap();
    assert_eq!(s.name(), "video/x-h265");

    let len = profile_tier_level.len() as u32;
    unsafe {
        let res: bool = from_glib(ffi::gst_codec_utils_h265_caps_set_level_tier_and_profile(
            caps.as_mut_ptr(),
            profile_tier_level.to_glib_none().0,
            len,
        ));

        if res {
            Ok(())
        } else {
            Err(glib::bool_error!(
                "Failed to set H265 level/tier/profile to caps"
            ))
        }
    }
}

#[doc(alias = "gst_codec_utils_mpeg4video_caps_set_level_and_profile")]
pub fn codec_utils_mpeg4video_caps_set_level_and_profile(
    caps: &mut gst::CapsRef,
    vis_obj_seq: &[u8],
) -> Result<(), glib::BoolError> {
    assert_initialized_main_thread!();

    assert_eq!(caps.size(), 1);

    let s = caps.structure(0).unwrap();
    assert_eq!(s.name(), "video/mpeg");
    assert!(s.get::<i32>("mpegversion").map_or(false, |v| v == 4));

    let len = vis_obj_seq.len() as u32;
    unsafe {
        let res: bool = from_glib(ffi::gst_codec_utils_mpeg4video_caps_set_level_and_profile(
            caps.as_mut_ptr(),
            vis_obj_seq.to_glib_none().0,
            len,
        ));

        if res {
            Ok(())
        } else {
            Err(glib::bool_error!(
                "Failed to set MPEG4 video level/profile to caps"
            ))
        }
    }
}

#[doc(alias = "gst_codec_utils_opus_create_caps")]
pub fn codec_utils_opus_create_caps(
    rate: u32,
    channels: u8,
    channel_mapping_family: u8,
    stream_count: u8,
    coupled_count: u8,
    channel_mapping: &[u8],
) -> Result<gst::Caps, glib::BoolError> {
    assert_initialized_main_thread!();

    assert!(channel_mapping.is_empty() || channel_mapping.len() == channels as usize);

    unsafe {
        let caps = ffi::gst_codec_utils_opus_create_caps(
            rate,
            channels,
            channel_mapping_family,
            stream_count,
            coupled_count,
            if channel_mapping.is_empty() {
                ptr::null()
            } else {
                channel_mapping.to_glib_none().0
            },
        );

        if caps.is_null() {
            Err(glib::bool_error!(
                "Failed to create caps from Opus configuration"
            ))
        } else {
            Ok(from_glib_full(caps))
        }
    }
}

#[doc(alias = "gst_codec_utils_opus_create_caps_from_header")]
pub fn codec_utils_opus_create_caps_from_header(
    header: &gst::BufferRef,
    comments: Option<&gst::BufferRef>,
) -> Result<gst::Caps, glib::BoolError> {
    assert_initialized_main_thread!();
    unsafe {
        Option::<_>::from_glib_full(ffi::gst_codec_utils_opus_create_caps_from_header(
            mut_override(header.as_ptr()),
            comments
                .map(|b| mut_override(b.as_ptr()))
                .unwrap_or(ptr::null_mut()),
        ))
        .ok_or_else(|| glib::bool_error!("Failed to create caps from Opus headers"))
    }
}

#[doc(alias = "gst_codec_utils_opus_create_header")]
#[allow(clippy::too_many_arguments)]
pub fn codec_utils_opus_create_header(
    rate: u32,
    channels: u8,
    channel_mapping_family: u8,
    stream_count: u8,
    coupled_count: u8,
    channel_mapping: &[u8],
    pre_skip: u16,
    output_gain: i16,
) -> Result<gst::Buffer, glib::BoolError> {
    assert_initialized_main_thread!();

    assert!(channel_mapping.is_empty() || channel_mapping.len() == channels as usize);

    unsafe {
        let header = ffi::gst_codec_utils_opus_create_header(
            rate,
            channels,
            channel_mapping_family,
            stream_count,
            coupled_count,
            if channel_mapping.is_empty() {
                ptr::null()
            } else {
                channel_mapping.to_glib_none().0
            },
            pre_skip,
            output_gain,
        );

        if header.is_null() {
            Err(glib::bool_error!(
                "Failed to create header from Opus configuration"
            ))
        } else {
            Ok(from_glib_full(header))
        }
    }
}

#[doc(alias = "gst_codec_utils_opus_parse_caps")]
pub fn codec_utils_opus_parse_caps(
    caps: &gst::CapsRef,
    channel_mapping: Option<&mut [u8; 256]>,
) -> Result<(u32, u8, u8, u8, u8), glib::BoolError> {
    assert_initialized_main_thread!();

    unsafe {
        let mut rate = mem::MaybeUninit::uninit();
        let mut channels = mem::MaybeUninit::uninit();
        let mut channel_mapping_family = mem::MaybeUninit::uninit();
        let mut stream_count = mem::MaybeUninit::uninit();
        let mut coupled_count = mem::MaybeUninit::uninit();

        let res: bool = from_glib(ffi::gst_codec_utils_opus_parse_caps(
            mut_override(caps.as_ptr()),
            rate.as_mut_ptr(),
            channels.as_mut_ptr(),
            channel_mapping_family.as_mut_ptr(),
            stream_count.as_mut_ptr(),
            coupled_count.as_mut_ptr(),
            if let Some(channel_mapping) = channel_mapping {
                channel_mapping.as_mut_ptr() as *mut [u8; 256]
            } else {
                ptr::null_mut()
            },
        ));

        if res {
            Ok((
                rate.assume_init(),
                channels.assume_init(),
                channel_mapping_family.assume_init(),
                stream_count.assume_init(),
                coupled_count.assume_init(),
            ))
        } else {
            Err(glib::bool_error!("Failed to parse Opus caps"))
        }
    }
}

#[doc(alias = "gst_codec_utils_opus_parse_header")]
#[allow(clippy::type_complexity)]
pub fn codec_utils_opus_parse_header(
    header: &gst::BufferRef,
    channel_mapping: Option<&mut [u8; 256]>,
) -> Result<(u32, u8, u8, u8, u8, u16, i16), glib::BoolError> {
    assert_initialized_main_thread!();

    unsafe {
        let mut rate = mem::MaybeUninit::uninit();
        let mut channels = mem::MaybeUninit::uninit();
        let mut channel_mapping_family = mem::MaybeUninit::uninit();
        let mut stream_count = mem::MaybeUninit::uninit();
        let mut coupled_count = mem::MaybeUninit::uninit();
        let mut pre_skip = mem::MaybeUninit::uninit();
        let mut output_gain = mem::MaybeUninit::uninit();

        let res: bool = from_glib(ffi::gst_codec_utils_opus_parse_header(
            mut_override(header.as_ptr()),
            rate.as_mut_ptr(),
            channels.as_mut_ptr(),
            channel_mapping_family.as_mut_ptr(),
            stream_count.as_mut_ptr(),
            coupled_count.as_mut_ptr(),
            if let Some(channel_mapping) = channel_mapping {
                channel_mapping.as_mut_ptr() as *mut [u8; 256]
            } else {
                ptr::null_mut()
            },
            pre_skip.as_mut_ptr(),
            output_gain.as_mut_ptr(),
        ));

        if res {
            Ok((
                rate.assume_init(),
                channels.assume_init(),
                channel_mapping_family.assume_init(),
                stream_count.assume_init(),
                coupled_count.assume_init(),
                pre_skip.assume_init(),
                output_gain.assume_init(),
            ))
        } else {
            Err(glib::bool_error!("Failed to parse Opus header"))
        }
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
#[doc(alias = "gst_codec_utils_caps_get_mime_codec")]
pub fn codec_utils_caps_get_mime_codec(
    caps: &gst::CapsRef,
) -> Result<glib::GString, glib::BoolError> {
    assert_initialized_main_thread!();
    unsafe {
        Option::<_>::from_glib_full(ffi::gst_codec_utils_caps_get_mime_codec(mut_override(
            caps.as_ptr(),
        )))
        .ok_or_else(|| glib::bool_error!("Unsupported caps"))
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
#[doc(alias = "gst_pb_utils_get_caps_description_flags")]
pub fn pb_utils_get_caps_description_flags(
    caps: &gst::CapsRef,
) -> crate::PbUtilsCapsDescriptionFlags {
    assert_initialized_main_thread!();
    unsafe { from_glib(ffi::gst_pb_utils_get_caps_description_flags(caps.as_ptr())) }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
#[doc(alias = "gst_pb_utils_get_file_extension_from_caps")]
pub fn pb_utils_get_file_extension_from_caps(caps: &gst::CapsRef) -> Option<glib::GString> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gst_pb_utils_get_file_extension_from_caps(
            caps.as_ptr(),
        ))
    }
}
