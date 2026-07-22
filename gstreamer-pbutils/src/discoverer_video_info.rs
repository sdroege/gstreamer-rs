// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

use glib::{prelude::*, translate::*};

use crate::{DiscovererStreamInfo, DiscovererVideoInfo, ffi};

impl DiscovererVideoInfo {
    #[doc(alias = "get_framerate")]
    #[doc(alias = "gst_discoverer_video_info_get_framerate_num")]
    #[doc(alias = "gst_discoverer_video_info_get_framerate_denom")]
    pub fn framerate(&self) -> gst::Fraction {
        unsafe {
            gst::Fraction::new(
                ffi::gst_discoverer_video_info_get_framerate_num(self.to_glib_none().0) as i32,
                ffi::gst_discoverer_video_info_get_framerate_denom(self.to_glib_none().0) as i32,
            )
        }
    }

    #[doc(alias = "get_par")]
    #[doc(alias = "gst_discoverer_video_info_get_par_num")]
    #[doc(alias = "gst_discoverer_video_info_get_par_denom")]
    pub fn par(&self) -> gst::Fraction {
        unsafe {
            gst::Fraction::new(
                ffi::gst_discoverer_video_info_get_par_num(self.to_glib_none().0) as i32,
                ffi::gst_discoverer_video_info_get_par_denom(self.to_glib_none().0) as i32,
            )
        }
    }

    pub fn debug(&self) -> Debug<'_> {
        Debug(self)
    }

    #[cfg(feature = "v1_30")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
    #[doc(alias = "gst_discoverer_video_info_builder_new")]
    pub fn builder(stream_id: &str, caps: &gst::Caps) -> DiscovererVideoInfoBuilder {
        skip_assert_initialized!();

        DiscovererVideoInfoBuilder::new(stream_id, caps)
    }
}

pub struct Debug<'a>(&'a DiscovererVideoInfo);

impl fmt::Debug for Debug<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let info = self.0.upcast_ref::<DiscovererStreamInfo>();

        f.debug_struct("DiscovererVideoInfo")
            .field("width", &self.0.width())
            .field("height", &self.0.height())
            .field("depth", &self.0.depth())
            .field("bitrate", &self.0.bitrate())
            .field("max-bitrate", &self.0.max_bitrate())
            .field("is-image", &self.0.is_image())
            .field("is-interlaced", &self.0.is_interlaced())
            .field("stream", &info.debug())
            .finish()
    }
}

#[cfg(feature = "v1_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
#[derive(Debug)]
#[doc(alias = "GstDiscovererVideoInfoBuilder")]
#[repr(transparent)]
pub struct DiscovererVideoInfoBuilder(std::ptr::NonNull<ffi::GstDiscovererVideoInfoBuilder>);

#[cfg(feature = "v1_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
impl DiscovererVideoInfoBuilder {
    #[doc(alias = "gst_discoverer_video_info_builder_new")]
    pub fn new(stream_id: &str, caps: &gst::Caps) -> Self {
        skip_assert_initialized!();

        unsafe {
            let ptr = ffi::gst_discoverer_video_info_builder_new(
                stream_id.to_glib_none().0,
                caps.to_glib_none().0,
            );
            DiscovererVideoInfoBuilder(std::ptr::NonNull::new_unchecked(ptr))
        }
    }

    #[doc(alias = "gst_discoverer_video_info_builder_set_bitrate")]
    pub fn bitrate(self, bitrate: u32) -> Self {
        unsafe {
            ffi::gst_discoverer_video_info_builder_set_bitrate(self.0.as_ptr(), bitrate);
        }
        self
    }

    #[doc(alias = "gst_discoverer_video_info_builder_set_interlaced")]
    pub fn interlaced(self, interlaced: bool) -> Self {
        unsafe {
            ffi::gst_discoverer_video_info_builder_set_interlaced(
                self.0.as_ptr(),
                interlaced.into_glib(),
            );
        }
        self
    }

    #[doc(alias = "gst_discoverer_video_info_builder_set_max_bitrate")]
    pub fn max_bitrate(self, max_bitrate: u32) -> Self {
        unsafe {
            ffi::gst_discoverer_video_info_builder_set_max_bitrate(self.0.as_ptr(), max_bitrate);
        }
        self
    }

    #[doc(alias = "gst_discoverer_video_info_builder_set_is_image")]
    pub fn is_image(self, is_image: bool) -> Self {
        unsafe {
            ffi::gst_discoverer_video_info_builder_set_is_image(
                self.0.as_ptr(),
                is_image.into_glib(),
            );
        }
        self
    }

    #[doc(alias = "gst_discoverer_video_info_builder_set_tags")]
    pub fn tags(self, tags: &gst::TagList) -> Self {
        unsafe {
            ffi::gst_discoverer_video_info_builder_set_tags(self.0.as_ptr(), tags.to_glib_none().0);
        }
        self
    }

    #[doc(alias = "gst_discoverer_video_info_builder_build")]
    pub fn build(self) -> DiscovererVideoInfo {
        unsafe {
            // the gst_discoverer_video_info_builder_build frees the builder before creating new
            // so prevent it from being dropped at the end of this scope/function
            let s = std::mem::ManuallyDrop::new(self);

            from_glib_full(ffi::gst_discoverer_video_info_builder_build(s.0.as_ptr()))
        }
    }
}

#[cfg(feature = "v1_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
impl Drop for DiscovererVideoInfoBuilder {
    #[inline]
    #[doc(alias = "gst_discoverer_video_info_builder_free")]
    fn drop(&mut self) {
        unsafe {
            ffi::gst_discoverer_video_info_builder_free(self.0.as_ptr());
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "v1_30")]
    fn discoverer_video_info_builder_basic() {
        use super::*;
        use std::str::FromStr;

        use crate::traits::DiscovererStreamInfoExt;

        gst::init().unwrap();

        let caps = gst::Caps::from_str(
            "video/x-raw, width=3840, height=2160, framerate=60/1, pixel-aspect-ratio=1/1",
        )
        .unwrap();

        let mut tags = gst::TagList::new();
        tags.get_mut()
            .unwrap()
            .add::<gst::tags::Title>(&"some title", gst::TagMergeMode::Append);

        let info = DiscovererVideoInfoBuilder::new("video_builder_test", &caps)
            .bitrate(20000000)
            .interlaced(true)
            .max_bitrate(30000000)
            .is_image(false)
            .tags(&tags)
            .build();

        assert_eq!(info.height(), 2160);
        assert_eq!(info.width(), 3840);
        assert_eq!(info.par(), gst::Fraction::new(1, 1));
        assert_eq!(info.framerate(), gst::Fraction::new(60, 1));
        assert_eq!(
            info.stream_id(),
            Some(glib::GString::from("video_builder_test"))
        );
        assert_eq!(info.bitrate(), 20000000);
        assert!(!info.is_image());
        assert!(info.is_interlaced());
        assert_eq!(info.max_bitrate(), 30000000);
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
