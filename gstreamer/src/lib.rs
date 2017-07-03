#[macro_use]
extern crate bitflags;
extern crate libc;

extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gstreamer_sys as ffi;

#[macro_use]
extern crate glib;

use glib::translate::{from_glib, from_glib_full};

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
pub use auto::functions::{parse_launch, parse_bin_from_description};

pub mod miniobject;
pub use miniobject::GstRc;
pub mod message;
pub use message::Message;

use std::ptr;

pub fn init() -> Result<(), glib::Error> {
    unsafe {
        let mut error = ptr::null_mut();
        if from_glib(ffi::gst_init_check(ptr::null_mut(), ptr::null_mut(), &mut error)) {
            Ok(())
        } else {
            Err(from_glib_full(error))
        }
    }
}
