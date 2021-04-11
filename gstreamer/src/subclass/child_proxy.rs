// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;

use crate::ChildProxy;

pub trait ChildProxyImpl: ObjectImpl + Send + Sync {
    fn get_child_by_name(&self, object: &Self::Type, name: &str) -> Option<glib::Object> {
        self.parent_get_child_by_name(object, name)
    }

    fn get_child_by_index(&self, object: &Self::Type, index: u32) -> Option<glib::Object>;
    fn get_children_count(&self, object: &Self::Type) -> u32;

    fn child_added(&self, object: &Self::Type, child: &glib::Object, name: &str) {
        self.parent_child_added(object, child, name);
    }
    fn child_removed(&self, object: &Self::Type, child: &glib::Object, name: &str) {
        self.parent_child_removed(object, child, name);
    }
}

pub trait ChildProxyImplExt: ObjectSubclass {
    fn parent_get_child_by_name(&self, object: &Self::Type, name: &str) -> Option<glib::Object>;

    fn parent_get_child_by_index(&self, object: &Self::Type, index: u32) -> Option<glib::Object>;
    fn parent_get_children_count(&self, object: &Self::Type) -> u32;

    fn parent_child_added(&self, _object: &Self::Type, _child: &glib::Object, _name: &str);
    fn parent_child_removed(&self, _object: &Self::Type, _child: &glib::Object, _name: &str);
}

impl<T: ChildProxyImpl> ChildProxyImplExt for T {
    fn parent_get_child_by_name(&self, object: &Self::Type, name: &str) -> Option<glib::Object> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().get_parent_interface::<ChildProxy>()
                as *const ffi::GstChildProxyInterface;

            let func = (*parent_iface)
                .get_child_by_name
                .expect("no parent \"get_child_by_name\" implementation");
            let ret = func(
                object.unsafe_cast_ref::<ChildProxy>().to_glib_none().0,
                name.to_glib_none().0,
            );
            from_glib_full(ret)
        }
    }

    fn parent_get_child_by_index(&self, object: &Self::Type, index: u32) -> Option<glib::Object> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().get_parent_interface::<ChildProxy>()
                as *const ffi::GstChildProxyInterface;

            let func = (*parent_iface)
                .get_child_by_index
                .expect("no parent \"get_child_by_index\" implementation");
            let ret = func(
                object.unsafe_cast_ref::<ChildProxy>().to_glib_none().0,
                index,
            );
            from_glib_full(ret)
        }
    }

    fn parent_get_children_count(&self, object: &Self::Type) -> u32 {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().get_parent_interface::<ChildProxy>()
                as *const ffi::GstChildProxyInterface;

            let func = (*parent_iface)
                .get_children_count
                .expect("no parent \"get_children_count\" implementation");
            func(object.unsafe_cast_ref::<ChildProxy>().to_glib_none().0)
        }
    }

    fn parent_child_added(&self, object: &Self::Type, child: &glib::Object, name: &str) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().get_parent_interface::<ChildProxy>()
                as *const ffi::GstChildProxyInterface;

            if let Some(func) = (*parent_iface).child_added {
                func(
                    object.unsafe_cast_ref::<ChildProxy>().to_glib_none().0,
                    child.to_glib_none().0,
                    name.to_glib_none().0,
                );
            }
        }
    }

    fn parent_child_removed(&self, object: &Self::Type, child: &glib::Object, name: &str) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().get_parent_interface::<ChildProxy>()
                as *const ffi::GstChildProxyInterface;

            if let Some(func) = (*parent_iface).child_removed {
                func(
                    object.unsafe_cast_ref::<ChildProxy>().to_glib_none().0,
                    child.to_glib_none().0,
                    name.to_glib_none().0,
                );
            }
        }
    }
}

unsafe impl<T: ChildProxyImpl> IsImplementable<T> for ChildProxy {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        iface.get_child_by_name = Some(child_proxy_get_child_by_name::<T>);
        iface.get_child_by_index = Some(child_proxy_get_child_by_index::<T>);
        iface.get_children_count = Some(child_proxy_get_children_count::<T>);
        iface.child_added = Some(child_proxy_child_added::<T>);
        iface.child_removed = Some(child_proxy_child_removed::<T>);
    }

    fn instance_init(_instance: &mut glib::subclass::InitializingObject<T>) {}
}

unsafe extern "C" fn child_proxy_get_child_by_name<T: ChildProxyImpl>(
    child_proxy: *mut ffi::GstChildProxy,
    name: *const libc::c_char,
) -> *mut glib::gobject_ffi::GObject {
    let instance = &*(child_proxy as *mut T::Instance);
    let imp = instance.impl_();

    imp.get_child_by_name(
        &from_glib_borrow::<_, ChildProxy>(child_proxy).unsafe_cast_ref(),
        &glib::GString::from_glib_borrow(name),
    )
    .to_glib_full()
}

unsafe extern "C" fn child_proxy_get_child_by_index<T: ChildProxyImpl>(
    child_proxy: *mut ffi::GstChildProxy,
    index: u32,
) -> *mut glib::gobject_ffi::GObject {
    let instance = &*(child_proxy as *mut T::Instance);
    let imp = instance.impl_();

    imp.get_child_by_index(
        &from_glib_borrow::<_, ChildProxy>(child_proxy).unsafe_cast_ref(),
        index,
    )
    .to_glib_full()
}

unsafe extern "C" fn child_proxy_get_children_count<T: ChildProxyImpl>(
    child_proxy: *mut ffi::GstChildProxy,
) -> u32 {
    let instance = &*(child_proxy as *mut T::Instance);
    let imp = instance.impl_();

    imp.get_children_count(&from_glib_borrow::<_, ChildProxy>(child_proxy).unsafe_cast_ref())
}

unsafe extern "C" fn child_proxy_child_added<T: ChildProxyImpl>(
    child_proxy: *mut ffi::GstChildProxy,
    child: *mut glib::gobject_ffi::GObject,
    name: *const libc::c_char,
) {
    let instance = &*(child_proxy as *mut T::Instance);
    let imp = instance.impl_();

    imp.child_added(
        &from_glib_borrow::<_, ChildProxy>(child_proxy).unsafe_cast_ref(),
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
    let imp = instance.impl_();

    imp.child_removed(
        &from_glib_borrow::<_, ChildProxy>(child_proxy).unsafe_cast_ref(),
        &from_glib_borrow(child),
        &glib::GString::from_glib_borrow(name),
    )
}
