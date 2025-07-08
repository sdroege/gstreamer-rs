use super::VulkanOperation;
use glib::{prelude::*, translate::*};

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::VulkanOperation>> Sealed for T {}
}

#[derive(Debug)]
#[must_use = "Need to call `end`, otherwise drop will panic."]
pub struct VulkanOperationGuard<'a> {
    obj: &'a VulkanOperation,
    ended: bool,
}

impl VulkanOperationGuard<'_> {
    #[doc(alias = "gst_vulkan_operation_end")]
    pub fn end(mut self) -> Result<(), glib::Error> {
        self.ended = true;
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::gst_vulkan_operation_end(self.obj.to_glib_none().0, &mut error);
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

impl Drop for VulkanOperationGuard<'_> {
    fn drop(&mut self) {
        if !self.ended {
            panic!("Dropped a VulkanOperationGuard without calling `end`.")
        }
    }
}
impl PartialEq for VulkanOperationGuard<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for VulkanOperationGuard<'_> {}

pub trait VulkanOperationExtManual: sealed::Sealed + IsA<VulkanOperation> + 'static {
    // rustdoc-stripper-ignore-next
    /// Returns a guard struct for the begun operation.
    /// The `end` method on the guard **must** be called; Dropping it without results in a panic
    #[doc(alias = "gst_vulkan_operation_begin")]
    fn begin<'a>(&'a self) -> Result<VulkanOperationGuard<'a>, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::gst_vulkan_operation_begin(self.as_ref().to_glib_none().0, &mut error);
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if !error.is_null() {
                return Err(from_glib_full(error));
            }
        }
        Ok(VulkanOperationGuard {
            obj: self.upcast_ref(),
            ended: false,
        })
    }
}
impl<O: IsA<VulkanOperation>> VulkanOperationExtManual for O {}
