// Copyright (C) 2016-2018 Sebastian Dröge <sebastian@centricular.com>
//               2016 Luis de Bethencourt <luisbg@osg.samsung.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
#![cfg_attr(feature = "cargo-clippy", allow(cast_ptr_alignment))]

#[macro_use]
pub mod error;
#[macro_use]
pub mod plugin;

pub mod bin;
pub mod child_proxy;
pub mod element;
pub mod ghost_pad;
pub mod pad;
pub mod pipeline;
pub mod uri_handler;

pub mod prelude {
    pub use super::bin::BinImpl;
    pub use super::child_proxy::ChildProxyImpl;
    pub use super::element::{ElementClassSubclassExt, ElementImpl, ElementImplExt};
    pub use super::ghost_pad::GhostPadImpl;
    pub use super::pad::PadImpl;
    pub use super::pipeline::PipelineImpl;
    pub use super::uri_handler::URIHandlerImpl;
    pub use super::PanicPoison;
    pub use glib::subclass::prelude::*;
}

use self::prelude::*;
use glib;
use std::sync::atomic::AtomicBool;

#[repr(C)]
pub struct ElementInstanceStruct<T: ObjectSubclass> {
    parent: <T::ParentType as glib::wrapper::Wrapper>::GlibType,
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
