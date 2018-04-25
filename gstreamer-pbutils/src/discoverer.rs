// Copyright (C) 2018 Thiago Santos <thiagossantos@gmail.com>
//                    Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use gst;

use auto::Discoverer;

use glib::object::Downcast;
use glib::signal::connect;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::IsA;
use glib::Value;

use ffi;
use glib_ffi;
use gobject_ffi;

use std::boxed::Box as Box_;
use std::mem::transmute;

impl Discoverer {
    pub fn set_property_timeout(&self, timeout: gst::ClockTime) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "timeout".to_glib_none().0,
                Value::from(&timeout).to_glib_none().0,
            );
        }
    }

    pub fn get_property_timeout(&self) -> gst::ClockTime {
        let mut value = Value::from(&0u64);
        unsafe {
            gobject_ffi::g_object_get_property(
                self.to_glib_none().0,
                "timeout".to_glib_none().0,
                value.to_glib_none_mut().0,
            );
        }
        value.get().unwrap()
    }

    pub fn connect_property_timeout_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::timeout",
                transmute(notify_timeout_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }
}

unsafe extern "C" fn notify_timeout_trampoline<P>(
    this: *mut ffi::GstDiscoverer,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<Discoverer>,
{
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&Discoverer::from_glib_borrow(this).downcast_unchecked())
}
