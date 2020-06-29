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
extern crate libc;
extern crate once_cell;
extern crate thiserror;

// Re-exported for the subclass gst_plugin_define! macro
#[doc(hidden)]
pub extern crate glib_sys;
#[doc(hidden)]
pub extern crate gobject_sys;
#[doc(hidden)]
pub extern crate gstreamer_sys as gst_sys;
#[doc(hidden)]
pub extern crate paste;

#[macro_use]
#[doc(hidden)]
pub extern crate glib;

extern crate num_rational;

extern crate futures_channel;
extern crate futures_core;
extern crate futures_util;

extern crate muldiv;

extern crate pretty_hex;

#[cfg(feature = "ser_de")]
extern crate serde;
#[cfg(feature = "ser_de")]
extern crate serde_bytes;
#[cfg(feature = "ser_de")]
#[macro_use]
extern crate serde_derive;

#[cfg(test)]
extern crate futures_executor;

use glib::translate::{from_glib, from_glib_full};

macro_rules! assert_initialized_main_thread {
    () => {
        if unsafe { ::gst_sys::gst_is_initialized() } != ::glib_sys::GTRUE {
            panic!("GStreamer has not been initialized. Call `gst::init` first.");
        }
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

#[allow(clippy::unreadable_literal)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::match_same_arms)]
#[allow(clippy::type_complexity)]
mod auto;
pub use auto::functions::*;
pub use auto::*;

#[macro_use]
mod log;
pub use log::*;

#[macro_use]
mod error;
pub use error::*;

#[macro_use]
pub mod miniobject;
pub mod message;
pub use message::{Message, MessageErrorDomain, MessageRef, MessageView};

mod value;
pub use value::*;
#[cfg(feature = "ser_de")]
#[macro_use]
mod value_serde;

pub mod structure;
pub use structure::{Structure, StructureRef};
#[cfg(feature = "ser_de")]
mod structure_serde;

pub mod caps;
pub use caps::{Caps, CapsRef};
mod caps_features;
#[cfg(feature = "ser_de")]
mod caps_serde;
pub use caps_features::{
    CapsFeatures, CapsFeaturesRef, CAPS_FEATURES_MEMORY_SYSTEM_MEMORY,
    CAPS_FEATURE_MEMORY_SYSTEM_MEMORY,
};
#[cfg(feature = "ser_de")]
mod caps_features_serde;

pub mod tags;
pub use tags::{
    tag_exists, tag_get_description, tag_get_flag, tag_get_nick, tag_get_type, Tag, TagList,
    TagListRef,
};
#[cfg(feature = "ser_de")]
mod tags_serde;

pub mod meta;
#[cfg(any(feature = "v1_16", feature = "dox"))]
pub use meta::MetaSeqnum;
#[cfg(any(feature = "v1_14", feature = "dox"))]
pub use meta::ReferenceTimestampMeta;
pub use meta::{Meta, MetaAPI, MetaRef, MetaRefMut, ParentBufferMeta, ProtectionMeta};
pub mod buffer;
pub use buffer::{
    Buffer, BufferMap, BufferRef, MappedBuffer, BUFFER_COPY_ALL, BUFFER_COPY_METADATA,
};
mod buffer_cursor;
pub use buffer_cursor::{BufferCursor, BufferRefCursor};
pub mod memory;
pub use memory::{MappedMemory, Memory, MemoryMap, MemoryRef};
#[cfg(feature = "ser_de")]
mod buffer_serde;

pub mod sample;
pub use sample::{Sample, SampleRef};
#[cfg(feature = "ser_de")]
mod sample_serde;

pub mod bufferlist;
pub use bufferlist::{BufferList, BufferListRef};
#[cfg(feature = "ser_de")]
mod bufferlist_serde;

pub mod query;
pub use query::{Query, QueryRef, QueryView};
pub mod event;
pub use event::{Event, EventRef, EventView, GroupId, Seqnum};
pub mod context;
pub use context::{Context, ContextRef};
mod static_caps;
pub use static_caps::*;
mod static_pad_template;
pub use static_pad_template::*;

#[cfg(any(feature = "v1_14", feature = "dox"))]
pub mod promise;
#[cfg(any(feature = "v1_14", feature = "dox"))]
pub use promise::{Promise, PromiseError};

pub mod bus;
mod element;

mod bin;

mod allocator;
pub use allocator::AllocatorExtManual;
mod pipeline;
pub use pipeline::GstPipelineExtManual;

mod allocation_params;
pub use self::allocation_params::AllocationParams;

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
#[cfg(feature = "ser_de")]
mod clock_time_serde;
mod date_time;
#[cfg(feature = "ser_de")]
mod date_time_serde;
mod device_monitor;
mod device_provider;
mod enums;
pub use enums::MessageType;
mod ghost_pad;
mod gobject;
mod iterator;
mod object;
mod pad;
pub use pad::PadBuilder;
mod parse_context;
mod proxy_pad;
mod tag_setter;
pub use bin::GstBinExtManual;
pub use element::{ElementExtManual, ElementMessageType, NotifyWatchId};
pub use element::{
    ELEMENT_METADATA_AUTHOR, ELEMENT_METADATA_DESCRIPTION, ELEMENT_METADATA_DOC_URI,
    ELEMENT_METADATA_ICON_NAME, ELEMENT_METADATA_KLASS, ELEMENT_METADATA_LONGNAME,
};
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

pub use self::iterator::{Iterator, IteratorError, IteratorImpl, StdIterator};
pub use child_proxy::ChildProxyExtManual;
pub use clock_time::ClockTime;
pub use device_monitor::{DeviceMonitorExtManual, DeviceMonitorFilterId};
pub use device_provider::DeviceProviderExtManual;
pub use enums::{
    ClockError, ClockSuccess, FlowError, FlowSuccess, PadLinkError, PadLinkSuccess,
    StateChangeError, StateChangeSuccess, TagError,
};
pub use gobject::GObjectExtManualGst;
pub use pad::{PadExtManual, PadGetRangeSuccess, PadProbeData, PadProbeId, PadProbeInfo};
pub use parse_context::ParseContext;
mod plugin_feature;
pub use plugin_feature::PluginFeatureExtManual;
pub use tag_setter::TagSetterExtManual;

mod plugin;
pub use plugin::GstPluginExtManual;
#[cfg(any(feature = "v1_10", feature = "dox"))]
pub mod stream;
#[cfg(any(feature = "v1_10", feature = "dox"))]
pub mod stream_collection;

mod typefind;
pub use typefind::*;

pub mod format;
pub use format::{FormattedValue, GenericFormattedValue, SpecificFormattedValue};
#[cfg(feature = "ser_de")]
mod format_serde;

mod segment;
pub use segment::*;
#[cfg(feature = "ser_de")]
mod segment_serde;

pub mod toc;
pub use toc::{Toc, TocEntry, TocEntryRef, TocRef};
#[cfg(feature = "ser_de")]
mod toc_serde;

mod clock;
pub use clock::{AtomicClockReturn, ClockExtManual, ClockId};

mod buffer_pool;
pub use buffer_pool::*;

mod pad_template;

mod param_spec;
pub use param_spec::*;

pub mod functions;
pub use functions::*;

use std::ptr;

pub fn init() -> Result<(), glib::Error> {
    unsafe {
        let mut error = ptr::null_mut();
        if from_glib(gst_sys::gst_init_check(
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

/// Deinitialize GStreamer
///
/// # Safety
///
/// This must only be called once during the lifetime of the process, once no GStreamer threads
/// are running anymore and all GStreamer resources are released.
pub unsafe fn deinit() {
    gst_sys::gst_deinit();
}

pub const BUFFER_OFFSET_NONE: u64 = gst_sys::GST_BUFFER_OFFSET_NONE;
pub const CLOCK_TIME_NONE: ClockTime = ClockTime(None);

pub const SECOND: ClockTime = ClockTime(Some(1_000_000_000));
pub const MSECOND: ClockTime = ClockTime(Some(1_000_000));
pub const USECOND: ClockTime = ClockTime(Some(1_000));
pub const NSECOND: ClockTime = ClockTime(Some(1));

pub const SECOND_VAL: u64 = 1_000_000_000;
pub const MSECOND_VAL: u64 = 1_000_000;
pub const USECOND_VAL: u64 = 1_000;
pub const NSECOND_VAL: u64 = 1;

pub const FORMAT_PERCENT_MAX: u32 = gst_sys::GST_FORMAT_PERCENT_MAX as u32;
pub const FORMAT_PERCENT_SCALE: u32 = gst_sys::GST_FORMAT_PERCENT_SCALE as u32;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst::prelude::*" without getting conflicts
pub mod prelude {
    pub use glib::prelude::*;

    pub use auto::traits::*;

    pub use meta::MetaAPI;

    pub use allocator::AllocatorExtManual;
    pub use bin::GstBinExtManual;
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
    pub use device_monitor::DeviceMonitorExtManual;
    pub use device_provider::DeviceProviderExtManual;
    pub use gobject::GObjectExtManualGst;
    pub use message::MessageErrorDomain;
    pub use object::GstObjectExtManual;
    pub use pad::PadExtManual;
    pub use param_spec::GstParamSpecExt;
    pub use pipeline::GstPipelineExtManual;
    pub use plugin::GstPluginExtManual;
    pub use plugin_feature::PluginFeatureExtManual;
    pub use tag_setter::TagSetterExtManual;
    pub use typefind::TypeFindImpl;
    pub use value::GstValueExt;

    pub use tags::{CustomTag, Tag};

    pub use muldiv::MulDiv;

    pub use format::{FormattedValue, SpecificFormattedValue};
}

mod utils;

#[macro_use]
pub mod subclass;
