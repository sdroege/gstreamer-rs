// Copyright (C) 2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib_ffi;
use gobject_ffi;

use glib;
use glib::subclass::prelude::*;
use glib::translate::*;

use libc;

use std::ptr;

use ChildProxy;

pub trait ChildProxyImpl: super::element::ElementImpl + Send + Sync + 'static {
    fn get_child_by_name(&self, object: &ChildProxy, name: &str) -> Option<glib::Object> {
        unsafe {
            let type_ = ffi::gst_child_proxy_get_type();
            let iface = gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GstChildProxyInterface;
            assert!(!iface.is_null());

            let ret = ((*iface).get_child_by_name.as_ref().unwrap())(
                object.to_glib_none().0,
                name.to_glib_none().0,
            );

            gobject_ffi::g_type_default_interface_unref(iface as glib_ffi::gpointer);

            from_glib_full(ret)
        }
    }

    fn get_child_by_index(&self, object: &ChildProxy, index: u32) -> Option<glib::Object>;
    fn get_children_count(&self, object: &ChildProxy) -> u32;

    fn child_added(&self, object: &ChildProxy, child: &glib::Object, name: &str);
    fn child_removed(&self, object: &ChildProxy, child: &glib::Object, name: &str);
}

unsafe extern "C" fn child_proxy_get_child_by_name<T: ObjectSubclass>(
    child_proxy: *mut ffi::GstChildProxy,
    name: *const libc::c_char,
) -> *mut gobject_ffi::GObject
where
    T: ChildProxyImpl,
{
    glib_floating_reference_guard!(child_proxy);
    let instance = &*(child_proxy as *mut T::Instance);
    let imp = instance.get_impl();

    imp.get_child_by_name(
        &from_glib_borrow(child_proxy),
        String::from_glib_none(name).as_str(),
    )
    .to_glib_full()
}

unsafe extern "C" fn child_proxy_get_child_by_index<T: ObjectSubclass>(
    child_proxy: *mut ffi::GstChildProxy,
    index: u32,
) -> *mut gobject_ffi::GObject
where
    T: ChildProxyImpl,
{
    glib_floating_reference_guard!(child_proxy);
    let instance = &*(child_proxy as *mut T::Instance);
    let imp = instance.get_impl();

    imp.get_child_by_index(&from_glib_borrow(child_proxy), index)
        .to_glib_full()
}

unsafe extern "C" fn child_proxy_get_children_count<T: ObjectSubclass>(
    child_proxy: *mut ffi::GstChildProxy,
) -> u32
where
    T: ChildProxyImpl,
{
    glib_floating_reference_guard!(child_proxy);
    let instance = &*(child_proxy as *mut T::Instance);
    let imp = instance.get_impl();

    imp.get_children_count(&from_glib_borrow(child_proxy))
}

unsafe extern "C" fn child_proxy_child_added<T: ObjectSubclass>(
    child_proxy: *mut ffi::GstChildProxy,
    child: *mut gobject_ffi::GObject,
    name: *const libc::c_char,
) where
    T: ChildProxyImpl,
{
    glib_floating_reference_guard!(child_proxy);
    let instance = &*(child_proxy as *mut T::Instance);
    let imp = instance.get_impl();

    imp.child_added(
        &from_glib_borrow(child_proxy),
        &from_glib_borrow(child),
        String::from_glib_none(name).as_str(),
    )
}

unsafe extern "C" fn child_proxy_child_removed<T: ObjectSubclass>(
    child_proxy: *mut ffi::GstChildProxy,
    child: *mut gobject_ffi::GObject,
    name: *const libc::c_char,
) where
    T: ChildProxyImpl,
{
    glib_floating_reference_guard!(child_proxy);
    let instance = &*(child_proxy as *mut T::Instance);
    let imp = instance.get_impl();

    imp.child_removed(
        &from_glib_borrow(child_proxy),
        &from_glib_borrow(child),
        String::from_glib_none(name).as_str(),
    )
}

unsafe extern "C" fn child_proxy_init<T: ObjectSubclass>(
    iface: glib_ffi::gpointer,
    _iface_data: glib_ffi::gpointer,
) where
    T: ChildProxyImpl,
{
    let child_proxy_iface = &mut *(iface as *mut ffi::GstChildProxyInterface);

    child_proxy_iface.get_child_by_name = Some(child_proxy_get_child_by_name::<T>);
    child_proxy_iface.get_child_by_index = Some(child_proxy_get_child_by_index::<T>);
    child_proxy_iface.get_children_count = Some(child_proxy_get_children_count::<T>);
    child_proxy_iface.child_added = Some(child_proxy_child_added::<T>);
    child_proxy_iface.child_removed = Some(child_proxy_child_removed::<T>);
}

pub fn register<T: ObjectSubclass + ChildProxyImpl>(type_: glib::subclass::InitializingType<T>) {
    unsafe {
        let iface_info = gobject_ffi::GInterfaceInfo {
            interface_init: Some(child_proxy_init::<T>),
            interface_finalize: None,
            interface_data: ptr::null_mut(),
        };
        gobject_ffi::g_type_add_interface_static(
            type_.to_glib(),
            ffi::gst_child_proxy_get_type(),
            &iface_info,
        );
    }
}
