// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use glib::IsA;

use crate::PipelineFlags;

pub trait GstPipelineExtManual: 'static {
    fn set_pipeline_flags(&self, flags: PipelineFlags);

    fn unset_pipeline_flags(&self, flags: PipelineFlags);

    fn pipeline_flags(&self) -> PipelineFlags;
}

impl<O: IsA<crate::Pipeline>> GstPipelineExtManual for O {
    fn set_pipeline_flags(&self, flags: PipelineFlags) {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = crate::utils::MutexGuard::lock(&(*ptr).lock);
            (*ptr).flags |= flags.to_glib();
        }
    }

    fn unset_pipeline_flags(&self, flags: PipelineFlags) {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = crate::utils::MutexGuard::lock(&(*ptr).lock);
            (*ptr).flags &= !flags.to_glib();
        }
    }

    fn pipeline_flags(&self) -> PipelineFlags {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = crate::utils::MutexGuard::lock(&(*ptr).lock);
            from_glib((*ptr).flags)
        }
    }
}
