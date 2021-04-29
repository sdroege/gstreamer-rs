mod gl_base_filter;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
mod gl_base_src;
mod gl_filter;

pub use self::gl_filter::GLFilterMode;

pub mod prelude {
    #[doc(hidden)]
    pub use gst_video::subclass::prelude::*;

    pub use super::gl_base_filter::{GLBaseFilterImpl, GLBaseFilterImplExt};
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    pub use super::gl_base_src::{GLBaseSrcImpl, GLBaseSrcImplExt};
    pub use super::gl_filter::{GLFilterImpl, GLFilterImplExt};
}
