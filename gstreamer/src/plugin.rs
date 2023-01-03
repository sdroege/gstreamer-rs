// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, translate::*};

use crate::{Plugin, PluginFlags, StructureRef};

impl Plugin {
    #[doc(alias = "get_cache_data")]
    #[doc(alias = "gst_plugin_get_cache_data")]
    pub fn cache_data(&self) -> Option<&StructureRef> {
        unsafe {
            let cache_data = ffi::gst_plugin_get_cache_data(self.to_glib_none().0);
            if cache_data.is_null() {
                None
            } else {
                Some(StructureRef::from_glib_borrow(cache_data))
            }
        }
    }

    #[doc(alias = "get_plugin_flags")]
    pub fn plugin_flags(&self) -> PluginFlags {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = crate::utils::MutexGuard::lock(&(*ptr).lock);
            from_glib((*ptr).flags)
        }
    }
}
