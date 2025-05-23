// Copyright (C) 2020 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::manual_c_str_literals)]

pub use gst_gl;
pub use gstreamer_gl_egl_sys as ffi;

macro_rules! assert_initialized_main_thread {
    () => {
        if !gst::INITIALIZED.load(std::sync::atomic::Ordering::SeqCst) {
            gst::assert_initialized();
        }
    };
}

#[allow(unused_imports)]
mod auto;
pub use auto::*;

mod gl_display_egl;

// Re-export all the traits in a prelude module, so that applications
// can always "use gst_egl::prelude::*" without getting conflicts
pub mod prelude {
    #[doc(hidden)]
    pub use gst_gl::prelude::*;

    pub use crate::auto::traits::*;
}
