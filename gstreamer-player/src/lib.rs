// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![doc = include_str!("../README.md")]

pub use ffi;
pub use gst;
pub use gst_video;

macro_rules! skip_assert_initialized {
    () => {};
}

macro_rules! assert_initialized_main_thread {
    () => {};
}

#[allow(clippy::unreadable_literal)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::match_same_arms)]
#[allow(clippy::type_complexity)]
#[allow(clippy::cast_ptr_alignment)]
#[allow(clippy::use_self)]
#[allow(clippy::needless_borrow)]
mod auto;
pub use crate::auto::*;

pub mod subclass;

mod config;
mod player;
pub use crate::config::*;

mod player_video_info;

mod player_g_main_context_signal_dispatcher;
mod player_video_overlay_video_renderer;
mod player_visualization;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst_player::prelude::*" without getting conflicts
pub mod prelude {
    #[doc(hidden)]
    pub use gst_video::prelude::*;

    pub use crate::auto::traits::*;
}
