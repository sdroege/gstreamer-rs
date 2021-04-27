// Take a look at the license at the top of the repository in the LICENSE file.

use bitflags::bitflags;
use glib::translate::*;

bitflags! {
    pub struct ElementFactoryListType: u64 {
        const DECODER          = 0b_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001;
        const ENCODER          = 0b_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0010;
        const SINK             = 0b_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0100;
        const SRC              = 0b_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1000;
        const MUXER            = 0b_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001_0000;
        const DEMUXER          = 0b_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0010_0000;
        const PARSER           = 0b_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0100_0000;
        const PAYLOADER        = 0b_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1000_0000;
        const DEPAYLOADER      = 0b_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001_0000_0000;
        const FORMATTER        = 0b_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0010_0000_0000;
        const DECRYPTOR        = 0b_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0100_0000_0000;
        const ENCRYPTOR        = 0b_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_1000_0000_0000;
        const HARDWARE         = 0b_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001_0000_0000_0000;

        const MEDIA_VIDEO      = 0b_0000_0000_0000_0010_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
        const MEDIA_AUDIO      = 0b_0000_0000_0000_0100_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
        const MEDIA_IMAGE      = 0b_0000_0000_0000_1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
        const MEDIA_SUBTITLE   = 0b_0000_0000_0001_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
        const MEDIA_METADATA   = 0b_0000_0000_0010_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;

        const ANY              = 0b_0000_0000_0000_0001_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;
        const MEDIA_ANY        = 0b_1111_1111_1111_1110_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;

        const VIDEO_ENCODER    = Self::ENCODER.bits | Self::MEDIA_VIDEO.bits | Self::MEDIA_IMAGE.bits;
        const AUDIO_ENCODER    = Self::ENCODER.bits | Self::MEDIA_AUDIO.bits;
        const AUDIOVIDEO_SINKS = Self::SINK.bits | Self::MEDIA_AUDIO.bits | Self::MEDIA_VIDEO.bits | Self::MEDIA_IMAGE.bits;
        const DECODABLE        = Self::DECODER.bits | Self::DEMUXER.bits | Self::DEPAYLOADER.bits | Self::PARSER.bits | Self::DECRYPTOR.bits;
    }
}

#[doc(hidden)]
impl IntoGlib for ElementFactoryListType {
    type GlibType = ffi::GstElementFactoryListType;

    fn into_glib(self) -> ffi::GstElementFactoryListType {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstElementFactoryListType> for ElementFactoryListType {
    unsafe fn from_glib(value: ffi::GstElementFactoryListType) -> ElementFactoryListType {
        skip_assert_initialized!();
        ElementFactoryListType::from_bits_truncate(value)
    }
}
