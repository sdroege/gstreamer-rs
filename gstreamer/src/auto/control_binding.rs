// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::ClockTime;
use crate::Object;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;

glib::wrapper! {
    pub struct ControlBinding(Object<ffi::GstControlBinding, ffi::GstControlBindingClass>) @extends Object;

    match fn {
        get_type => || ffi::gst_control_binding_get_type(),
    }
}

unsafe impl Send for ControlBinding {}
unsafe impl Sync for ControlBinding {}

pub const NONE_CONTROL_BINDING: Option<&ControlBinding> = None;

pub trait ControlBindingExt: 'static {
    #[doc(alias = "gst_control_binding_get_value")]
    fn get_value(&self, timestamp: ClockTime) -> Option<glib::Value>;

    //#[doc(alias = "gst_control_binding_get_value_array")]
    //fn get_value_array(&self, timestamp: ClockTime, interval: ClockTime, values: /*Unimplemented*/&[&Fundamental: Pointer]) -> bool;

    #[doc(alias = "gst_control_binding_is_disabled")]
    fn is_disabled(&self) -> bool;

    #[doc(alias = "gst_control_binding_set_disabled")]
    fn set_disabled(&self, disabled: bool);

    #[doc(alias = "gst_control_binding_sync_values")]
    fn sync_values<P: IsA<Object>>(
        &self,
        object: &P,
        timestamp: ClockTime,
        last_sync: ClockTime,
    ) -> bool;

    fn get_property_object(&self) -> Option<Object>;
}

impl<O: IsA<ControlBinding>> ControlBindingExt for O {
    fn get_value(&self, timestamp: ClockTime) -> Option<glib::Value> {
        unsafe {
            from_glib_full(ffi::gst_control_binding_get_value(
                self.as_ref().to_glib_none().0,
                timestamp.to_glib(),
            ))
        }
    }

    //fn get_value_array(&self, timestamp: ClockTime, interval: ClockTime, values: /*Unimplemented*/&[&Fundamental: Pointer]) -> bool {
    //    unsafe { TODO: call ffi:gst_control_binding_get_value_array() }
    //}

    fn is_disabled(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_control_binding_is_disabled(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_disabled(&self, disabled: bool) {
        unsafe {
            ffi::gst_control_binding_set_disabled(
                self.as_ref().to_glib_none().0,
                disabled.to_glib(),
            );
        }
    }

    fn sync_values<P: IsA<Object>>(
        &self,
        object: &P,
        timestamp: ClockTime,
        last_sync: ClockTime,
    ) -> bool {
        unsafe {
            from_glib(ffi::gst_control_binding_sync_values(
                self.as_ref().to_glib_none().0,
                object.as_ref().to_glib_none().0,
                timestamp.to_glib(),
                last_sync.to_glib(),
            ))
        }
    }

    fn get_property_object(&self) -> Option<Object> {
        unsafe {
            let mut value = glib::Value::from_type(<Object as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"object\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `object` getter")
        }
    }
}