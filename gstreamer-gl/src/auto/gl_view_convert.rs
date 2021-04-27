// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::GLContext;
use crate::GLStereoDownmix;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::wrapper! {
    pub struct GLViewConvert(Object<ffi::GstGLViewConvert, ffi::GstGLViewConvertClass>) @extends gst::Object;

    match fn {
        type_ => || ffi::gst_gl_view_convert_get_type(),
    }
}

impl GLViewConvert {
    #[doc(alias = "gst_gl_view_convert_new")]
    pub fn new() -> GLViewConvert {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_gl_view_convert_new()) }
    }

    #[doc(alias = "gst_gl_view_convert_perform")]
    pub fn perform(&self, inbuf: &gst::Buffer) -> Option<gst::Buffer> {
        unsafe {
            from_glib_full(ffi::gst_gl_view_convert_perform(
                self.to_glib_none().0,
                inbuf.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_gl_view_convert_reset")]
    pub fn reset(&self) {
        unsafe {
            ffi::gst_gl_view_convert_reset(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gst_gl_view_convert_set_caps")]
    pub fn set_caps(
        &self,
        in_caps: &gst::Caps,
        out_caps: &gst::Caps,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_gl_view_convert_set_caps(
                    self.to_glib_none().0,
                    in_caps.to_glib_none().0,
                    out_caps.to_glib_none().0
                ),
                "Failed to set caps"
            )
        }
    }

    #[doc(alias = "gst_gl_view_convert_set_context")]
    pub fn set_context<P: IsA<GLContext>>(&self, context: &P) {
        unsafe {
            ffi::gst_gl_view_convert_set_context(
                self.to_glib_none().0,
                context.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_gl_view_convert_transform_caps")]
    pub fn transform_caps(
        &self,
        direction: gst::PadDirection,
        caps: &gst::Caps,
        filter: &gst::Caps,
    ) -> Option<gst::Caps> {
        unsafe {
            from_glib_full(ffi::gst_gl_view_convert_transform_caps(
                self.to_glib_none().0,
                direction.into_glib(),
                caps.to_glib_none().0,
                filter.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "get_property_downmix_mode")]
    pub fn downmix_mode(&self) -> GLStereoDownmix {
        unsafe {
            let mut value = glib::Value::from_type(<GLStereoDownmix as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"downmix-mode\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `downmix-mode` getter")
        }
    }

    #[doc(alias = "set_property_downmix_mode")]
    pub fn set_downmix_mode(&self, downmix_mode: GLStereoDownmix) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"downmix-mode\0".as_ptr() as *const _,
                downmix_mode.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "get_property_input_flags_override")]
    pub fn input_flags_override(&self) -> gst_video::VideoMultiviewFlags {
        unsafe {
            let mut value = glib::Value::from_type(
                <gst_video::VideoMultiviewFlags as StaticType>::static_type(),
            );
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"input-flags-override\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `input-flags-override` getter")
        }
    }

    #[doc(alias = "set_property_input_flags_override")]
    pub fn set_input_flags_override(&self, input_flags_override: gst_video::VideoMultiviewFlags) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"input-flags-override\0".as_ptr() as *const _,
                input_flags_override.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "get_property_input_mode_override")]
    pub fn input_mode_override(&self) -> gst_video::VideoMultiviewMode {
        unsafe {
            let mut value = glib::Value::from_type(
                <gst_video::VideoMultiviewMode as StaticType>::static_type(),
            );
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"input-mode-override\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `input-mode-override` getter")
        }
    }

    #[doc(alias = "set_property_input_mode_override")]
    pub fn set_input_mode_override(&self, input_mode_override: gst_video::VideoMultiviewMode) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"input-mode-override\0".as_ptr() as *const _,
                input_mode_override.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "get_property_output_flags_override")]
    pub fn output_flags_override(&self) -> gst_video::VideoMultiviewFlags {
        unsafe {
            let mut value = glib::Value::from_type(
                <gst_video::VideoMultiviewFlags as StaticType>::static_type(),
            );
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"output-flags-override\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `output-flags-override` getter")
        }
    }

    #[doc(alias = "set_property_output_flags_override")]
    pub fn set_output_flags_override(&self, output_flags_override: gst_video::VideoMultiviewFlags) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"output-flags-override\0".as_ptr() as *const _,
                output_flags_override.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "get_property_output_mode_override")]
    pub fn output_mode_override(&self) -> gst_video::VideoMultiviewMode {
        unsafe {
            let mut value = glib::Value::from_type(
                <gst_video::VideoMultiviewMode as StaticType>::static_type(),
            );
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"output-mode-override\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `output-mode-override` getter")
        }
    }

    #[doc(alias = "set_property_output_mode_override")]
    pub fn set_output_mode_override(&self, output_mode_override: gst_video::VideoMultiviewMode) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"output-mode-override\0".as_ptr() as *const _,
                output_mode_override.to_value().to_glib_none().0,
            );
        }
    }

    pub fn connect_property_downmix_mode_notify<F: Fn(&GLViewConvert) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_downmix_mode_trampoline<
            F: Fn(&GLViewConvert) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstGLViewConvert,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::downmix-mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_downmix_mode_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_input_flags_override_notify<
        F: Fn(&GLViewConvert) + Send + Sync + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_input_flags_override_trampoline<
            F: Fn(&GLViewConvert) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstGLViewConvert,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::input-flags-override\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_input_flags_override_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_input_mode_override_notify<
        F: Fn(&GLViewConvert) + Send + Sync + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_input_mode_override_trampoline<
            F: Fn(&GLViewConvert) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstGLViewConvert,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::input-mode-override\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_input_mode_override_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_output_flags_override_notify<
        F: Fn(&GLViewConvert) + Send + Sync + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_output_flags_override_trampoline<
            F: Fn(&GLViewConvert) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstGLViewConvert,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::output-flags-override\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_output_flags_override_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_output_mode_override_notify<
        F: Fn(&GLViewConvert) + Send + Sync + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_output_mode_override_trampoline<
            F: Fn(&GLViewConvert) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstGLViewConvert,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::output-mode-override\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_output_mode_override_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for GLViewConvert {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for GLViewConvert {}
unsafe impl Sync for GLViewConvert {}
