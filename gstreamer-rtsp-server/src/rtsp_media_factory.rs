// Take a look at the license at the top of the repository in the LICENSE file.

use crate::RTSPMediaFactory;

use glib::prelude::*;
use glib::translate::*;

pub trait RTSPMediaFactoryExtManual: 'static {
    #[doc(alias = "gst_rtsp_media_factory_add_role_from_structure")]
    fn add_role_from_structure(&self, structure: &gst::StructureRef);
}

impl<O: IsA<RTSPMediaFactory>> RTSPMediaFactoryExtManual for O {
    fn add_role_from_structure(&self, structure: &gst::StructureRef) {
        unsafe {
            ffi::gst_rtsp_media_factory_add_role_from_structure(
                self.as_ref().to_glib_none().0,
                structure.as_mut_ptr(),
            );
        }
    }
}
