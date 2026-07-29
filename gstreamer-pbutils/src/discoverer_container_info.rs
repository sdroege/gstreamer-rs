// Take a look at the license at the top of the repository in the LICENSE file.
use std::fmt;

use crate::{DiscovererContainerInfo, prelude::*};

#[cfg(feature = "v1_30")]
use glib::translate::*;

#[cfg(feature = "v1_30")]
use crate::{DiscovererStreamInfo, ffi};

pub struct Debug<'a>(&'a DiscovererContainerInfo);

impl fmt::Debug for Debug<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let streams = self.0.streams();

        let mut d = f.debug_struct("DiscovererContainerInfo");

        d.field("tags", &self.0.tags()).field(
            "streams",
            &streams.iter().map(|info| info.debug()).collect::<Vec<_>>(),
        );

        #[cfg(feature = "v1_20")]
        d.field("stream-number", &self.0.stream_number());
        #[cfg(feature = "v1_20")]
        d.field("tags", &self.0.tags());

        d.finish()
    }
}

impl DiscovererContainerInfo {
    pub fn debug(&self) -> Debug<'_> {
        Debug(self)
    }

    #[cfg(feature = "v1_30")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
    #[doc(alias = "gst_discoverer_container_info_builder_new")]
    pub fn builder(caps: &gst::Caps) -> DiscovererContainerInfoBuilder {
        skip_assert_initialized!();

        DiscovererContainerInfoBuilder::new(caps)
    }
}

#[cfg(feature = "v1_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
#[derive(Debug)]
#[doc(alias = "GstDiscovererContainerInfoBuilder")]
#[repr(transparent)]
pub struct DiscovererContainerInfoBuilder(
    std::ptr::NonNull<ffi::GstDiscovererContainerInfoBuilder>,
);

#[cfg(feature = "v1_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
impl DiscovererContainerInfoBuilder {
    #[doc(alias = "gst_discoverer_container_info_builder_new")]
    pub fn new(caps: &gst::Caps) -> Self {
        skip_assert_initialized!();

        unsafe {
            let ptr = ffi::gst_discoverer_container_info_builder_new(caps.to_glib_none().0);
            DiscovererContainerInfoBuilder(std::ptr::NonNull::new_unchecked(ptr))
        }
    }

    #[doc(alias = "gst_discoverer_container_info_builder_add_stream")]
    pub fn add_stream(self, stream_info: impl IsA<DiscovererStreamInfo>) -> Self {
        unsafe {
            ffi::gst_discoverer_container_info_builder_add_stream(
                self.0.as_ptr(),
                stream_info.upcast().into_glib_ptr(),
            );
        }
        self
    }

    #[doc(alias = "gst_discoverer_container_info_builder_set_tags")]
    pub fn tags(self, tags: &gst::TagList) -> Self {
        unsafe {
            ffi::gst_discoverer_container_info_builder_set_tags(
                self.0.as_ptr(),
                tags.to_glib_none().0,
            );
        }
        self
    }

    #[doc(alias = "gst_discoverer_container_info_builder_build")]
    pub fn build(self) -> DiscovererContainerInfo {
        unsafe {
            // gst_discoverer_container_info_builder_build frees the builder,
            // so prevent it from being dropped at the end of this scope/function
            let s = std::mem::ManuallyDrop::new(self);

            from_glib_full(ffi::gst_discoverer_container_info_builder_build(
                s.0.as_ptr(),
            ))
        }
    }
}

#[cfg(feature = "v1_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
impl Drop for DiscovererContainerInfoBuilder {
    #[inline]
    #[doc(alias = "gst_discoverer_container_info_builder_free")]
    fn drop(&mut self) {
        unsafe {
            ffi::gst_discoverer_container_info_builder_free(self.0.as_ptr());
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "v1_30")]
    fn discoverer_container_info_builder_basic() {
        use std::str::FromStr;

        use super::*;

        gst::init().unwrap();

        let container_caps = gst::Caps::from_str("video/quicktime").unwrap();
        let audio_caps = gst::Caps::from_str("audio/x-raw, format=S16LE").unwrap();

        let audio_info =
            crate::DiscovererAudioInfoBuilder::new("audio_stream", &audio_caps).build();

        let mut tags = gst::TagList::new();
        tags.get_mut()
            .unwrap()
            .add::<gst::tags::Title>(&"some title", gst::TagMergeMode::Append);

        let info = DiscovererContainerInfoBuilder::new(&container_caps)
            .add_stream(audio_info)
            .tags(&tags)
            .build();

        assert_eq!(info.streams().len(), 1);
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
