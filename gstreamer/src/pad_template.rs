// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Caps;
use crate::PadDirection;
use crate::PadPresence;
use crate::PadTemplate;
use crate::StaticPadTemplate;
use glib::prelude::*;
use glib::translate::*;
use std::ffi::CStr;

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

    #[doc(alias = "gst_pad_template_get_caps")]
    #[doc(alias = "get_caps")]
    pub fn caps(&self) -> &Caps {
        unsafe {
            let templ = &*(self.as_ptr() as *const ffi::GstPadTemplate);
            &*(&templ.caps as *const *mut ffi::GstCaps as *const Caps)
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_pad_template_get_documentation_caps")]
    #[doc(alias = "get_documentation_caps")]
    pub fn documentation_caps(&self) -> &Caps {
        unsafe {
            let templ = &*(self.as_ptr() as *const ffi::GstPadTemplate);
            if !templ.ABI.abi.documentation_caps.is_null() {
                &*(&templ.ABI.abi.documentation_caps as *const *mut ffi::GstCaps as *const Caps)
            } else {
                &*(&templ.caps as *const *mut ffi::GstCaps as *const Caps)
            }
        }
    }

    pub fn direction(&self) -> PadDirection {
        unsafe {
            let templ = &*(self.as_ptr() as *const ffi::GstPadTemplate);
            from_glib(templ.direction)
        }
    }

    pub fn gtype(&self) -> glib::types::Type {
        unsafe {
            let templ = &*(self.as_ptr() as *const ffi::GstPadTemplate);
            from_glib(templ.ABI.abi.gtype)
        }
    }

    #[doc(alias = "name-template")]
    pub fn name_template(&self) -> &str {
        unsafe {
            let templ = &*(self.as_ptr() as *const ffi::GstPadTemplate);
            CStr::from_ptr(templ.name_template).to_str().unwrap()
        }
    }

    pub fn presence(&self) -> PadPresence {
        unsafe {
            let templ = &*(self.as_ptr() as *const ffi::GstPadTemplate);
            from_glib(templ.presence)
        }
    }
}
