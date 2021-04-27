// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct FlowCombiner(Shared<ffi::GstFlowCombiner>);

    match fn {
        ref => |ptr| {
            // cfg_if emits code in blocks (curly braces) and `ref` handling inserts a
            // trailing semicolon to void an optionally returned value. The linter
            // requests the resulting { ..ref() }; to be simplified making it unsuitable.
            #[cfg(feature = "v1_12_1")]
            ffi::gst_flow_combiner_ref(ptr);
            #[cfg(not(feature = "v1_12_1"))]
            glib::gobject_ffi::g_boxed_copy(ffi::gst_flow_combiner_get_type(), ptr as *mut _);
        },
        unref => |ptr| {
            cfg_if::cfg_if! {
                if #[cfg(feature = "v1_12_1")] {
                    ffi::gst_flow_combiner_unref(ptr)
                } else {
                    glib::gobject_ffi::g_boxed_free(ffi::gst_flow_combiner_get_type(), ptr as *mut _)
                }
            }
        },
        type_ => || ffi::gst_flow_combiner_get_type(),
    }
}

impl FlowCombiner {
    pub fn new() -> FlowCombiner {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_flow_combiner_new()) }
    }

    pub fn add_pad<P: IsA<gst::Pad>>(&self, pad: &P) {
        unsafe {
            ffi::gst_flow_combiner_add_pad(self.to_glib_none().0, pad.as_ref().to_glib_none().0);
        }
    }

    pub fn clear(&self) {
        unsafe {
            ffi::gst_flow_combiner_clear(self.to_glib_none().0);
        }
    }

    pub fn remove_pad<P: IsA<gst::Pad>>(&self, pad: &P) {
        unsafe {
            ffi::gst_flow_combiner_remove_pad(self.to_glib_none().0, pad.as_ref().to_glib_none().0);
        }
    }

    pub fn reset(&self) {
        unsafe {
            ffi::gst_flow_combiner_reset(self.to_glib_none().0);
        }
    }

    pub fn update_flow<FRet: Into<gst::FlowReturn>>(
        &self,
        fret: FRet,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        let fret: gst::FlowReturn = fret.into();
        let ret: gst::FlowReturn = unsafe {
            from_glib(ffi::gst_flow_combiner_update_flow(
                self.to_glib_none().0,
                fret.into_glib(),
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
            from_glib(ffi::gst_flow_combiner_update_pad_flow(
                self.to_glib_none().0,
                pad.as_ref().to_glib_none().0,
                fret.into_glib(),
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
