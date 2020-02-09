// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::object::IsA;
use glib::translate::*;
use gobject_sys;
use gst;
use gst_base_sys;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct FlowCombiner(Shared<gst_base_sys::GstFlowCombiner>);

    match fn {
        ref => |ptr| gobject_sys::g_boxed_copy(gst_base_sys::gst_flow_combiner_get_type(), ptr as *mut _),
        unref => |ptr| gobject_sys::g_boxed_free(gst_base_sys::gst_flow_combiner_get_type(), ptr as *mut _),
        get_type => || gst_base_sys::gst_flow_combiner_get_type(),
    }
}

impl FlowCombiner {
    pub fn new() -> FlowCombiner {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gst_base_sys::gst_flow_combiner_new()) }
    }

    pub fn add_pad<P: IsA<gst::Pad>>(&self, pad: &P) {
        unsafe {
            gst_base_sys::gst_flow_combiner_add_pad(
                self.to_glib_none().0,
                pad.as_ref().to_glib_none().0,
            );
        }
    }

    pub fn clear(&self) {
        unsafe {
            gst_base_sys::gst_flow_combiner_clear(self.to_glib_none().0);
        }
    }

    pub fn remove_pad<P: IsA<gst::Pad>>(&self, pad: &P) {
        unsafe {
            gst_base_sys::gst_flow_combiner_remove_pad(
                self.to_glib_none().0,
                pad.as_ref().to_glib_none().0,
            );
        }
    }

    pub fn reset(&self) {
        unsafe {
            gst_base_sys::gst_flow_combiner_reset(self.to_glib_none().0);
        }
    }

    pub fn update_flow<FRet: Into<gst::FlowReturn>>(
        &self,
        fret: FRet,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        let fret: gst::FlowReturn = fret.into();
        let ret: gst::FlowReturn = unsafe {
            from_glib(gst_base_sys::gst_flow_combiner_update_flow(
                self.to_glib_none().0,
                fret.to_glib(),
            ))
        };
        ret.into_result()
    }

    pub fn update_pad_flow<P: IsA<gst::Pad>, FRet: Into<gst::FlowReturn>>(
        &self,
        pad: &P,
        fret: FRet,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        let fret: gst::FlowReturn = fret.into();
        let ret: gst::FlowReturn = unsafe {
            from_glib(gst_base_sys::gst_flow_combiner_update_pad_flow(
                self.to_glib_none().0,
                pad.as_ref().to_glib_none().0,
                fret.to_glib(),
            ))
        };
        ret.into_result()
    }
}

impl Default for FlowCombiner {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct UniqueFlowCombiner(FlowCombiner);

unsafe impl Sync for UniqueFlowCombiner {}
unsafe impl Send for UniqueFlowCombiner {}

impl UniqueFlowCombiner {
    pub fn new() -> UniqueFlowCombiner {
        UniqueFlowCombiner(FlowCombiner::new())
    }

    pub fn add_pad<P: IsA<gst::Pad>>(&mut self, pad: &P) {
        self.0.add_pad(pad);
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn remove_pad<P: IsA<gst::Pad>>(&mut self, pad: &P) {
        self.0.remove_pad(pad);
    }

    pub fn reset(&mut self) {
        self.0.reset();
    }

    pub fn update_flow(
        &mut self,
        fret: Result<gst::FlowSuccess, gst::FlowError>,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.0.update_flow(fret)
    }

    pub fn update_pad_flow<P: IsA<gst::Pad>>(
        &mut self,
        pad: &P,
        fret: Result<gst::FlowSuccess, gst::FlowError>,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        self.0.update_pad_flow(pad, fret)
    }
}

impl Default for UniqueFlowCombiner {
    fn default() -> Self {
        Self::new()
    }
}
