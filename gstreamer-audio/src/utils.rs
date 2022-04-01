// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::mut_override;

#[must_use = "if unused the Mutex will immediately unlock"]
#[doc(alias = "GMutex")]
pub struct MutexGuard<'a>(&'a glib::ffi::GMutex);

impl<'a> MutexGuard<'a> {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    #[allow(dead_code)]
    #[doc(alias = "g_mutex_lock")]
    pub fn lock(mutex: &'a glib::ffi::GMutex) -> Self {
        skip_assert_initialized!();
        unsafe {
            glib::ffi::g_mutex_lock(mut_override(mutex));
        }
        MutexGuard(mutex)
    }
}

impl<'a> Drop for MutexGuard<'a> {
    fn drop(&mut self) {
        unsafe {
            glib::ffi::g_mutex_unlock(mut_override(self.0));
        }
    }
}
