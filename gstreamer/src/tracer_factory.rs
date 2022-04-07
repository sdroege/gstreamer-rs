// Take a look at the license at the top of the repository in the LICENSE file.

use crate::TracerFactory;

use glib::translate::*;

impl TracerFactory {
    #[doc(alias = "gst_tracer_factory_get_list")]
    #[doc(alias = "get_list")]
    pub fn factories() -> glib::List<TracerFactory> {
        assert_initialized_main_thread!();
        unsafe { FromGlibPtrContainer::from_glib_full(ffi::gst_tracer_factory_get_list()) }
    }
}
