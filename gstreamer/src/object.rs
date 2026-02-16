// Take a look at the license at the top of the repository in the LICENSE file.

use std::ptr;
#[cfg(feature = "v1_28")]
use std::{future::Future, pin::Pin};

use glib::{prelude::*, signal::SignalHandlerId, translate::*};

use crate::{ClockTime, Object, ObjectFlags, ffi};

pub trait GstObjectExtManual: IsA<Object> + 'static {
    #[doc(alias = "deep-notify")]
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
            let obj: Self = unsafe {
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
            let _guard = self.as_ref().object_lock();
            (*ptr).flags |= flags.into_glib();
        }
    }

    fn unset_object_flags(&self, flags: ObjectFlags) {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = self.as_ref().object_lock();
            (*ptr).flags &= !flags.into_glib();
        }
    }

    #[doc(alias = "get_object_flags")]
    fn object_flags(&self) -> ObjectFlags {
        unsafe {
            let ptr: *mut ffi::GstObject = self.as_ptr() as *mut _;
            let _guard = self.as_ref().object_lock();
            from_glib((*ptr).flags)
        }
    }

    #[doc(alias = "get_g_value_array")]
    #[doc(alias = "gst_object_get_g_value_array")]
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
    fn object_lock(&self) -> crate::utils::ObjectLockGuard<'_, Self> {
        crate::utils::ObjectLockGuard::acquire(self)
    }

    #[cfg(feature = "v1_28")]
    #[doc(alias = "gst_object_call_async")]
    fn call_async<F>(&self, func: F)
    where
        F: FnOnce(&Self) + Send + 'static,
    {
        let user_data: Box<F> = Box::new(func);

        unsafe extern "C" fn trampoline<O: IsA<Object>, F: FnOnce(&O) + Send + 'static>(
            object: *mut ffi::GstObject,
            user_data: glib::ffi::gpointer,
        ) {
            unsafe {
                let callback: Box<F> = Box::from_raw(user_data as *mut _);
                callback(Object::from_glib_borrow(object).unsafe_cast_ref());
            }
        }

        unsafe {
            ffi::gst_object_call_async(
                self.as_ref().to_glib_none().0,
                Some(trampoline::<Self, F>),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(feature = "v1_28")]
    fn call_async_future<F, T>(&self, func: F) -> Pin<Box<dyn Future<Output = T> + Send + 'static>>
    where
        F: FnOnce(&Self) -> T + Send + 'static,
        T: Send + 'static,
    {
        use futures_channel::oneshot;

        let (sender, receiver) = oneshot::channel();

        self.call_async(move |object| {
            let _ = sender.send(func(object));
        });

        Box::pin(async move { receiver.await.expect("sender dropped") })
    }

    // rustdoc-stripper-ignore-next
    /// Sets the parent of `self` to `parent`. If `self` already has a parent this will fail.
    ///
    /// The returned reference to `self` on success must be kept alive until the `parent` is unset
    /// again. Also `parent` must be kept alive until `child` gets its parent unset, or in other
    /// words if `child` doesn't get its parent unset until `parent` is disposed then during
    /// disposal this must happen. Not doing so causes dangling references and potential
    /// use-after-frees.
    #[doc(alias = "gst_object_set_parent")]
    #[doc(alias = "parent")]
    unsafe fn set_parent(&self, parent: &impl IsA<Object>) -> Result<Self, glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_object_set_parent(
                    self.as_ref().to_glib_none().0,
                    parent.as_ref().to_glib_none().0
                ),
                "Failed to set parent object"
            )
            .map(|_| {
                // set_parent() increases the reference count of the child but only on success.
                Object::from_glib_full(self.as_ref().as_ptr()).unsafe_cast()
            })
        }
    }

    // rustdoc-stripper-ignore-next
    /// Unsets the parent of `self` from `parent`. If `self` has no parent or a different parent
    /// than `parent` this will fail.
    #[doc(alias = "gst_object_set_parent")]
    #[doc(alias = "parent")]
    unsafe fn unset_parent(&self, parent: &impl IsA<Object>) -> Result<(), glib::error::BoolError> {
        unsafe {
            let _lock = self.object_lock();

            if (*self.as_ref().as_ptr()).parent != parent.as_ref().as_ptr() {
                return Err(glib::bool_error!("Failed to unset parent object"));
            }

            (*self.as_ref().as_ptr()).parent = ptr::null_mut();

            Ok(())
        }
    }
}

