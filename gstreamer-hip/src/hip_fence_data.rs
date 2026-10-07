// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::{HipFenceData, ffi};

impl HipFenceData {
    #[doc(alias = "gst_hip_fence_data_push")]
    pub fn push<F>(&self, func: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let f: Box<F> = Box::new(func);
        let f = Box::into_raw(f);

        unsafe extern "C" fn trampoline<F: FnOnce() + Send + 'static>(data: glib::ffi::gpointer) {
            unsafe {
                let func = Box::from_raw(data as *mut F);
                func()
            }
        }

        unsafe {
            ffi::gst_hip_fence_data_push(
                self.to_glib_none().0,
                f as glib::ffi::gpointer,
                Some(trampoline::<F>),
            )
        }
    }
}
