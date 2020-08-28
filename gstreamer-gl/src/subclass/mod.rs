mod gl_base_filter;
mod gl_filter;

pub use self::gl_filter::GLFilterMode;

pub mod prelude {
    pub use super::gl_base_filter::{GLBaseFilterImpl, GLBaseFilterImplExt};
    pub use super::gl_filter::{GLFilterImpl, GLFilterImplExt};
}
