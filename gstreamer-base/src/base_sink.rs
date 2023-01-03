// Take a look at the license at the top of the repository in the LICENSE file.

use std::mem;

use glib::{prelude::*, translate::*};

use crate::BaseSink;

pub trait BaseSinkExtManual: 'static {
    #[doc(alias = "get_segment")]
    fn segment(&self) -> gst::Segment;

    #[doc(alias = "gst_base_sink_query_latency")]
    fn query_latency(
        &self,
    ) -> Result<(bool, bool, Option<gst::ClockTime>, Option<gst::ClockTime>), glib::BoolError>;

    fn sink_pad(&self) -> &gst::Pad;
}

impl<O: IsA<BaseSink>> BaseSinkExtManual for O {
    fn segment(&self) -> gst::Segment {
        unsafe {
            let sink: &ffi::GstBaseSink = &*(self.as_ptr() as *const _);
            let _guard = crate::utils::MutexGuard::lock(&sink.element.object.lock);
            from_glib_none(&sink.segment as *const _)
        }
    }

    fn query_latency(
        &self,
    ) -> Result<(bool, bool, Option<gst::ClockTime>, Option<gst::ClockTime>), glib::BoolError> {
        unsafe {
            let mut live = mem::MaybeUninit::uninit();
            let mut upstream_live = mem::MaybeUninit::uninit();
            let mut min_latency = mem::MaybeUninit::uninit();
            let mut max_latency = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_base_sink_query_latency(
                self.as_ref().to_glib_none().0,
                live.as_mut_ptr(),
                upstream_live.as_mut_ptr(),
                min_latency.as_mut_ptr(),
                max_latency.as_mut_ptr(),
            ));
            let live = live.assume_init();
            let upstream_live = upstream_live.assume_init();
            let min_latency = min_latency.assume_init();
            let max_latency = max_latency.assume_init();
            if ret {
                Ok((
                    from_glib(live),
                    from_glib(upstream_live),
                    from_glib(min_latency),
                    from_glib(max_latency),
                ))
            } else {
                Err(glib::bool_error!("Failed to query latency"))
            }
        }
    }

    fn sink_pad(&self) -> &gst::Pad {
        unsafe {
            let elt = &*(self.as_ptr() as *const ffi::GstBaseSink);
            &*(&elt.sinkpad as *const *mut gst::ffi::GstPad as *const gst::Pad)
        }
    }
}
