// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![recursion_limit = "256"]
#[macro_use]
extern crate bitflags;
extern crate libc;
#[macro_use]
extern crate lazy_static;

extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gstreamer_sys as ffi;

#[macro_use]
extern crate glib;

extern crate num_rational;

use glib::translate::{from_glib, from_glib_full};

macro_rules! callback_guard {
    () => (
        let _guard = ::glib::CallbackGuard::new();
    )
}

macro_rules! assert_initialized_main_thread {
    () => (
        assert_eq!(unsafe {ffi::gst_is_initialized()}, ::glib_ffi::GTRUE)
    )
}

macro_rules! skip_assert_initialized {
    () => (
    )
}

pub use glib::{Cast, Continue, Error, IsA, StaticType, ToValue, Type, TypedValue, Value};

#[cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]
#[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
#[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
#[cfg_attr(feature = "cargo-clippy", allow(match_same_arms))]
mod auto;
pub use auto::*;
pub use auto::traits::*;
pub use auto::functions::{parse_bin_from_description, parse_launch};

pub mod miniobject;
pub use miniobject::GstRc;
pub mod message;
pub use message::{Message, MessageRef, MessageView};
pub mod structure;
pub use structure::{Structure, StructureRef};
pub mod caps;
pub use caps::{Caps, CapsRef};
pub mod tags;
pub use tags::*;
pub mod buffer;
pub use buffer::{Buffer, BufferRef, ReadBufferMap, ReadMappedBuffer, ReadWriteBufferMap,
                 ReadWriteMappedBuffer};
pub mod sample;
pub use sample::{Sample, SampleRef};
pub mod bufferlist;
pub use bufferlist::{BufferList, BufferListRef};
pub mod query;
pub use query::{Query, QueryRef, QueryView};
pub mod event;
pub use event::{Event, EventRef, EventView};
pub mod context;
pub use context::{Context, ContextRef};

mod object;
mod element;
mod bin;
mod bus;
mod pad;
mod gobject;
mod proxy_pad;
mod ghost_pad;
mod child_proxy;
mod tag_setter;
mod iterator;
pub use object::{GstObjectExt, Object};
pub use element::ElementExtManual;
pub use bin::BinExtManual;
pub use pad::{PadExtManual, PadProbeData, PadProbeId, PadProbeInfo, PAD_PROBE_ID_INVALID};
pub use gobject::GObjectExtManualGst;
pub use child_proxy::ChildProxyExtManual;
pub use tag_setter::TagSetterExtManual;
pub use self::iterator::Iterator;

mod value;
pub use value::*;

mod segment;
pub use segment::*;

use std::ptr;

pub fn init() -> Result<(), glib::Error> {
    unsafe {
        let mut error = ptr::null_mut();
        if from_glib(ffi::gst_init_check(
            ptr::null_mut(),
            ptr::null_mut(),
            &mut error,
        )) {
            Ok(())
        } else {
            Err(from_glib_full(error))
        }
    }
}

pub const BUFFER_OFFSET_NONE: u64 = ffi::GST_BUFFER_OFFSET_NONE;
pub const CLOCK_TIME_NONE: u64 = ffi::GST_CLOCK_TIME_NONE;
