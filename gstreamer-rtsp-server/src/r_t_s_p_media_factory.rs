// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use RTSPMediaFactory;

use glib::translate::*;
use gst;

pub trait RTSPMediaFactoryExtManual {
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    fn add_role_from_structure(&self, structure: &gst::StructureRef);
}

impl<O: IsA<RTSPMediaFactory>> RTSPMediaFactoryExtManual for O {
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    fn add_role_from_structure(&self, structure: &gst::StructureRef) {
        unsafe {
            ffi::gst_rtsp_media_factory_add_role_from_structure(self.to_glib_none().0, structure.as_mut_ptr());
        }
    }
}
