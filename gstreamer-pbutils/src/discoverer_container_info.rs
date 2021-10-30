// Take a look at the license at the top of the repository in the LICENSE file.

use crate::DiscovererContainerInfo;

#[cfg(any(feature = "v1_20", feature = "dox"))]
use glib::translate::*;

impl DiscovererContainerInfo {
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "get_tags")]
    #[doc(alias = "gst_discoverer_container_info_get_tags")]
    pub fn tags(&self) -> Option<gst::TagList> {
        unsafe {
            from_glib_none(ffi::gst_discoverer_container_info_get_tags(
                self.to_glib_none().0,
            ))
        }
    }
}