impl<O: IsA<Object>> GstObjectExtManual for O {}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_deep_notify() {
        crate::init().unwrap();

        let bin = crate::Bin::new();
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

    mod test_object {
        use super::*;

        use glib::subclass::prelude::*;

        pub mod imp {
            use std::collections::BTreeSet;

            use super::*;

            use crate::subclass::prelude::*;

            #[derive(Default)]
            pub struct TestObject {
                pub(super) children: Mutex<BTreeSet<Object>>,
            }

            #[glib::object_subclass]
            impl ObjectSubclass for TestObject {
                const NAME: &'static str = "TestObject";
                type Type = super::TestObject;
                type ParentType = crate::Object;
            }

            impl ObjectImpl for TestObject {
                fn dispose(&self) {
                    // Safety: Need to make sure to keep a reference to the child until
                    // it is removed or the parent goes away
                    unsafe {
                        let mut children = self.children.lock().unwrap();
                        for child in children.iter() {
                            child.unset_parent(&*self.obj()).unwrap();
                        }
                        children.clear();
                    }
                }
            }

            impl GstObjectImpl for TestObject {}
        }

        glib::wrapper! {
            pub struct TestObject(ObjectSubclass<imp::TestObject>) @extends crate::Object;
        }

        impl TestObject {
            pub fn new() -> Self {
                glib::Object::builder().build()
            }

            pub fn add(&self, child: &impl IsA<Object>) -> bool {
                if child.as_ref() == self.upcast_ref::<Object>() {
                    return false;
                }

                let mut children = self.imp().children.lock().unwrap();

                if children.iter().any(|other| other.name() == child.name()) {
                    return false;
                }

                // Safety: Need to make sure to keep a reference to the child until
                // it is removed or the parent goes away
                unsafe {
                    if let Ok(child) = child.as_ref().set_parent(self) {
                        let inserted = children.insert(child);
                        assert!(inserted);
                        true
                    } else {
                        false
                    }
                }
            }

            pub fn remove(&self, child: &impl IsA<Object>) -> bool {
                let mut children = self.imp().children.lock().unwrap();

                // Safety: Need to make sure to keep a reference to the child until
                // it is removed or the parent goes away
                unsafe {
                    if child.unset_parent(self).is_ok() {
                        let found = children.remove(child.as_ref());
                        assert!(found);
                        true
                    } else {
                        false
                    }
                }
            }
        }
    }

    #[test]
    fn test_set_unset_parent() {
        crate::init().unwrap();

        let p1 = test_object::TestObject::new();
        let p2 = test_object::TestObject::new();

        let c1 = test_object::TestObject::new();
        let c2 = test_object::TestObject::new();

        assert!(p1.add(&c1));
        assert!(p1.parent().is_none());
        assert_eq!(c1.parent().as_ref(), Some(p1.upcast_ref()));
        assert_eq!(p1.ref_count(), 1);
        assert_eq!(c1.ref_count(), 2);

        assert!(p2.add(&c2));
        assert!(p2.parent().is_none());
        assert_eq!(c2.parent().as_ref(), Some(p2.upcast_ref()));
        assert_eq!(p2.ref_count(), 1);
        assert_eq!(c2.ref_count(), 2);

        assert!(!p2.add(&c1));
        assert_eq!(c1.parent().as_ref(), Some(p1.upcast_ref()));

        assert!(p2.remove(&c2));
        assert_eq!(c2.parent().as_ref(), None::<&Object>);
        assert_eq!(p2.ref_count(), 1);
        assert_eq!(c2.ref_count(), 1);

        assert!(p1.add(&c2));
        assert_eq!(c2.parent().as_ref(), Some(p1.upcast_ref()));
        assert_eq!(p1.ref_count(), 1);
        assert_eq!(c2.ref_count(), 2);

        assert!(p1.remove(&c2));
        assert_eq!(c1.parent().as_ref(), Some(p1.upcast_ref()));
        assert_eq!(c2.parent().as_ref(), None::<&Object>);

        assert_eq!(p1.ref_count(), 1);
        assert_eq!(p2.ref_count(), 1);
        assert_eq!(c1.ref_count(), 2);
        assert_eq!(c2.ref_count(), 1);
    }
}
