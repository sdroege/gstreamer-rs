// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(feature = "v1_30")]
use glib::{prelude::*, translate::*};

#[cfg(feature = "v1_30")]
use crate::{Clock, ffi};
use crate::{ClockType, SystemClock};

impl SystemClock {
    #[cfg(feature = "v1_30")]
    #[doc(alias = "gst_system_clock_new")]
    pub fn new(name: Option<&str>, clock_type: ClockType) -> SystemClock {
        assert_initialized_main_thread!();
        unsafe {
            Clock::from_glib_full(ffi::gst_system_clock_new(
                name.to_glib_none().0,
                clock_type.into_glib(),
            ))
            .unsafe_cast()
        }
    }

    #[cfg(not(feature = "v1_30"))]
    #[doc(alias = "gst_system_clock_new")]
    pub fn new(name: Option<&str>, clock_type: ClockType) -> SystemClock {
        assert_initialized_main_thread!();
        glib::Object::builder::<SystemClock>()
            .property("name", name)
            .property("clock-type", clock_type)
            .build()
    }
}
