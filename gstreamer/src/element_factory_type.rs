// Take a look at the license at the top of the repository in the LICENSE file.

use bitflags::bitflags;
use glib::translate::*;

bitflags! {
    #[doc(alias = "GstElementFactoryListType")]
    pub struct ElementFactoryType: u64 {
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_DECODER")]
        const DECODER          = ffi::GST_ELEMENT_FACTORY_TYPE_DECODER;
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_ENCODER")]
        const ENCODER          = ffi::GST_ELEMENT_FACTORY_TYPE_ENCODER;
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_SINK")]
        const SINK             = ffi::GST_ELEMENT_FACTORY_TYPE_SINK;
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_SRC")]
        const SRC              = ffi::GST_ELEMENT_FACTORY_TYPE_SRC;
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_MUXER")]
        const MUXER            = ffi::GST_ELEMENT_FACTORY_TYPE_MUXER;
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_DEMUXER")]
        const DEMUXER          = ffi::GST_ELEMENT_FACTORY_TYPE_DEMUXER;
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_PARSER")]
        const PARSER           = ffi::GST_ELEMENT_FACTORY_TYPE_PARSER;
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_PAYLOADER")]
        const PAYLOADER        = ffi::GST_ELEMENT_FACTORY_TYPE_PAYLOADER;
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_DEPAYLOADER")]
        const DEPAYLOADER      = ffi::GST_ELEMENT_FACTORY_TYPE_DEPAYLOADER;
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_FORMATTER")]
        const FORMATTER        = ffi::GST_ELEMENT_FACTORY_TYPE_FORMATTER;
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_DECRYPTOR")]
        const DECRYPTOR        = ffi::GST_ELEMENT_FACTORY_TYPE_DECRYPTOR;
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_ENCRYPTOR")]
        const ENCRYPTOR        = ffi::GST_ELEMENT_FACTORY_TYPE_ENCRYPTOR;
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_HARDWARE")]
        const HARDWARE         = ffi::GST_ELEMENT_FACTORY_TYPE_HARDWARE;

        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_MEDIA_VIDEO")]
        const MEDIA_VIDEO      = ffi::GST_ELEMENT_FACTORY_TYPE_MEDIA_VIDEO;
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_MEDIA_AUDIO")]
        const MEDIA_AUDIO      = ffi::GST_ELEMENT_FACTORY_TYPE_MEDIA_AUDIO;
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_MEDIA_IMAGE")]
        const MEDIA_IMAGE      = ffi::GST_ELEMENT_FACTORY_TYPE_MEDIA_IMAGE;
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_MEDIA_SUBTITLE")]
        const MEDIA_SUBTITLE   = ffi::GST_ELEMENT_FACTORY_TYPE_MEDIA_SUBTITLE;
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_MEDIA_METADATA")]
        const MEDIA_METADATA   = ffi::GST_ELEMENT_FACTORY_TYPE_MEDIA_METADATA;

        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_ANY")]
        const ANY              = ffi::GST_ELEMENT_FACTORY_TYPE_ANY;
        #[doc(alias = "GST_ELEMENT_FACTORY_TYPE_MEDIA_ANY")]
        const MEDIA_ANY        = ffi::GST_ELEMENT_FACTORY_TYPE_MEDIA_ANY;

        const VIDEO_ENCODER    = Self::ENCODER.bits | Self::MEDIA_VIDEO.bits | Self::MEDIA_IMAGE.bits;
        const AUDIO_ENCODER    = Self::ENCODER.bits | Self::MEDIA_AUDIO.bits;
        const AUDIOVIDEO_SINKS = Self::SINK.bits | Self::MEDIA_AUDIO.bits | Self::MEDIA_VIDEO.bits | Self::MEDIA_IMAGE.bits;
        const DECODABLE        = Self::DECODER.bits | Self::DEMUXER.bits | Self::DEPAYLOADER.bits | Self::PARSER.bits | Self::DECRYPTOR.bits;
    }
}

#[doc(hidden)]
impl IntoGlib for ElementFactoryType {
    type GlibType = ffi::GstElementFactoryListType;

    fn into_glib(self) -> ffi::GstElementFactoryListType {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstElementFactoryListType> for ElementFactoryType {
    unsafe fn from_glib(value: ffi::GstElementFactoryListType) -> ElementFactoryType {
        skip_assert_initialized!();
        ElementFactoryType::from_bits_truncate(value)
    }
}
