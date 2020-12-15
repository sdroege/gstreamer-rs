// Take a look at the license at the top of the repository in the LICENSE file.

use crate::WebRTCDataChannel;
use glib::translate::*;

use std::mem;

impl WebRTCDataChannel {
    pub fn on_error(&self, error: glib::Error) {
        let error = mem::ManuallyDrop::new(error);
        unsafe {
            ffi::gst_webrtc_data_channel_on_error(
                self.to_glib_none().0,
                mut_override(error.to_glib_none().0),
            );
        }
    }
}
