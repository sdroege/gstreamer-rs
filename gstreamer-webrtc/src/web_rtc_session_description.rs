// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::{WebRTCSDPType, WebRTCSessionDescription};

impl WebRTCSessionDescription {
    #[doc(alias = "gst_webrtc_session_description_new")]
    pub fn new(type_: WebRTCSDPType, sdp: gst_sdp::SDPMessage) -> WebRTCSessionDescription {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gst_webrtc_session_description_new(
                type_.into_glib(),
                sdp.into_glib_ptr(),
            ))
        }
    }

    #[doc(alias = "get_type")]
    pub fn type_(&self) -> crate::WebRTCSDPType {
        unsafe { from_glib((*self.as_ptr()).type_) }
    }

    // rustdoc-stripper-ignore-next
    /// Changes the type of this Session Description to the specified variant.
    pub fn set_type(&mut self, type_: WebRTCSDPType) {
        unsafe {
            (*self.as_ptr()).type_ = type_.into_glib();
        }
    }

    #[doc(alias = "get_sdp")]
    pub fn sdp(&self) -> &gst_sdp::SDPMessageRef {
        unsafe { &*((*self.as_ptr()).sdp as *const gst_sdp::SDPMessageRef) }
    }

    pub fn sdp_mut(&mut self) -> &mut gst_sdp::SDPMessageRef {
        unsafe { &mut *((*self.as_ptr()).sdp as *mut gst_sdp::SDPMessageRef) }
    }
}

#[cfg(test)]
mod tests {
    use crate::WebRTCSDPType;
    use gst_sdp::SDPMessage;

    #[test]
    fn change_type() {
        gst::init().unwrap();

        let mut desc =
            crate::WebRTCSessionDescription::new(crate::WebRTCSDPType::Offer, SDPMessage::new());
        assert_eq!(desc.type_(), WebRTCSDPType::Offer);

        desc.set_type(WebRTCSDPType::Rollback);
        assert_eq!(desc.type_(), WebRTCSDPType::Rollback);
    }

    #[test]
    fn update_inner_msg() {
        gst::init().unwrap();

        let mut sdp = SDPMessage::new();
        sdp.set_information("init");

        let mut desc = crate::WebRTCSessionDescription::new(WebRTCSDPType::Offer, sdp);
        assert_eq!(desc.sdp().information(), Some("init"));

        let sdp_owned = desc.sdp().to_owned();

        // update inner sdp message
        desc.sdp_mut().set_information("update");
        assert_eq!(desc.sdp().information(), Some("update"));

        // previously acquired owned sdp message unchanged
        assert_eq!(sdp_owned.information(), Some("init"));
    }
}
