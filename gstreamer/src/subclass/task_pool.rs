// Take a look at the license at the top of the repository in the LICENSE file.

use super::prelude::*;
use crate::{TaskHandle, TaskPool};

use crate::gst_warning;

use std::hash::{Hash, Hasher};
use std::ptr;
use std::sync::{Arc, Mutex};

use glib::ffi::gpointer;
use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;

pub trait TaskPoolImpl: GstObjectImpl + Send + Sync {
    // rustdoc-stripper-ignore-next
    /// Handle to be returned from the `push` function to allow the caller to wait for the task's
    /// completion.
    ///
    /// If unneeded, you can specify `()` or [`Infallible`](std::convert::Infallible) for a handle
    /// that does nothing on `join` or drop.
    type Handle: TaskHandle;

    // rustdoc-stripper-ignore-next
    /// Prepare the task pool to accept tasks.
    ///
    /// This defaults to doing nothing.
    fn prepare(&self, _task_pool: &Self::Type) -> Result<(), glib::Error> {
        Ok(())
    }

    // rustdoc-stripper-ignore-next
    /// Clean up, rejecting further tasks and waiting for all accepted tasks to be stopped.
    ///
    /// This is mainly used internally to ensure proper cleanup of internal data structures in test
    /// suites.
    fn cleanup(&self, _task_pool: &Self::Type) {}

    // rustdoc-stripper-ignore-next
    /// Deliver a task to the pool.
    ///
    /// If returning `Ok`, you need to call the `func` eventually.
    ///
    /// If returning `Err`, the `func` must be dropped without calling it.
    fn push(
        &self,
        task_pool: &Self::Type,
        func: TaskPoolFunction,
    ) -> Result<Option<Self::Handle>, glib::Error>;
}

unsafe impl<T: TaskPoolImpl> IsSubclassable<T> for TaskPool {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);
        let klass = klass.as_mut();
        klass.prepare = Some(task_pool_prepare::<T>);
        klass.cleanup = Some(task_pool_cleanup::<T>);
        klass.push = Some(task_pool_push::<T>);
        klass.join = Some(task_pool_join::<T>);

        #[cfg(any(feature = "v1_20", feature = "dox"))]
        {
            klass.dispose_handle = Some(task_pool_dispose_handle::<T>);
        }
    }
}

unsafe extern "C" fn task_pool_prepare<T: TaskPoolImpl>(
    ptr: *mut ffi::GstTaskPool,
    error: *mut *mut glib::ffi::GError,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<TaskPool> = from_glib_borrow(ptr);

    match imp.prepare(wrap.unsafe_cast_ref()) {
        Ok(()) => {}
        Err(err) => {
            if !error.is_null() {
                *error = err.into_raw();
            }
        }
    }
}

unsafe extern "C" fn task_pool_cleanup<T: TaskPoolImpl>(ptr: *mut ffi::GstTaskPool) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<TaskPool> = from_glib_borrow(ptr);

    imp.cleanup(wrap.unsafe_cast_ref());
}

unsafe extern "C" fn task_pool_push<T: TaskPoolImpl>(
    ptr: *mut ffi::GstTaskPool,
    func: ffi::GstTaskPoolFunction,
    user_data: gpointer,
    error: *mut *mut glib::ffi::GError,
) -> gpointer {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<TaskPool> = from_glib_borrow(ptr);

    let func = TaskPoolFunction::new(func.expect("Tried to push null func"), user_data);

    match imp.push(wrap.unsafe_cast_ref(), func.clone()) {
        Ok(None) => ptr::null_mut(),
        Ok(Some(handle)) => Box::into_raw(Box::new(handle)) as gpointer,
        Err(err) => {
            func.prevent_call();
            if !error.is_null() {
                *error = err.into_raw();
            }
            ptr::null_mut()
        }
    }
}

