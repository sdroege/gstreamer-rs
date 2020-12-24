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
    let instance = &*(video_renderer as *mut T::Instance);
    let imp = instance.get_impl();

    imp.create_video_sink(
        from_glib_borrow::<_, PlayerVideoRenderer>(video_renderer).unsafe_cast_ref(),
        &Player::from_glib_borrow(player),
    )
    .to_glib_full()
}
