// Take a look at the license at the top of the repository in the LICENSE file.

use super::prelude::*;
use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;

use crate::Clock;
use crate::ClockError;
use crate::ClockId;
use crate::ClockReturn;
use crate::ClockSuccess;
use crate::ClockTime;
use crate::ClockTimeDiff;

pub trait ClockImpl: ClockImplExt + GstObjectImpl + Send + Sync {
    fn change_resolution(
        &self,
        clock: &Self::Type,
        old_resolution: ClockTime,
        new_resolution: ClockTime,
    ) -> ClockTime {
        self.parent_change_resolution(clock, old_resolution, new_resolution)
    }

    fn resolution(&self, clock: &Self::Type) -> ClockTime {
        self.parent_resolution(clock)
    }

    fn internal_time(&self, clock: &Self::Type) -> ClockTime {
        self.parent_internal_time(clock)
    }

    fn wait(
        &self,
        clock: &Self::Type,
        id: &ClockId,
    ) -> (Result<ClockSuccess, ClockError>, ClockTimeDiff) {
        self.parent_wait(clock, id)
    }

    fn wait_async(&self, clock: &Self::Type, id: &ClockId) -> Result<ClockSuccess, ClockError> {
        self.parent_wait_async(clock, id)
    }

    fn unschedule(&self, clock: &Self::Type, id: &ClockId) {
        self.parent_unschedule(clock, id)
    }
}

pub trait ClockImplExt: ObjectSubclass {
    fn parent_change_resolution(
        &self,
        clock: &Self::Type,
        old_resolution: ClockTime,
        new_resolution: ClockTime,
    ) -> ClockTime;

    fn parent_resolution(&self, clock: &Self::Type) -> ClockTime;

    fn parent_internal_time(&self, clock: &Self::Type) -> ClockTime;

    fn parent_wait(
        &self,
        clock: &Self::Type,
        id: &ClockId,
    ) -> (Result<ClockSuccess, ClockError>, ClockTimeDiff);

    fn parent_wait_async(
        &self,
        clock: &Self::Type,
        id: &ClockId,
    ) -> Result<ClockSuccess, ClockError>;

    fn parent_unschedule(&self, clock: &Self::Type, id: &ClockId);

    fn wake_id(&self, id: &ClockId)
    where
        Self: ObjectSubclass,
        <Self as ObjectSubclass>::Type: IsA<Clock>;
}

impl<T: ClockImpl> ClockImplExt for T {
    fn parent_change_resolution(
        &self,
        clock: &Self::Type,
        old_resolution: ClockTime,
        new_resolution: ClockTime,
    ) -> ClockTime {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstClockClass;

            if let Some(func) = (*parent_class).change_resolution {
                try_from_glib(func(
                    clock.unsafe_cast_ref::<Clock>().to_glib_none().0,
                    old_resolution.into_glib(),
                    new_resolution.into_glib(),
                ))
                .expect("undefined resolution")
            } else {
                self.resolution(clock)
            }
        }
    }

    fn parent_resolution(&self, clock: &Self::Type) -> ClockTime {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstClockClass;

            try_from_glib(
                (*parent_class)
                    .get_resolution
                    .map(|f| f(clock.unsafe_cast_ref::<Clock>().to_glib_none().0))
                    .unwrap_or(1),
            )
            .expect("undefined resolution")
        }
    }

    fn parent_internal_time(&self, clock: &Self::Type) -> ClockTime {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstClockClass;

            try_from_glib(
                (*parent_class)
                    .get_internal_time
                    .map(|f| f(clock.unsafe_cast_ref::<Clock>().to_glib_none().0))
                    .unwrap_or(0),
            )
            .expect("undefined internal_time")
        }
    }

    fn parent_wait(
        &self,
        clock: &Self::Type,
        id: &ClockId,
    ) -> (Result<ClockSuccess, ClockError>, ClockTimeDiff) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstClockClass;
            let mut jitter = 0;

            (
                try_from_glib(
                    (*parent_class)
                        .wait
                        .map(|f| {
                            f(
                                clock.unsafe_cast_ref::<Clock>().to_glib_none().0,
                                id.to_glib_none().0 as *mut ffi::GstClockEntry,
                                &mut jitter,
                            )
                        })
                        .unwrap_or(ffi::GST_CLOCK_UNSUPPORTED),
                ),
                jitter,
            )
        }
    }

    fn parent_wait_async(
        &self,
        clock: &Self::Type,
        id: &ClockId,
    ) -> Result<ClockSuccess, ClockError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstClockClass;
            try_from_glib(
                (*parent_class)
                    .wait_async
                    .map(|f| {
                        f(
                            clock.unsafe_cast_ref::<Clock>().to_glib_none().0,
                            id.to_glib_none().0 as *mut ffi::GstClockEntry,
                        )
                    })
                    .unwrap_or(ffi::GST_CLOCK_UNSUPPORTED),
            )
        }
    }

    fn parent_unschedule(&self, clock: &Self::Type, id: &ClockId) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstClockClass;
            if let Some(func) = (*parent_class).unschedule {
                func(
                    clock.unsafe_cast_ref::<Clock>().to_glib_none().0,
                    id.to_glib_none().0 as *mut ffi::GstClockEntry,
                );
            }
        }
    }

    fn wake_id(&self, id: &ClockId)
    where
        Self: ObjectSubclass,
        <Self as ObjectSubclass>::Type: IsA<Clock>,
    {
        let clock = self.instance();

        cfg_if::cfg_if! {
            if #[cfg(feature = "v1_16")] {
                assert!(id.uses_clock(&clock));
            } else {
                unsafe {
                    let ptr: *mut ffi::GstClockEntry = id.to_glib_none().0 as *mut _;
                    assert_eq!((*ptr).clock, clock.as_ref().to_glib_none().0);
                }
            }
        }

        unsafe {
            let ptr: *mut ffi::GstClockEntry = id.to_glib_none().0 as *mut _;
            if let Some(func) = (*ptr).func {
                func(
                    clock.as_ref().to_glib_none().0,
                    (*ptr).time,
                    ptr as ffi::GstClockID,
                    (*ptr).user_data,
                );
            }
            if (*ptr).type_ == ffi::GST_CLOCK_ENTRY_PERIODIC {
                (*ptr).time += (*ptr).interval;
            }
        }
    }
}

