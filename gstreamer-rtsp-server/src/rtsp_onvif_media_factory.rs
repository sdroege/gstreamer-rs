// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, translate::*};

use crate::{RTSPContext, RTSPOnvifMediaFactory};

pub trait RTSPOnvifMediaFactoryExtManual: 'static {
    #[doc(alias = "gst_rtsp_onvif_media_factory_requires_backchannel")]
    fn requires_backchannel(&self, ctx: &RTSPContext) -> bool;
}

impl<O: IsA<RTSPOnvifMediaFactory>> RTSPOnvifMediaFactoryExtManual for O {
    fn requires_backchannel(&self, ctx: &RTSPContext) -> bool {
        skip_assert_initialized!();
        unsafe {
            from_glib(ffi::gst_rtsp_onvif_media_factory_requires_backchannel(
                self.as_ref()
                    .upcast_ref::<crate::RTSPMediaFactory>()
                    .to_glib_none()
                    .0,
                ctx.to_glib_none().0,
            ))
        }
    }
}
