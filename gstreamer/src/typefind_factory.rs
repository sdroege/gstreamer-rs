// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::TypeFindFactory;

impl TypeFindFactory {
    #[doc(alias = "gst_type_find_factory_get_list")]
    #[doc(alias = "get_list")]
    pub fn factories() -> glib::List<TypeFindFactory> {
        assert_initialized_main_thread!();
        unsafe { FromGlibPtrContainer::from_glib_full(ffi::gst_type_find_factory_get_list()) }
    }
}
