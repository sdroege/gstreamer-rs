// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib;
use glib::object::{Cast, ObjectExt};
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::IsA;

use gobject_sys;

use ObjectFlags;

pub trait GstObjectExtManual: 'static {
    fn connect_deep_notify<F: Fn(&Self, &::Object, &glib::ParamSpec) + Send + Sync + 'static>(
        &self,
        name: Option<&str>,
        f: F,
    ) -> SignalHandlerId;

    fn set_object_flags(&self, flags: ObjectFlags);

    fn unset_object_flags(&self, flags: ObjectFlags);

    fn get_object_flags(&self) -> ObjectFlags;
}

impl<O: IsA<::Object>> GstObjectExtManual for O {
    fn connect_deep_notify<F: Fn(&Self, &::Object, &glib::ParamSpec) + Send + Sync + 'static>(
        &self,
        name: Option<&str>,
        f: F,
    ) -> SignalHandlerId {
        let signal_name = if let Some(name) = name {
            format!("deep-notify::{}", name)
        } else {
            "deep-notify".into()
        };

        let obj: Borrowed<glib::Object> =
            unsafe { from_glib_borrow(self.as_ptr() as *mut gobject_sys::GObject) };

        obj.connect(signal_name.as_str(), false, move |values| {
            // It would be nice to display the actual signal name in the panic messages below,
            // but that would require to copy `signal_name` so as to move it into the closure
            // which seems too much for the messages of development errors
            let obj: O = unsafe {
                values[0]
                    .get::<::Object>()
                    .unwrap_or_else(|err| {
                        panic!("Object signal \"deep-notify\": values[0]: {}", err)
                    })
                    .expect("Object signal \"deep-notify\": values[0] not defined")
                    .unsafe_cast()
            };
            let prop_obj: ::Object = values[1]
                .get()
                .unwrap_or_else(|err| panic!("Object signal \"deep-notify\": values[1]: {}", err))
                .expect("Object signal \"deep-notify\": values[1] not defined");

            let pspec = unsafe {
                let pspec = gobject_sys::g_value_get_param(values[2].to_glib_none().0);
                from_glib_none(pspec)
            };

            f(&obj, &prop_obj, &pspec);

            None
        })
        .unwrap()
    }

    fn set_object_flags(&self, flags: ObjectFlags) {
        unsafe {
            let ptr: *mut gst_sys::GstObject = self.as_ptr() as *mut _;
            let _guard = ::utils::MutexGuard::lock(&(*ptr).lock);
            (*ptr).flags |= flags.to_glib();
        }
    }

    fn unset_object_flags(&self, flags: ObjectFlags) {
        unsafe {
            let ptr: *mut gst_sys::GstObject = self.as_ptr() as *mut _;
            let _guard = ::utils::MutexGuard::lock(&(*ptr).lock);
            (*ptr).flags &= !flags.to_glib();
        }
    }

    fn get_object_flags(&self) -> ObjectFlags {
        unsafe {
            let ptr: *mut gst_sys::GstObject = self.as_ptr() as *mut _;
            let _guard = ::utils::MutexGuard::lock(&(*ptr).lock);
            from_glib((*ptr).flags)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prelude::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_deep_notify() {
        ::init().unwrap();

        let bin = ::Bin::new(None);
        let identity = ::ElementFactory::make("identity", Some("id")).unwrap();
        bin.add(&identity).unwrap();

        let notify = Arc::new(Mutex::new(None));
        let notify_clone = notify.clone();
        bin.connect_deep_notify(None, move |_, id, prop| {
            *notify_clone.lock().unwrap() = Some((id.clone(), prop.get_name()));
        });

        identity.set_property("silent", &false).unwrap();
        assert_eq!(
            *notify.lock().unwrap(),
            Some((identity.upcast::<::Object>(), String::from("silent")))
        );
    }
}
