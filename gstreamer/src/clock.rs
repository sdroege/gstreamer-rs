// Take a look at the license at the top of the repository in the LICENSE file.

use std::{
    cmp,
    marker::Unpin,
    pin::Pin,
    ptr,
    sync::{atomic, atomic::AtomicI32},
};

use futures_core::{Future, Stream};
use glib::{
    ffi::{gboolean, gpointer},
    prelude::*,
    translate::*,
};
use libc::c_void;

use crate::{
    Clock, ClockEntryType, ClockError, ClockFlags, ClockReturn, ClockSuccess, ClockTime,
    ClockTimeDiff,
};

glib::wrapper! {
    #[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
    pub struct ClockId(Shared<c_void>);

    match fn {
        ref => |ptr| ffi::gst_clock_id_ref(ptr),
        unref => |ptr| ffi::gst_clock_id_unref(ptr),
    }
}

impl ClockId {
    #[doc(alias = "get_time")]
    #[doc(alias = "gst_clock_id_get_time")]
    #[doc(alias = "GST_CLOCK_ENTRY_TIME")]
    pub fn time(&self) -> ClockTime {
        unsafe {
            try_from_glib(ffi::gst_clock_id_get_time(self.to_glib_none().0))
                .expect("undefined time")
        }
    }

    #[doc(alias = "gst_clock_id_unschedule")]
    pub fn unschedule(&self) {
        unsafe { ffi::gst_clock_id_unschedule(self.to_glib_none().0) }
    }

    #[doc(alias = "gst_clock_id_wait")]
    pub fn wait(&self) -> (Result<ClockSuccess, ClockError>, ClockTimeDiff) {
        unsafe {
            let mut jitter = 0;
            let res = try_from_glib(ffi::gst_clock_id_wait(self.to_glib_none().0, &mut jitter));
            (res, jitter)
        }
    }

