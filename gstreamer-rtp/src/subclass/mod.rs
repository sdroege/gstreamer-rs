// Take a look at the license at the top of the repository in the LICENSE file.

#![allow(clippy::cast_ptr_alignment)]

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
mod rtp_base_payload;

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
mod rtp_header_extension;

pub mod prelude {
    #[doc(hidden)]
    pub use gst::subclass::prelude::*;

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    pub use super::rtp_base_payload::{RTPBasePayloadImpl, RTPBasePayloadImplExt};

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    pub use super::rtp_header_extension::{RTPHeaderExtensionImpl, RTPHeaderExtensionImplExt};
}
