// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use GLContext;
use GLDisplay;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gst;
use gst_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct GLWindow(Object<ffi::GstGLWindow, ffi::GstGLWindowClass>): [
        gst::Object => gst_ffi::GstObject,
    ];

    match fn {
        get_type => || ffi::gst_gl_window_get_type(),
    }
}

impl GLWindow {
    pub fn new<P: IsA<GLDisplay>>(display: &P) -> GLWindow {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gst_gl_window_new(display.to_glib_none().0))
        }
    }
}

unsafe impl Send for GLWindow {}
unsafe impl Sync for GLWindow {}

pub trait GLWindowExt {
    fn draw(&self);

    fn get_context(&self) -> Option<GLContext>;

    fn get_surface_dimensions(&self) -> (u32, u32);

    fn handle_events(&self, handle_events: bool);

    fn queue_resize(&self);

    fn quit(&self);

    fn resize(&self, width: u32, height: u32);

    fn run(&self);

    fn send_key_event(&self, event_type: &str, key_str: &str);

    fn send_mouse_event(&self, event_type: &str, button: i32, posx: f64, posy: f64);

    fn set_preferred_size(&self, width: i32, height: i32);

    fn set_render_rectangle(&self, x: i32, y: i32, width: i32, height: i32) -> bool;

    fn show(&self);

    fn connect_key_event<F: Fn(&Self, &str, &str) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_mouse_event<F: Fn(&Self, &str, i32, f64, f64) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<GLWindow> + IsA<glib::object::Object>> GLWindowExt for O {
    fn draw(&self) {
        unsafe {
            ffi::gst_gl_window_draw(self.to_glib_none().0);
        }
    }

    fn get_context(&self) -> Option<GLContext> {
        unsafe {
            from_glib_full(ffi::gst_gl_window_get_context(self.to_glib_none().0))
        }
    }

    fn get_surface_dimensions(&self) -> (u32, u32) {
        unsafe {
            let mut width = mem::uninitialized();
            let mut height = mem::uninitialized();
            ffi::gst_gl_window_get_surface_dimensions(self.to_glib_none().0, &mut width, &mut height);
            (width, height)
        }
    }

    fn handle_events(&self, handle_events: bool) {
        unsafe {
            ffi::gst_gl_window_handle_events(self.to_glib_none().0, handle_events.to_glib());
        }
    }

    fn queue_resize(&self) {
        unsafe {
            ffi::gst_gl_window_queue_resize(self.to_glib_none().0);
        }
    }

    fn quit(&self) {
        unsafe {
            ffi::gst_gl_window_quit(self.to_glib_none().0);
        }
    }

    fn resize(&self, width: u32, height: u32) {
        unsafe {
            ffi::gst_gl_window_resize(self.to_glib_none().0, width, height);
        }
    }

    fn run(&self) {
        unsafe {
            ffi::gst_gl_window_run(self.to_glib_none().0);
        }
    }

    fn send_key_event(&self, event_type: &str, key_str: &str) {
        unsafe {
            ffi::gst_gl_window_send_key_event(self.to_glib_none().0, event_type.to_glib_none().0, key_str.to_glib_none().0);
        }
    }

    fn send_mouse_event(&self, event_type: &str, button: i32, posx: f64, posy: f64) {
        unsafe {
            ffi::gst_gl_window_send_mouse_event(self.to_glib_none().0, event_type.to_glib_none().0, button, posx, posy);
        }
    }

    fn set_preferred_size(&self, width: i32, height: i32) {
        unsafe {
            ffi::gst_gl_window_set_preferred_size(self.to_glib_none().0, width, height);
        }
    }

    fn set_render_rectangle(&self, x: i32, y: i32, width: i32, height: i32) -> bool {
        unsafe {
            from_glib(ffi::gst_gl_window_set_render_rectangle(self.to_glib_none().0, x, y, width, height))
        }
    }

    fn show(&self) {
        unsafe {
            ffi::gst_gl_window_show(self.to_glib_none().0);
        }
    }

    fn connect_key_event<F: Fn(&Self, &str, &str) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str, &str) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "key-event",
                transmute(key_event_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_mouse_event<F: Fn(&Self, &str, i32, f64, f64) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str, i32, f64, f64) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "mouse-event",
                transmute(mouse_event_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn key_event_trampoline<P>(this: *mut ffi::GstGLWindow, id: *mut libc::c_char, key: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<GLWindow> {
    let f: &&(Fn(&P, &str, &str) + Send + Sync + 'static) = transmute(f);
    f(&GLWindow::from_glib_borrow(this).downcast_unchecked(), &String::from_glib_none(id), &String::from_glib_none(key))
}

unsafe extern "C" fn mouse_event_trampoline<P>(this: *mut ffi::GstGLWindow, id: *mut libc::c_char, button: libc::c_int, x: libc::c_double, y: libc::c_double, f: glib_ffi::gpointer)
where P: IsA<GLWindow> {
    let f: &&(Fn(&P, &str, i32, f64, f64) + Send + Sync + 'static) = transmute(f);
    f(&GLWindow::from_glib_borrow(this).downcast_unchecked(), &String::from_glib_none(id), button, x, y)
}
