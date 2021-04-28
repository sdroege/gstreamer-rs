// Take a look at the license at the top of the repository in the LICENSE file.

use crate::utils::HasStreamLock;
use crate::video_codec_state::{InNegotiation, Readable, VideoCodecState, VideoCodecStateContext};
use crate::VideoCodecFrame;
use crate::VideoEncoder;
use glib::prelude::*;
use glib::translate::*;
use std::mem;
use std::ptr;

pub trait VideoEncoderExtManual: 'static {
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    fn allocate_output_frame(
        &self,
        frame: &mut VideoCodecFrame,
        size: usize,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    #[doc(alias = "get_frame")]
    fn frame(&self, frame_number: i32) -> Option<VideoCodecFrame>;
    #[doc(alias = "get_frames")]
    fn frames(&self) -> Vec<VideoCodecFrame>;
    #[doc(alias = "get_oldest_frame")]
    fn oldest_frame(&self) -> Option<VideoCodecFrame>;

    #[doc(alias = "get_allocator")]
    fn allocator(&self) -> (Option<gst::Allocator>, gst::AllocationParams);

    fn finish_frame(
        &self,
        frame: Option<VideoCodecFrame>,
    ) -> Result<gst::FlowSuccess, gst::FlowError>;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn finish_subframe(&self, frame: &VideoCodecFrame) -> Result<gst::FlowSuccess, gst::FlowError>;

    #[doc(alias = "get_latency")]
    fn latency(&self) -> (gst::ClockTime, gst::ClockTime);
    fn set_latency(&self, min_latency: gst::ClockTime, max_latency: gst::ClockTime);

    #[doc(alias = "get_output_state")]
    fn output_state(&self) -> Option<VideoCodecState<'static, Readable>>;
    fn set_output_state(
        &self,
        caps: gst::Caps,
        reference: Option<&VideoCodecState<Readable>>,
    ) -> Result<VideoCodecState<InNegotiation>, gst::FlowError>;

    fn negotiate<'a>(
        &'a self,
        output_state: VideoCodecState<'a, InNegotiation<'a>>,
    ) -> Result<(), gst::FlowError>;
}

