// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Edge;
use EditMode;
use Extractable;
use Layer;
use TimelineElement;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Container(Object<ffi::GESContainer, ffi::GESContainerClass>): TimelineElement, Extractable;

    match fn {
        get_type => || ffi::ges_container_get_type(),
    }
}

impl Container {
    pub fn group(containers: &[Container]) -> Option<Container> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::ges_container_group(containers.to_glib_none().0))
        }
    }
}

pub trait GESContainerExt {
    fn add<P: IsA<TimelineElement>>(&self, child: &P) -> Result<(), glib::error::BoolError>;

    fn edit(&self, layers: &[Layer], new_layer_priority: i32, mode: EditMode, edge: Edge, position: u64) -> bool;

    fn get_children(&self, recursive: bool) -> Vec<TimelineElement>;

    fn remove<P: IsA<TimelineElement>>(&self, child: &P) -> Result<(), glib::error::BoolError>;

    fn ungroup(&self, recursive: bool) -> Vec<Container>;

    fn get_property_height(&self) -> u32;

    fn connect_child_added<F: Fn(&Self, &TimelineElement) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_child_removed<F: Fn(&Self, &TimelineElement) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Container> + IsA<glib::object::Object>> GESContainerExt for O {
    fn add<P: IsA<TimelineElement>>(&self, child: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::ges_container_add(self.to_glib_none().0, child.to_glib_none().0), "Failed to add element")
        }
    }

    fn edit(&self, layers: &[Layer], new_layer_priority: i32, mode: EditMode, edge: Edge, position: u64) -> bool {
        unsafe {
            from_glib(ffi::ges_container_edit(self.to_glib_none().0, layers.to_glib_none().0, new_layer_priority, mode.to_glib(), edge.to_glib(), position))
        }
    }

    fn get_children(&self, recursive: bool) -> Vec<TimelineElement> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::ges_container_get_children(self.to_glib_none().0, recursive.to_glib()))
        }
    }

    fn remove<P: IsA<TimelineElement>>(&self, child: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::ges_container_remove(self.to_glib_none().0, child.to_glib_none().0), "Failed to remove element")
        }
    }

    fn ungroup(&self, recursive: bool) -> Vec<Container> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::ges_container_ungroup(self.to_glib_full(), recursive.to_glib()))
        }
    }

    fn get_property_height(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "height".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn connect_child_added<F: Fn(&Self, &TimelineElement) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &TimelineElement) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "child-added",
                transmute(child_added_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_child_removed<F: Fn(&Self, &TimelineElement) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &TimelineElement) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "child-removed",
                transmute(child_removed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::height",
                transmute(notify_height_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn child_added_trampoline<P>(this: *mut ffi::GESContainer, element: *mut ffi::GESTimelineElement, f: glib_ffi::gpointer)
where P: IsA<Container> {
    let f: &&(Fn(&P, &TimelineElement) + 'static) = transmute(f);
    f(&Container::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(element))
}

unsafe extern "C" fn child_removed_trampoline<P>(this: *mut ffi::GESContainer, element: *mut ffi::GESTimelineElement, f: glib_ffi::gpointer)
where P: IsA<Container> {
    let f: &&(Fn(&P, &TimelineElement) + 'static) = transmute(f);
    f(&Container::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(element))
}

unsafe extern "C" fn notify_height_trampoline<P>(this: *mut ffi::GESContainer, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Container> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Container::from_glib_borrow(this).downcast_unchecked())
}
