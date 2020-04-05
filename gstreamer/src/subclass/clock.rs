// Copyright (C) 2019 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use gst_sys;

use glib;
use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;

use Clock;
use ClockClass;
use ClockError;
use ClockId;
use ClockReturn;
use ClockSuccess;
use ClockTime;
use ClockTimeDiff;

pub trait ClockImpl: ClockImplExt + ObjectImpl + Send + Sync + 'static {
    fn change_resolution(
        &self,
        clock: &Clock,
        old_resolution: ClockTime,
        new_resolution: ClockTime,
    ) -> ClockTime {
        self.parent_change_resolution(clock, old_resolution, new_resolution)
    }

    fn get_resolution(&self, clock: &Clock) -> ClockTime {
        self.parent_get_resolution(clock)
    }

    fn get_internal_time(&self, clock: &Clock) -> ClockTime {
        self.parent_get_internal_time(clock)
    }

    fn wait(
        &self,
        clock: &Clock,
        id: &ClockId,
    ) -> (Result<ClockSuccess, ClockError>, ClockTimeDiff) {
        self.parent_wait(clock, id)
    }

    fn wait_async(&self, clock: &Clock, id: &ClockId) -> Result<ClockSuccess, ClockError> {
        self.parent_wait_async(clock, id)
    }

    fn unschedule(&self, clock: &Clock, id: &ClockId) {
        self.parent_unschedule(clock, id)
    }
}

pub trait ClockImplExt {
    fn parent_change_resolution(
        &self,
        clock: &Clock,
        old_resolution: ClockTime,
        new_resolution: ClockTime,
    ) -> ClockTime;

    fn parent_get_resolution(&self, clock: &Clock) -> ClockTime;

    fn parent_get_internal_time(&self, clock: &Clock) -> ClockTime;

    fn parent_wait(
        &self,
        clock: &Clock,
        id: &ClockId,
    ) -> (Result<ClockSuccess, ClockError>, ClockTimeDiff);

    fn parent_wait_async(&self, clock: &Clock, id: &ClockId) -> Result<ClockSuccess, ClockError>;

    fn parent_unschedule(&self, clock: &Clock, id: &ClockId);

    fn wake_id(&self, id: &ClockId)
    where
        Self: ObjectSubclass,
        <Self as ObjectSubclass>::ParentType: IsA<Clock>;
}

impl<T: ClockImpl + ObjectImpl> ClockImplExt for T {
    fn parent_change_resolution(
        &self,
        clock: &Clock,
        old_resolution: ClockTime,
        new_resolution: ClockTime,
    ) -> ClockTime {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gst_sys::GstClockClass;

            if let Some(func) = (*parent_class).change_resolution {
                from_glib(func(
                    clock.to_glib_none().0,
                    old_resolution.to_glib(),
                    new_resolution.to_glib(),
                ))
            } else {
                self.get_resolution(clock)
            }
        }
    }

    fn parent_get_resolution(&self, clock: &Clock) -> ClockTime {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gst_sys::GstClockClass;

            from_glib(
                (*parent_class)
                    .get_resolution
                    .map(|f| f(clock.to_glib_none().0))
                    .unwrap_or(1),
            )
        }
    }

    fn parent_get_internal_time(&self, clock: &Clock) -> ClockTime {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gst_sys::GstClockClass;

            from_glib(
                (*parent_class)
                    .get_internal_time
                    .map(|f| f(clock.to_glib_none().0))
                    .unwrap_or(0),
            )
        }
    }

    fn parent_wait(
        &self,
        clock: &Clock,
        id: &ClockId,
    ) -> (Result<ClockSuccess, ClockError>, ClockTimeDiff) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gst_sys::GstClockClass;
            let mut jitter = 0;

            (
                ClockReturn::from_glib(
                    (*parent_class)
                        .wait
                        .map(|f| {
                            f(
                                clock.to_glib_none().0,
                                id.to_glib_none().0 as *mut gst_sys::GstClockEntry,
                                &mut jitter,
                            )
                        })
                        .unwrap_or(gst_sys::GST_CLOCK_UNSUPPORTED),
                )
                .into_result(),
                jitter,
            )
        }
    }

    fn parent_wait_async(&self, clock: &Clock, id: &ClockId) -> Result<ClockSuccess, ClockError> {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gst_sys::GstClockClass;
            ClockReturn::from_glib(
                (*parent_class)
                    .wait_async
                    .map(|f| {
                        f(
                            clock.to_glib_none().0,
                            id.to_glib_none().0 as *mut gst_sys::GstClockEntry,
                        )
                    })
                    .unwrap_or(gst_sys::GST_CLOCK_UNSUPPORTED),
            )
            .into_result()
        }
    }

    fn parent_unschedule(&self, clock: &Clock, id: &ClockId) {
        unsafe {
            let data = self.get_type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut gst_sys::GstClockClass;
            if let Some(func) = (*parent_class).unschedule {
                func(
                    clock.to_glib_none().0,
                    id.to_glib_none().0 as *mut gst_sys::GstClockEntry,
                );
            }
        }
    }

    fn wake_id(&self, id: &ClockId)
    where
        Self: ObjectSubclass,
        <Self as ObjectSubclass>::ParentType: IsA<Clock>,
    {
        let clock = self.get_instance();

        #[cfg(feature = "v1_16")]
        {
            assert!(id.uses_clock(&clock));
        }
        #[cfg(not(feature = "v1_16"))]
        {
            unsafe {
                let ptr: *mut gst_sys::GstClockEntry = id.to_glib_none().0 as *mut _;
                assert_eq!((*ptr).clock, clock.as_ref().to_glib_none().0);
            }
        }

        unsafe {
            let ptr: *mut gst_sys::GstClockEntry = id.to_glib_none().0 as *mut _;
            if let Some(func) = (*ptr).func {
                func(
                    clock.as_ref().to_glib_none().0,
                    (*ptr).time,
                    ptr as gst_sys::GstClockID,
                    (*ptr).user_data,
                );
            }
            if (*ptr).type_ == gst_sys::GST_CLOCK_ENTRY_PERIODIC {
                (*ptr).time += (*ptr).interval;
            }
        }
    }
}

