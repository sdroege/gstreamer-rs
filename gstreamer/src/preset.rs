// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;

use crate::Preset;

pub trait PresetExtManual: IsA<Preset> + 'static {
    #[cfg(feature = "v1_30")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
    #[doc(alias = "gst_preset_get_property")]
    #[doc(alias = "get_property")]
    fn property(&self, name: &str, prop: &str) -> Result<glib::Value, glib::BoolError> {
        use glib::translate::*;
        unsafe {
            let mut value = glib::Value::uninitialized();
            let ret = from_glib(crate::ffi::gst_preset_get_property(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                prop.to_glib_none().0,
                value.to_glib_none_mut().0,
            ));
            if ret {
                Ok(value)
            } else {
                Err(glib::bool_error!(
                    "Failed to get {prop} for the preset {name}"
                ))
            }
        }
    }

    #[cfg(feature = "v1_30")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_30")))]
    #[doc(alias = "gst_preset_get_property_alternatives")]
    #[doc(alias = "get_property_alternatives")]
    fn property_alternatives(&self, name: &str, prop: &str) -> Result<glib::StrV, glib::BoolError> {
        use glib::translate::*;
        unsafe {
            let arr = crate::ffi::gst_preset_get_property_alternatives(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                prop.to_glib_none().0,
            );

            if arr.is_null() {
                Err(glib::bool_error!(
                    "Failed get alternatives to property {prop} for the preset {name}"
                ))
            } else {
                Ok(FromGlibPtrContainer::from_glib_full(arr))
            }
        }
    }
}

impl<O: glib::prelude::IsA<Preset>> PresetExtManual for O {}
