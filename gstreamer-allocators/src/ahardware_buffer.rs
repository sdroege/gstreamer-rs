// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

use crate::ffi;
use glib::translate::*;
use gst::{Memory, MemoryRef};

gst::memory_object_wrapper!(
    AHardwareBufferMemory,
    AHardwareBufferMemoryRef,
    gst::ffi::GstMemory,
    |mem: &gst::MemoryRef| {
        unsafe { from_glib(ffi::gst_is_ahardware_buffer_memory(mem.as_mut_ptr())) }
    },
    Memory,
    MemoryRef,
);

impl fmt::Debug for AHardwareBufferMemory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        AHardwareBufferMemoryRef::fmt(self, f)
    }
}

impl fmt::Debug for AHardwareBufferMemoryRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        MemoryRef::fmt(self, f)
    }
}

#[doc(alias = "gst_is_ahardware_buffer_buffer")]
pub fn is_ahardware_buffer_buffer(buffer: &gst::BufferRef) -> bool {
    skip_assert_initialized!();
    unsafe { from_glib(ffi::gst_is_ahardware_buffer_buffer(buffer.as_mut_ptr())) }
}

impl fmt::Display for crate::AHardwareBufferFormat {
    #[doc(alias = "gst_ahardware_buffer_format_to_caps_string")]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        assert_initialized_main_thread!();
        let format = self;
        let s = unsafe {
            glib::GString::from_glib_full(ffi::gst_ahardware_buffer_format_to_caps_string(
                format.into_glib() as u32,
            ))
        };

        f.write_str(&s)
    }
}

impl std::str::FromStr for crate::AHardwareBufferFormat {
    type Err = glib::BoolError;

    #[doc(alias = "gst_ahardware_buffer_format_from_caps_string")]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_initialized_main_thread!();
        unsafe {
            let mut format = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_ahardware_buffer_format_from_caps_string(
                s.to_glib_none().0,
                format.as_mut_ptr(),
            ));
            if ret {
                Ok(from_glib(
                    format.assume_init() as ffi::GstAHardwareBufferFormat
                ))
            } else {
                Err(glib::bool_error!("Unknown AHardwareBufferFormat"))
            }
        }
    }
}

// TODO:: Look for better way. There is no provision of user_data to wrap the callback as a closure and get it back on every invocation
// so we need to implement the find query function similar to C implementation with list/vector fo query functions.
//
//pub fn ahardware_buffer_memory_register_query_function<P: Fn(&gst::Memory) -> bool + Send + Sync + 'static>(allocator_type: glib::types::Type, query: P) {
//    unsafe { TODO: call ffi:gst_ahardware_buffer_memory_register_query_function() }
//}

impl AHardwareBufferMemoryRef {
    #[doc(alias = "gst_ahardware_buffer_memory_peek_buffer")]
    pub fn peek_buffer(&self) -> Option<std::ptr::NonNull<std::ffi::c_void>> {
        unsafe {
            let mut buffer = std::ptr::null_mut();
            if from_glib(ffi::gst_ahardware_buffer_memory_peek_buffer(
                self.as_mut_ptr(),
                &mut buffer,
            )) {
                std::ptr::NonNull::new(buffer)
            } else {
                None
            }
        }
    }
}
