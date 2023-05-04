use std::ptr;

use glib::{object::IsA, translate::*};

use crate::RTPBasePayload;

pub trait RTPBasePayloadExtManual: 'static {
    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_rtp_base_payload_set_outcaps_structure")]
    #[doc(alias = "gst_rtp_base_payload_set_outcaps")]
    fn set_outcaps(&self, s: Option<&gst::StructureRef>) -> Result<(), glib::error::BoolError>;

    fn sink_pad(&self) -> &gst::Pad;

    fn src_pad(&self) -> &gst::Pad;
}

impl<O: IsA<RTPBasePayload>> RTPBasePayloadExtManual for O {
    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    fn set_outcaps(&self, s: Option<&gst::StructureRef>) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_rtp_base_payload_set_outcaps_structure(
                    self.as_ref().to_glib_none().0,
                    s.as_ref()
                        .map(|s| s.as_ptr() as *mut _)
                        .unwrap_or(ptr::null_mut()),
                ),
                "Failed to negotiate by setting outcaps structure"
            )
        }
    }

    fn sink_pad(&self) -> &gst::Pad {
        unsafe {
            let elt = &*(self.as_ptr() as *const ffi::GstRTPBasePayload);
            &*(&elt.sinkpad as *const *mut gst::ffi::GstPad as *const gst::Pad)
        }
    }

    fn src_pad(&self) -> &gst::Pad {
        unsafe {
            let elt = &*(self.as_ptr() as *const ffi::GstRTPBasePayload);
            &*(&elt.srcpad as *const *mut gst::ffi::GstPad as *const gst::Pad)
        }
    }
}
