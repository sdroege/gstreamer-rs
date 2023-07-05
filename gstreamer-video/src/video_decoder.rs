// Take a look at the license at the top of the repository in the LICENSE file.

use std::{mem, ptr};

use glib::{prelude::*, translate::*};

#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
use crate::VideoInterlaceMode;
use crate::{
    utils::HasStreamLock,
    video_codec_state::{InNegotiation, Readable, VideoCodecState, VideoCodecStateContext},
    VideoCodecFrame, VideoDecoder, VideoFormat,
};

extern "C" {
    fn _gst_video_decoder_error(
        dec: *mut ffi::GstVideoDecoder,
        weight: i32,
        domain: glib::ffi::GQuark,
        code: i32,
        txt: *mut libc::c_char,
        debug: *mut libc::c_char,
        file: *const libc::c_char,
        function: *const libc::c_char,
        line: i32,
    ) -> gst::ffi::GstFlowReturn;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::VideoDecoder>> Sealed for T {}
}

pub trait VideoDecoderExtManual: sealed::Sealed + IsA<VideoDecoder> + 'static {
    #[doc(alias = "gst_video_decoder_allocate_output_frame")]
    fn allocate_output_frame(
        &self,
        frame: &mut VideoCodecFrame,
        params: Option<&gst::BufferPoolAcquireParams>,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            let params_ptr = params.to_glib_none().0 as *mut _;
            try_from_glib(ffi::gst_video_decoder_allocate_output_frame_with_params(
                self.as_ref().to_glib_none().0,
                frame.to_glib_none().0,
                params_ptr,
            ))
        }
    }

    #[doc(alias = "get_frame")]
    #[doc(alias = "gst_video_decoder_get_frame")]
    fn frame(&self, frame_number: i32) -> Option<VideoCodecFrame> {
        let frame = unsafe {
            ffi::gst_video_decoder_get_frame(self.as_ref().to_glib_none().0, frame_number)
        };

        if frame.is_null() {
            None
        } else {
            unsafe { Some(VideoCodecFrame::new(frame, self.as_ref())) }
        }
    }

    #[doc(alias = "get_frames")]
    #[doc(alias = "gst_video_decoder_get_frames")]
    fn frames(&self) -> Vec<VideoCodecFrame> {
        unsafe {
            let frames = ffi::gst_video_decoder_get_frames(self.as_ref().to_glib_none().0);
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

    #[doc(alias = "get_oldest_frame")]
    #[doc(alias = "gst_video_decoder_get_oldest_frame")]
    fn oldest_frame(&self) -> Option<VideoCodecFrame> {
        let frame =
            unsafe { ffi::gst_video_decoder_get_oldest_frame(self.as_ref().to_glib_none().0) };

        if frame.is_null() {
            None
        } else {
            unsafe { Some(VideoCodecFrame::new(frame, self.as_ref())) }
        }
    }

    #[doc(alias = "get_allocator")]
    #[doc(alias = "gst_video_decoder_get_allocator")]
    fn allocator(&self) -> (Option<gst::Allocator>, gst::AllocationParams) {
        unsafe {
            let mut allocator = ptr::null_mut();
            let mut params = mem::MaybeUninit::uninit();
            ffi::gst_video_decoder_get_allocator(
                self.as_ref().to_glib_none().0,
                &mut allocator,
                params.as_mut_ptr(),
            );
            (from_glib_full(allocator), params.assume_init().into())
        }
    }
    #[doc(alias = "get_latency")]
    #[doc(alias = "gst_video_decoder_get_latency")]
    fn latency(&self) -> (gst::ClockTime, Option<gst::ClockTime>) {
        let mut min_latency = gst::ffi::GST_CLOCK_TIME_NONE;
        let mut max_latency = gst::ffi::GST_CLOCK_TIME_NONE;

        unsafe {
            ffi::gst_video_decoder_get_latency(
                self.as_ref().to_glib_none().0,
                &mut min_latency,
                &mut max_latency,
            );

            (
                try_from_glib(min_latency).expect("undefined min_latency"),
                from_glib(max_latency),
            )
        }
    }

    #[doc(alias = "gst_video_decoder_set_latency")]
    fn set_latency(
        &self,
        min_latency: gst::ClockTime,
        max_latency: impl Into<Option<gst::ClockTime>>,
    ) {
        unsafe {
            ffi::gst_video_decoder_set_latency(
                self.as_ref().to_glib_none().0,
                min_latency.into_glib(),
                max_latency.into().into_glib(),
            );
        }
    }

    #[doc(alias = "get_output_state")]
    #[doc(alias = "gst_video_decoder_get_output_state")]
    fn output_state(&self) -> Option<VideoCodecState<'static, Readable>> {
        let state =
            unsafe { ffi::gst_video_decoder_get_output_state(self.as_ref().to_glib_none().0) };

        if state.is_null() {
            None
        } else {
            unsafe { Some(VideoCodecState::<Readable>::new(state)) }
        }
    }

    #[doc(alias = "gst_video_decoder_set_output_state")]
    fn set_output_state(
        &self,
        fmt: VideoFormat,
        width: u32,
        height: u32,
        reference: Option<&VideoCodecState<Readable>>,
    ) -> Result<VideoCodecState<InNegotiation>, gst::FlowError> {
        let state = unsafe {
            let reference = match reference {
                Some(reference) => reference.as_mut_ptr(),
                None => ptr::null_mut(),
            };
            ffi::gst_video_decoder_set_output_state(
                self.as_ref().to_glib_none().0,
                fmt.into_glib(),
                width,
                height,
                reference,
            )
        };

        if state.is_null() {
            Err(gst::FlowError::NotNegotiated)
        } else {
            unsafe { Ok(VideoCodecState::<InNegotiation>::new(state, self.as_ref())) }
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_video_decoder_set_interlaced_output_state")]
    fn set_interlaced_output_state(
        &self,
        fmt: VideoFormat,
        mode: VideoInterlaceMode,
        width: u32,
        height: u32,
        reference: Option<&VideoCodecState<Readable>>,
    ) -> Result<VideoCodecState<InNegotiation>, gst::FlowError> {
        let state = unsafe {
            let reference = match reference {
                Some(reference) => reference.as_mut_ptr(),
                None => ptr::null_mut(),
            };
            ffi::gst_video_decoder_set_interlaced_output_state(
                self.as_ref().to_glib_none().0,
                fmt.into_glib(),
                mode.into_glib(),
                width,
                height,
                reference,
            )
        };

        if state.is_null() {
            Err(gst::FlowError::NotNegotiated)
        } else {
            unsafe { Ok(VideoCodecState::<InNegotiation>::new(state, self.as_ref())) }
        }
    }

    #[doc(alias = "gst_video_decoder_negotiate")]
    fn negotiate<'a>(
        &'a self,
        output_state: VideoCodecState<'a, InNegotiation<'a>>,
    ) -> Result<(), gst::FlowError> {
        // Consume output_state so user won't be able to modify it anymore
        let self_ptr = self.to_glib_none().0 as *const gst::ffi::GstElement;
        assert_eq!(output_state.context.element_as_ptr(), self_ptr);

        let ret = unsafe {
            from_glib(ffi::gst_video_decoder_negotiate(
                self.as_ref().to_glib_none().0,
            ))
        };
        if ret {
            Ok(())
        } else {
            Err(gst::FlowError::NotNegotiated)
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn error<T: gst::MessageErrorDomain>(
        &self,
        weight: i32,
        code: T,
        message: Option<&str>,
        debug: Option<&str>,
        file: &str,
        function: &str,
        line: u32,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        unsafe {
            try_from_glib(_gst_video_decoder_error(
                self.as_ref().to_glib_none().0,
                weight,
                T::domain().into_glib(),
                code.code(),
                message.to_glib_full(),
                debug.to_glib_full(),
                file.to_glib_none().0,
                function.to_glib_none().0,
                line as i32,
            ))
        }
    }

    fn sink_pad(&self) -> &gst::Pad {
        unsafe {
            let elt = &*(self.as_ptr() as *const ffi::GstVideoDecoder);
            &*(&elt.sinkpad as *const *mut gst::ffi::GstPad as *const gst::Pad)
        }
    }

    fn src_pad(&self) -> &gst::Pad {
        unsafe {
            let elt = &*(self.as_ptr() as *const ffi::GstVideoDecoder);
            &*(&elt.srcpad as *const *mut gst::ffi::GstPad as *const gst::Pad)
        }
    }
}

impl<O: IsA<VideoDecoder>> VideoDecoderExtManual for O {}

impl HasStreamLock for VideoDecoder {
    fn stream_lock(&self) -> *mut glib::ffi::GRecMutex {
        let decoder_sys: *const ffi::GstVideoDecoder = self.to_glib_none().0;
        unsafe { &(*decoder_sys).stream_lock as *const _ as usize as *mut _ }
    }

    fn element_as_ptr(&self) -> *const gst::ffi::GstElement {
        let decoder_sys: *const ffi::GstVideoDecoder = self.to_glib_none().0;
        decoder_sys as *const gst::ffi::GstElement
    }
}

#[macro_export]
macro_rules! video_decoder_error(
    ($obj:expr, $weight:expr, $err:expr, ($($msg:tt)*), [$($debug:tt)*]) => { {
        use $crate::prelude::VideoDecoderExtManual;
        $obj.error(
            $weight,
            $err,
            Some(&format!($($msg)*)),
            Some(&format!($($debug)*)),
            file!(),
            $crate::glib::function_name!(),
            line!(),
        )
    }};
    ($obj:expr, $weight:expr, $err:expr, ($($msg:tt)*)) => { {
        use $crate::prelude::VideoDecoderExtManual;
        $obj.error(
            $weight,
            $err,
            Some(&format!($($msg)*)),
            None,
            file!(),
            $crate::glib::function_name!(),
            line!(),
        )
    }};
    ($obj:expr, $weight:expr, $err:expr, [$($debug:tt)*]) => { {
        use $crate::prelude::VideoDecoderExtManual;
        $obj.error(
            $weight,
            $err,
            None,
            Some(&format!($($debug)*)),
            file!(),
            $crate::glib::function_name!(),
            line!(),
        )
    }};
);
