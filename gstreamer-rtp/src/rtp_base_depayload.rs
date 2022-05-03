use crate::RTPBaseDepayload;
use glib::object::IsA;
use glib::translate::*;

pub trait RTPBaseDepayloadExtManual: 'static {
    #[doc(alias = "gst_rtp_base_depayload_push")]
    fn push(&self, buffer: gst::Buffer) -> Result<gst::FlowSuccess, gst::FlowError>;

    #[doc(alias = "gst_rtp_base_depayload_push_list")]
    fn push_list(&self, list: gst::BufferList) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn sink_pad(&self) -> gst::Pad;

    fn src_pad(&self) -> gst::Pad;
}

impl<O: IsA<RTPBaseDepayload>> RTPBaseDepayloadExtManual for O {
    fn push(&self, out_buf: gst::Buffer) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            try_from_glib(ffi::gst_rtp_base_depayload_push(
                self.as_ref().to_glib_none().0,
                out_buf.into_ptr(),
            ))
        }
    }

    fn push_list(&self, out_list: gst::BufferList) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            try_from_glib(ffi::gst_rtp_base_depayload_push_list(
                self.as_ref().to_glib_none().0,
                out_list.into_ptr(),
            ))
        }
    }

    fn sink_pad(&self) -> gst::Pad {
        unsafe {
            let elt: &ffi::GstRTPBaseDepayload = &*(self.as_ptr() as *const _);
            from_glib_none(elt.sinkpad)
        }
    }

    fn src_pad(&self) -> gst::Pad {
        unsafe {
            let elt: &ffi::GstRTPBaseDepayload = &*(self.as_ptr() as *const _);
            from_glib_none(elt.srcpad)
        }
    }
}
