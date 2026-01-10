// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::{DynamicTypeFactory, Plugin, ffi};

impl DynamicTypeFactory {
    #[doc(alias = "gst_dynamic_type_register")]
    pub fn register(
        plugin: Option<&Plugin>,
        type_: glib::types::Type,
    ) -> Result<(), glib::error::BoolError> {
        skip_assert_initialized!();
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_dynamic_type_register(plugin.to_glib_none().0, type_.into_glib()),
                "Failed to register dynamic type factory"
            )
        }
    }
}
