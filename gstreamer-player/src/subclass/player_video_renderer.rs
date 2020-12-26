// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Player, PlayerVideoRenderer};

use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

pub trait PlayerVideoRendererImpl: ObjectImpl {
    fn create_video_sink(&self, video_renderer: &Self::Type, player: &Player) -> gst::Element;
}

unsafe impl<T: PlayerVideoRendererImpl> IsImplementable<T> for PlayerVideoRenderer {
    unsafe extern "C" fn interface_init(
        iface: glib::ffi::gpointer,
        _iface_data: glib::ffi::gpointer,
    ) {
        let video_renderer_iface = &mut *(iface as *mut ffi::GstPlayerVideoRendererInterface);

        video_renderer_iface.create_video_sink = Some(video_renderer_create_video_sink::<T>);
    }
}

unsafe extern "C" fn video_renderer_create_video_sink<T: PlayerVideoRendererImpl>(
    video_renderer: *mut ffi::GstPlayerVideoRenderer,
    player: *mut ffi::GstPlayer,
) -> *mut gst::ffi::GstElement {
    use once_cell::sync::Lazy;
    static VIDEO_SINK_QUARK: Lazy<glib::Quark> =
        Lazy::new(|| glib::Quark::from_string("gstreamer-rs-player-video-sink"));

    let instance = &*(video_renderer as *mut T::Instance);
    let imp = instance.get_impl();

    let sink = imp.create_video_sink(
        from_glib_borrow::<_, PlayerVideoRenderer>(video_renderer).unsafe_cast_ref(),
        &Player::from_glib_borrow(player),
    );

    let sink_ptr: *mut gst::ffi::GstElement = sink.to_glib_none().0;

    let old_sink_ptr =
        glib::gobject_ffi::g_object_get_qdata(video_renderer as *mut _, VIDEO_SINK_QUARK.to_glib())
            as *mut gst::ffi::GstElement;
    if !old_sink_ptr.is_null() && old_sink_ptr != sink_ptr {
        panic!("Video sink must not change");
    }

    unsafe extern "C" fn unref(ptr: glib::ffi::gpointer) {
        glib::gobject_ffi::g_object_unref(ptr as *mut _);
    }

    glib::gobject_ffi::g_object_set_qdata_full(
        video_renderer as *mut _,
        VIDEO_SINK_QUARK.to_glib(),
        glib::gobject_ffi::g_object_ref(sink_ptr as *mut _) as *mut _,
        Some(unref),
    );

    sink_ptr
}
