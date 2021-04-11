// Take a look at the license at the top of the repository in the LICENSE file.

use crate::WebRTCDataChannel;
use glib::translate::*;

impl WebRTCDataChannel {
    pub fn on_error(&self, error: glib::Error) {
        unsafe {
            ffi::gst_webrtc_data_channel_on_error(self.to_glib_none().0, error.into_raw());
        }
    }
}