unsafe extern "C" fn task_pool_join<T: TaskPoolImpl>(ptr: *mut ffi::GstTaskPool, id: gpointer) {
    let wrap: Borrowed<TaskPool> = from_glib_borrow(ptr);

    if id.is_null() {
        gst_warning!(crate::CAT_RUST, obj: wrap.as_ref(), "Tried to join null handle");
        return;
    }

    let handle = Box::from_raw(id as *mut T::Handle);
    handle.join();
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
unsafe extern "C" fn task_pool_dispose_handle<T: TaskPoolImpl>(
    ptr: *mut ffi::GstTaskPool,
    id: gpointer,
) {
    let wrap: Borrowed<TaskPool> = from_glib_borrow(ptr);

    if id.is_null() {
        gst_warning!(crate::CAT_RUST, obj: wrap.as_ref(), "Tried to dispose null handle");
        return;
    }

    let handle = Box::from_raw(id as *mut T::Handle);
    drop(handle);
}

// rustdoc-stripper-ignore-next
/// Function the task pool should execute, provided to [`push`](TaskPoolImpl::push).
#[derive(Debug)]
pub struct TaskPoolFunction(Arc<Mutex<Option<TaskPoolFunctionInner>>>);

// `Arc<Mutex<Option<…>>>` is required so that we can enforce that the function
// has not been called and will never be called after `push` returns `Err`.

#[derive(Debug)]
struct TaskPoolFunctionInner {
    func: unsafe extern "C" fn(gpointer),
    user_data: gpointer,
    warn_on_drop: bool,
}

unsafe impl Send for TaskPoolFunctionInner {}

impl TaskPoolFunction {
    fn new(func: unsafe extern "C" fn(gpointer), user_data: gpointer) -> Self {
        let inner = TaskPoolFunctionInner {
            func,
            user_data,
            warn_on_drop: true,
        };
        Self(Arc::new(Mutex::new(Some(inner))))
    }

    fn clone(&self) -> Self {
        Self(self.0.clone())
    }

    // rustdoc-stripper-ignore-next
    /// Consume and execute the function.
    pub fn call(self) {
        let mut inner = self
            .0
            .lock()
            .unwrap()
            .take()
            .expect("TaskPoolFunction has already been dropped");
        inner.warn_on_drop = false;
        unsafe { (inner.func)(inner.user_data) }
    }

    fn prevent_call(self) {
        let mut inner = self
            .0
            .lock()
            .unwrap()
            .take()
            .expect("TaskPoolFunction has already been called");
        inner.warn_on_drop = false;
        drop(inner);
    }

    fn as_ptr(&self) -> *const Mutex<Option<TaskPoolFunctionInner>> {
        Arc::as_ptr(&self.0)
    }
}

impl Drop for TaskPoolFunctionInner {
    fn drop(&mut self) {
        if self.warn_on_drop {
            gst_warning!(crate::CAT_RUST, "Leaked task function");
        }
    }
}

impl PartialEq for TaskPoolFunction {
    fn eq(&self, other: &Self) -> bool {
        self.as_ptr() == other.as_ptr()
    }
}

impl Eq for TaskPoolFunction {}

impl PartialOrd for TaskPoolFunction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_ptr().partial_cmp(&other.as_ptr())
    }
}

impl Ord for TaskPoolFunction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_ptr().cmp(&other.as_ptr())
    }
}

impl Hash for TaskPoolFunction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_ptr().hash(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use std::sync::atomic;
    use std::sync::mpsc::{channel, TryRecvError};
    use std::thread;

    pub mod imp {
        use super::*;

        #[derive(Default)]
        pub struct TestPool {
            pub(super) prepared: atomic::AtomicBool,
            pub(super) cleaned_up: atomic::AtomicBool,
        }

        #[glib::object_subclass]
        impl ObjectSubclass for TestPool {
            const NAME: &'static str = "TestPool";
            type Type = super::TestPool;
            type ParentType = TaskPool;
        }

        impl ObjectImpl for TestPool {}

        impl GstObjectImpl for TestPool {}

        impl TaskPoolImpl for TestPool {
            type Handle = TestHandle;

            fn prepare(&self, _task_pool: &Self::Type) -> Result<(), glib::Error> {
                self.prepared.store(true, atomic::Ordering::SeqCst);
                Ok(())
            }

            fn cleanup(&self, _task_pool: &Self::Type) {
                self.cleaned_up.store(true, atomic::Ordering::SeqCst);
            }

            fn push(
                &self,
                _task_pool: &Self::Type,
                func: TaskPoolFunction,
            ) -> Result<Option<Self::Handle>, glib::Error> {
                let handle = thread::spawn(move || func.call());
                Ok(Some(TestHandle(handle)))
            }
        }

        pub struct TestHandle(thread::JoinHandle<()>);

        impl TaskHandle for TestHandle {
            fn join(self) {
                self.0.join().unwrap();
            }
        }
    }

    glib::wrapper! {
        pub struct TestPool(ObjectSubclass<imp::TestPool>) @extends TaskPool, crate::Object;
    }

    unsafe impl Send for TestPool {}
    unsafe impl Sync for TestPool {}

    impl TestPool {
        pub fn new() -> Self {
            Self::default()
        }
    }

    impl Default for TestPool {
        fn default() -> Self {
            glib::Object::new(&[]).unwrap()
        }
    }

    #[test]
    fn test_simple_subclass() {
        crate::init().unwrap();

        let pool = TestPool::new();
        pool.prepare().unwrap();

        let (sender, receiver) = channel();

        let handle = pool
            .push(move || {
                sender.send(()).unwrap();
            })
            .unwrap();
        let handle = handle.unwrap();

        assert_eq!(receiver.recv(), Ok(()));

        handle.join();
        assert_eq!(receiver.try_recv(), Err(TryRecvError::Disconnected));

        pool.cleanup();

        let imp = imp::TestPool::from_instance(&pool);
        assert!(imp.prepared.load(atomic::Ordering::SeqCst));
        assert!(imp.cleaned_up.load(atomic::Ordering::SeqCst));
    }
}
