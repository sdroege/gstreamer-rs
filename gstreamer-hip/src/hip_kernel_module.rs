// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{HipCompiledBlob, HipDevice, HipKernelArgs, HipKernelModule, HipStream, ffi};
use glib::prelude::*;
use glib::translate::*;

impl HipKernelModule {
    #[doc(alias = "gst_hip_kernel_module_new_from_blob")]
    pub fn from_blob(
        device: &impl IsA<HipDevice>,
        blob: &HipCompiledBlob,
    ) -> Result<Self, glib::Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = std::ptr::null_mut();
            let module = ffi::gst_hip_kernel_module_new_from_blob(
                device.as_ref().to_glib_none().0,
                blob.as_ptr(),
                &mut error,
            );

            if !error.is_null() {
                Err(from_glib_full(error))
            } else if module.is_null() {
                Err(glib::Error::new(
                    gst::LibraryError::Failed,
                    "Failed to load HIP module",
                ))
            } else {
                Ok(from_glib_full(module))
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Launches a kernel asynchronously, using the default stream for `None`.
    ///
    /// # Safety
    /// Arguments must match the kernel ABI and referenced resources must remain
    /// valid until GPU completion.
    #[doc(alias = "gst_hip_kernel_module_launch")]
    pub unsafe fn launch(
        &self,
        entry_point: &str,
        stream: Option<&HipStream>,
        grid: [u32; 3],
        block: [u32; 3],
        args: &HipKernelArgs,
    ) -> Result<(), glib::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_hip_kernel_module_launch(
                    self.to_glib_none().0,
                    entry_point.to_glib_none().0,
                    stream.to_glib_none().0,
                    grid[0],
                    grid[1],
                    grid[2],
                    block[0],
                    block[1],
                    block[2],
                    args.to_glib_none().0
                ),
                "Failed to launch HIP kernel"
            )
        }
    }
}
