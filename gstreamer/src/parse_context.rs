// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::translate::*;
use gobject_sys;
use gst_sys;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ParseContext(Boxed<gst_sys::GstParseContext>);

    match fn {
        copy => |ptr| {
            gobject_sys::g_boxed_copy(gst_sys::gst_parse_context_get_type(), ptr as *mut _) as *mut gst_sys::GstParseContext
        },
        free => |ptr| {
            gobject_sys::g_boxed_free(gst_sys::gst_parse_context_get_type(), ptr as *mut _)
        },
        get_type => || gst_sys::gst_parse_context_get_type(),
    }
}

unsafe impl Send for ParseContext {}
unsafe impl Sync for ParseContext {}

impl ParseContext {
    pub fn new() -> Self {
        unsafe { from_glib_full(gst_sys::gst_parse_context_new()) }
    }

    pub fn get_missing_elements(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(gst_sys::gst_parse_context_get_missing_elements(
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
