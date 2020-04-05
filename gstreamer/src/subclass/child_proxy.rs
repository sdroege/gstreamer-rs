// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib_sys;
use gobject_sys;
use gst_sys;

use glib;
use glib::subclass::prelude::*;
use glib::translate::*;

use libc;

use ChildProxy;

pub trait ChildProxyImpl: super::element::ElementImpl + Send + Sync + 'static {
    fn get_child_by_name(&self, object: &ChildProxy, name: &str) -> Option<glib::Object> {
        unsafe {
            let type_ = gst_sys::gst_child_proxy_get_type();
            let iface = gobject_sys::g_type_default_interface_ref(type_)
                as *mut gst_sys::GstChildProxyInterface;
            assert!(!iface.is_null());

            let ret = ((*iface).get_child_by_name.as_ref().unwrap())(
                object.to_glib_none().0,
                name.to_glib_none().0,
            );

            gobject_sys::g_type_default_interface_unref(iface as glib_sys::gpointer);

            from_glib_full(ret)
        }
    }

    fn get_child_by_index(&self, object: &ChildProxy, index: u32) -> Option<glib::Object>;
    fn get_children_count(&self, object: &ChildProxy) -> u32;

    fn child_added(&self, _object: &ChildProxy, _child: &glib::Object, _name: &str) {}
    fn child_removed(&self, _object: &ChildProxy, _child: &glib::Object, _name: &str) {}
}

unsafe impl<T: ObjectSubclass + ChildProxyImpl> IsImplementable<T> for ChildProxy {
    unsafe extern "C" fn interface_init(
        iface: glib_sys::gpointer,
        _iface_data: glib_sys::gpointer,
    ) {
        let child_proxy_iface = &mut *(iface as *mut gst_sys::GstChildProxyInterface);

        child_proxy_iface.get_child_by_name = Some(child_proxy_get_child_by_name::<T>);
        child_proxy_iface.get_child_by_index = Some(child_proxy_get_child_by_index::<T>);
        child_proxy_iface.get_children_count = Some(child_proxy_get_children_count::<T>);
        child_proxy_iface.child_added = Some(child_proxy_child_added::<T>);
        child_proxy_iface.child_removed = Some(child_proxy_child_removed::<T>);
    }
}

unsafe extern "C" fn child_proxy_get_child_by_name<T: ObjectSubclass>(
    child_proxy: *mut gst_sys::GstChildProxy,
    name: *const libc::c_char,
) -> *mut gobject_sys::GObject
where
    T: ChildProxyImpl,
{
    let instance = &*(child_proxy as *mut T::Instance);
    let imp = instance.get_impl();

    imp.get_child_by_name(
        &from_glib_borrow(child_proxy),
        &glib::GString::from_glib_borrow(name),
    )
    .to_glib_full()
}

unsafe extern "C" fn child_proxy_get_child_by_index<T: ObjectSubclass>(
    child_proxy: *mut gst_sys::GstChildProxy,
    index: u32,
) -> *mut gobject_sys::GObject
where
    T: ChildProxyImpl,
{
    let instance = &*(child_proxy as *mut T::Instance);
    let imp = instance.get_impl();

    imp.get_child_by_index(&from_glib_borrow(child_proxy), index)
        .to_glib_full()
}

unsafe extern "C" fn child_proxy_get_children_count<T: ObjectSubclass>(
    child_proxy: *mut gst_sys::GstChildProxy,
) -> u32
where
    T: ChildProxyImpl,
{
    let instance = &*(child_proxy as *mut T::Instance);
    let imp = instance.get_impl();

    imp.get_children_count(&from_glib_borrow(child_proxy))
}

unsafe extern "C" fn child_proxy_child_added<T: ObjectSubclass>(
    child_proxy: *mut gst_sys::GstChildProxy,
    child: *mut gobject_sys::GObject,
    name: *const libc::c_char,
) where
    T: ChildProxyImpl,
{
    let instance = &*(child_proxy as *mut T::Instance);
    let imp = instance.get_impl();

    imp.child_added(
        &from_glib_borrow(child_proxy),
        &from_glib_borrow(child),
        &glib::GString::from_glib_borrow(name),
    )
}

unsafe extern "C" fn child_proxy_child_removed<T: ObjectSubclass>(
    child_proxy: *mut gst_sys::GstChildProxy,
    child: *mut gobject_sys::GObject,
    name: *const libc::c_char,
) where
    T: ChildProxyImpl,
{
    let instance = &*(child_proxy as *mut T::Instance);
    let imp = instance.get_impl();

    imp.child_removed(
        &from_glib_borrow(child_proxy),
        &from_glib_borrow(child),
        &glib::GString::from_glib_borrow(name),
    )
}
