// Take a look at the license at the top of the repository in the LICENSE file.

use crate::auto::Discoverer;

use glib::prelude::*;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::Value;

use std::boxed::Box as Box_;
use std::mem::transmute;

impl Discoverer {
    pub fn set_property_timeout(&self, timeout: gst::ClockTime) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut _,
                "timeout".to_glib_none().0,
                Value::from(&timeout).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "get_property_timeout")]
    pub fn property_timeout(&self) -> gst::ClockTime {
        let mut value = Value::from(&0u64);
        unsafe {
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut _,
                "timeout".to_glib_none().0,
                value.to_glib_none_mut().0,
            );
        }
        value.get().expect("Discoverer::get_property_timeout")
    }

    pub fn connect_property_timeout_notify<F: Fn(&Self) + Send + Sync + 'static>(
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
    f(&Discoverer::from_glib_borrow(this).unsafe_cast_ref())
}
