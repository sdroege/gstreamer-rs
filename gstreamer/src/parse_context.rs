// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::translate::*;

glib::glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ParseContext(Boxed<ffi::GstParseContext>);

    match fn {
        copy => |ptr| {
            glib::gobject_ffi::g_boxed_copy(ffi::gst_parse_context_get_type(), ptr as *mut _) as *mut ffi::GstParseContext
        },
        free => |ptr| {
            glib::gobject_ffi::g_boxed_free(ffi::gst_parse_context_get_type(), ptr as *mut _)
        },
        get_type => || ffi::gst_parse_context_get_type(),
    }
}

unsafe impl Send for ParseContext {}
unsafe impl Sync for ParseContext {}

impl ParseContext {
    pub fn new() -> Self {
        unsafe { from_glib_full(ffi::gst_parse_context_new()) }
    }

    pub fn get_missing_elements(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_parse_context_get_missing_elements(
                mut_override(self.to_glib_none().0),
            ))
        }
    }
}

impl Default for ParseContext {
    fn default() -> Self {
        Self::new()
    }
}
