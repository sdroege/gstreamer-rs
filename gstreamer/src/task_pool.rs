// Take a look at the license at the top of the repository in the LICENSE file.

use std::ptr;

use glib::{ffi::gpointer, prelude::*, translate::*};

use crate::TaskPool;

unsafe extern "C" fn task_pool_trampoline<P: FnOnce() + Send + 'static>(data: gpointer) {
    let func = Box::from_raw(data as *mut P);
    func()
}

pub trait TaskPoolExtManual: 'static {
    #[doc(alias = "gst_task_pool_push")]
    fn push<P: FnOnce() + Send + 'static>(
        &self,
        func: P,
    ) -> Result<Option<TaskPoolTaskHandle>, glib::Error>;
}

impl<O: IsA<TaskPool>> TaskPoolExtManual for O {
    fn push<P: FnOnce() + Send + 'static>(
        &self,
        func: P,
    ) -> Result<Option<TaskPoolTaskHandle>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let func: Box<P> = Box::new(func);
            let func = Box::into_raw(func);

            let handle = ffi::gst_task_pool_push(
                self.as_ref().to_glib_none().0,
                Some(task_pool_trampoline::<P>),
                func as gpointer,
                &mut error,
            );

            if !error.is_null() {
                debug_assert!(handle.is_null());

                // Assume that task_pool_trampoline was
                // not called and will not be called
                drop(Box::from_raw(func));

                return Err(from_glib_full(error));
            }

            let handle = ptr::NonNull::new(handle).map(|handle| TaskPoolTaskHandle {
                handle,
                task_pool: Some(self.as_ref().clone()),
            });

            Ok(handle)
        }
    }
}

impl TaskPool {
    unsafe fn join(&self, id: ptr::NonNull<libc::c_void>) {
        ffi::gst_task_pool_join(self.to_glib_none().0, id.as_ptr())
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    unsafe fn dispose_handle(&self, id: ptr::NonNull<libc::c_void>) {
        ffi::gst_task_pool_dispose_handle(self.to_glib_none().0, id.as_ptr())
    }
}

// rustdoc-stripper-ignore-next
/// A handle for a task which was pushed to a task pool.
pub trait TaskHandle {
    // rustdoc-stripper-ignore-next
    /// Wait for the task to complete.
    fn join(self);
}

impl TaskHandle for () {
    fn join(self) {}
}

impl TaskHandle for std::convert::Infallible {
    fn join(self) {}
}

// rustdoc-stripper-ignore-next
/// An opaque handle for a task associated with a particular task pool.
///
/// Keeps a reference to the pool alive.
///
/// If the `v1_20` feature is enabled, requests the task pool to dispose of the handle when it is
/// dropped. Otherwise, needs to be `join`ed to avoid a leak.
#[cfg_attr(not(any(feature = "v1_20", feature = "dox")), must_use)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskPoolTaskHandle {
    handle: ptr::NonNull<libc::c_void>,
    task_pool: Option<TaskPool>,
}

impl TaskHandle for TaskPoolTaskHandle {
    #[doc(alias = "gst_task_pool_join")]
    fn join(mut self) {
        let task_pool = self.task_pool.take().unwrap();
        unsafe { task_pool.join(self.handle) }
    }
}

impl Drop for TaskPoolTaskHandle {
    #[doc(alias = "gst_task_pool_dispose_handle")]
    #[inline]
    fn drop(&mut self) {
        if let Some(task_pool) = self.task_pool.take() {
            cfg_if::cfg_if! {
                if #[cfg(any(feature = "v1_20", feature = "dox"))] {
                    unsafe { task_pool.dispose_handle(self.handle) }
                } else {
                    crate::warning!(crate::CAT_RUST, obj: &task_pool, "Leaked task handle");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::{channel, RecvError};

    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_simple() {
        crate::init().unwrap();
        let pool = TaskPool::new();
        pool.prepare().unwrap();

        let (sender, receiver) = channel();

        let handle = pool
            .push(move || {
                sender.send(()).unwrap();
            })
            .unwrap();

        assert_eq!(receiver.recv(), Ok(()));

        if let Some(handle) = handle {
            handle.join();
        }

        // Can't test try_recv here as the default task pool produces no
        // handles and thus no way to wait for channel destruction
        assert_eq!(receiver.recv(), Err(RecvError));

        pool.cleanup();
    }
}
