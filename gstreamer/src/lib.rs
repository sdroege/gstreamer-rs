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
#[cfg(any(feature = "v1_14", feature = "dox"))]
#[macro_use]
extern crate cfg_if;
#[macro_use]
extern crate lazy_static;
extern crate libc;

extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gstreamer_sys as ffi;

#[macro_use]
extern crate glib;

extern crate num_rational;

#[cfg(any(feature = "futures", feature = "dox"))]
extern crate futures_core;

extern crate muldiv;

use glib::translate::{from_glib, from_glib_full};

macro_rules! callback_guard {
    () => {
        let _guard = ::glib::CallbackGuard::new();
    };
}

macro_rules! assert_initialized_main_thread {
    () => {
        if unsafe { ::ffi::gst_is_initialized() } != ::glib_ffi::GTRUE {
            panic!("GStreamer has not been initialized. Call `gst::init` first.");
        }
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

pub use glib::{Cast, Continue, Error, IsA, StaticType, ToValue, Type, TypedValue, Value};

#[cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]
#[cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
#[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
#[cfg_attr(feature = "cargo-clippy", allow(match_same_arms))]
#[cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
mod auto;
pub use auto::functions::*;
pub use auto::*;

#[macro_use]
mod log;
pub use log::*;

#[macro_use]
mod error;
pub use error::*;

pub mod miniobject;
pub use miniobject::{GstRc, MiniObject};
pub mod message;
pub use message::{Message, MessageErrorDomain, MessageRef, MessageView};
pub mod structure;
pub use structure::{Structure, StructureRef};
pub mod caps;
pub use caps::{Caps, CapsRef};
pub mod tags;
pub use tags::{Tag, TagList, TagListRef};
pub mod buffer;
pub use buffer::{Buffer, BufferMap, BufferRef, MappedBuffer, BUFFER_COPY_ALL, BUFFER_COPY_METADATA};
pub mod sample;
pub use sample::{Sample, SampleRef};
pub mod bufferlist;
pub use bufferlist::{BufferList, BufferListRef};
pub mod query;
pub use query::{Query, QueryRef, QueryView};
pub mod event;
pub use event::{Event, EventRef, EventView, GroupId, Seqnum, GROUP_ID_INVALID, SEQNUM_INVALID};
pub mod context;
pub use context::{Context, ContextRef};
mod static_caps;
pub use static_caps::*;
mod static_pad_template;
pub use static_pad_template::*;

#[cfg(any(feature = "v1_14", feature = "dox"))]
mod promise;
#[cfg(any(feature = "v1_14", feature = "dox"))]
pub use promise::*;

mod bin;
mod bus;
mod element;

// OS dependent Bus extensions (also import the other plateform mod for doc)
#[cfg(any(feature = "v1_14", feature = "dox"))]
cfg_if! {
    if #[cfg(unix)] {
        mod bus_unix;
        #[cfg(feature = "dox")]
        mod bus_windows;
    } else {
        mod bus_windows;
        #[cfg(feature = "dox")]
        mod bus_unix;
    }
}

mod child_proxy;
mod clock_time;
mod date_time;
mod device_provider;
mod enums;
mod ghost_pad;
mod gobject;
mod iterator;
mod object;
mod pad;
mod parse_context;
mod proxy_pad;
mod tag_setter;
pub use bin::BinExtManual;
pub use element::{ELEMENT_METADATA_AUTHOR, ELEMENT_METADATA_DESCRIPTION, ELEMENT_METADATA_DOC_URI,
                  ELEMENT_METADATA_ICON_NAME, ELEMENT_METADATA_KLASS, ELEMENT_METADATA_LONGNAME};
pub use element::{ElementExtManual, ElementMessageType, NotifyWatchId};
pub use object::GstObjectExtManual;

// OS dependent Bus extensions (also import the other plateform trait for doc)
#[cfg(any(feature = "v1_14", feature = "dox"))]
cfg_if! {
    if #[cfg(unix)] {
        pub use bus_unix::UnixBusExtManual;
        #[cfg(feature = "dox")]
        pub use bus_windows::WindowsBusExtManual;
    } else {
        pub use bus_windows::WindowsBusExtManual;
        #[cfg(feature = "dox")]
        pub use bus_unix::UnixBusExtManual;
    }
}

