mod gl_base_filter;
#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
mod gl_base_src;
mod gl_filter;

pub use self::gl_filter::GLFilterMode;

pub mod prelude {
    #[doc(hidden)]
    pub use gst_video::subclass::prelude::*;

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    pub use super::gl_base_src::{GLBaseSrcImpl, GLBaseSrcImplExt};
    pub use super::{
        gl_base_filter::{GLBaseFilterImpl, GLBaseFilterImplExt},
        gl_filter::{GLFilterImpl, GLFilterImplExt},
    };
}