    #[doc(alias = "gst_clock_id_compare_func")]
    pub fn compare_by_time(&self, other: &Self) -> cmp::Ordering {
        unsafe {
            let res = ffi::gst_clock_id_compare_func(self.to_glib_none().0, other.to_glib_none().0);
            res.cmp(&0)
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "get_clock")]
    #[doc(alias = "gst_clock_id_get_clock")]
    pub fn clock(&self) -> Option<Clock> {
        unsafe { from_glib_full(ffi::gst_clock_id_get_clock(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_clock_id_uses_clock")]
    pub fn uses_clock<P: IsA<Clock>>(&self, clock: &P) -> bool {
        unsafe {
            from_glib(ffi::gst_clock_id_uses_clock(
                self.to_glib_none().0,
                clock.as_ref().as_ptr(),
            ))
        }
    }

    #[doc(alias = "get_type")]
    #[doc(alias = "GST_CLOCK_ENTRY_TYPE")]
    pub fn type_(&self) -> ClockEntryType {
        unsafe {
            let ptr = self.as_ptr() as *mut ffi::GstClockEntry;
            from_glib((*ptr).type_)
        }
    }

    #[doc(alias = "get_status")]
    #[doc(alias = "GST_CLOCK_ENTRY_STATUS")]
    pub fn status(&self) -> &AtomicClockReturn {
        unsafe {
            let ptr = self.as_ptr() as *mut ffi::GstClockEntry;
            &*((&(*ptr).status) as *const i32 as *const AtomicClockReturn)
        }
    }
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct SingleShotClockId(ClockId);

impl std::ops::Deref for SingleShotClockId {
    type Target = ClockId;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<SingleShotClockId> for ClockId {
    #[inline]
    fn from(id: SingleShotClockId) -> ClockId {
        skip_assert_initialized!();
        id.0
    }
}

impl TryFrom<ClockId> for SingleShotClockId {
    type Error = glib::BoolError;

    #[inline]
    fn try_from(id: ClockId) -> Result<SingleShotClockId, glib::BoolError> {
        skip_assert_initialized!();
        match id.type_() {
            ClockEntryType::Single => Ok(SingleShotClockId(id)),
            _ => Err(glib::bool_error!("Not a single-shot clock id")),
        }
    }
}

impl SingleShotClockId {
    #[doc(alias = "gst_clock_id_compare_func")]
    #[inline]
    pub fn compare_by_time(&self, other: &Self) -> cmp::Ordering {
        self.0.compare_by_time(&other.0)
    }

    #[doc(alias = "gst_clock_id_wait_async")]
    pub fn wait_async<F>(&self, func: F) -> Result<ClockSuccess, ClockError>
    where
        F: FnOnce(&Clock, Option<ClockTime>, &ClockId) + Send + 'static,
    {
        unsafe extern "C" fn trampoline<
            F: FnOnce(&Clock, Option<ClockTime>, &ClockId) + Send + 'static,
        >(
            clock: *mut ffi::GstClock,
            time: ffi::GstClockTime,
            id: gpointer,
            func: gpointer,
        ) -> gboolean {
            let f: &mut Option<F> = &mut *(func as *mut Option<F>);
            let f = f.take().unwrap();

            f(
                &from_glib_borrow(clock),
                from_glib(time),
                &from_glib_borrow(id),
            );

            glib::ffi::GTRUE
        }

        unsafe extern "C" fn destroy_notify<
            F: FnOnce(&Clock, Option<ClockTime>, &ClockId) + Send + 'static,
        >(
            ptr: gpointer,
        ) {
            let _ = Box::<Option<F>>::from_raw(ptr as *mut _);
        }

        let func: Box<Option<F>> = Box::new(Some(func));

        unsafe {
            try_from_glib(ffi::gst_clock_id_wait_async(
                self.to_glib_none().0,
                Some(trampoline::<F>),
                Box::into_raw(func) as gpointer,
                Some(destroy_notify::<F>),
            ))
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn wait_async_future(
        &self,
    ) -> Result<
        Pin<
            Box<
                dyn Future<Output = Result<(Option<ClockTime>, ClockId), ClockError>>
                    + Send
                    + 'static,
            >,
        >,
        ClockError,
    > {
        use futures_channel::oneshot;

        let (sender, receiver) = oneshot::channel();

        self.wait_async(move |_clock, jitter, id| {
            if sender.send((jitter, id.clone())).is_err() {
                // Unschedule any future calls if the receiver end is disconnected
                id.unschedule();
            }
        })?;

        Ok(Box::pin(async move {
            receiver.await.map_err(|_| ClockError::Unscheduled)
        }))
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct PeriodicClockId(ClockId);

impl std::ops::Deref for PeriodicClockId {
    type Target = ClockId;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<PeriodicClockId> for ClockId {
    #[inline]
    fn from(id: PeriodicClockId) -> ClockId {
        skip_assert_initialized!();
        id.0
    }
}

impl TryFrom<ClockId> for PeriodicClockId {
    type Error = glib::BoolError;

    #[inline]
    fn try_from(id: ClockId) -> Result<PeriodicClockId, glib::BoolError> {
        skip_assert_initialized!();
        match id.type_() {
            ClockEntryType::Periodic => Ok(PeriodicClockId(id)),
            _ => Err(glib::bool_error!("Not a periodic clock id")),
        }
    }
}

impl PeriodicClockId {
    #[doc(alias = "get_interval")]
    #[doc(alias = "GST_CLOCK_ENTRY_INTERVAL")]
    #[inline]
    pub fn interval(&self) -> ClockTime {
        unsafe {
            let ptr = self.as_ptr() as *mut ffi::GstClockEntry;
            try_from_glib((*ptr).interval).expect("undefined interval")
        }
    }

    #[doc(alias = "gst_clock_id_compare_func")]
    #[inline]
    pub fn compare_by_time(&self, other: &Self) -> cmp::Ordering {
        self.0.compare_by_time(&other.0)
    }

    #[doc(alias = "gst_clock_id_wait_async")]
    pub fn wait_async<F>(&self, func: F) -> Result<ClockSuccess, ClockError>
    where
        F: Fn(&Clock, Option<ClockTime>, &ClockId) + Send + 'static,
    {
        unsafe extern "C" fn trampoline<
            F: Fn(&Clock, Option<ClockTime>, &ClockId) + Send + 'static,
        >(
            clock: *mut ffi::GstClock,
            time: ffi::GstClockTime,
            id: gpointer,
            func: gpointer,
        ) -> gboolean {
            let f: &F = &*(func as *const F);
            f(
                &from_glib_borrow(clock),
                from_glib(time),
                &from_glib_borrow(id),
            );
            glib::ffi::GTRUE
        }

        unsafe extern "C" fn destroy_notify<
            F: Fn(&Clock, Option<ClockTime>, &ClockId) + Send + 'static,
        >(
            ptr: gpointer,
        ) {
            let _ = Box::<F>::from_raw(ptr as *mut _);
        }

        let func: Box<F> = Box::new(func);
        unsafe {
            try_from_glib(ffi::gst_clock_id_wait_async(
                self.to_glib_none().0,
                Some(trampoline::<F>),
                Box::into_raw(func) as gpointer,
                Some(destroy_notify::<F>),
            ))
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn wait_async_stream(
        &self,
    ) -> Result<
        Pin<Box<dyn Stream<Item = (Option<ClockTime>, ClockId)> + Unpin + Send + 'static>>,
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
}

#[repr(transparent)]
#[derive(Debug)]
pub struct AtomicClockReturn(AtomicI32);

impl AtomicClockReturn {
    #[inline]
    pub fn load(&self) -> ClockReturn {
        unsafe { from_glib(self.0.load(atomic::Ordering::SeqCst)) }
    }

    #[inline]
    pub fn store(&self, val: ClockReturn) {
        self.0.store(val.into_glib(), atomic::Ordering::SeqCst)
    }

    #[inline]
    pub fn swap(&self, val: ClockReturn) -> ClockReturn {
        unsafe { from_glib(self.0.swap(val.into_glib(), atomic::Ordering::SeqCst)) }
    }

    #[inline]
    pub fn compare_exchange(
        &self,
        current: ClockReturn,
        new: ClockReturn,
    ) -> Result<ClockReturn, ClockReturn> {
        unsafe {
            self.0
                .compare_exchange(
                    current.into_glib(),
                    new.into_glib(),
                    atomic::Ordering::SeqCst,
                    atomic::Ordering::SeqCst,
                )
                .map(|v| from_glib(v))
                .map_err(|v| from_glib(v))
        }
    }
}

unsafe impl Send for ClockId {}
unsafe impl Sync for ClockId {}

impl Clock {
    #[doc(alias = "gst_clock_adjust_with_calibration")]
    pub fn adjust_with_calibration(
        internal_target: ClockTime,
        cinternal: ClockTime,
        cexternal: ClockTime,
        cnum: ClockTime,
        cdenom: ClockTime,
    ) -> ClockTime {
        skip_assert_initialized!();
        unsafe {
            try_from_glib(ffi::gst_clock_adjust_with_calibration(
                ptr::null_mut(),
                internal_target.into_glib(),
                cinternal.into_glib(),
                cexternal.into_glib(),
                cnum.into_glib(),
                cdenom.into_glib(),
            ))
            .expect("undefined ClockTime")
        }
    }

    #[doc(alias = "gst_clock_unadjust_with_calibration")]
    pub fn unadjust_with_calibration(
        external_target: ClockTime,
        cinternal: ClockTime,
        cexternal: ClockTime,
        cnum: ClockTime,
        cdenom: ClockTime,
    ) -> ClockTime {
        skip_assert_initialized!();
        unsafe {
            try_from_glib(ffi::gst_clock_unadjust_with_calibration(
                ptr::null_mut(),
                external_target.into_glib(),
                cinternal.into_glib(),
                cexternal.into_glib(),
                cnum.into_glib(),
                cdenom.into_glib(),
            ))
            .expect("undefined ClockTime")
        }
    }
}

pub trait ClockExtManual: 'static {
    #[doc(alias = "gst_clock_new_periodic_id")]
    fn new_periodic_id(&self, start_time: ClockTime, interval: ClockTime) -> PeriodicClockId;

    #[doc(alias = "gst_clock_periodic_id_reinit")]
    fn periodic_id_reinit(
        &self,
        id: &PeriodicClockId,
        start_time: ClockTime,
        interval: ClockTime,
    ) -> Result<(), glib::BoolError>;

    #[doc(alias = "gst_clock_new_single_shot_id")]
    fn new_single_shot_id(&self, time: ClockTime) -> SingleShotClockId;

    #[doc(alias = "gst_clock_single_shot_id_reinit")]
    fn single_shot_id_reinit(
        &self,
        id: &SingleShotClockId,
        time: ClockTime,
    ) -> Result<(), glib::BoolError>;

    fn set_clock_flags(&self, flags: ClockFlags);

    fn unset_clock_flags(&self, flags: ClockFlags);

    #[doc(alias = "get_clock_flags")]
    fn clock_flags(&self) -> ClockFlags;
}

impl<O: IsA<Clock>> ClockExtManual for O {
    fn new_periodic_id(&self, start_time: ClockTime, interval: ClockTime) -> PeriodicClockId {
        assert_ne!(interval, ClockTime::ZERO);

        unsafe {
            PeriodicClockId(from_glib_full(ffi::gst_clock_new_periodic_id(
                self.as_ref().to_glib_none().0,
                start_time.into_glib(),
                interval.into_glib(),
            )))
        }
    }

    fn periodic_id_reinit(
        &self,
        id: &PeriodicClockId,
        start_time: ClockTime,
        interval: ClockTime,
    ) -> Result<(), glib::BoolError> {
        unsafe {
            let res: bool = from_glib(ffi::gst_clock_periodic_id_reinit(
                self.as_ref().to_glib_none().0,
                id.to_glib_none().0,
                start_time.into_glib(),
                interval.into_glib(),
            ));
            if res {
                Ok(())
            } else {
                Err(glib::bool_error!("Failed to reinit periodic clock id"))
            }
        }
    }

    fn new_single_shot_id(&self, time: ClockTime) -> SingleShotClockId {
        unsafe {
            SingleShotClockId(from_glib_full(ffi::gst_clock_new_single_shot_id(
                self.as_ref().to_glib_none().0,
                time.into_glib(),
            )))
        }
    }

    fn single_shot_id_reinit(
        &self,
        id: &SingleShotClockId,
        time: ClockTime,
    ) -> Result<(), glib::BoolError> {
        unsafe {
            let res: bool = from_glib(ffi::gst_clock_single_shot_id_reinit(
                self.as_ref().to_glib_none().0,
                id.to_glib_none().0,
                time.into_glib(),
            ));
            if res {
                Ok(())
            } else {
                Err(glib::bool_error!("Failed to reinit single shot clock id"))
            }
        }
    }

    fn set_clock_flags(&self, flags: ClockFlags) {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = crate::utils::MutexGuard::lock(&(*ptr).lock);
            (*ptr).flags |= flags.into_glib();
        }
    }

    fn unset_clock_flags(&self, flags: ClockFlags) {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = crate::utils::MutexGuard::lock(&(*ptr).lock);
            (*ptr).flags &= !flags.into_glib();
        }
    }

    fn clock_flags(&self) -> ClockFlags {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = crate::utils::MutexGuard::lock(&(*ptr).lock);
            from_glib((*ptr).flags)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::channel;

    use super::{
        super::{prelude::*, *},
        *,
    };

    #[test]
    fn test_wait() {
        crate::init().unwrap();

        let clock = SystemClock::obtain();
        let now = clock.time().unwrap();
        let id = clock.new_single_shot_id(now + 20 * ClockTime::MSECOND);
        let (res, _) = id.wait();

        assert!(res == Ok(ClockSuccess::Ok) || res == Err(ClockError::Early));
    }

    #[test]
    fn test_wait_async() {
        crate::init().unwrap();

        let (sender, receiver) = channel();

        let clock = SystemClock::obtain();
        let now = clock.time().unwrap();
        let id = clock.new_single_shot_id(now + 20 * ClockTime::MSECOND);
        let res = id.wait_async(move |_, _, _| {
            sender.send(()).unwrap();
        });

        assert!(res == Ok(ClockSuccess::Ok));

        assert_eq!(receiver.recv(), Ok(()));
    }

    #[test]
    fn test_wait_periodic() {
        crate::init().unwrap();

        let clock = SystemClock::obtain();
        let now = clock.time().unwrap();
        let id = clock.new_periodic_id(now + 20 * ClockTime::MSECOND, 20 * ClockTime::MSECOND);

        let (res, _) = id.wait();
        assert!(res == Ok(ClockSuccess::Ok) || res == Err(ClockError::Early));

        let (res, _) = id.wait();
        assert!(res == Ok(ClockSuccess::Ok) || res == Err(ClockError::Early));
    }

    #[test]
    fn test_wait_async_periodic() {
        crate::init().unwrap();

        let (sender, receiver) = channel();

        let clock = SystemClock::obtain();
        let now = clock.time().unwrap();
        let id = clock.new_periodic_id(now + 20 * ClockTime::MSECOND, 20 * ClockTime::MSECOND);
        let res = id.wait_async(move |_, _, _| {
            let _ = sender.send(());
        });

        assert!(res == Ok(ClockSuccess::Ok));

        assert_eq!(receiver.recv(), Ok(()));
        assert_eq!(receiver.recv(), Ok(()));
    }
}
