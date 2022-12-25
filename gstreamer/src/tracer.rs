// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::{Plugin, Tracer};

impl Tracer {
    #[doc(alias = "gst_tracer_register")]
    pub fn register(
        plugin: Option<&Plugin>,
        name: &str,
        type_: glib::types::Type,
    ) -> Result<(), glib::error::BoolError> {
        skip_assert_initialized!();
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_tracer_register(
                    plugin.to_glib_none().0,
                    name.to_glib_none().0,
                    type_.into_glib()
                ),
                "Failed to register tracer factory"
            )
        }
    }
}
