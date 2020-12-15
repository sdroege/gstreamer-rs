// Take a look at the license at the top of the repository in the LICENSE file.

#![allow(clippy::cast_ptr_alignment)]

#[macro_use]
mod error;

#[cfg(any(feature = "v1_14"))]
#[macro_use]
#[path = "plugin_1_14.rs"]
mod plugin;

#[cfg(not(any(feature = "v1_14")))]
#[macro_use]
#[path = "plugin_1_12.rs"]
mod plugin;

mod bin;
mod child_proxy;
mod element;
mod ghost_pad;
mod pad;
mod pipeline;

mod device;
mod device_provider;

mod clock;
mod system_clock;

mod preset;
mod tag_setter;
mod uri_handler;

pub use self::error::FlowError;
pub use self::plugin::{MAJOR_VERSION, MINOR_VERSION};

pub mod prelude {
    pub use super::bin::{BinImpl, BinImplExt};
    pub use super::child_proxy::ChildProxyImpl;
    pub use super::clock::{ClockImpl, ClockImplExt};
    pub use super::device::{DeviceImpl, DeviceImplExt};
    pub use super::device_provider::{
        DeviceProviderClassSubclassExt, DeviceProviderImpl, DeviceProviderImplExt,
    };
    pub use super::element::{ElementClassSubclassExt, ElementImpl, ElementImplExt};
    pub use super::ghost_pad::GhostPadImpl;
    pub use super::pad::{PadImpl, PadImplExt};
    pub use super::pipeline::PipelineImpl;
    pub use super::preset::PresetImpl;
    pub use super::system_clock::SystemClockImpl;
    pub use super::tag_setter::TagSetterImpl;
    pub use super::uri_handler::URIHandlerImpl;
    pub use super::PanicPoison;
    pub use glib::subclass::prelude::*;
}

use self::prelude::*;
use std::sync::atomic::AtomicBool;

#[repr(C)]
pub struct ElementInstanceStruct<T: ObjectSubclass> {
    parent: <T::ParentType as glib::object::ObjectType>::GlibType,
    panicked: AtomicBool,
}

unsafe impl<T: ObjectSubclass> InstanceStruct for ElementInstanceStruct<T> {
    type Type = T;
}

pub trait PanicPoison {
    fn panicked(&self) -> &AtomicBool;
}

impl<T: ObjectSubclass> PanicPoison for ElementInstanceStruct<T> {
    fn panicked(&self) -> &AtomicBool {
        &self.panicked
    }
}