impl<O: IsA<VideoEncoder>> VideoEncoderExtManual for O {
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    fn allocate_output_frame(
        &self,
        frame: &mut VideoCodecFrame,
        size: usize,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            gst::FlowSuccess::try_from_glib(ffi::gst_video_encoder_allocate_output_frame(
                self.as_ref().to_glib_none().0,
                frame.to_glib_none().0,
                size,
            ))
        }
    }

    fn allocator(&self) -> (Option<gst::Allocator>, gst::AllocationParams) {
        unsafe {
            let mut allocator = ptr::null_mut();
            let mut params = mem::zeroed();
            ffi::gst_video_encoder_get_allocator(
                self.as_ref().to_glib_none().0,
                &mut allocator,
                &mut params,
            );
            (from_glib_full(allocator), params.into())
        }
    }

    fn finish_frame(
        &self,
        frame: Option<VideoCodecFrame>,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            gst::FlowSuccess::try_from_glib(ffi::gst_video_encoder_finish_frame(
                self.as_ref().to_glib_none().0,
                frame.map(|f| f.into_ptr()).unwrap_or(ptr::null_mut()),
            ))
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    fn finish_subframe(&self, frame: &VideoCodecFrame) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            gst::FlowSuccess::try_from_glib(ffi::gst_video_encoder_finish_subframe(
                self.as_ref().to_glib_none().0,
                frame.to_glib_none().0,
            ))
        }
    }

    fn latency(&self) -> (gst::ClockTime, gst::ClockTime) {
        let mut min_latency = gst::ffi::GST_CLOCK_TIME_NONE;
        let mut max_latency = gst::ffi::GST_CLOCK_TIME_NONE;

        unsafe {
            ffi::gst_video_encoder_get_latency(
                self.as_ref().to_glib_none().0,
                &mut min_latency,
                &mut max_latency,
            );

            (from_glib(min_latency), from_glib(max_latency))
        }
    }

    fn set_latency(&self, min_latency: gst::ClockTime, max_latency: gst::ClockTime) {
        unsafe {
            ffi::gst_video_encoder_set_latency(
                self.as_ref().to_glib_none().0,
                min_latency.into_glib(),
                max_latency.into_glib(),
            );
        }
    }

    fn frame(&self, frame_number: i32) -> Option<VideoCodecFrame> {
        let frame = unsafe {
            ffi::gst_video_encoder_get_frame(self.as_ref().to_glib_none().0, frame_number)
        };

        if frame.is_null() {
            None
        } else {
            unsafe { Some(VideoCodecFrame::new(frame, self.as_ref())) }
        }
    }

    fn frames(&self) -> Vec<VideoCodecFrame> {
        unsafe {
            let frames = ffi::gst_video_encoder_get_frames(self.as_ref().to_glib_none().0);
            let mut iter: *const glib::ffi::GList = frames;
            let mut vec = Vec::new();

            while !iter.is_null() {
                let frame_ptr = Ptr::from((*iter).data);
                /* transfer ownership of the frame */
                let frame = VideoCodecFrame::new(frame_ptr, self.as_ref());
                vec.push(frame);
                iter = (*iter).next;
            }

            glib::ffi::g_list_free(frames);
            vec
        }
    }

    fn oldest_frame(&self) -> Option<VideoCodecFrame> {
        let frame =
            unsafe { ffi::gst_video_encoder_get_oldest_frame(self.as_ref().to_glib_none().0) };

        if frame.is_null() {
            None
        } else {
            unsafe { Some(VideoCodecFrame::new(frame, self.as_ref())) }
        }
    }

    fn output_state(&self) -> Option<VideoCodecState<'static, Readable>> {
        let state =
            unsafe { ffi::gst_video_encoder_get_output_state(self.as_ref().to_glib_none().0) };

        if state.is_null() {
            None
        } else {
            unsafe { Some(VideoCodecState::<Readable>::new(state)) }
        }
    }

    fn set_output_state(
        &self,
        caps: gst::Caps,
        reference: Option<&VideoCodecState<Readable>>,
    ) -> Result<VideoCodecState<InNegotiation>, gst::FlowError> {
        let state = unsafe {
            let reference = match reference {
                Some(reference) => reference.as_mut_ptr(),
                None => ptr::null_mut(),
            };
            ffi::gst_video_encoder_set_output_state(
                self.as_ref().to_glib_none().0,
                caps.into_ptr(),
                reference,
            )
        };

        if state.is_null() {
            Err(gst::FlowError::NotNegotiated)
        } else {
            unsafe { Ok(VideoCodecState::<InNegotiation>::new(state, self.as_ref())) }
        }
    }

    fn negotiate<'a>(
        &'a self,
        output_state: VideoCodecState<'a, InNegotiation<'a>>,
    ) -> Result<(), gst::FlowError> {
        // Consume output_state so user won't be able to modify it anymore
        let self_ptr = self.to_glib_none().0 as *const gst::ffi::GstElement;
        assert_eq!(output_state.context.element_as_ptr(), self_ptr);

        let ret = unsafe {
            from_glib(ffi::gst_video_encoder_negotiate(
                self.as_ref().to_glib_none().0,
            ))
        };
        if ret {
            Ok(())
        } else {
            Err(gst::FlowError::NotNegotiated)
        }
    }
}

impl HasStreamLock for VideoEncoder {
    fn stream_lock(&self) -> *mut glib::ffi::GRecMutex {
        let encoder_sys: *const ffi::GstVideoEncoder = self.to_glib_none().0;
        unsafe { &(*encoder_sys).stream_lock as *const _ as usize as *mut _ }
    }

    fn element_as_ptr(&self) -> *const gst::ffi::GstElement {
        let encoder_sys: *const ffi::GstVideoEncoder = self.to_glib_none().0;
        encoder_sys as *const gst::ffi::GstElement
    }
}
