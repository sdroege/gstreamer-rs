// This file was generated by gir (https://github.com/gtk-rs/gir @ d1e0127)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use Bin;
use ClockTime;
use DebugGraphDetails;
use DebugLevel;
use Element;
use Error;
#[cfg(any(feature = "v1_12", feature = "dox"))]
use StackTraceFlags;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use std;
use std::mem;
use std::ptr;


#[cfg(any(feature = "v1_14", feature = "dox"))]
pub fn debug_add_ring_buffer_logger(max_size_per_thread: u32, thread_timeout: u32) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_debug_add_ring_buffer_logger(max_size_per_thread, thread_timeout);
    }
}

pub fn debug_bin_to_dot_data<P: IsA<Bin>>(bin: &P, details: DebugGraphDetails) -> String {
    skip_assert_initialized!();
    unsafe {
        from_glib_full(ffi::gst_debug_bin_to_dot_data(bin.to_glib_none().0, details.to_glib()))
    }
}

pub fn debug_bin_to_dot_file<P: IsA<Bin>, Q: AsRef<std::path::Path>>(bin: &P, details: DebugGraphDetails, file_name: Q) {
    skip_assert_initialized!();
    unsafe {
        ffi::gst_debug_bin_to_dot_file(bin.to_glib_none().0, details.to_glib(), file_name.as_ref().to_glib_none().0);
    }
}

pub fn debug_bin_to_dot_file_with_ts<P: IsA<Bin>, Q: AsRef<std::path::Path>>(bin: &P, details: DebugGraphDetails, file_name: Q) {
    skip_assert_initialized!();
    unsafe {
        ffi::gst_debug_bin_to_dot_file_with_ts(bin.to_glib_none().0, details.to_glib(), file_name.as_ref().to_glib_none().0);
    }
}

pub fn debug_get_default_threshold() -> DebugLevel {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gst_debug_get_default_threshold())
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
pub fn debug_get_stack_trace(flags: StackTraceFlags) -> Option<String> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gst_debug_get_stack_trace(flags.to_glib()))
    }
}

pub fn debug_is_active() -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gst_debug_is_active())
    }
}

pub fn debug_is_colored() -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gst_debug_is_colored())
    }
}

pub fn debug_print_stack_trace() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_debug_print_stack_trace();
    }
}

#[cfg(any(feature = "v1_14", feature = "dox"))]
pub fn debug_remove_ring_buffer_logger() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_debug_remove_ring_buffer_logger();
    }
}

#[cfg(any(feature = "v1_14", feature = "dox"))]
pub fn debug_ring_buffer_logger_get_logs() -> Vec<String> {
    assert_initialized_main_thread!();
    unsafe {
        FromGlibPtrContainer::from_glib_full(ffi::gst_debug_ring_buffer_logger_get_logs())
    }
}

pub fn debug_set_active(active: bool) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_debug_set_active(active.to_glib());
    }
}

pub fn debug_set_colored(colored: bool) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_debug_set_colored(colored.to_glib());
    }
}

pub fn debug_set_default_threshold(level: DebugLevel) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_debug_set_default_threshold(level.to_glib());
    }
}

pub fn debug_set_threshold_for_name(name: &str, level: DebugLevel) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_debug_set_threshold_for_name(name.to_glib_none().0, level.to_glib());
    }
}

pub fn debug_set_threshold_from_string(list: &str, reset: bool) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_debug_set_threshold_from_string(list.to_glib_none().0, reset.to_glib());
    }
}

pub fn debug_unset_threshold_for_name(name: &str) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_debug_unset_threshold_for_name(name.to_glib_none().0);
    }
}

#[cfg(any(feature = "v1_14", feature = "dox"))]
pub fn get_main_executable_path() -> Option<String> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_none(ffi::gst_get_main_executable_path())
    }
}

pub fn parse_bin_from_description(bin_description: &str, ghost_unlinked_pads: bool) -> Result<Bin, Error> {
    assert_initialized_main_thread!();
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::gst_parse_bin_from_description(bin_description.to_glib_none().0, ghost_unlinked_pads.to_glib(), &mut error);
        if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
    }
}

pub fn parse_launch(pipeline_description: &str) -> Result<Element, Error> {
    assert_initialized_main_thread!();
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::gst_parse_launch(pipeline_description.to_glib_none().0, &mut error);
        if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
    }
}

pub fn parse_launchv(argv: &[&str]) -> Result<Element, Error> {
    assert_initialized_main_thread!();
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::gst_parse_launchv(argv.to_glib_none().0, &mut error);
        if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
    }
}

#[cfg(any(feature = "v1_14", feature = "dox"))]
pub fn protection_filter_systems_by_available_decryptors(system_identifiers: &str) -> Vec<String> {
    assert_initialized_main_thread!();
    unsafe {
        FromGlibPtrContainer::from_glib_full(ffi::gst_protection_filter_systems_by_available_decryptors(system_identifiers.to_glib_none().0))
    }
}

pub fn update_registry() -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gst_update_registry())
    }
}

pub fn util_get_timestamp() -> ClockTime {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gst_util_get_timestamp())
    }
}

pub fn version() -> (u32, u32, u32, u32) {
    assert_initialized_main_thread!();
    unsafe {
        let mut major = mem::uninitialized();
        let mut minor = mem::uninitialized();
        let mut micro = mem::uninitialized();
        let mut nano = mem::uninitialized();
        ffi::gst_version(&mut major, &mut minor, &mut micro, &mut nano);
        (major, minor, micro, nano)
    }
}

pub fn version_string() -> String {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gst_version_string())
    }
}
