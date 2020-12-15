// Take a look at the license at the top of the repository in the LICENSE file.

use crate::RTSPMediaFactory;

#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
use glib::translate::*;
use glib::IsA;

pub trait RTSPMediaFactoryExtManual: 'static {
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    fn add_role_from_structure(&self, structure: &gst::StructureRef);
}

impl<O: IsA<RTSPMediaFactory>> RTSPMediaFactoryExtManual for O {
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    fn add_role_from_structure(&self, structure: &gst::StructureRef) {
        unsafe {
            ffi::gst_rtsp_media_factory_add_role_from_structure(
                self.as_ref().to_glib_none().0,
                structure.as_mut_ptr(),
            );
        }
    }
}