unsafe impl<T: ObjectSubclass + ClockImpl> IsSubclassable<T> for ClockClass {
    fn override_vfuncs(&mut self) {
        <glib::ObjectClass as IsSubclassable<T>>::override_vfuncs(self);

        unsafe {
            let klass = &mut *(self as *mut Self as *mut gst_sys::GstClockClass);
            klass.change_resolution = Some(clock_change_resolution::<T>);
            klass.get_resolution = Some(clock_get_resolution::<T>);
            klass.get_internal_time = Some(clock_get_internal_time::<T>);
            klass.wait = Some(clock_wait::<T>);
            klass.wait_async = Some(clock_wait_async::<T>);
            klass.unschedule = Some(clock_unschedule::<T>);
        }
    }
}

unsafe extern "C" fn clock_change_resolution<T: ObjectSubclass>(
    ptr: *mut gst_sys::GstClock,
    old_resolution: gst_sys::GstClockTime,
    new_resolution: gst_sys::GstClockTime,
) -> gst_sys::GstClockTime
where
    T: ClockImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Clock> = from_glib_borrow(ptr);

    imp.change_resolution(&wrap, from_glib(old_resolution), from_glib(new_resolution))
        .to_glib()
}

unsafe extern "C" fn clock_get_resolution<T: ObjectSubclass>(
    ptr: *mut gst_sys::GstClock,
) -> gst_sys::GstClockTime
where
    T: ClockImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Clock> = from_glib_borrow(ptr);

    imp.get_resolution(&wrap).to_glib()
}

unsafe extern "C" fn clock_get_internal_time<T: ObjectSubclass>(
    ptr: *mut gst_sys::GstClock,
) -> gst_sys::GstClockTime
where
    T: ClockImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Clock> = from_glib_borrow(ptr);

    imp.get_internal_time(&wrap).to_glib()
}

unsafe extern "C" fn clock_wait<T: ObjectSubclass>(
    ptr: *mut gst_sys::GstClock,
    id: *mut gst_sys::GstClockEntry,
    jitter: *mut gst_sys::GstClockTimeDiff,
) -> gst_sys::GstClockReturn
where
    T: ClockImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Clock> = from_glib_borrow(ptr);

    let (res, j) = imp.wait(&wrap, &from_glib_borrow(id as gst_sys::GstClockID));
    if !jitter.is_null() {
        *jitter = j;
    }

    ClockReturn::from(res).to_glib()
}

unsafe extern "C" fn clock_wait_async<T: ObjectSubclass>(
    ptr: *mut gst_sys::GstClock,
    id: *mut gst_sys::GstClockEntry,
) -> gst_sys::GstClockReturn
where
    T: ClockImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Clock> = from_glib_borrow(ptr);

    ClockReturn::from(imp.wait_async(&wrap, &from_glib_borrow(id as gst_sys::GstClockID))).to_glib()
}

unsafe extern "C" fn clock_unschedule<T: ObjectSubclass>(
    ptr: *mut gst_sys::GstClock,
    id: *mut gst_sys::GstClockEntry,
) where
    T: ClockImpl,
{
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Clock> = from_glib_borrow(ptr);

    imp.unschedule(&wrap, &from_glib_borrow(id as gst_sys::GstClockID));
}
