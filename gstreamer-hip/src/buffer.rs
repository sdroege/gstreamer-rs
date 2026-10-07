// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::{HipEvent, ffi};

#[doc(alias = "gst_hip_buffer_copy_into")]
pub fn buffer_copy_into(
    dest: &mut gst::BufferRef,
    src: &gst::BufferRef,
    info: &gst_video::VideoInfo,
) -> Result<(), glib::BoolError> {
    skip_assert_initialized!();
    unsafe {
        glib::result_from_gboolean!(
            ffi::gst_hip_buffer_copy_into(
                dest.as_mut_ptr(),
                src.as_mut_ptr(),
                info.to_glib_none().0
            ),
            "Failed to copy HIP buffer"
        )
    }
}

#[doc(alias = "gst_hip_buffer_set_event")]
pub fn buffer_set_event(buffer: &mut gst::BufferRef, event: Option<&HipEvent>) {
    skip_assert_initialized!();
    unsafe {
        ffi::gst_hip_buffer_set_event(buffer.as_mut_ptr(), event.to_glib_none().0);
    }
}
