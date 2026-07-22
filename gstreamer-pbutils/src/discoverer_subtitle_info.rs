// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

use glib::prelude::*;

use crate::{DiscovererStreamInfo, DiscovererSubtitleInfo};

#[cfg(feature = "v1_30")]
use glib::translate::*;

#[cfg(feature = "v1_30")]
use crate::ffi;

pub struct Debug<'a>(&'a DiscovererSubtitleInfo);

impl fmt::Debug for Debug<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let info = self.0.upcast_ref::<DiscovererStreamInfo>();

        f.debug_struct("DiscovererSubtitleInfo")
            .field("language", &self.0.language())
            .field("stream", &info.debug())
            .finish()
    }
}

impl DiscovererSubtitleInfo {
    pub fn debug(&self) -> Debug<'_> {
        Debug(self)
    }

    #[cfg(feature = "v1_30")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
    #[doc(alias = "gst_discoverer_subtitle_info_builder_new")]
    pub fn builder(stream_id: &str, caps: &gst::Caps) -> DiscovererSubtitleInfoBuilder {
        skip_assert_initialized!();

        DiscovererSubtitleInfoBuilder::new(stream_id, caps)
    }
}

#[cfg(feature = "v1_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
#[derive(Debug)]
#[doc(alias = "GstDiscovererSubtitleInfoBuilder")]
#[repr(transparent)]
pub struct DiscovererSubtitleInfoBuilder(std::ptr::NonNull<ffi::GstDiscovererSubtitleInfoBuilder>);

#[cfg(feature = "v1_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
impl DiscovererSubtitleInfoBuilder {
    #[doc(alias = "gst_discoverer_subtitle_info_builder_new")]
    pub fn new(stream_id: &str, caps: &gst::Caps) -> Self {
        skip_assert_initialized!();

        unsafe {
            let ptr = ffi::gst_discoverer_subtitle_info_builder_new(
                stream_id.to_glib_none().0,
                caps.to_glib_none().0,
            );
            DiscovererSubtitleInfoBuilder(std::ptr::NonNull::new_unchecked(ptr))
        }
    }

    #[doc(alias = "gst_discoverer_subtitle_info_builder_set_language")]
    pub fn language(self, language: &str) -> Self {
        unsafe {
            ffi::gst_discoverer_subtitle_info_builder_set_language(
                self.0.as_ptr(),
                language.to_glib_none().0,
            );
        }
        self
    }

    #[doc(alias = "gst_discoverer_subtitle_info_builder_set_tags")]
    pub fn tags(self, tags: &gst::TagList) -> Self {
        unsafe {
            ffi::gst_discoverer_subtitle_info_builder_set_tags(
                self.0.as_ptr(),
                tags.to_glib_none().0,
            );
        }
        self
    }

    #[doc(alias = "gst_discoverer_subtitle_info_builder_build")]
    pub fn build(self) -> DiscovererSubtitleInfo {
        unsafe {
            // gst_discoverer_subtitle_info_builder_build frees the builder,
            // so prevent it from being dropped at the end of this scope/function
            let s = std::mem::ManuallyDrop::new(self);

            from_glib_full(ffi::gst_discoverer_subtitle_info_builder_build(
                s.0.as_ptr(),
            ))
        }
    }
}

#[cfg(feature = "v1_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
impl Drop for DiscovererSubtitleInfoBuilder {
    #[inline]
    #[doc(alias = "gst_discoverer_subtitle_info_builder_free")]
    fn drop(&mut self) {
        unsafe {
            ffi::gst_discoverer_subtitle_info_builder_free(self.0.as_ptr());
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "v1_30")]
    fn discoverer_subtitle_info_builder_basic() {
        use std::str::FromStr;

        use super::*;

        use crate::traits::DiscovererStreamInfoExt;

        gst::init().unwrap();

        let caps = gst::Caps::from_str("text/x-raw, format=utf8").unwrap();

        let mut tags = gst::TagList::new();
        tags.get_mut()
            .unwrap()
            .add::<gst::tags::Title>(&"some title", gst::TagMergeMode::Append);

        let info = DiscovererSubtitleInfoBuilder::new("subtitle_builder_test", &caps)
            .language("en")
            .tags(&tags)
            .build();

        assert_eq!(info.language(), Some(glib::GString::from("en")));
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
