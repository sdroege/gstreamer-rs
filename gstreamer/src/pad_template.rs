// Take a look at the license at the top of the repository in the LICENSE file.

use crate::PadTemplate;
use crate::StaticPadTemplate;
use glib::translate::*;

impl PadTemplate {
    #[doc(alias = "gst_pad_template_new_from_static_pad_template_with_gtype")]
    pub fn from_static_pad_template_with_gtype(
        pad_template: &StaticPadTemplate,
        pad_type: glib::types::Type,
    ) -> Result<PadTemplate, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<_>::from_glib_none(
                ffi::gst_pad_template_new_from_static_pad_template_with_gtype(
                    mut_override(pad_template.to_glib_none().0),
                    pad_type.into_glib(),
                ),
            )
            .ok_or_else(|| glib::bool_error!("Failed to create PadTemplate"))
        }
    }
}
