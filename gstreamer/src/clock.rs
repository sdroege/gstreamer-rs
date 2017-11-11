// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use Clock;
use ClockTime;
use ClockTimeDiff;
use ClockReturn;
use std::mem;
use std::cmp;
use std::ptr;
use std::mem::transmute;
use ffi;
use glib;
use glib::IsA;
use glib::translate::*;
use glib::source::CallbackGuard;
use glib_ffi;
use glib_ffi::{gboolean, gpointer};
use libc::c_void;

glib_wrapper! {
    pub struct ClockId(Shared<c_void>);

    match fn {
        ref => |ptr| ffi::gst_clock_id_ref(ptr),
        unref => |ptr| ffi::gst_clock_id_unref(ptr),
    }
}

unsafe extern "C" fn trampoline_wait_async(
    clock: *mut ffi::GstClock,
    time: ffi::GstClockTime,
    id: gpointer,
    func: gpointer,
) -> gboolean {
    let _guard = CallbackGuard::new();
    #[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
    let f: &&(Fn(&Clock, ClockTime, &ClockId) -> bool + Send + 'static) = transmute(func);
    f(
        &from_glib_borrow(clock),
        from_glib(time),
        &from_glib_borrow(id),
    ).to_glib()
}

unsafe extern "C" fn destroy_closure_wait_async(ptr: gpointer) {
    let _guard = CallbackGuard::new();
    Box::<Box<Fn(&Clock, ClockTime, &ClockId) -> bool + Send + 'static>>::from_raw(ptr as *mut _);
}

fn into_raw_wait_async<F: Fn(&Clock, ClockTime, &ClockId) -> bool + Send + 'static>(
    func: F,
) -> gpointer {
    #[cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
    let func: Box<Box<Fn(&Clock, ClockTime, &ClockId) -> bool + Send + 'static>> =
        Box::new(Box::new(func));
    Box::into_raw(func) as gpointer
}

impl ClockId {
    pub fn get_time(&self) -> ClockTime {
        unsafe { from_glib(ffi::gst_clock_id_get_time(self.to_glib_none().0)) }
    }

    pub fn unschedule(&self) {
        unsafe { ffi::gst_clock_id_unschedule(self.to_glib_none().0) }
    }

    pub fn wait(&self) -> (ClockReturn, ClockTimeDiff) {
        unsafe {
            let mut jitter = mem::uninitialized();
            let res = ffi::gst_clock_id_wait(self.to_glib_none().0, &mut jitter);
            (from_glib(res), jitter)
        }
    }

    pub fn wait_async<F>(&self, func: F) -> ClockReturn
    where
        F: Fn(&Clock, ClockTime, &ClockId) -> bool + Send + 'static,
    {
        unsafe {
            from_glib(ffi::gst_clock_id_wait_async(
                self.to_glib_none().0,
                Some(trampoline_wait_async),
                into_raw_wait_async(func),
                Some(destroy_closure_wait_async),
            ))
        }
    }
}

impl PartialOrd for ClockId {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ClockId {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        unsafe {
            let res = ffi::gst_clock_id_compare_func(self.to_glib_none().0, other.to_glib_none().0);
            if res < 0 {
                cmp::Ordering::Less
            } else if res > 0 {
                cmp::Ordering::Greater
            } else {
                cmp::Ordering::Equal
            }
        }
    }
}

impl PartialEq for ClockId {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == cmp::Ordering::Equal
    }
}

impl Eq for ClockId {}

unsafe impl Send for ClockId {}
unsafe impl Sync for ClockId {}

pub trait ClockExtManual {
    fn new_periodic_id(&self, start_time: ClockTime, interval: ClockTime) -> Option<ClockId>;

    fn periodic_id_reinit(
        &self,
        id: &ClockId,
        start_time: ClockTime,
        interval: ClockTime,
    ) -> Result<(), glib::BoolError>;

    fn new_single_shot_id(&self, time: ClockTime) -> Option<ClockId>;

    fn single_shot_id_reinit(&self, id: &ClockId, time: ClockTime) -> Result<(), glib::BoolError>;
}

impl<O: IsA<Clock> + IsA<glib::object::Object>> ClockExtManual for O {
    fn new_periodic_id(&self, start_time: ClockTime, interval: ClockTime) -> Option<ClockId> {
        unsafe {
            from_glib_full(ffi::gst_clock_new_periodic_id(
                self.to_glib_none().0,
                start_time.to_glib(),
                interval.to_glib(),
            ))
        }
    }

    fn periodic_id_reinit(
        &self,
        id: &ClockId,
        start_time: ClockTime,
        interval: ClockTime,
    ) -> Result<(), glib::BoolError> {
        skip_assert_initialized!();
        unsafe {
            let res: bool = from_glib(ffi::gst_clock_periodic_id_reinit(
                self.to_glib_none().0,
                id.to_glib_none().0,
                start_time.to_glib(),
                interval.to_glib(),
            ));
            if res {
                Ok(())
            } else {
                Err(glib::BoolError("Failed to reinit periodic clock id"))
            }
        }
    }

    fn new_single_shot_id(&self, time: ClockTime) -> Option<ClockId> {
        unsafe {
            from_glib_full(ffi::gst_clock_new_single_shot_id(
                self.to_glib_none().0,
                time.to_glib(),
            ))
        }
    }

    fn single_shot_id_reinit(&self, id: &ClockId, time: ClockTime) -> Result<(), glib::BoolError> {
        unsafe {
            let res: bool = from_glib(ffi::gst_clock_single_shot_id_reinit(
                self.to_glib_none().0,
                id.to_glib_none().0,
                time.to_glib(),
            ));
            if res {
                Ok(())
            } else {
                Err(glib::BoolError("Failed to reinit single shot clock id"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;
    use std::sync::mpsc::channel;

    #[test]
    fn test_wait() {
        ::init().unwrap();

        let clock = SystemClock::obtain();
        let now = clock.get_time();
        let id = clock.new_single_shot_id(now + 20 * ::MSECOND).unwrap();
        let (res, _) = id.wait();

        assert!(res == ClockReturn::Ok || res == ClockReturn::Early);
    }

    #[test]
    fn test_wait_async() {
        ::init().unwrap();

        let (sender, receiver) = channel();

        let clock = SystemClock::obtain();
        let now = clock.get_time();
        let id = clock.new_single_shot_id(now + 20 * ::MSECOND).unwrap();
        let res = id.wait_async(move |_, _, _| {
            sender.send(()).unwrap();

            true
        });

        assert!(res == ClockReturn::Ok);

        assert_eq!(receiver.recv(), Ok(()));
    }
}