unsafe impl<T: ClockImpl> IsSubclassable<T> for Clock {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);
        let klass = klass.as_mut();
        klass.change_resolution = Some(clock_change_resolution::<T>);
        klass.get_resolution = Some(clock_get_resolution::<T>);
        klass.get_internal_time = Some(clock_get_internal_time::<T>);
        klass.wait = Some(clock_wait::<T>);
        klass.wait_async = Some(clock_wait_async::<T>);
        klass.unschedule = Some(clock_unschedule::<T>);
    }
}

unsafe extern "C" fn clock_change_resolution<T: ClockImpl>(
    ptr: *mut ffi::GstClock,
    old_resolution: ffi::GstClockTime,
    new_resolution: ffi::GstClockTime,
) -> ffi::GstClockTime {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Clock> = from_glib_borrow(ptr);

    let old_resolution = match from_glib(old_resolution) {
        Some(old_resolution) => old_resolution,
        None => return ffi::GST_CLOCK_TIME_NONE,
    };
    let new_resolution = match from_glib(new_resolution) {
        Some(new_resolution) => new_resolution,
        None => return ffi::GST_CLOCK_TIME_NONE,
    };

    imp.change_resolution(wrap.unsafe_cast_ref(), old_resolution, new_resolution)
        .into_glib()
}

unsafe extern "C" fn clock_get_resolution<T: ClockImpl>(
    ptr: *mut ffi::GstClock,
) -> ffi::GstClockTime {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Clock> = from_glib_borrow(ptr);

    imp.resolution(wrap.unsafe_cast_ref()).into_glib()
}

unsafe extern "C" fn clock_get_internal_time<T: ClockImpl>(
    ptr: *mut ffi::GstClock,
) -> ffi::GstClockTime {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Clock> = from_glib_borrow(ptr);

    imp.internal_time(wrap.unsafe_cast_ref()).into_glib()
}

unsafe extern "C" fn clock_wait<T: ClockImpl>(
    ptr: *mut ffi::GstClock,
    id: *mut ffi::GstClockEntry,
    jitter: *mut ffi::GstClockTimeDiff,
) -> ffi::GstClockReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Clock> = from_glib_borrow(ptr);

    let (res, j) = imp.wait(
        wrap.unsafe_cast_ref(),
        &from_glib_borrow(id as ffi::GstClockID),
    );
    if !jitter.is_null() {
        *jitter = j;
    }

    ClockReturn::from(res).into_glib()
}

unsafe extern "C" fn clock_wait_async<T: ClockImpl>(
    ptr: *mut ffi::GstClock,
    id: *mut ffi::GstClockEntry,
) -> ffi::GstClockReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Clock> = from_glib_borrow(ptr);

    ClockReturn::from(imp.wait_async(
        wrap.unsafe_cast_ref(),
        &from_glib_borrow(id as ffi::GstClockID),
    ))
    .into_glib()
}

unsafe extern "C" fn clock_unschedule<T: ClockImpl>(
    ptr: *mut ffi::GstClock,
    id: *mut ffi::GstClockEntry,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<Clock> = from_glib_borrow(ptr);

    imp.unschedule(
        wrap.unsafe_cast_ref(),
        &from_glib_borrow(id as ffi::GstClockID),
    );
}
