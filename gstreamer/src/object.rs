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
    fn connect_deep_notify<
        'a,
        P: Into<Option<&'a str>>,
        F: Fn(&Self, &::Object, &glib::ParamSpec) + Send + Sync + 'static,
    >(
        &self,
        name: P,
        f: F,
    ) -> SignalHandlerId;

    fn set_object_flags(&self, flags: ObjectFlags);

    fn unset_object_flags(&self, flags: ObjectFlags);

    fn get_object_flags(&self) -> ObjectFlags;
}

impl<O: IsA<::Object>> GstObjectExtManual for O {
    fn connect_deep_notify<
        'a,
        P: Into<Option<&'a str>>,
        F: Fn(&Self, &::Object, &glib::ParamSpec) + Send + Sync + 'static,
    >(
        &self,
        name: P,
        f: F,
    ) -> SignalHandlerId {
        let name = name.into();
        let signal_name = if let Some(name) = name {
            format!("deep-notify::{}", name)
        } else {
            "deep-notify".into()
        };

        let obj: glib::Object =
            unsafe { from_glib_borrow(self.as_ptr() as *mut gobject_sys::GObject) };

        obj.connect(signal_name.as_str(), false, move |values| {
            let obj: O = unsafe { values[0].get::<::Object>().unwrap().unsafe_cast() };
            let prop_obj: ::Object = values[1].get().unwrap();

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
            *notify_clone.lock().unwrap() = Some((id.clone(), String::from(prop.get_name())));
        });

        identity.set_property("silent", &false).unwrap();
        assert_eq!(
            *notify.lock().unwrap(),
            Some((identity.upcast::<::Object>(), String::from("silent")))
        );
    }
}
