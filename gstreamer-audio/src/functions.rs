// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::{from_glib_full, IntoGlibPtr, ToGlibPtr};

use std::i32;

#[doc(alias = "gst_audio_buffer_clip")]
pub fn audio_buffer_clip(
    buffer: gst::Buffer,
    segment: &gst::Segment,
    rate: u32,
    bpf: u32,
) -> Option<gst::Buffer> {
    skip_assert_initialized!();

    unsafe {
        from_glib_full(ffi::gst_audio_buffer_clip(
            buffer.into_glib_ptr(),
            segment.to_glib_none().0,
            rate as i32,
            bpf as i32,
        ))
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
#[doc(alias = "gst_audio_buffer_truncate")]
pub fn audio_buffer_truncate(
    buffer: gst::Buffer,
    bpf: u32,
    trim: usize,
    samples: Option<usize>,
) -> gst::Buffer {
    skip_assert_initialized!();

    unsafe {
        from_glib_full(ffi::gst_audio_buffer_truncate(
            buffer.into_glib_ptr(),
            bpf as i32,
            trim,
            samples.unwrap_or(std::usize::MAX),
        ))
    }
}

pub fn audio_make_raw_caps(
    formats: &[crate::AudioFormat],
    layout: crate::AudioLayout,
) -> crate::AudioCapsBuilder<gst::caps::NoFeature> {
    assert_initialized_main_thread!();

    let formats = formats.iter().copied().map(|f| match f {
        crate::AudioFormat::Encoded => panic!("Invalid encoded format"),
        crate::AudioFormat::Unknown => panic!("Invalid unknown format"),
        _ => f,
    });

    let builder = crate::AudioCapsBuilder::new().format_list(formats);

    match layout {
        crate::AudioLayout::Interleaved => builder.field("layout", "interleaved"),
        crate::AudioLayout::NonInterleaved => builder.field("layout", "non-interleaved"),
        crate::AudioLayout::__Unknown(_) => builder,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audio_caps() {
        gst::init().unwrap();

        let caps = audio_make_raw_caps(
            &[crate::AudioFormat::S16be, crate::AudioFormat::S16le],
            crate::AudioLayout::Interleaved,
        )
        .build();
        assert_eq!(caps.to_string(), "audio/x-raw, rate=(int)[ 1, 2147483647 ], channels=(int)[ 1, 2147483647 ], layout=(string)interleaved, format=(string){ S16BE, S16LE }");

        #[cfg(feature = "v1_18")]
        {
            use glib::translate::{from_glib_full, IntoGlib};

            /* audio_make_raw_caps() is a re-implementation so ensure it returns the same caps as the C API */
            let c_caps = unsafe {
                let formats: Vec<ffi::GstAudioFormat> =
                    [crate::AudioFormat::S16be, crate::AudioFormat::S16le]
                        .iter()
                        .map(|f| f.into_glib())
                        .collect();
                let caps = ffi::gst_audio_make_raw_caps(
                    formats.as_ptr(),
                    formats.len() as u32,
                    ffi::GST_AUDIO_LAYOUT_INTERLEAVED,
                );
                from_glib_full(caps)
            };
            assert_eq!(caps, c_caps);
        }

        let caps = audio_make_raw_caps(
            &[crate::AudioFormat::S16be, crate::AudioFormat::S16le],
            crate::AudioLayout::NonInterleaved,
        )
        .rate(16000)
        .channels(2)
        .build();
        assert_eq!(
            caps.to_string(),
            "audio/x-raw, rate=(int)16000, channels=(int)2, layout=(string)non-interleaved, format=(string){ S16BE, S16LE }",
        );
    }

    #[test]
    #[should_panic(expected = "Invalid encoded format")]
    fn audio_caps_encoded() {
        gst::init().unwrap();
        let _caps = audio_make_raw_caps(
            &[crate::AudioFormat::Encoded],
            crate::AudioLayout::Interleaved,
        );
    }

    #[test]
    #[should_panic(expected = "Invalid unknown format")]
    fn audio_caps_unknown() {
        gst::init().unwrap();
        let _caps = audio_make_raw_caps(
            &[crate::AudioFormat::Unknown],
            crate::AudioLayout::Interleaved,
        );
    }
}
