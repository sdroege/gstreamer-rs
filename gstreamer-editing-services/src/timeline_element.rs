use glib::translate::*;

use crate::prelude::*;
use crate::{TimelineElement, ffi};

pub trait TimelineElementExtManual: IsA<TimelineElement> + 'static {
    #[doc(alias = "ges_timeline_element_set_child_property")]
    fn set_child_property(
        &self,
        property_name: &str,
        value: impl Into<glib::Value>,
    ) -> Result<(), glib::error::BoolError> {
        self.set_child_property_by_pspec(
            self.as_ref()
                .lookup_child(property_name)
                .ok_or_else(|| glib::bool_error!("No such child property: {property_name}"))?
                .1,
            value,
        );

        Ok(())
    }

    #[doc(alias = "ges_timeline_element_set_child_property_by_pspec")]
    fn set_child_property_by_pspec(
        &self,
        pspec: impl AsRef<glib::ParamSpec>,
        value: impl Into<glib::Value>,
    ) {
        unsafe {
            ffi::ges_timeline_element_set_child_property_by_pspec(
                self.as_ref().to_glib_none().0,
                pspec.as_ref().to_glib_none().0,
                value.into().to_glib_none().0,
            );
        }
    }
}

impl<O: IsA<TimelineElement>> TimelineElementExtManual for O {}
