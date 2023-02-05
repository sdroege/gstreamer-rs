// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::mut_override;

#[must_use = "if unused the Mutex will immediately unlock"]
#[doc(alias = "GMutex")]
pub struct MutexGuard<'a>(&'a glib::ffi::GMutex);

impl<'a> MutexGuard<'a> {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    #[doc(alias = "g_mutex_lock")]
    #[inline]
    pub fn lock(mutex: &'a glib::ffi::GMutex) -> Self {
        skip_assert_initialized!();
        unsafe {
            glib::ffi::g_mutex_lock(mut_override(mutex));
        }
        MutexGuard(mutex)
    }
}

impl<'a> Drop for MutexGuard<'a> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            glib::ffi::g_mutex_unlock(mut_override(self.0));
        }
    }
}

// rustdoc-stripper-ignore-next
/// Trait that allows accessing `Display` implementation on types external to this crate.
pub trait Displayable {
    type DisplayImpl: std::fmt::Display;

    fn display(self) -> Self::DisplayImpl;
}

#[must_use = "if unused the object lock will immediately be released"]
pub struct ObjectLockGuard<'a, T: ?Sized> {
    obj: &'a T,
    mutex: &'a mut glib::ffi::GMutex,
}

impl<'a, T> ObjectLockGuard<'a, T>
where
    T: glib::IsA<crate::Object> + ?Sized,
{
    #[inline]
    pub fn acquire(obj: &'a T) -> ObjectLockGuard<'a, T> {
        skip_assert_initialized!();
        unsafe {
            use glib::ObjectType;
            let mutex = &mut (*obj.as_ref().as_ptr()).lock;
            glib::ffi::g_mutex_lock(mutex);
            Self { obj, mutex }
        }
    }
}

impl<T> AsRef<T> for ObjectLockGuard<'_, T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self.obj
    }
}

impl<T> std::ops::Deref for ObjectLockGuard<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.obj
    }
}

impl<T> std::fmt::Debug for ObjectLockGuard<'_, T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.obj.fmt(f)
    }
}

impl<T> std::cmp::PartialEq for ObjectLockGuard<'_, T>
where
    T: std::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.obj.eq(other)
    }
}

impl<T> std::cmp::Eq for ObjectLockGuard<'_, T> where T: std::cmp::Eq {}

impl<T> Drop for ObjectLockGuard<'_, T>
where
    T: ?Sized,
{
    #[inline]
    fn drop(&mut self) {
        unsafe {
            glib::ffi::g_mutex_unlock(self.mutex);
        }
    }
}
