// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ChildProxy;
use glib::prelude::*;
use glib::translate::*;
use std::ptr;

pub trait ChildProxyExtManual: 'static {
    #[doc(alias = "gst_child_proxy_lookup")]
    fn lookup(&self, name: &str) -> Result<(glib::Object, glib::ParamSpec), glib::BoolError>;

    #[doc(alias = "get_child_property")]
    #[doc(alias = "gst_child_proxy_get")]
    fn child_property<V: for<'b> glib::value::FromValue<'b> + 'static>(&self, name: &str) -> V;
    #[doc(alias = "get_child_property")]
    #[doc(alias = "gst_child_proxy_get")]
    fn child_property_value(&self, name: &str) -> glib::Value;
    #[doc(alias = "get_child_property")]
    #[doc(alias = "gst_child_proxy_get")]
    fn try_child_property<V: for<'b> glib::value::FromValue<'b> + 'static>(
        &self,
        name: &str,
    ) -> Result<V, glib::BoolError>;
    #[doc(alias = "get_child_property")]
    #[doc(alias = "gst_child_proxy_get")]
    fn try_child_property_value(&self, name: &str) -> Result<glib::Value, glib::BoolError>;

    #[doc(alias = "gst_child_proxy_set")]
    fn set_child_property<V: glib::ToValue>(&self, name: &str, value: V);
    #[doc(alias = "gst_child_proxy_set_property")]
    fn set_child_property_from_value(&self, name: &str, value: &glib::Value);
    #[doc(alias = "gst_child_proxy_set")]
    fn try_set_child_property<V: glib::ToValue>(
        &self,
        name: &str,
        value: V,
    ) -> Result<(), glib::BoolError>;
    #[doc(alias = "gst_child_proxy_set_property")]
    fn try_set_child_property_from_value(
        &self,
        name: &str,
        value: &glib::Value,
    ) -> Result<(), glib::BoolError>;
}

impl<O: IsA<ChildProxy>> ChildProxyExtManual for O {
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

    fn child_property<V: for<'b> glib::value::FromValue<'b> + 'static>(&self, name: &str) -> V {
        let (child, pspec) = self.lookup(name).unwrap();
        child.property(pspec.name())
    }

    fn child_property_value(&self, name: &str) -> glib::Value {
        let (child, pspec) = self.lookup(name).unwrap();
        child.property_value(pspec.name())
    }

    fn try_child_property<V: for<'b> glib::value::FromValue<'b> + 'static>(
        &self,
        name: &str,
    ) -> Result<V, glib::BoolError> {
        let (child, pspec) = self.lookup(name)?;
        child.try_property(pspec.name())
    }

    fn try_child_property_value(&self, name: &str) -> Result<glib::Value, glib::BoolError> {
        let (child, pspec) = self.lookup(name)?;
        child.try_property_value(pspec.name())
    }

    fn set_child_property<V: glib::ToValue>(&self, name: &str, value: V) {
        let (child, pspec) = self.lookup(name).unwrap();
        child.set_property(pspec.name(), value)
    }

    fn set_child_property_from_value(&self, name: &str, value: &glib::Value) {
        let (child, pspec) = self.lookup(name).unwrap();
        child.set_property_from_value(pspec.name(), value)
    }

    fn try_set_child_property<V: glib::ToValue>(
        &self,
        name: &str,
        value: V,
    ) -> Result<(), glib::BoolError> {
        let (child, pspec) = self.lookup(name)?;
        child.try_set_property(pspec.name(), value)
    }

    fn try_set_child_property_from_value(
        &self,
        name: &str,
        value: &glib::Value,
    ) -> Result<(), glib::BoolError> {
        let (child, pspec) = self.lookup(name)?;
        child.try_set_property_from_value(pspec.name(), value)
    }
}
