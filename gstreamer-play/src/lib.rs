// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::non_send_fields_in_send_ty)]
#![doc = include_str!("../README.md")]

pub use ffi;
pub use gst;
pub use gst_video;

macro_rules! skip_assert_initialized {
    () => {};
}

macro_rules! assert_initialized_main_thread {
    () => {
        if !gst::INITIALIZED.load(std::sync::atomic::Ordering::SeqCst) {
            gst::assert_initialized();
        }
    };
}

#[allow(clippy::needless_borrow)]
#[allow(unused_imports)]
mod auto;
pub(crate) use crate::auto::PlayMessage as PlayMessageType;
pub use crate::auto::*;

pub mod subclass;

mod config;
mod play;
pub use crate::config::*;

mod play_video_info;

mod play_signal_adapter;
mod play_video_overlay_video_renderer;
mod play_visualization;

mod play_message;
pub use crate::play_message::PlayMessage;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst_player::prelude::*" without getting conflicts
pub mod prelude {
    #[doc(hidden)]
    pub use gst_video::prelude::*;

    pub use crate::auto::traits::*;
}
