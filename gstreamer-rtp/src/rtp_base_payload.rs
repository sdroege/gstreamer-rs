use crate::RTPBasePayload;
use glib::object::IsA;
use glib::translate::*;
use std::ptr;

pub trait RTPBasePayloadExtManual: 'static {
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_rtp_base_payload_set_outcaps_structure")]
    #[doc(alias = "gst_rtp_base_payload_set_outcaps")]
    fn set_outcaps(&self, s: Option<&gst::StructureRef>) -> Result<(), glib::error::BoolError>;

    #[doc(alias = "gst_rtp_base_payload_push")]
    fn push(&self, buffer: gst::Buffer) -> Result<gst::FlowSuccess, gst::FlowError>;

    #[doc(alias = "gst_rtp_base_payload_push_list")]
    fn push_list(&self, list: gst::BufferList) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn sink_pad(&self) -> &gst::Pad;

    fn src_pad(&self) -> &gst::Pad;
}

impl<O: IsA<RTPBasePayload>> RTPBasePayloadExtManual for O {
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
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

    fn push(&self, buffer: gst::Buffer) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            try_from_glib(ffi::gst_rtp_base_payload_push(
                self.as_ref().to_glib_none().0,
                buffer.into_ptr(),
            ))
        }
    }

    fn push_list(&self, list: gst::BufferList) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            try_from_glib(ffi::gst_rtp_base_payload_push_list(
                self.as_ref().to_glib_none().0,
                list.into_ptr(),
            ))
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
