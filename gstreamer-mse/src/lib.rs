// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::manual_c_str_literals)]
#![doc = include_str!("../README.md")]

pub use gstreamer_mse_sys as ffi;

macro_rules! assert_initialized_main_thread {
    () => {
        if !gst::INITIALIZED.load(std::sync::atomic::Ordering::SeqCst) {
            gst::assert_initialized();
        }
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

#[allow(unused_imports)]
mod auto;
#[allow(unused_imports)]
pub use auto::*;

mod source_buffer_interval;
pub use source_buffer_interval::SourceBufferInterval;

mod media_source;

mod media_source_range;
pub use media_source_range::MediaSourceRange;
