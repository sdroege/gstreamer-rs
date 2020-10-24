// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use glib::prelude::*;
use glib::translate::*;
use glib::IsA;
use glib_sys::{gboolean, gpointer};
use gst_sys;
use libc::c_void;
use std::cmp;
use std::ptr;
use Clock;
use ClockEntryType;
use ClockError;
use ClockFlags;
use ClockReturn;
use ClockSuccess;
use ClockTime;
use ClockTimeDiff;

use futures_core::Stream;
use std::marker::Unpin;
use std::pin::Pin;
use std::sync::atomic;
use std::sync::atomic::AtomicI32;

glib_wrapper! {
    #[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
    pub struct ClockId(Shared<c_void>);

    match fn {
        ref => |ptr| gst_sys::gst_clock_id_ref(ptr),
        unref => |ptr| gst_sys::gst_clock_id_unref(ptr),
    }
}

unsafe extern "C" fn trampoline_wait_async<F: Fn(&Clock, ClockTime, &ClockId) + Send + 'static>(
    clock: *mut gst_sys::GstClock,
    time: gst_sys::GstClockTime,
    id: gpointer,
    func: gpointer,
) -> gboolean {
    let f: &F = &*(func as *const F);
    f(
        &from_glib_borrow(clock),
        from_glib(time),
        &from_glib_borrow(id),
    );
    glib_sys::GTRUE
}

unsafe extern "C" fn destroy_closure_wait_async<
    F: Fn(&Clock, ClockTime, &ClockId) + Send + 'static,
>(
    ptr: gpointer,
) {
    Box::<F>::from_raw(ptr as *mut _);
}

fn into_raw_wait_async<F: Fn(&Clock, ClockTime, &ClockId) + Send + 'static>(func: F) -> gpointer {
    #[allow(clippy::type_complexity)]
    let func: Box<F> = Box::new(func);
    Box::into_raw(func) as gpointer
}

impl ClockId {
    pub fn get_time(&self) -> ClockTime {
        unsafe { from_glib(gst_sys::gst_clock_id_get_time(self.to_glib_none().0)) }
    }

    pub fn unschedule(&self) {
        unsafe { gst_sys::gst_clock_id_unschedule(self.to_glib_none().0) }
    }

    pub fn wait(&self) -> (Result<ClockSuccess, ClockError>, ClockTimeDiff) {
        unsafe {
            let mut jitter = 0;
            let res: ClockReturn = from_glib(gst_sys::gst_clock_id_wait(
                self.to_glib_none().0,
                &mut jitter,
            ));
            (res.into_result(), jitter)
        }
    }

    pub fn wait_async<F>(&self, func: F) -> Result<ClockSuccess, ClockError>
    where
        F: Fn(&Clock, ClockTime, &ClockId) + Send + 'static,
    {
        let ret: ClockReturn = unsafe {
            from_glib(gst_sys::gst_clock_id_wait_async(
                self.to_glib_none().0,
                Some(trampoline_wait_async::<F>),
                into_raw_wait_async(func),
                Some(destroy_closure_wait_async::<F>),
            ))
        };
        ret.into_result()
    }

    #[allow(clippy::type_complexity)]
    pub fn wait_async_stream(
        &self,
    ) -> Result<
        Pin<Box<dyn Stream<Item = (ClockTime, ClockId)> + Unpin + Send + 'static>>,
        ClockError,
    > {
        use futures_channel::mpsc;

        let (sender, receiver) = mpsc::unbounded();

        self.wait_async(move |_clock, jitter, id| {
            if sender.unbounded_send((jitter, id.clone())).is_err() {
                // Unschedule any future calls if the receiver end is disconnected
                id.unschedule();
            }
        })?;

        Ok(Box::pin(receiver))
    }

