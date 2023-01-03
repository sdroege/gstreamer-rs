use glib::object::IsA;

use crate::RTPBaseDepayload;

pub trait RTPBaseDepayloadExtManual: 'static {
    fn sink_pad(&self) -> &gst::Pad;

    fn src_pad(&self) -> &gst::Pad;
}

impl<O: IsA<RTPBaseDepayload>> RTPBaseDepayloadExtManual for O {
    fn sink_pad(&self) -> &gst::Pad {
        unsafe {
            let elt = &*(self.as_ptr() as *const ffi::GstRTPBaseDepayload);
            &*(&elt.sinkpad as *const *mut gst::ffi::GstPad as *const gst::Pad)
        }
    }

    fn src_pad(&self) -> &gst::Pad {
        unsafe {
            let elt = &*(self.as_ptr() as *const ffi::GstRTPBaseDepayload);
            &*(&elt.srcpad as *const *mut gst::ffi::GstPad as *const gst::Pad)
        }
    }
}
