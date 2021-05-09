// Take a look at the license at the top of the repository in the LICENSE file.

#![allow(clippy::cast_ptr_alignment)]

mod video_decoder;
mod video_encoder;
mod video_filter;
mod video_sink;

pub mod prelude {
    #[doc(hidden)]
    pub use gst_base::subclass::prelude::*;

    pub use super::video_decoder::{VideoDecoderImpl, VideoDecoderImplExt};
    pub use super::video_encoder::{VideoEncoderImpl, VideoEncoderImplExt};
    pub use super::video_filter::{VideoFilterImpl, VideoFilterImplExt};
    pub use super::video_sink::{VideoSinkImpl, VideoSinkImplExt};
}
