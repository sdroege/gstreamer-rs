// Take a look at the license at the top of the repository in the LICENSE file.

use std::{boxed::Box as Box_, fmt, mem::transmute};

use glib::{
    prelude::*,
    signal::{SignalHandlerId, connect_raw},
    translate::*,
};

use crate::{Discoverer, DiscovererInfo, ffi};

#[cfg(feature = "v1_30")]
use crate::{DiscovererResult, DiscovererStreamInfo};

impl Discoverer {
    pub fn set_timeout(&self, timeout: gst::ClockTime) {
        self.set_property("timeout", timeout);
    }

    pub fn timeout(&self) -> gst::ClockTime {
        self.property("timeout")
    }

    #[doc(alias = "timeout")]
    pub fn connect_timeout_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::timeout\0".as_ptr() as *const _,
                Some(transmute::<*const (), unsafe extern "C" fn()>(
                    notify_timeout_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe extern "C" fn notify_timeout_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
    this: *mut ffi::GstDiscoverer,
    _param_spec: glib::ffi::gpointer,
    f: glib::ffi::gpointer,
) where
    P: IsA<Discoverer>,
{
    unsafe {
        let f: &F = &*(f as *const F);
        f(Discoverer::from_glib_borrow(this).unsafe_cast_ref())
    }
}

pub struct DebugInfo<'a>(&'a DiscovererInfo);

impl fmt::Debug for DebugInfo<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let stream_info = self.0.stream_info();
        let stream_list = self.0.stream_list();
        let container_streams = self.0.container_streams();
        let audio_streams = self.0.audio_streams();
        let video_streams = self.0.video_streams();
        let subtitle_streams = self.0.subtitle_streams();

        f.debug_struct("DiscovererInfo")
            .field("uri", &self.0.uri())
            .field("result", &self.0.result())
            .field("duration", &self.0.duration())
            .field("is-live", &self.0.is_live())
            .field("is-seekable", &self.0.is_seekable())
            .field(
                "stream-info",
                &stream_info.as_ref().map(|info| info.debug()),
            )
            .field(
                "stream-list",
                &stream_list
                    .iter()
                    .map(|info| info.debug())
                    .collect::<Vec<_>>(),
            )
            .field(
                "container-streams",
                &container_streams
                    .iter()
                    .map(|info| info.debug())
                    .collect::<Vec<_>>(),
            )
            .field(
                "audio-streams",
                &audio_streams
                    .iter()
                    .map(|info| info.debug())
                    .collect::<Vec<_>>(),
            )
            .field(
                "video-streams",
                &video_streams
                    .iter()
                    .map(|info| info.debug())
                    .collect::<Vec<_>>(),
            )
            .field(
                "subtitle-streams",
                &subtitle_streams
                    .iter()
                    .map(|info| info.debug())
                    .collect::<Vec<_>>(),
            )
            .field("toc", &self.0.toc())
            .field("misc", &self.0.misc())
            .field(
                "missing-elements-installer-details",
                &self.0.missing_elements_installer_details(),
            )
            .finish()
    }
}

impl DiscovererInfo {
    pub fn debug(&self) -> DebugInfo<'_> {
        DebugInfo(self)
    }

    #[cfg(feature = "v1_30")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
    #[doc(alias = "gst_discoverer_info_builder_new")]
    pub fn builder(
        uri: &str,
        stream_info: impl IsA<DiscovererStreamInfo>,
    ) -> DiscovererInfoBuilder {
        skip_assert_initialized!();

        DiscovererInfoBuilder::new(uri, stream_info)
    }
}

#[cfg(feature = "v1_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
#[derive(Debug)]
#[doc(alias = "GstDiscovererInfoBuilder")]
#[repr(transparent)]
pub struct DiscovererInfoBuilder(std::ptr::NonNull<ffi::GstDiscovererInfoBuilder>);

#[cfg(feature = "v1_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
impl DiscovererInfoBuilder {
    #[doc(alias = "gst_discoverer_info_builder_new")]
    pub fn new(uri: &str, stream_info: impl IsA<DiscovererStreamInfo>) -> Self {
        skip_assert_initialized!();

        unsafe {
            let ptr = ffi::gst_discoverer_info_builder_new(
                uri.to_glib_none().0,
                stream_info.upcast().into_glib_ptr(),
            );
            DiscovererInfoBuilder(std::ptr::NonNull::new_unchecked(ptr))
        }
    }

    #[doc(alias = "gst_discoverer_info_builder_set_duration")]
    pub fn duration(self, duration: gst::ClockTime) -> Self {
        unsafe {
            ffi::gst_discoverer_info_builder_set_duration(self.0.as_ptr(), duration.into_glib());
        }
        self
    }

    #[doc(alias = "gst_discoverer_info_builder_set_live")]
    pub fn live(self, live: bool) -> Self {
        unsafe {
            ffi::gst_discoverer_info_builder_set_live(self.0.as_ptr(), live.into_glib());
        }
        self
    }

    #[doc(alias = "gst_discoverer_info_builder_set_result")]
    pub fn result(self, result: DiscovererResult) -> Self {
        unsafe {
            ffi::gst_discoverer_info_builder_set_result(self.0.as_ptr(), result.into_glib());
        }
        self
    }

    #[doc(alias = "gst_discoverer_info_builder_set_seekable")]
    pub fn seekable(self, seekable: bool) -> Self {
        unsafe {
            ffi::gst_discoverer_info_builder_set_seekable(self.0.as_ptr(), seekable.into_glib());
        }
        self
    }

    #[doc(alias = "gst_discoverer_info_builder_set_tags")]
    pub fn tags(self, tags: &gst::TagList) -> Self {
        unsafe {
            ffi::gst_discoverer_info_builder_set_tags(self.0.as_ptr(), tags.to_glib_none().0);
        }
        self
    }

    #[doc(alias = "gst_discoverer_info_builder_build")]
    pub fn build(self) -> DiscovererInfo {
        unsafe {
            // gst_discoverer_info_builder_build frees the builder,
            // so prevent it from being dropped at the end of this scope/function
            let s = std::mem::ManuallyDrop::new(self);

            from_glib_full(ffi::gst_discoverer_info_builder_build(s.0.as_ptr()))
        }
    }
}

#[cfg(feature = "v1_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
impl Drop for DiscovererInfoBuilder {
    #[inline]
    #[doc(alias = "gst_discoverer_info_builder_free")]
    fn drop(&mut self) {
        unsafe {
            ffi::gst_discoverer_info_builder_free(self.0.as_ptr());
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "v1_30")]
    fn discoverer_info_builder_basic() {
        use super::*;

        gst::init().unwrap();

        let audio_caps = gst::Caps::builder("audio/x-raw")
            .field("format", "S16LE")
            .build();
        let audio_info =
            crate::DiscovererAudioInfoBuilder::new("audio_stream", &audio_caps).build();

        let mut tags = gst::TagList::new();
        tags.get_mut()
            .unwrap()
            .add::<gst::tags::Title>(&"some title", gst::TagMergeMode::Append);

        let info = DiscovererInfoBuilder::new("file:///tmp/test.wav", audio_info)
            .duration(gst::ClockTime::from_seconds(5))
            .live(false)
            .seekable(true)
            .result(DiscovererResult::Ok)
            .tags(&tags)
            .build();

        assert_eq!(info.duration(), Some(gst::ClockTime::from_seconds(5)));
        assert!(!info.is_live());
        assert!(info.is_seekable());
        assert_eq!(info.result(), DiscovererResult::Ok);
        assert_eq!(info.uri(), glib::GString::from("file:///tmp/test.wav"));
    }
}
