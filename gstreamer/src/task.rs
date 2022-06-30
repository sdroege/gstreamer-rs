// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Task;

use std::mem;
use std::ptr;
use std::sync::Arc;

use glib::prelude::*;
use glib::translate::*;

#[allow(clippy::type_complexity)]
pub struct TaskBuilder<F: FnMut(&Task) + Send + 'static> {
    func: Box<(F, *mut ffi::GstTask)>,
    lock: Option<TaskLock>,
    enter_callback: Option<Box<dyn FnMut(&Task) + Send + 'static>>,
    leave_callback: Option<Box<dyn FnMut(&Task) + Send + 'static>>,
}

impl<F: FnMut(&Task) + Send + 'static> TaskBuilder<F> {
    #[doc(alias = "gst_task_set_enter_callback")]
    pub fn enter_callback<E: FnMut(&Task) + Send + 'static>(self, enter_callback: E) -> Self {
        Self {
            enter_callback: Some(Box::new(enter_callback)),
            ..self
        }
    }

    #[doc(alias = "gst_task_set_leave_callback")]
    pub fn leave_callback<E: FnMut(&Task) + Send + 'static>(self, leave_callback: E) -> Self {
        Self {
            leave_callback: Some(Box::new(leave_callback)),
            ..self
        }
    }

    #[doc(alias = "gst_task_set_lock")]
    pub fn lock(self, lock: &TaskLock) -> Self {
        Self {
            lock: Some(lock.clone()),
            ..self
        }
    }

    #[doc(alias = "gst_task_new")]
    pub fn build(self) -> Task {
        unsafe extern "C" fn func_trampoline<F: FnMut(&Task) + Send + 'static>(
            user_data: glib::ffi::gpointer,
        ) {
            let callback: &mut (F, *mut ffi::GstTask) = &mut *(user_data as *mut _);
            (callback.0)(&from_glib_borrow(callback.1));
        }

        unsafe extern "C" fn destroy_func<F: FnMut(&Task) + Send + 'static>(
            data: glib::ffi::gpointer,
        ) {
            let _callback: Box<(F, *mut ffi::GstTask)> = Box::from_raw(data as *mut _);
        }

        unsafe extern "C" fn callback_trampoline(
            task: *mut ffi::GstTask,
            _thread: *mut glib::ffi::GThread,
            data: glib::ffi::gpointer,
        ) {
            let callback: &mut Box<dyn FnMut(&Task) + Send + 'static> = &mut *(data as *mut _);
            callback(&from_glib_borrow(task));
        }

        #[allow(clippy::type_complexity)]
        unsafe extern "C" fn destroy_callback(data: glib::ffi::gpointer) {
            let _callback: Box<Box<dyn FnMut(&Task) + Send + 'static>> =
                Box::from_raw(data as *mut _);
        }

        unsafe {
            let func_ptr = Box::into_raw(self.func);

            let task: Task = from_glib_full(ffi::gst_task_new(
                Some(func_trampoline::<F> as _),
                func_ptr as *mut _,
                Some(destroy_func::<F> as _),
            ));

            (*func_ptr).1 = task.to_glib_none().0;

            let lock = self.lock.unwrap_or_else(TaskLock::new);
            ffi::gst_task_set_lock(task.to_glib_none().0, mut_override(&lock.0 .0));
            task.set_data("gstreamer-rs-task-lock", Arc::clone(&lock.0));

            if let Some(enter_callback) = self.enter_callback {
                ffi::gst_task_set_enter_callback(
                    task.to_glib_none().0,
                    Some(callback_trampoline),
                    Box::into_raw(Box::new(enter_callback)) as *mut _,
                    Some(destroy_callback),
                );
            }

            if let Some(leave_callback) = self.leave_callback {
                ffi::gst_task_set_leave_callback(
                    task.to_glib_none().0,
                    Some(callback_trampoline),
                    Box::into_raw(Box::new(leave_callback)) as *mut _,
                    Some(destroy_callback),
                );
            }

            task
        }
    }
}

impl Task {
    #[doc(alias = "gst_task_new")]
    pub fn builder<F: FnMut(&Task) + Send + 'static>(func: F) -> TaskBuilder<F> {
        assert_initialized_main_thread!();
        TaskBuilder {
            func: Box::new((func, ptr::null_mut())),
            lock: None,
            enter_callback: None,
            leave_callback: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TaskLock(Arc<RecMutex>);

impl Default for TaskLock {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct RecMutex(glib::ffi::GRecMutex);

unsafe impl Send for RecMutex {}
unsafe impl Sync for RecMutex {}

#[must_use = "if unused the TaskLock will immediately unlock"]
pub struct TaskLockGuard<'a>(&'a RecMutex);

impl TaskLock {
    pub fn new() -> Self {
        unsafe {
            let lock = TaskLock(Arc::new(RecMutex(mem::zeroed())));
            glib::ffi::g_rec_mutex_init(mut_override(&lock.0 .0));
            lock
        }
    }

    // checker-ignore-item
    pub fn lock(&self) -> TaskLockGuard {
        unsafe {
            let guard = TaskLockGuard(&self.0);
            glib::ffi::g_rec_mutex_lock(mut_override(&self.0 .0));
            guard
        }
    }
}

impl Drop for RecMutex {
    fn drop(&mut self) {
        unsafe {
            glib::ffi::g_rec_mutex_clear(&mut self.0);
        }
    }
}

impl<'a> Drop for TaskLockGuard<'a> {
    fn drop(&mut self) {
        unsafe {
            glib::ffi::g_rec_mutex_unlock(mut_override(&self.0 .0));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use std::sync::mpsc::channel;

    #[test]
    fn test_simple() {
        crate::init().unwrap();

        #[derive(Debug, PartialEq, Eq)]
        enum Called {
            Enter,
            Func,
            Leave,
        }

        let (send, recv) = channel();
        let lock = TaskLock::new();

        let task = Task::builder({
            let send = send.clone();
            let mut count = 0;
            move |task| {
                count += 1;
                if count >= 3 {
                    task.pause().unwrap();
                }
                send.send(Called::Func).unwrap();
            }
        })
        .enter_callback({
            let send = send.clone();
            move |_task| {
                send.send(Called::Enter).unwrap();
            }
        })
        .leave_callback({
            move |_task| {
                send.send(Called::Leave).unwrap();
            }
        })
        .lock(&lock)
        .build();

        task.start().unwrap();

        assert_eq!(recv.recv(), Ok(Called::Enter));
        assert_eq!(recv.recv(), Ok(Called::Func));
        assert_eq!(recv.recv(), Ok(Called::Func));
        assert_eq!(recv.recv(), Ok(Called::Func));

        assert_eq!(task.state(), crate::TaskState::Paused);
        task.stop().unwrap();
        assert_eq!(recv.recv(), Ok(Called::Leave));
        task.join().unwrap();
    }
}
