// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{HipEvent, ffi};
use glib::translate::*;

impl HipEvent {
    #[doc(alias = "gst_hip_event_set_notify")]
    pub fn set_notify<F: FnOnce() + Send + 'static>(&self, notify: F) {
        let notify = Box::into_raw(Box::new(notify));

        unsafe extern "C" fn trampoline<F: FnOnce() + Send + 'static>(data: glib::ffi::gpointer) {
            unsafe {
                let notify = Box::from_raw(data as *mut F);
                notify()
            }
        }

        unsafe {
            ffi::gst_hip_event_set_notify(
                self.to_glib_none().0,
                notify as glib::ffi::gpointer,
                Some(trampoline::<F>),
            );
        }
    }

    #[doc(alias = "gst_hip_event_synchronize")]
    pub fn synchronize(&self) -> Result<(), glib::BoolError> {
        let status = unsafe { ffi::gst_hip_event_synchronize(self.to_glib_none().0) };
        if status == 0 {
            Ok(())
        } else {
            Err(glib::bool_error!(
                "Failed to synchronize HIP event: {status}"
            ))
        }
    }
}
