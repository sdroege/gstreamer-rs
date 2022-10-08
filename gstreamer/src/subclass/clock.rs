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
    fn change_resolution(&self, old_resolution: ClockTime, new_resolution: ClockTime) -> ClockTime {
        self.parent_change_resolution(old_resolution, new_resolution)
    }

    fn resolution(&self) -> ClockTime {
        self.parent_resolution()
    }

    fn internal_time(&self) -> ClockTime {
        self.parent_internal_time()
    }

    fn wait(&self, id: &ClockId) -> (Result<ClockSuccess, ClockError>, ClockTimeDiff) {
        self.parent_wait(id)
    }

    fn wait_async(&self, id: &ClockId) -> Result<ClockSuccess, ClockError> {
        self.parent_wait_async(id)
    }

    fn unschedule(&self, id: &ClockId) {
        self.parent_unschedule(id)
    }
}

pub trait ClockImplExt: ObjectSubclass {
    fn parent_change_resolution(
        &self,
        old_resolution: ClockTime,
        new_resolution: ClockTime,
    ) -> ClockTime;

    fn parent_resolution(&self) -> ClockTime;

    fn parent_internal_time(&self) -> ClockTime;

    fn parent_wait(&self, id: &ClockId) -> (Result<ClockSuccess, ClockError>, ClockTimeDiff);

    fn parent_wait_async(&self, id: &ClockId) -> Result<ClockSuccess, ClockError>;

    fn parent_unschedule(&self, id: &ClockId);

    fn wake_id(&self, id: &ClockId);
}

impl<T: ClockImpl> ClockImplExt for T {
    fn parent_change_resolution(
        &self,
        old_resolution: ClockTime,
        new_resolution: ClockTime,
    ) -> ClockTime {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstClockClass;

            if let Some(func) = (*parent_class).change_resolution {
                try_from_glib(func(
                    self.instance().unsafe_cast_ref::<Clock>().to_glib_none().0,
                    old_resolution.into_glib(),
                    new_resolution.into_glib(),
                ))
                .expect("undefined resolution")
            } else {
                self.resolution()
            }
        }
    }

    fn parent_resolution(&self) -> ClockTime {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstClockClass;

            try_from_glib(
                (*parent_class)
                    .get_resolution
                    .map(|f| f(self.instance().unsafe_cast_ref::<Clock>().to_glib_none().0))
                    .unwrap_or(1),
            )
            .expect("undefined resolution")
        }
    }

    fn parent_internal_time(&self) -> ClockTime {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstClockClass;

            try_from_glib(
                (*parent_class)
                    .get_internal_time
                    .map(|f| f(self.instance().unsafe_cast_ref::<Clock>().to_glib_none().0))
                    .unwrap_or(0),
            )
            .expect("undefined internal_time")
        }
    }

    fn parent_wait(&self, id: &ClockId) -> (Result<ClockSuccess, ClockError>, ClockTimeDiff) {
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
                                self.instance().unsafe_cast_ref::<Clock>().to_glib_none().0,
                                id.as_ptr() as *mut ffi::GstClockEntry,
                                &mut jitter,
                            )
                        })
                        .unwrap_or(ffi::GST_CLOCK_UNSUPPORTED),
                ),
                jitter,
            )
        }
    }

    fn parent_wait_async(&self, id: &ClockId) -> Result<ClockSuccess, ClockError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstClockClass;
            try_from_glib(
                (*parent_class)
                    .wait_async
                    .map(|f| {
                        f(
                            self.instance().unsafe_cast_ref::<Clock>().to_glib_none().0,
                            id.as_ptr() as *mut ffi::GstClockEntry,
                        )
                    })
                    .unwrap_or(ffi::GST_CLOCK_UNSUPPORTED),
            )
        }
    }

    fn parent_unschedule(&self, id: &ClockId) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstClockClass;
            if let Some(func) = (*parent_class).unschedule {
                func(
                    self.instance().unsafe_cast_ref::<Clock>().to_glib_none().0,
                    id.as_ptr() as *mut ffi::GstClockEntry,
                );
            }
        }
    }

    fn wake_id(&self, id: &ClockId) {
        let clock = self.instance();
        let clock = unsafe { clock.unsafe_cast_ref::<Clock>() };

        cfg_if::cfg_if! {
            if #[cfg(feature = "v1_16")] {
                assert!(id.uses_clock(clock));
            } else {
                unsafe {
                    let ptr = id.as_ptr() as *mut ffi::GstClockEntry;
                    assert_eq!((*ptr).clock, clock.to_glib_none().0);
                }
            }
        }

        unsafe {
            let ptr = id.as_ptr() as *mut ffi::GstClockEntry;
            if let Some(func) = (*ptr).func {
                func(
                    clock.to_glib_none().0,
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
    let imp = instance.imp();

    let old_resolution = match from_glib(old_resolution) {
        Some(old_resolution) => old_resolution,
        None => return ffi::GST_CLOCK_TIME_NONE,
    };
    let new_resolution = match from_glib(new_resolution) {
        Some(new_resolution) => new_resolution,
        None => return ffi::GST_CLOCK_TIME_NONE,
    };

    imp.change_resolution(old_resolution, new_resolution)
        .into_glib()
}

unsafe extern "C" fn clock_get_resolution<T: ClockImpl>(
    ptr: *mut ffi::GstClock,
) -> ffi::GstClockTime {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.resolution().into_glib()
}

unsafe extern "C" fn clock_get_internal_time<T: ClockImpl>(
    ptr: *mut ffi::GstClock,
) -> ffi::GstClockTime {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.internal_time().into_glib()
}

unsafe extern "C" fn clock_wait<T: ClockImpl>(
    ptr: *mut ffi::GstClock,
    id: *mut ffi::GstClockEntry,
    jitter: *mut ffi::GstClockTimeDiff,
) -> ffi::GstClockReturn {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    let (res, j) = imp.wait(&from_glib_borrow(id as ffi::GstClockID));
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
    let imp = instance.imp();

    ClockReturn::from(imp.wait_async(&from_glib_borrow(id as ffi::GstClockID))).into_glib()
}

unsafe extern "C" fn clock_unschedule<T: ClockImpl>(
    ptr: *mut ffi::GstClock,
    id: *mut ffi::GstClockEntry,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.unschedule(&from_glib_borrow(id as ffi::GstClockID));
}
