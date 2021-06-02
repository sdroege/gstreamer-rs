// Take a look at the license at the top of the repository in the LICENSE file.

use bitflags::bitflags;
use glib::translate::*;

bitflags! {
    #[doc(alias = "GstElementFactoryListType")]
    pub struct ElementFactoryListType: u64 {
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_DECODER")]
        const DECODER          = ffi::GST_ELEMENT_FACTORY_TYPE_DECODER as u64;
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_ENCODER")]
        const ENCODER          = ffi::GST_ELEMENT_FACTORY_TYPE_ENCODER as u64;
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_SINK")]
        const SINK             = ffi::GST_ELEMENT_FACTORY_TYPE_SINK as u64;
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_SRC")]
        const SRC              = ffi::GST_ELEMENT_FACTORY_TYPE_SRC as u64;
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_MUXER")]
        const MUXER            = ffi::GST_ELEMENT_FACTORY_TYPE_MUXER as u64;
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_DEMUXER")]
        const DEMUXER          = ffi::GST_ELEMENT_FACTORY_TYPE_DEMUXER as u64;
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_PARSER")]
        const PARSER           = ffi::GST_ELEMENT_FACTORY_TYPE_PARSER as u64;
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_PAYLOADER")]
        const PAYLOADER        = ffi::GST_ELEMENT_FACTORY_TYPE_PAYLOADER as u64;
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_DEPAYLOADER")]
        const DEPAYLOADER      = ffi::GST_ELEMENT_FACTORY_TYPE_DEPAYLOADER as u64;
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_FORMATTER")]
        const FORMATTER        = ffi::GST_ELEMENT_FACTORY_TYPE_FORMATTER as u64;
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_DECRYPTOR")]
        const DECRYPTOR        = ffi::GST_ELEMENT_FACTORY_TYPE_DECRYPTOR as u64;
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_ENCRYPTOR")]
        const ENCRYPTOR        = ffi::GST_ELEMENT_FACTORY_TYPE_ENCRYPTOR as u64;
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_HARDWARE")]
        const HARDWARE         = ffi::GST_ELEMENT_FACTORY_TYPE_HARDWARE as u64;

        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_MEDIA_VIDEO")]
        const MEDIA_VIDEO      = ffi::GST_ELEMENT_FACTORY_TYPE_MEDIA_VIDEO as u64;
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_MEDIA_AUDIO")]
        const MEDIA_AUDIO      = ffi::GST_ELEMENT_FACTORY_TYPE_MEDIA_AUDIO as u64;
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_MEDIA_IMAGE")]
        const MEDIA_IMAGE      = ffi::GST_ELEMENT_FACTORY_TYPE_MEDIA_IMAGE as u64;
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_MEDIA_SUBTITLE")]
        const MEDIA_SUBTITLE   = ffi::GST_ELEMENT_FACTORY_TYPE_MEDIA_SUBTITLE as u64;
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_MEDIA_METADATA")]
        const MEDIA_METADATA   = ffi::GST_ELEMENT_FACTORY_TYPE_MEDIA_METADATA as u64;

        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_ANY")]
        const ANY              = ffi::GST_ELEMENT_FACTORY_TYPE_ANY as u64;
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_MEDIA_ANY")]
        const MEDIA_ANY        = ffi::GST_ELEMENT_FACTORY_TYPE_MEDIA_ANY as u64;

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
