// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(any(feature = "v1_20", feature = "dox"))]
use crate::Element;
use crate::ElementFactory;

#[cfg(any(feature = "v1_20", feature = "dox"))]
use glib::prelude::*;
#[cfg(any(feature = "v1_20", feature = "dox"))]
use glib::translate::*;

impl ElementFactory {
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_element_factory_create_with_properties")]
    pub fn create_with_properties(
        &self,
        properties: &[(&str, &dyn ToValue)],
    ) -> Result<Element, glib::BoolError> {
        assert_initialized_main_thread!();
        let n = properties.len() as u32;
        let names = properties.iter().map(|(name, _)| *name).collect::<Vec<_>>();
        let values = properties
            .iter()
            .map(|(_, value)| value.to_value())
            .collect::<Vec<_>>();

        unsafe {
            Option::<_>::from_glib_none(ffi::gst_element_factory_create_with_properties(
                self.to_glib_none().0,
                n,
                names.to_glib_none().0,
                values.as_ptr() as *const glib::gobject_ffi::GValue,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to create element from factory"))
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_element_factory_make_with_properties")]
    pub fn make_with_properties(
        factoryname: &str,
        properties: &[(&str, &dyn ToValue)],
    ) -> Result<Element, glib::BoolError> {
        assert_initialized_main_thread!();
        let n = properties.len() as u32;
        let names = properties.iter().map(|(name, _)| *name).collect::<Vec<_>>();
        let values = properties
            .iter()
            .map(|(_, value)| value.to_value())
            .collect::<Vec<_>>();

        assert_initialized_main_thread!();
        unsafe {
            Option::<_>::from_glib_none(ffi::gst_element_factory_make_with_properties(
                factoryname.to_glib_none().0,
                n,
                names.to_glib_none().0,
                values.as_ptr() as *const glib::gobject_ffi::GValue,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to create element from factory name"))
        }
    }
}
