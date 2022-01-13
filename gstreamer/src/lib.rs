// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![recursion_limit = "256"]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::non_send_fields_in_send_ty)]
#![doc = include_str!("../README.md")]

// Re-exported for the subclass gst_plugin_define! macro
pub use ffi;
pub use glib;
pub use paste;

use glib::translate::{from_glib, from_glib_full};

macro_rules! assert_initialized_main_thread {
    () => {
        if unsafe { ffi::gst_is_initialized() } != glib::ffi::GTRUE {
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
#[allow(clippy::use_self)]
#[allow(clippy::needless_borrow)]
#[allow(unused_imports)]
mod auto;
pub use crate::auto::functions::*;
pub use crate::auto::*;

#[macro_use]
mod macros;

#[macro_use]
mod log;
pub use crate::log::*;

#[macro_use]
mod error;
pub use crate::error::*;

#[macro_use]
pub mod miniobject;
pub use miniobject::{MiniObject, MiniObjectRef};
pub mod message;
pub use crate::message::{Message, MessageErrorDomain, MessageRef, MessageView};

mod value;
pub use crate::value::*;
#[cfg(feature = "ser_de")]
#[macro_use]
mod value_serde;

pub mod structure;
pub use crate::structure::{Structure, StructureRef};
#[cfg(feature = "ser_de")]
mod structure_serde;

pub mod caps;
pub use crate::caps::{Caps, CapsFilterMapAction, CapsRef};
mod caps_features;
#[cfg(feature = "ser_de")]
mod caps_serde;
pub use crate::caps_features::{
    CapsFeatures, CapsFeaturesRef, CAPS_FEATURES_MEMORY_SYSTEM_MEMORY,
    CAPS_FEATURE_MEMORY_SYSTEM_MEMORY,
};
#[cfg(feature = "ser_de")]
mod caps_features_serde;

pub mod tags;
pub use crate::tags::{
    tag_exists, tag_get_description, tag_get_flag, tag_get_nick, tag_get_type, Tag, TagList,
    TagListRef,
};
#[cfg(feature = "ser_de")]
mod tags_serde;

pub mod meta;
#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
pub use crate::meta::MetaSeqnum;
#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
pub use crate::meta::ReferenceTimestampMeta;
pub use crate::meta::{Meta, MetaAPI, MetaRef, MetaRefMut, ParentBufferMeta, ProtectionMeta};
pub mod buffer;
pub use crate::buffer::{
    Buffer, BufferMap, BufferRef, MappedBuffer, BUFFER_COPY_ALL, BUFFER_COPY_METADATA,
};
mod buffer_cursor;
pub use crate::buffer_cursor::{BufferCursor, BufferRefCursor};
pub mod memory;
pub use crate::memory::{MappedMemory, Memory, MemoryMap, MemoryRef};
#[cfg(feature = "ser_de")]
mod buffer_serde;

pub mod sample;
pub use crate::sample::{Sample, SampleRef};
#[cfg(feature = "ser_de")]
mod sample_serde;

pub mod bufferlist;
pub use crate::bufferlist::{BufferList, BufferListRef};
#[cfg(feature = "ser_de")]
mod bufferlist_serde;

pub mod query;
pub use crate::query::{Query, QueryRef, QueryView};
pub mod event;
pub use crate::event::{Event, EventRef, EventView, GroupId, Seqnum};
pub mod context;
pub use crate::context::{Context, ContextRef};
mod static_caps;
pub use crate::static_caps::*;
mod static_pad_template;
pub use crate::static_pad_template::*;

#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
pub mod promise;
#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
pub use promise::{Promise, PromiseError};

pub mod bus;
mod element;
mod element_factory;

mod bin;

mod pipeline;

mod allocation_params;
pub use self::allocation_params::AllocationParams;

mod element_factory_type;
pub use element_factory_type::*;

mod tracer;
mod tracer_factory;

// OS dependent Bus extensions (also import the other platform mod for doc)
#[cfg(any(all(unix, feature = "v1_14"), feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
mod bus_unix;
#[cfg(any(all(windows, feature = "v1_14"), feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
mod bus_windows;

mod child_proxy;
mod clock_time;
#[cfg(feature = "ser_de")]
mod clock_time_serde;
mod date_time;
#[cfg(feature = "ser_de")]
mod date_time_serde;
mod device_monitor;
mod device_provider;
mod device_provider_factory;
mod enums;
pub use crate::enums::MessageType;
mod ghost_pad;
mod gobject;
mod iterator;
mod object;
mod pad;
pub use pad::*;
mod registry;
pub use crate::pad::PadBuilder;
mod control_binding;
mod control_source;
mod parse_context;
mod proxy_pad;
mod tag_setter;
pub use crate::element::{ElementMessageType, NotifyWatchId};
pub use crate::element::{
    ELEMENT_METADATA_AUTHOR, ELEMENT_METADATA_DESCRIPTION, ELEMENT_METADATA_DOC_URI,
    ELEMENT_METADATA_ICON_NAME, ELEMENT_METADATA_KLASS, ELEMENT_METADATA_LONGNAME,
};

pub use self::iterator::{Iterator, IteratorError, IteratorImpl, StdIterator};
pub use crate::clock_time::ClockTime;
pub use crate::device_monitor::DeviceMonitorFilterId;
pub use crate::enums::{
    ClockError, ClockSuccess, FlowError, FlowSuccess, PadLinkError, PadLinkSuccess,
    StateChangeError, StateChangeSuccess, TagError,
};
pub use crate::pad::{PadGetRangeSuccess, PadProbeData, PadProbeId, PadProbeInfo};
pub use crate::parse_context::ParseContext;
mod plugin_feature;

mod plugin;
#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
pub mod stream;
#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
pub mod stream_collection;

mod typefind;
pub use crate::typefind::*;
mod typefind_factory;

pub mod format;
pub use crate::format::{
    FormattedValue, FormattedValueIntrinsic, GenericFormattedValue, SpecificFormattedValue,
    SpecificFormattedValueIntrinsic,
};
#[cfg(feature = "ser_de")]
mod format_serde;

mod segment;
pub use crate::segment::*;
#[cfg(feature = "ser_de")]
mod segment_serde;

pub mod toc;
pub use crate::toc::{Toc, TocEntry, TocEntryRef, TocRef};
#[cfg(feature = "ser_de")]
mod toc_serde;

mod clock;
pub use crate::clock::{AtomicClockReturn, ClockId, PeriodicClockId, SingleShotClockId};

mod buffer_pool;
pub use crate::buffer_pool::*;

mod pad_template;

mod param_spec;
pub use crate::param_spec::*;

pub mod functions;
pub use crate::functions::*;

mod utils;

use std::ptr;

#[doc(alias = "gst_init_check")]
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

// rustdoc-stripper-ignore-next
/// Deinitialize GStreamer
///
/// # Safety
///
/// This must only be called once during the lifetime of the process, once no GStreamer threads
/// are running anymore and all GStreamer resources are released.
pub unsafe fn deinit() {
    ffi::gst_deinit();
}

pub const PARAM_FLAG_CONTROLLABLE: glib::ParamFlags = glib::ParamFlags::USER_1;
pub const PARAM_FLAG_MUTABLE_READY: glib::ParamFlags = glib::ParamFlags::USER_2;
pub const PARAM_FLAG_MUTABLE_PAUSED: glib::ParamFlags = glib::ParamFlags::USER_3;
pub const PARAM_FLAG_MUTABLE_PLAYING: glib::ParamFlags = glib::ParamFlags::USER_4;
#[cfg(any(feature = "v1_18", feature = "dox"))]
pub const PARAM_FLAG_DOC_SHOW_DEFAULT: glib::ParamFlags = glib::ParamFlags::USER_5;
#[cfg(any(feature = "v1_18", feature = "dox"))]
pub const PARAM_FLAG_CONDITIONALLY_AVAILABLE: glib::ParamFlags = glib::ParamFlags::USER_6;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst::prelude::*" without getting conflicts
pub mod prelude {
    #[doc(hidden)]
    pub use glib::prelude::*;
    pub use opt_ops::prelude::*;

    pub use crate::auto::traits::*;

    pub use crate::meta::MetaAPI;

    // OS dependent Bus extensions (also import the other platform trait for doc)
    #[cfg(any(all(unix, feature = "v1_14"), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    pub use crate::bus_unix::UnixBusExtManual;
    #[cfg(any(all(windows, feature = "v1_14"), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
    pub use crate::bus_windows::WindowsBusExtManual;

    pub use crate::bin::GstBinExtManual;
    pub use crate::buffer_pool::BufferPoolExtManual;
    pub use crate::child_proxy::ChildProxyExtManual;
    pub use crate::clock::ClockExtManual;
    pub use crate::device_monitor::DeviceMonitorExtManual;
    pub use crate::device_provider::DeviceProviderExtManual;
    pub use crate::element::{ElementClassExt, ElementExtManual};
    pub use crate::gobject::GObjectExtManualGst;
    pub use crate::message::MessageErrorDomain;
    pub use crate::object::GstObjectExtManual;
    pub use crate::pad::PadExtManual;
    pub use crate::pipeline::GstPipelineExtManual;
    pub use crate::plugin::GstPluginExtManual;
    pub use crate::plugin_feature::PluginFeatureExtManual;
    pub use crate::proxy_pad::ProxyPadExtManual;
    pub use crate::tag_setter::TagSetterExtManual;
    pub use crate::typefind::TypeFindImpl;
    pub use crate::value::GstValueExt;

    pub use crate::miniobject::IsMiniObject;

    pub use crate::tags::{CustomTag, Tag};

    pub use muldiv::MulDiv;

    pub use crate::format::{
        FormattedValue, FormattedValueIntrinsic, SpecificFormattedValue,
        SpecificFormattedValueIntrinsic,
    };
    pub use crate::utils::Displayable;
}

#[macro_use]
pub mod subclass;