pub use self::iterator::{Iterator, IteratorError, IteratorImpl};
#[cfg(any(feature = "futures", feature = "dox"))]
pub use bus::BusStream;
pub use child_proxy::ChildProxyExtManual;
pub use clock_time::ClockTime;
pub use device_provider::DeviceProviderExtManual;
pub use enums::{ClockError, ClockSuccess, FlowError, FlowSuccess, PadLinkError, PadLinkSuccess,
                StateChangeError, StateChangeSuccess};
pub use gobject::GObjectExtManualGst;
pub use pad::{PadExtManual, PadProbeData, PadProbeId, PadProbeInfo};
pub use parse_context::ParseContext;
pub use tag_setter::TagSetterExtManual;

mod plugin;
#[cfg(any(feature = "v1_10", feature = "dox"))]
pub mod stream_collection;

mod typefind;
pub use typefind::*;

pub mod format;
pub use format::{FormattedValue, GenericFormattedValue, SpecificFormattedValue};

mod value;
pub use value::*;

mod segment;
pub use segment::*;

pub mod toc;
pub use toc::{Toc, TocEntry, TocEntryRef, TocRef};

mod clock;
pub use clock::{ClockExtManual, ClockId};

mod buffer_pool;
pub use buffer_pool::*;

mod pad_template;

pub mod functions;
pub use functions::*;

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
pub const CLOCK_TIME_NONE: ClockTime = ClockTime(None);

pub const SECOND: ClockTime = ClockTime(Some(1_000_000_000));
pub const MSECOND: ClockTime = ClockTime(Some(1_000_000));
pub const USECOND: ClockTime = ClockTime(Some(1_000));
pub const NSECOND: ClockTime = ClockTime(Some(1));

pub const SECOND_VAL: u64 = 1_000_000_000;
pub const MSECOND_VAL: u64 = 1_000_000;
pub const USECOND_VAL: u64 = 1_000;
pub const NSECOND_VAL: u64 = 1;

pub const FORMAT_PERCENT_MAX: u32 = ffi::GST_FORMAT_PERCENT_MAX as u32;
pub const FORMAT_PERCENT_SCALE: u32 = ffi::GST_FORMAT_PERCENT_SCALE as u32;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst::prelude::*" without getting conflicts
pub mod prelude {
    pub use glib::prelude::*;

    pub use auto::traits::*;

    pub use bin::BinExtManual;
    pub use element::ElementExtManual;

    // OS dependent Bus extensions (also import the other plateform trait for doc)
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    cfg_if! {
        if #[cfg(unix)] {
            pub use bus_unix::UnixBusExtManual;
            #[cfg(feature = "dox")]
            pub use bus_windows::WindowsBusExtManual;
        } else {
            pub use bus_windows::WindowsBusExtManual;
            #[cfg(feature = "dox")]
            pub use bus_unix::UnixBusExtManual;
        }
    }

    pub use buffer_pool::BufferPoolExtManual;
    pub use child_proxy::ChildProxyExtManual;
    pub use clock::ClockExtManual;
    pub use device_provider::DeviceProviderExtManual;
    pub use gobject::GObjectExtManualGst;
    pub use object::GstObjectExtManual;
    pub use pad::PadExtManual;
    pub use tag_setter::TagSetterExtManual;
    pub use value::GstValueExt;

    pub use miniobject::MiniObject;
    pub use tags::Tag;

    pub use muldiv::MulDiv;

    pub use format::{FormattedValue, SpecificFormattedValue};
}

mod utils;
