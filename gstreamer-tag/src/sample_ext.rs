// Take a look at the license at the top of the repository in the LICENSE file.
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{ffi, TagImageType};
use glib::translate::{from_glib_full, IntoGlib as _, ToGlibPtr as _};

pub trait ImageSampleExt: Sized {
    // rustdoc-stripper-ignore-next
    /// # Example
    /// ```
    /// # use gstreamer_tag as gst_tag;
    /// # fn test() -> Result<(), glib::BoolError> {
    /// # let data = Vec::new();
    /// use gst_tag::prelude::*;
    /// // let data: Vec<u8> = ...;
    /// let sample = gst::Sample::from_image_data(&data, gst_tag::TagImageType::FrontCover)?;
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "gst_tag_image_data_to_image_sample")]
    fn from_image_data(data: &[u8], image_type: TagImageType) -> Result<Self, glib::BoolError>;
}

impl ImageSampleExt for gst::Sample {
    fn from_image_data(data: &[u8], image_type: TagImageType) -> Result<Self, glib::BoolError> {
        assert_initialized_main_thread!();
        let data_len = u32::try_from(data.len())
            .map_err(|_| glib::bool_error!("image is larger than 4GiB"))?;
        let sample: Option<_> = unsafe {
            from_glib_full(ffi::gst_tag_image_data_to_image_sample(
                data.to_glib_none().0,
                data_len,
                image_type.into_glib(),
            ))
        };
        sample.ok_or_else(|| glib::bool_error!("invalid or unsupported image data"))
    }
}
