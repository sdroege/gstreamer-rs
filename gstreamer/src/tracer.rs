// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Plugin, Tracer};
use glib::translate::*;

impl Tracer {
    #[doc(alias = "gst_tracer_register")]
    pub fn register(
        plugin: Option<&Plugin>,
        name: &str,
        type_: glib::types::Type,
    ) -> Result<(), glib::error::BoolError> {
        assert_initialized_main_thread!();
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
