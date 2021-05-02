// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Plugin;
use crate::PluginFlags;
use crate::Structure;
use crate::StructureRef;

use glib::prelude::*;
use glib::translate::*;

impl Plugin {
    #[doc(alias = "get_cache_data")]
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

    pub fn set_cache_data(&self, cache_data: Structure) {
        unsafe {
            ffi::gst_plugin_set_cache_data(self.to_glib_none().0, cache_data.into_ptr());
        }
    }
}

pub trait GstPluginExtManual: 'static {
    #[doc(alias = "get_plugin_flags")]
    fn plugin_flags(&self) -> PluginFlags;

    #[doc(alias = "get_plugin_name")]
    fn plugin_name(&self) -> glib::GString;
}

impl<O: IsA<crate::Plugin>> GstPluginExtManual for O {
    fn plugin_flags(&self) -> PluginFlags {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = crate::utils::MutexGuard::lock(&(*ptr).lock);
            from_glib((*ptr).flags)
        }
    }

    fn plugin_name(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::gst_plugin_get_name(self.as_ref().to_glib_none().0)) }
    }
}
