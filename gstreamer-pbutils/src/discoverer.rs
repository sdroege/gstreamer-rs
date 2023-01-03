// Take a look at the license at the top of the repository in the LICENSE file.

use std::{boxed::Box as Box_, mem::transmute};

use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};

use crate::auto::Discoverer;

impl Discoverer {
    pub fn set_timeout(&self, timeout: gst::ClockTime) {
        self.set_property("timeout", &timeout);
    }

    pub fn timeout(&self) -> gst::ClockTime {
        self.property("timeout")
    }

    #[doc(alias = "timeout")]
    pub fn connect_timeout_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::timeout\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_timeout_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe extern "C" fn notify_timeout_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
    this: *mut ffi::GstDiscoverer,
    _param_spec: glib::ffi::gpointer,
    f: glib::ffi::gpointer,
) where
    P: IsA<Discoverer>,
{
    let f: &F = &*(f as *const F);
    f(Discoverer::from_glib_borrow(this).unsafe_cast_ref())
}
