// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, signal::SignalHandlerId, translate::*};

use crate::{ClockTime, ObjectFlags};

pub trait GstObjectExtManual: 'static {
    #[doc(alias = "deep-notify")]
    fn connect_deep_notify<F: Fn(&Self, &crate::Object, &glib::ParamSpec) + Send + Sync + 'static>(
        &self,
        name: Option<&str>,
        f: F,
    ) -> SignalHandlerId;

    fn set_object_flags(&self, flags: ObjectFlags);

    fn unset_object_flags(&self, flags: ObjectFlags);

    #[doc(alias = "get_object_flags")]
    fn object_flags(&self) -> ObjectFlags;

    #[doc(alias = "get_g_value_array")]
    #[doc(alias = "gst_object_get_g_value_array")]
    fn g_value_array(
        &self,
        property_name: &str,
        timestamp: ClockTime,
        interval: ClockTime,
        values: &mut [glib::Value],
    ) -> Result<(), glib::error::BoolError>;

    fn object_lock(&self) -> crate::utils::ObjectLockGuard<Self>;
}

impl<O: IsA<crate::Object>> GstObjectExtManual for O {
    fn connect_deep_notify<
        F: Fn(&Self, &crate::Object, &glib::ParamSpec) + Send + Sync + 'static,
    >(
        &self,
        name: Option<&str>,
        f: F,
    ) -> SignalHandlerId {
        let signal_name = if let Some(name) = name {
            format!("deep-notify::{name}")
        } else {
            "deep-notify".into()
        };

        let obj: Borrowed<glib::Object> =
            unsafe { from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject) };

        obj.connect(signal_name.as_str(), false, move |values| {
            // It would be nice to display the actual signal name in the panic messages below,
            // but that would require to copy `signal_name` so as to move it into the closure
            // which seems too much for the messages of development errors
            let obj: O = unsafe {
                values[0]
                    .get::<crate::Object>()
                    .unwrap_or_else(|err| panic!("Object signal \"deep-notify\": values[0]: {err}"))
                    .unsafe_cast()
            };
            let prop_obj: crate::Object = values[1]
                .get()
                .unwrap_or_else(|err| panic!("Object signal \"deep-notify\": values[1]: {err}"));

            let pspec = unsafe {
                let pspec = glib::gobject_ffi::g_value_get_param(values[2].to_glib_none().0);
                from_glib_none(pspec)
            };

            f(&obj, &prop_obj, &pspec);

            None
        })
    }

    fn set_object_flags(&self, flags: ObjectFlags) {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = crate::utils::MutexGuard::lock(&(*ptr).lock);
            (*ptr).flags |= flags.into_glib();
        }
    }

    fn unset_object_flags(&self, flags: ObjectFlags) {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = crate::utils::MutexGuard::lock(&(*ptr).lock);
            (*ptr).flags &= !flags.into_glib();
        }
    }

    fn object_flags(&self) -> ObjectFlags {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = crate::utils::MutexGuard::lock(&(*ptr).lock);
            from_glib((*ptr).flags)
        }
    }

    fn g_value_array(
        &self,
        property_name: &str,
        timestamp: ClockTime,
        interval: ClockTime,
        values: &mut [glib::Value],
    ) -> Result<(), glib::error::BoolError> {
        let n_values = values.len() as u32;
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_object_get_g_value_array(
                    self.as_ref().to_glib_none().0,
                    property_name.to_glib_none().0,
                    timestamp.into_glib(),
                    interval.into_glib(),
                    n_values,
                    values.as_mut_ptr() as *mut glib::gobject_ffi::GValue,
                ),
                "Failed to get value array"
            )
        }
    }

    #[inline]
    fn object_lock(&self) -> crate::utils::ObjectLockGuard<Self> {
        crate::utils::ObjectLockGuard::acquire(self)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_deep_notify() {
        crate::init().unwrap();

        let bin = crate::Bin::new(None);
        let identity = crate::ElementFactory::make("identity")
            .name("id")
            .build()
            .unwrap();
        bin.add(&identity).unwrap();

        let notify = Arc::new(Mutex::new(None));
        let notify_clone = notify.clone();
        bin.connect_deep_notify(None, move |_, id, prop| {
            *notify_clone.lock().unwrap() = Some((id.clone(), prop.name()));
        });

        identity.set_property("silent", false);
        assert_eq!(
            *notify.lock().unwrap(),
            Some((identity.upcast::<crate::Object>(), "silent"))
        );
    }
}
