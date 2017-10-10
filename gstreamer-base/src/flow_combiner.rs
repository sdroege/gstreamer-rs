// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gst;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct FlowCombiner(Shared<ffi::GstFlowCombiner>);

    match fn {
        ref => |ptr| {
            gobject_ffi::g_boxed_copy(ffi::gst_flow_combiner_get_type(), ptr as *mut _) as *mut ffi::GstFlowCombiner
        },
        unref => |ptr| {
            gobject_ffi::g_boxed_free(ffi::gst_flow_combiner_get_type(), ptr as *mut _)
        },
        get_type => || ffi::gst_flow_combiner_get_type(),
    }
}

impl FlowCombiner {
    pub fn new() -> FlowCombiner {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_flow_combiner_new())
        }
    }

    pub fn add_pad<P: IsA<gst::Pad>>(&self, pad: &P) {
        unsafe {
            ffi::gst_flow_combiner_add_pad(self.to_glib_none().0, pad.to_glib_none().0);
        }
    }

    pub fn clear(&self) {
        unsafe {
            ffi::gst_flow_combiner_clear(self.to_glib_none().0);
        }
    }

    pub fn remove_pad<P: IsA<gst::Pad>>(&self, pad: &P) {
        unsafe {
            ffi::gst_flow_combiner_remove_pad(self.to_glib_none().0, pad.to_glib_none().0);
        }
    }

    pub fn reset(&self) {
        unsafe {
            ffi::gst_flow_combiner_reset(self.to_glib_none().0);
        }
    }

    pub fn update_flow(&self, fret: gst::FlowReturn) -> gst::FlowReturn {
        unsafe {
            from_glib(ffi::gst_flow_combiner_update_flow(self.to_glib_none().0, fret.to_glib()))
        }
    }

    pub fn update_pad_flow<P: IsA<gst::Pad>>(&self, pad: &P, fret: gst::FlowReturn) -> gst::FlowReturn {
        unsafe {
            from_glib(ffi::gst_flow_combiner_update_pad_flow(self.to_glib_none().0, pad.to_glib_none().0, fret.to_glib()))
        }
    }
}

impl Default for FlowCombiner {
    fn default() -> Self {
        Self::new()
    }
}
