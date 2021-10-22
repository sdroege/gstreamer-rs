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
mod buffer_pool;
mod child_proxy;
mod element;
mod ghost_pad;
mod object;
mod pad;
mod pipeline;
mod proxy_pad;
mod task_pool;
mod tracer;

mod device;
mod device_provider;

mod clock;
mod system_clock;

mod preset;
mod tag_setter;
mod uri_handler;

pub use self::device_provider::DeviceProviderMetadata;
pub use self::element::ElementMetadata;

pub use self::error::FlowError;
pub use self::plugin::{MAJOR_VERSION, MINOR_VERSION};

pub mod prelude {
    #[doc(hidden)]
    pub use glib::subclass::prelude::*;

    pub use super::bin::{BinImpl, BinImplExt};
    pub use super::buffer_pool::{BufferPoolImpl, BufferPoolImplExt};
    pub use super::child_proxy::{ChildProxyImpl, ChildProxyImplExt};
    pub use super::clock::{ClockImpl, ClockImplExt};
    pub use super::device::{DeviceImpl, DeviceImplExt};
    pub use super::device_provider::{DeviceProviderImpl, DeviceProviderImplExt};
    pub use super::element::{ElementImpl, ElementImplExt};
    pub use super::ghost_pad::GhostPadImpl;
    pub use super::object::GstObjectImpl;
    pub use super::pad::{PadImpl, PadImplExt};
    pub use super::pipeline::PipelineImpl;
    pub use super::preset::PresetImpl;
    pub use super::proxy_pad::ProxyPadImpl;
    pub use super::system_clock::SystemClockImpl;
    pub use super::tag_setter::TagSetterImpl;
    pub use super::task_pool::TaskPoolImpl;
    pub use super::tracer::{TracerHook, TracerImpl, TracerImplExt};
    pub use super::uri_handler::{URIHandlerImpl, URIHandlerImplExt};
}
