#[macro_use]
extern crate bitflags;
extern crate libc;

extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gstreamer_sys as ffi;

#[macro_use]
extern crate glib;

macro_rules! callback_guard {
    () => (
        let _guard = ::glib::CallbackGuard::new();
    )
}

pub use glib::{
    Cast,
    Continue,
    Error,
    IsA,
    StaticType,
    ToValue,
    Type,
    TypedValue,
    Value,
};

pub use auto::*;
mod auto;

use std::ptr;

pub fn init() {
    unsafe {
        ffi::gst_init(ptr::null_mut(), ptr::null_mut())
    }
}
