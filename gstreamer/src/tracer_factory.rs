// Take a look at the license at the top of the repository in the LICENSE file.

use crate::TracerFactory;

#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
use glib::translate::*;

impl TracerFactory {
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    #[doc(alias = "gst_tracer_factory_get_list")]
    #[doc(alias = "get_list")]
    pub fn factories() -> glib::List<TracerFactory> {
        assert_initialized_main_thread!();
        unsafe { FromGlibPtrContainer::from_glib_full(ffi::gst_tracer_factory_get_list()) }
    }
}
