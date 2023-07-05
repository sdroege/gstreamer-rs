// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, subclass::prelude::*, translate::*};

use super::prelude::*;
use crate::ChildProxy;

pub trait ChildProxyImpl: GstObjectImpl + Send + Sync {
    fn child_by_name(&self, name: &str) -> Option<glib::Object> {
        self.parent_child_by_name(name)
    }

    fn child_by_index(&self, index: u32) -> Option<glib::Object>;
    fn children_count(&self) -> u32;

    fn child_added(&self, child: &glib::Object, name: &str) {
        self.parent_child_added(child, name);
    }
    fn child_removed(&self, child: &glib::Object, name: &str) {
        self.parent_child_removed(child, name);
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::ChildProxyImplExt> Sealed for T {}
}

pub trait ChildProxyImplExt: sealed::Sealed + ObjectSubclass {
    fn parent_child_by_name(&self, name: &str) -> Option<glib::Object> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<ChildProxy>()
                as *const ffi::GstChildProxyInterface;

            let func = (*parent_iface)
                .get_child_by_name
                .expect("no parent \"child_by_name\" implementation");
            let ret = func(
                self.obj().unsafe_cast_ref::<ChildProxy>().to_glib_none().0,
                name.to_glib_none().0,
            );
            from_glib_full(ret)
        }
    }

    fn parent_child_by_index(&self, index: u32) -> Option<glib::Object> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<ChildProxy>()
                as *const ffi::GstChildProxyInterface;

            let func = (*parent_iface)
                .get_child_by_index
                .expect("no parent \"child_by_index\" implementation");
            let ret = func(
                self.obj().unsafe_cast_ref::<ChildProxy>().to_glib_none().0,
                index,
            );
            from_glib_full(ret)
        }
    }

    fn parent_children_count(&self) -> u32 {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<ChildProxy>()
                as *const ffi::GstChildProxyInterface;

            let func = (*parent_iface)
                .get_children_count
                .expect("no parent \"children_count\" implementation");
            func(self.obj().unsafe_cast_ref::<ChildProxy>().to_glib_none().0)
        }
    }

    fn parent_child_added(&self, child: &glib::Object, name: &str) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<ChildProxy>()
                as *const ffi::GstChildProxyInterface;

            if let Some(func) = (*parent_iface).child_added {
                func(
                    self.obj().unsafe_cast_ref::<ChildProxy>().to_glib_none().0,
                    child.to_glib_none().0,
                    name.to_glib_none().0,
                );
            }
        }
    }

    fn parent_child_removed(&self, child: &glib::Object, name: &str) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<ChildProxy>()
                as *const ffi::GstChildProxyInterface;

            if let Some(func) = (*parent_iface).child_removed {
                func(
                    self.obj().unsafe_cast_ref::<ChildProxy>().to_glib_none().0,
                    child.to_glib_none().0,
                    name.to_glib_none().0,
                );
            }
        }
    }
}

impl<T: ChildProxyImpl> ChildProxyImplExt for T {}

unsafe impl<T: ChildProxyImpl> IsImplementable<T> for ChildProxy {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        iface.get_child_by_name = Some(child_proxy_get_child_by_name::<T>);
        iface.get_child_by_index = Some(child_proxy_get_child_by_index::<T>);
        iface.get_children_count = Some(child_proxy_get_children_count::<T>);
        iface.child_added = Some(child_proxy_child_added::<T>);
        iface.child_removed = Some(child_proxy_child_removed::<T>);
    }
}

unsafe extern "C" fn child_proxy_get_child_by_name<T: ChildProxyImpl>(
    child_proxy: *mut ffi::GstChildProxy,
    name: *const libc::c_char,
) -> *mut glib::gobject_ffi::GObject {
    let instance = &*(child_proxy as *mut T::Instance);
    let imp = instance.imp();

    imp.child_by_name(&glib::GString::from_glib_borrow(name))
        .into_glib_ptr()
}

unsafe extern "C" fn child_proxy_get_child_by_index<T: ChildProxyImpl>(
    child_proxy: *mut ffi::GstChildProxy,
    index: u32,
) -> *mut glib::gobject_ffi::GObject {
    let instance = &*(child_proxy as *mut T::Instance);
    let imp = instance.imp();

    imp.child_by_index(index).into_glib_ptr()
}

unsafe extern "C" fn child_proxy_get_children_count<T: ChildProxyImpl>(
    child_proxy: *mut ffi::GstChildProxy,
) -> u32 {
    let instance = &*(child_proxy as *mut T::Instance);
    let imp = instance.imp();

    imp.children_count()
}

unsafe extern "C" fn child_proxy_child_added<T: ChildProxyImpl>(
    child_proxy: *mut ffi::GstChildProxy,
    child: *mut glib::gobject_ffi::GObject,
    name: *const libc::c_char,
) {
    let instance = &*(child_proxy as *mut T::Instance);
    let imp = instance.imp();

    imp.child_added(
        &from_glib_borrow(child),
        &glib::GString::from_glib_borrow(name),
    )
}

unsafe extern "C" fn child_proxy_child_removed<T: ChildProxyImpl>(
    child_proxy: *mut ffi::GstChildProxy,
    child: *mut glib::gobject_ffi::GObject,
    name: *const libc::c_char,
) {
    let instance = &*(child_proxy as *mut T::Instance);
    let imp = instance.imp();

    imp.child_removed(
        &from_glib_borrow(child),
        &glib::GString::from_glib_borrow(name),
    )
}
