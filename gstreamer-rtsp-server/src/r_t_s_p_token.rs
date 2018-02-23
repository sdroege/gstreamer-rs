use RTSPToken;
use glib::value::ToSendValue;
use gst;
use ffi;
use glib::translate::*;

impl RTSPToken {
    pub fn new(values: &[(&str, &ToSendValue)]) -> RTSPToken {
        let mut token = RTSPToken::new_empty();

        {
            let structure = token.writable_structure().unwrap();

            for &(f, v) in values {
                structure.set_value(f, v.to_send_value());
            }
        }

        token
    }

    pub fn writable_structure(&mut self) -> Option<&mut gst::StructureRef> {
        unsafe {
            let structure = ffi::gst_rtsp_token_writable_structure(self.to_glib_none_mut().0);
            if structure.is_null() {
                None
            } else {
                Some(gst::StructureRef::from_glib_borrow_mut(structure))
            }
        }
    }
}
