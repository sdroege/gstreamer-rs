// Take a look at the license at the top of the repository in the LICENSE file.

use glib::object::IsA;
use glib::translate::*;
use std::mem;
use std::ptr;

use crate::BaseSrc;

pub trait BaseSrcExtManual: 'static {
    fn get_allocator(&self) -> (Option<gst::Allocator>, gst::AllocationParams);

    fn get_segment(&self) -> gst::Segment;

    fn start_complete(&self, ret: Result<gst::FlowSuccess, gst::FlowError>);

    fn start_wait(&self) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn wait_playing(&self) -> Result<gst::FlowSuccess, gst::FlowError>;

    fn query_latency(&self) -> Result<(bool, gst::ClockTime, gst::ClockTime), glib::BoolError>;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn new_segment(&self, segment: &gst::Segment) -> Result<(), glib::BoolError>;
}

impl<O: IsA<BaseSrc>> BaseSrcExtManual for O {
    fn get_allocator(&self) -> (Option<gst::Allocator>, gst::AllocationParams) {
        unsafe {
            let mut allocator = ptr::null_mut();
            let mut params = mem::zeroed();
            ffi::gst_base_src_get_allocator(
                self.as_ref().to_glib_none().0,
                &mut allocator,
                &mut params,
            );
            (from_glib_full(allocator), params.into())
        }
    }

    fn get_segment(&self) -> gst::Segment {
        unsafe {
            let src: &ffi::GstBaseSrc = &*(self.as_ptr() as *const _);
            let _guard = crate::utils::MutexGuard::lock(&src.element.object.lock);
            from_glib_none(&src.segment as *const _)
        }
    }

    fn start_complete(&self, ret: Result<gst::FlowSuccess, gst::FlowError>) {
        let ret: gst::FlowReturn = ret.into();
        unsafe {
            ffi::gst_base_src_start_complete(self.as_ref().to_glib_none().0, ret.to_glib());
        }
    }

    fn start_wait(&self) -> Result<gst::FlowSuccess, gst::FlowError> {
        let ret: gst::FlowReturn =
            unsafe { from_glib(ffi::gst_base_src_start_wait(self.as_ref().to_glib_none().0)) };
        ret.into_result()
    }

    fn wait_playing(&self) -> Result<gst::FlowSuccess, gst::FlowError> {
        let ret: gst::FlowReturn = unsafe {
            from_glib(ffi::gst_base_src_wait_playing(
                self.as_ref().to_glib_none().0,
            ))
        };
        ret.into_result()
    }

    fn query_latency(&self) -> Result<(bool, gst::ClockTime, gst::ClockTime), glib::BoolError> {
        unsafe {
            let mut live = mem::MaybeUninit::uninit();
            let mut min_latency = mem::MaybeUninit::uninit();
            let mut max_latency = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_base_src_query_latency(
                self.as_ref().to_glib_none().0,
                live.as_mut_ptr(),
                min_latency.as_mut_ptr(),
                max_latency.as_mut_ptr(),
            ));
            let live = live.assume_init();
            let min_latency = min_latency.assume_init();
            let max_latency = max_latency.assume_init();
            if ret {
                Ok((
                    from_glib(live),
                    from_glib(min_latency),
                    from_glib(max_latency),
                ))
            } else {
                Err(glib::bool_error!("Failed to query latency"))
            }
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn new_segment(&self, segment: &gst::Segment) -> Result<(), glib::BoolError> {
        unsafe {
            let ret = from_glib(ffi::gst_base_src_new_segment(
                self.as_ref().to_glib_none().0,
                segment.to_glib_none().0,
            ));

            if ret {
                Ok(())
            } else {
                Err(glib::bool_error!("Failed to configure new segment"))
            }
        }
    }
}
