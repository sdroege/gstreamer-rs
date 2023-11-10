use glib::translate::*;

use crate::ShmAllocator;

impl ShmAllocator {
    #[doc(alias = "gst_shm_allocator_get")]
    pub fn get() -> Option<gst::Allocator> {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gst_shm_allocator_init_once();
            from_glib_full(ffi::gst_shm_allocator_get())
        }
    }
}