    pub fn compare_by_time(&self, other: &Self) -> cmp::Ordering {
        unsafe {
            let res =
                gst_sys::gst_clock_id_compare_func(self.to_glib_none().0, other.to_glib_none().0);
            res.cmp(&0)
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn get_clock(&self) -> Option<Clock> {
        unsafe { from_glib_full(gst_sys::gst_clock_id_get_clock(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn uses_clock<P: IsA<Clock>>(&self, clock: &P) -> bool {
        unsafe {
            from_glib(gst_sys::gst_clock_id_uses_clock(
                self.to_glib_none().0,
                clock.as_ref().as_ptr(),
            ))
        }
    }

    pub fn get_type(&self) -> ClockEntryType {
        unsafe {
            let ptr: *mut gst_sys::GstClockEntry = self.to_glib_none().0 as *mut _;
            from_glib((*ptr).type_)
        }
    }

    pub fn get_interval(&self) -> Option<ClockTime> {
        if self.get_type() != ClockEntryType::Periodic {
            return None;
        }

        unsafe {
            let ptr: *mut gst_sys::GstClockEntry = self.to_glib_none().0 as *mut _;
            Some(from_glib((*ptr).interval))
        }
    }

    pub fn get_status(&self) -> &AtomicClockReturn {
        unsafe {
            let ptr: *mut gst_sys::GstClockEntry = self.to_glib_none().0 as *mut _;
            &*((&(*ptr).status) as *const i32 as *const AtomicClockReturn)
        }
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct AtomicClockReturn(AtomicI32);

impl AtomicClockReturn {
    pub fn load(&self) -> ClockReturn {
        from_glib(self.0.load(atomic::Ordering::SeqCst))
    }

    pub fn store(&self, val: ClockReturn) {
        self.0.store(val.to_glib(), atomic::Ordering::SeqCst)
    }

    pub fn swap(&self, val: ClockReturn) -> ClockReturn {
        from_glib(self.0.swap(val.to_glib(), atomic::Ordering::SeqCst))
    }

    pub fn compare_and_swap(&self, current: ClockReturn, new: ClockReturn) -> ClockReturn {
        from_glib(self.0.compare_and_swap(
            current.to_glib(),
            new.to_glib(),
            atomic::Ordering::SeqCst,
        ))
    }
}

unsafe impl Send for ClockId {}
unsafe impl Sync for ClockId {}

impl Clock {
    pub fn adjust_with_calibration(
        internal_target: ClockTime,
        cinternal: ClockTime,
        cexternal: ClockTime,
        cnum: ClockTime,
        cdenom: ClockTime,
    ) -> ClockTime {
        skip_assert_initialized!();
        unsafe {
            from_glib(gst_sys::gst_clock_adjust_with_calibration(
                ptr::null_mut(),
                internal_target.to_glib(),
                cinternal.to_glib(),
                cexternal.to_glib(),
                cnum.to_glib(),
                cdenom.to_glib(),
            ))
        }
    }

    pub fn unadjust_with_calibration(
        external_target: ClockTime,
        cinternal: ClockTime,
        cexternal: ClockTime,
        cnum: ClockTime,
        cdenom: ClockTime,
    ) -> ClockTime {
        skip_assert_initialized!();
        unsafe {
            from_glib(gst_sys::gst_clock_unadjust_with_calibration(
                ptr::null_mut(),
                external_target.to_glib(),
                cinternal.to_glib(),
                cexternal.to_glib(),
                cnum.to_glib(),
                cdenom.to_glib(),
            ))
        }
    }
}

pub trait ClockExtManual: 'static {
    fn new_periodic_id(
        &self,
        start_time: ClockTime,
        interval: ClockTime,
    ) -> Result<ClockId, glib::BoolError>;

    fn periodic_id_reinit(
        &self,
        id: &ClockId,
        start_time: ClockTime,
        interval: ClockTime,
    ) -> Result<(), glib::BoolError>;

    fn new_single_shot_id(&self, time: ClockTime) -> Result<ClockId, glib::BoolError>;

    fn single_shot_id_reinit(&self, id: &ClockId, time: ClockTime) -> Result<(), glib::BoolError>;

    fn set_clock_flags(&self, flags: ClockFlags);

    fn unset_clock_flags(&self, flags: ClockFlags);

    fn get_clock_flags(&self) -> ClockFlags;
}

impl<O: IsA<Clock>> ClockExtManual for O {
    fn new_periodic_id(
        &self,
        start_time: ClockTime,
        interval: ClockTime,
    ) -> Result<ClockId, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(gst_sys::gst_clock_new_periodic_id(
                self.as_ref().to_glib_none().0,
                start_time.to_glib(),
                interval.to_glib(),
            ))
            .ok_or_else(|| glib_bool_error!("Failed to create new periodic clock id"))
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
            let res: bool = from_glib(gst_sys::gst_clock_periodic_id_reinit(
                self.as_ref().to_glib_none().0,
                id.to_glib_none().0,
                start_time.to_glib(),
                interval.to_glib(),
            ));
            if res {
                Ok(())
            } else {
                Err(glib_bool_error!("Failed to reinit periodic clock id"))
            }
        }
    }

    fn new_single_shot_id(&self, time: ClockTime) -> Result<ClockId, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(gst_sys::gst_clock_new_single_shot_id(
                self.as_ref().to_glib_none().0,
                time.to_glib(),
            ))
            .ok_or_else(|| glib_bool_error!("Failed to create new single shot clock id"))
        }
    }

    fn single_shot_id_reinit(&self, id: &ClockId, time: ClockTime) -> Result<(), glib::BoolError> {
        unsafe {
            let res: bool = from_glib(gst_sys::gst_clock_single_shot_id_reinit(
                self.as_ref().to_glib_none().0,
                id.to_glib_none().0,
                time.to_glib(),
            ));
            if res {
                Ok(())
            } else {
                Err(glib_bool_error!("Failed to reinit single shot clock id"))
            }
        }
    }

    fn set_clock_flags(&self, flags: ClockFlags) {
        unsafe {
            let ptr: *mut gst_sys::GstObject = self.as_ptr() as *mut _;
            let _guard = ::utils::MutexGuard::lock(&(*ptr).lock);
            (*ptr).flags |= flags.to_glib();
        }
    }

    fn unset_clock_flags(&self, flags: ClockFlags) {
        unsafe {
            let ptr: *mut gst_sys::GstObject = self.as_ptr() as *mut _;
            let _guard = ::utils::MutexGuard::lock(&(*ptr).lock);
            (*ptr).flags &= !flags.to_glib();
        }
    }

    fn get_clock_flags(&self) -> ClockFlags {
        unsafe {
            let ptr: *mut gst_sys::GstObject = self.as_ptr() as *mut _;
            let _guard = ::utils::MutexGuard::lock(&(*ptr).lock);
            from_glib((*ptr).flags)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    use std::sync::mpsc::channel;

    #[test]
    fn test_wait() {
        ::init().unwrap();

        let clock = SystemClock::obtain();
        let now = clock.get_time();
        let id = clock.new_single_shot_id(now + 20 * ::MSECOND).unwrap();
        let (res, _) = id.wait();

        assert!(res == Ok(ClockSuccess::Ok) || res == Err(ClockError::Early));
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
        });

        assert!(res == Ok(ClockSuccess::Ok));

        assert_eq!(receiver.recv(), Ok(()));
    }
}
