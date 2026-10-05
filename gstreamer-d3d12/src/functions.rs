// Take a look at the license at the top of the repository in the LICENSE file.

use windows::Win32::Foundation::LUID;

use crate::ffi;

#[doc(alias = "gst_d3d12_create_user_token")]
pub fn create_user_token() -> i64 {
    assert_initialized_main_thread!();
    unsafe { ffi::gst_d3d12_create_user_token() }
}

#[doc(alias = "gst_d3d12_flush_all_devices")]
pub fn flush_all_devices() {
    assert_initialized_main_thread!();
    unsafe { ffi::gst_d3d12_flush_all_devices() }
}

#[doc(alias = "gst_d3d12_luid_to_int64")]
pub fn luid_to_int64(luid: &LUID) -> i64 {
    skip_assert_initialized!();
    unsafe { ffi::gst_d3d12_luid_to_int64(luid as *const _ as glib::ffi::gconstpointer) }
}
