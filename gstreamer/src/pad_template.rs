// Take a look at the license at the top of the repository in the LICENSE file.

use std::ffi::CStr;

use glib::{prelude::*, translate::*};

use crate::{Caps, PadDirection, PadPresence, PadTemplate, StaticPadTemplate};

impl PadTemplate {
    #[doc(alias = "gst_pad_template_new_from_static_pad_template_with_gtype")]
    pub fn from_static_pad_template_with_gtype(
        pad_template: &StaticPadTemplate,
        pad_type: glib::types::Type,
    ) -> Result<PadTemplate, glib::BoolError> {
        skip_assert_initialized!();
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

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
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

    pub fn builder<'a>(
        name_template: &'a str,
        direction: PadDirection,
        presence: PadPresence,
        caps: &'a Caps,
    ) -> PadTemplateBuilder<'a> {
        skip_assert_initialized!();

        PadTemplateBuilder {
            name_template,
            direction,
            presence,
            caps,
            gtype: None,
            #[cfg(feature = "v1_18")]
            #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
            documentation_caps: None,
        }
    }
}

#[must_use = "The builder must be built to be used"]
#[derive(Debug)]
pub struct PadTemplateBuilder<'a> {
    name_template: &'a str,
    direction: PadDirection,
    presence: PadPresence,
    caps: &'a Caps,
    gtype: Option<glib::Type>,
    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    documentation_caps: Option<&'a Caps>,
}

impl<'a> PadTemplateBuilder<'a> {
    pub fn gtype(self, gtype: glib::Type) -> Self {
        PadTemplateBuilder {
            gtype: Some(gtype),
            ..self
        }
    }

    pub fn gtype_if_some(self, gtype: Option<glib::Type>) -> Self {
        if let Some(gtype) = gtype {
            self.gtype(gtype)
        } else {
            self
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    pub fn documentation_caps(self, documentation_caps: &'a Caps) -> Self {
        PadTemplateBuilder {
            documentation_caps: Some(documentation_caps),
            ..self
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    pub fn documentation_caps_if_some(self, documentation_caps: Option<&'a Caps>) -> Self {
        if let Some(documentation_caps) = documentation_caps {
            self.documentation_caps(documentation_caps)
        } else {
            self
        }
    }

    pub fn build(self) -> Result<PadTemplate, glib::BoolError> {
        let templ = if let Some(gtype) = self.gtype {
            PadTemplate::with_gtype(
                self.name_template,
                self.direction,
                self.presence,
                self.caps,
                gtype,
            )?
        } else {
            PadTemplate::new(self.name_template, self.direction, self.presence, self.caps)?
        };

        #[cfg(feature = "v1_18")]
        #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
        if let Some(documentation_caps) = self.documentation_caps {
            unsafe {
                ffi::gst_pad_template_set_documentation_caps(
                    templ.to_glib_none().0,
                    documentation_caps.to_glib_none().0,
                );
            }
        }

        Ok(templ)
    }
}
