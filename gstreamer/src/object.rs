// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib;
use glib::object::{Downcast, ObjectExt};
use glib::signal::SignalHandlerId;
use glib::translate::{from_glib_none, ToGlibPtr};
use glib::IsA;

use gobject_ffi;

pub trait GstObjectExtManual {
    fn connect_deep_notify<
        'a,
        P: Into<Option<&'a str>>,
        F: Fn(&Self, &::Object, &glib::ParamSpec) + Send + Sync + 'static,
    >(
        &self,
        name: P,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<::Object> + IsA<glib::Object> + glib::value::SetValue> GstObjectExtManual for O {
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

        self.connect(signal_name.as_str(), false, move |values| {
            let obj: O = unsafe {
                values[0]
                    .get::<glib::Object>()
                    .unwrap()
                    .downcast_unchecked()
            };
            let prop_obj: ::Object = values[1].get().unwrap();

            let pspec = unsafe {
                let pspec = gobject_ffi::g_value_get_param(values[2].to_glib_none().0);
                from_glib_none(pspec)
            };

            f(&obj, &prop_obj, &pspec);

            None
        }).unwrap()
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
        let identity = ::ElementFactory::make("identity", "id").unwrap();
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
