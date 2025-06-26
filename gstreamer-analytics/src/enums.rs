// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(feature = "v1_28")]
use crate::ffi;
#[cfg(feature = "v1_28")]
use crate::TensorDataType;

#[cfg(feature = "v1_28")]
use glib::translate::*;

#[cfg(feature = "v1_28")]
impl TensorDataType {
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
    #[doc(alias = "gst_tensor_data_type_get_name")]
    pub fn name<'a>(self) -> &'a glib::GStr {
        unsafe {
            glib::GStr::from_ptr(
                ffi::gst_tensor_data_type_get_name(self.into_glib())
                    .as_ref()
                    .expect("gst_tensor_data_type_get_name returned NULL"),
            )
        }
    }
}

#[cfg(feature = "v1_28")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
impl std::fmt::Display for TensorDataType {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
