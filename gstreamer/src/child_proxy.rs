// Take a look at the license at the top of the repository in the LICENSE file.

use std::ptr;

use glib::{prelude::*, translate::*};

use crate::ChildProxy;

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::ChildProxy>> Sealed for T {}
}

pub trait ChildProxyExtManual: sealed::Sealed + IsA<ChildProxy> + 'static {
    #[doc(alias = "gst_child_proxy_lookup")]
    fn lookup(&self, name: &str) -> Result<(glib::Object, glib::ParamSpec), glib::BoolError> {
        unsafe {
            let mut target = ptr::null_mut();
            let mut pspec = ptr::null_mut();
            let ret = from_glib(ffi::gst_child_proxy_lookup(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                &mut target,
                &mut pspec,
            ));
            if ret {
                Ok((from_glib_full(target), from_glib_none(pspec)))
            } else {
                Err(glib::bool_error!("Failed to find child property"))
            }
        }
    }

    #[doc(alias = "get_child_property")]
    #[doc(alias = "gst_child_proxy_get")]
    #[track_caller]
    fn child_property<V: for<'b> glib::value::FromValue<'b> + 'static>(&self, name: &str) -> V {
        let (child, pspec) = self.lookup(name).unwrap();
        child.property(pspec.name())
    }

    #[doc(alias = "get_child_property")]
    #[doc(alias = "gst_child_proxy_get")]
    #[track_caller]
    fn child_property_value(&self, name: &str) -> glib::Value {
        let (child, pspec) = self.lookup(name).unwrap();
        child.property_value(pspec.name())
    }

    #[doc(alias = "gst_child_proxy_set")]
    #[track_caller]
    fn set_child_property(&self, name: &str, value: impl Into<glib::Value>) {
        let (child, pspec) = self.lookup(name).unwrap();
        child.set_property(pspec.name(), value)
    }

    #[doc(alias = "gst_child_proxy_set_property")]
    #[track_caller]
    fn set_child_property_from_value(&self, name: &str, value: &glib::Value) {
        let (child, pspec) = self.lookup(name).unwrap();
        child.set_property_from_value(pspec.name(), value)
    }
}

impl<O: IsA<ChildProxy>> ChildProxyExtManual for O {}
