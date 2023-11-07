// Take a look at the license at the top of the repository in the LICENSE file.

#![allow(clippy::cast_ptr_alignment)]

#[macro_use]
mod error;

#[macro_use]
mod plugin;

mod allocator;
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

pub use self::{
    device_provider::DeviceProviderMetadata,
    element::{CatchPanic, ElementMetadata},
    error::{post_panic_error_message, FlowError},
    plugin::{MAJOR_VERSION, MINOR_VERSION},
    task_pool::TaskPoolFunction,
};

pub mod prelude {
    #[doc(hidden)]
    pub use glib::subclass::prelude::*;

    pub use super::{
        allocator::{AllocatorImpl, AllocatorImplExt},
        bin::{BinImpl, BinImplExt},
        buffer_pool::{BufferPoolImpl, BufferPoolImplExt},
        child_proxy::{ChildProxyImpl, ChildProxyImplExt},
        clock::{ClockImpl, ClockImplExt},
        device::{DeviceImpl, DeviceImplExt},
        device_provider::{DeviceProviderImpl, DeviceProviderImplExt},
        element::{ElementImpl, ElementImplExt},
        ghost_pad::GhostPadImpl,
        object::GstObjectImpl,
        pad::{PadImpl, PadImplExt},
        pipeline::PipelineImpl,
        preset::PresetImpl,
        proxy_pad::ProxyPadImpl,
        system_clock::SystemClockImpl,
        tag_setter::TagSetterImpl,
        task_pool::TaskPoolImpl,
        tracer::{TracerHook, TracerImpl, TracerImplExt},
        uri_handler::{URIHandlerImpl, URIHandlerImplExt},
    };
}
