// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

use glib::prelude::*;

use crate::{DiscovererAudioInfo, DiscovererStreamInfo};

#[cfg(feature = "v1_30")]
use glib::translate::*;

#[cfg(feature = "v1_30")]
use crate::ffi;

pub struct Debug<'a>(&'a DiscovererAudioInfo);

impl fmt::Debug for Debug<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let info = self.0.upcast_ref::<DiscovererStreamInfo>();

        f.debug_struct("DiscovererAudioInfo")
            .field("channels", &self.0.channels())
            .field("sample-rate", &self.0.sample_rate())
            .field("depth", &self.0.depth())
            .field("bitrate", &self.0.bitrate())
            .field("max-bitrate", &self.0.max_bitrate())
            .field("channel-mask", &self.0.channel_mask())
            .field("language", &self.0.language())
            .field("stream", &info.debug())
            .finish()
    }
}

impl DiscovererAudioInfo {
    pub fn debug(&self) -> Debug<'_> {
        Debug(self)
    }

    #[cfg(feature = "v1_30")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
    #[doc(alias = "gst_discoverer_audio_info_builder_new")]
    pub fn builder(stream_id: &str, caps: &gst::Caps) -> DiscovererAudioInfoBuilder {
        skip_assert_initialized!();

        DiscovererAudioInfoBuilder::new(stream_id, caps)
    }
}

#[cfg(feature = "v1_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
#[derive(Debug)]
#[doc(alias = "GstDiscovererAudioInfoBuilder")]
#[repr(transparent)]
pub struct DiscovererAudioInfoBuilder(std::ptr::NonNull<ffi::GstDiscovererAudioInfoBuilder>);

#[cfg(feature = "v1_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
impl DiscovererAudioInfoBuilder {
    #[doc(alias = "gst_discoverer_audio_info_builder_new")]
    pub fn new(stream_id: &str, caps: &gst::Caps) -> Self {
        skip_assert_initialized!();
        unsafe {
            let ptr = ffi::gst_discoverer_audio_info_builder_new(
                stream_id.to_glib_none().0,
                caps.to_glib_none().0,
            );
            DiscovererAudioInfoBuilder(std::ptr::NonNull::new_unchecked(ptr))
        }
    }

    #[doc(alias = "gst_discoverer_audio_info_builder_set_bitrate")]
    pub fn bitrate(self, bitrate: u32) -> Self {
        unsafe {
            ffi::gst_discoverer_audio_info_builder_set_bitrate(self.0.as_ptr(), bitrate);
        }
        self
    }

    #[doc(alias = "gst_discoverer_audio_info_builder_set_max_bitrate")]
    pub fn max_bitrate(self, max_bitrate: u32) -> Self {
        unsafe {
            ffi::gst_discoverer_audio_info_builder_set_max_bitrate(self.0.as_ptr(), max_bitrate);
        }
        self
    }

    #[doc(alias = "gst_discoverer_audio_info_builder_set_language")]
    pub fn language(self, language: &str) -> Self {
        unsafe {
            ffi::gst_discoverer_audio_info_builder_set_language(
                self.0.as_ptr(),
                language.to_glib_none().0,
            );
        }
        self
    }

    #[doc(alias = "gst_discoverer_audio_info_builder_set_tags")]
    pub fn tags(self, tags: &gst::TagList) -> Self {
        unsafe {
            ffi::gst_discoverer_audio_info_builder_set_tags(self.0.as_ptr(), tags.to_glib_none().0);
        }
        self
    }

    #[doc(alias = "gst_discoverer_audio_info_builder_build")]
    pub fn build(self) -> DiscovererAudioInfo {
        unsafe {
            // the gst_discoverer_audio_info_builder_build frees the builder before creating new
            // so prevent it from being dropped at the end of this scope/function
            let s = std::mem::ManuallyDrop::new(self);

            from_glib_full(ffi::gst_discoverer_audio_info_builder_build(s.0.as_ptr()))
        }
    }
}

#[cfg(feature = "v1_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
impl Drop for DiscovererAudioInfoBuilder {
    #[inline]
    #[doc(alias = "gst_discoverer_audio_info_builder_free")]
    fn drop(&mut self) {
        unsafe {
            ffi::gst_discoverer_audio_info_builder_free(self.0.as_ptr());
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "v1_30")]
    fn discoverer_audio_info_builder_basic() {
        use super::*;
        use std::str::FromStr;

        use crate::traits::DiscovererStreamInfoExt;

        gst::init().unwrap();

        let caps = gst::Caps::from_str("audio/x-raw, rate=96000, channels=6, depth=24").unwrap();

        let mut tags = gst::TagList::new();
        tags.get_mut()
            .unwrap()
            .add::<gst::tags::Title>(&"some title", gst::TagMergeMode::Append);

        let info = DiscovererAudioInfoBuilder::new("audio_builder_test", &caps)
            .bitrate(20000000)
            .language("en")
            .max_bitrate(30000000)
            .tags(&tags)
            .build();

        assert_eq!(
            info.stream_id(),
            Some(glib::GString::from("audio_builder_test"))
        );
        assert_eq!(info.language(), Some(glib::GString::from("en")));
        assert_eq!(info.bitrate(), 20000000);
        assert_eq!(info.max_bitrate(), 30000000);
        assert_eq!(info.depth(), 24);
        assert_eq!(info.channels(), 6);
        assert_eq!(info.sample_rate(), 96000);
        assert_eq!(
            info.tags()
                .unwrap()
                .get::<gst::tags::Title>()
                .unwrap()
                .get(),
            "some title"
        );
    }
}
