// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{HipKernelArgs, HipMemoryRef, ffi};
use glib::translate::*;

mod sealed {
    pub trait Sealed {}
}

pub trait HipKernelArg: sealed::Sealed {
    #[doc(hidden)]
    fn add_to(self, args: &mut HipKernelArgs);
}

macro_rules! impl_kernel_arg {
    ($($value_type:ty => $add_method:ident),+ $(,)?) => {
        $(
            impl sealed::Sealed for $value_type {}

            impl HipKernelArg for $value_type {
                fn add_to(self, args: &mut HipKernelArgs) {
                    args.$add_method(self);
                }
            }
        )+
    };
}

impl_kernel_arg! {
    u8 => add_uint8,
    i8 => add_int8,
    u16 => add_uint16,
    i16 => add_int16,
    u32 => add_uint32,
    i32 => add_int32,
    u64 => add_uint64,
    i64 => add_int64,
    f32 => add_float,
    f64 => add_double,
}

impl HipKernelArgs {
    pub fn add<T: HipKernelArg>(&mut self, value: T) -> &mut Self {
        value.add_to(self);
        self
    }

    // rustdoc-stripper-ignore-next
    /// Adds a borrowed memory argument without taking a reference.
    ///
    /// # Safety
    /// The memory must remain valid through all launches using these arguments or
    /// copies of them, and its allocation must remain alive until GPU completion.
    #[doc(alias = "gst_hip_kernel_args_add_memory")]
    pub unsafe fn add_memory(
        &mut self,
        memory: &HipMemoryRef,
        offset: usize,
    ) -> Result<(), glib::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_hip_kernel_args_add_memory(
                    self.to_glib_none_mut().0,
                    memory.upcast_memory_ref::<gst::Memory>().as_mut_ptr(),
                    offset
                ),
                "Failed to add HIP memory argument"
            )
        }
    }

    #[doc(alias = "gst_hip_kernel_args_add_value")]
    pub fn add_value(&mut self, value: &[u8], alignment: usize) -> Result<(), glib::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_hip_kernel_args_add_value(
                    self.to_glib_none_mut().0,
                    value.as_ptr().cast(),
                    value.len(),
                    alignment
                ),
                "Failed to add HIP value argument"
            )
        }
    }
}
