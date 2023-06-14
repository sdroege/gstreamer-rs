// Take a look at the license at the top of the repository in the LICENSE file.

use std::{boxed::Box as Box_, fmt, mem::transmute};

use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};

use crate::{auto::Discoverer, DiscovererInfo};

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
                Some(transmute::<_, unsafe extern "C" fn()>(
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
    let f: &F = &*(f as *const F);
    f(Discoverer::from_glib_borrow(this).unsafe_cast_ref())
}

pub struct DebugInfo<'a>(&'a DiscovererInfo);

impl<'a> fmt::Debug for DebugInfo<'a> {
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
    pub fn debug(&self) -> DebugInfo {
        DebugInfo(self)
    }
}
