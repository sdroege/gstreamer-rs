// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{HipDevice, ffi};
use glib::{prelude::*, translate::*};
use std::{ffi::c_void, ptr::NonNull};

#[derive(Debug)]
pub struct HipCompiledBlob(NonNull<c_void>);

impl HipCompiledBlob {
    pub(crate) fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }
}

// The allocation is immutable after compilation and is freed only by its owner.
unsafe impl Send for HipCompiledBlob {}
unsafe impl Sync for HipCompiledBlob {}

impl Drop for HipCompiledBlob {
    fn drop(&mut self) {
        unsafe { glib::ffi::g_free(self.0.as_ptr()) }
    }
}

#[doc(alias = "gst_hip_rtc_compile")]
pub fn rtc_compile(
    device: &impl IsA<HipDevice>,
    source: &str,
    options: &[&str],
) -> Result<HipCompiledBlob, glib::BoolError> {
    assert_initialized_main_thread!();
    let num_options = u32::try_from(options.len())
        .map_err(|_| glib::bool_error!("Too many HIP compiler options"))?;
    unsafe {
        let blob = ffi::gst_hip_rtc_compile(
            device.as_ref().to_glib_none().0,
            source.to_glib_none().0,
            options.to_glib_none().0,
            num_options,
        );
        NonNull::new(blob.cast())
            .map(HipCompiledBlob)
            .ok_or_else(|| glib::bool_error!("Failed to compile HIP kernel"))
    }
}
