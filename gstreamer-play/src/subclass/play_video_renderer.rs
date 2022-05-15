// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Play, PlayVideoRenderer};

use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;

pub trait PlayVideoRendererImpl: ObjectImpl {
    fn create_video_sink(&self, video_renderer: &Self::Type, play: &Play) -> gst::Element;
}

unsafe impl<T: PlayVideoRendererImpl> IsImplementable<T> for PlayVideoRenderer {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        iface.create_video_sink = Some(video_renderer_create_video_sink::<T>);
    }
}

pub trait PlayVideoRendererImplExt: ObjectSubclass {
    fn parent_create_video_sink(&self, video_renderer: &Self::Type, play: &Play) -> gst::Element;
}

impl<T: PlayVideoRendererImpl> PlayVideoRendererImplExt for T {
    fn parent_create_video_sink(&self, video_renderer: &Self::Type, play: &Play) -> gst::Element {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<PlayVideoRenderer>()
                as *const ffi::GstPlayVideoRendererInterface;

            let func = (*parent_iface)
                .create_video_sink
                .expect("no parent \"create_video_sink\" implementation");
            let ret = func(
                video_renderer
                    .unsafe_cast_ref::<PlayVideoRenderer>()
                    .to_glib_none()
                    .0,
                play.to_glib_none().0,
            );
            from_glib_none(ret)
        }
    }
}

unsafe extern "C" fn video_renderer_create_video_sink<T: PlayVideoRendererImpl>(
    video_renderer: *mut ffi::GstPlayVideoRenderer,
    play: *mut ffi::GstPlay,
) -> *mut gst::ffi::GstElement {
    use once_cell::sync::Lazy;
    static VIDEO_SINK_QUARK: Lazy<glib::Quark> =
        Lazy::new(|| glib::Quark::from_str("gstreamer-rs-play-video-sink"));

    let instance = &*(video_renderer as *mut T::Instance);
    let imp = instance.imp();

    let sink = imp.create_video_sink(
        from_glib_borrow::<_, PlayVideoRenderer>(video_renderer).unsafe_cast_ref(),
        &Play::from_glib_borrow(play),
    );

    let sink_ptr: *mut gst::ffi::GstElement = sink.to_glib_none().0;

    let old_sink_ptr = glib::gobject_ffi::g_object_get_qdata(
        video_renderer as *mut _,
        VIDEO_SINK_QUARK.into_glib(),
    ) as *mut gst::ffi::GstElement;
    if !old_sink_ptr.is_null() && old_sink_ptr != sink_ptr {
        panic!("Video sink must not change");
    }

    unsafe extern "C" fn unref(ptr: glib::ffi::gpointer) {
        glib::gobject_ffi::g_object_unref(ptr as *mut _);
    }

    glib::gobject_ffi::g_object_set_qdata_full(
        video_renderer as *mut _,
        VIDEO_SINK_QUARK.into_glib(),
        glib::gobject_ffi::g_object_ref(sink_ptr as *mut _) as *mut _,
        Some(unref),
    );

    sink_ptr
}
