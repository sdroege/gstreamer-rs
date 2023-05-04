// Take a look at the license at the top of the repository in the LICENSE file.

use std::{fmt, marker::PhantomData, mem};

use glib::translate::*;

use crate::{utils::HasStreamLock, VideoCodecFrameFlags};

pub struct VideoCodecFrame<'a> {
    frame: *mut ffi::GstVideoCodecFrame,
    /* GstVideoCodecFrame API isn't safe so protect the frame using the
     * element (decoder or encoder) stream lock */
    element: &'a dyn HasStreamLock,
}

#[doc(hidden)]
impl<'a> ::glib::translate::ToGlibPtr<'a, *mut ffi::GstVideoCodecFrame> for VideoCodecFrame<'a> {
    type Storage = PhantomData<&'a Self>;

    #[inline]
    fn to_glib_none(&'a self) -> ::glib::translate::Stash<'a, *mut ffi::GstVideoCodecFrame, Self> {
        Stash(self.frame, PhantomData)
    }

    #[inline]
    fn to_glib_full(&self) -> *mut ffi::GstVideoCodecFrame {
        unsafe { ffi::gst_video_codec_frame_ref(self.frame) }
    }
}

impl<'a> fmt::Debug for VideoCodecFrame<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut b = f.debug_struct("VideoCodecFrame");

        b.field("flags", &self.flags())
            .field("system_frame_number", &self.system_frame_number())
            .field("decode_frame_number", &self.decode_frame_number())
            .field(
                "presentation_frame_number",
                &self.presentation_frame_number(),
            )
            .field("dts", &self.dts())
            .field("pts", &self.pts())
            .field("duration", &self.duration())
            .field("distance_from_sync", &self.distance_from_sync())
            .field("input_buffer", &self.input_buffer())
            .field("output_buffer", &self.output_buffer())
            .field("deadline", &self.deadline());

        b.finish()
    }
}

impl<'a> VideoCodecFrame<'a> {
    // Take ownership of @frame
    pub(crate) unsafe fn new<T: HasStreamLock>(
        frame: *mut ffi::GstVideoCodecFrame,
        element: &'a T,
    ) -> Self {
        skip_assert_initialized!();
        let stream_lock = element.stream_lock();
        glib::ffi::g_rec_mutex_lock(stream_lock);
        Self { frame, element }
    }

    #[doc(alias = "get_flags")]
    #[inline]
    pub fn flags(&self) -> VideoCodecFrameFlags {
        let flags = unsafe { (*self.to_glib_none().0).flags };
        VideoCodecFrameFlags::from_bits_truncate(flags)
    }

    #[inline]
    pub fn set_flags(&mut self, flags: VideoCodecFrameFlags) {
        unsafe { (*self.to_glib_none().0).flags |= flags.bits() }
    }

    #[inline]
    pub fn unset_flags(&mut self, flags: VideoCodecFrameFlags) {
        unsafe { (*self.to_glib_none().0).flags &= !flags.bits() }
    }

    #[doc(alias = "get_system_frame_number")]
    #[inline]
    pub fn system_frame_number(&self) -> u32 {
        unsafe { (*self.to_glib_none().0).system_frame_number }
    }

    #[doc(alias = "get_decode_frame_number")]
    #[inline]
    pub fn decode_frame_number(&self) -> u32 {
        unsafe { (*self.to_glib_none().0).decode_frame_number }
    }

    #[doc(alias = "get_presentation_frame_number")]
    #[inline]
    pub fn presentation_frame_number(&self) -> u32 {
        unsafe { (*self.to_glib_none().0).presentation_frame_number }
    }

    #[doc(alias = "get_dts")]
    #[inline]
    pub fn dts(&self) -> Option<gst::ClockTime> {
        unsafe { from_glib((*self.to_glib_none().0).dts) }
    }

    #[inline]
    pub fn set_dts(&mut self, dts: impl Into<Option<gst::ClockTime>>) {
        unsafe {
            (*self.to_glib_none().0).dts = dts.into().into_glib();
        }
    }

    #[doc(alias = "get_pts")]
    #[inline]
    pub fn pts(&self) -> Option<gst::ClockTime> {
        unsafe { from_glib((*self.to_glib_none().0).pts) }
    }

    #[inline]
    pub fn set_pts(&mut self, pts: impl Into<Option<gst::ClockTime>>) {
        unsafe {
            (*self.to_glib_none().0).pts = pts.into().into_glib();
        }
    }

    #[doc(alias = "get_duration")]
    #[inline]
    pub fn duration(&self) -> Option<gst::ClockTime> {
        unsafe { from_glib((*self.to_glib_none().0).duration) }
    }

    #[inline]
    pub fn set_duration(&mut self, duration: impl Into<Option<gst::ClockTime>>) {
        unsafe {
            (*self.to_glib_none().0).duration = duration.into().into_glib();
        }
    }

    #[doc(alias = "get_distance_from_sync")]
    #[inline]
    pub fn distance_from_sync(&self) -> i32 {
        unsafe { (*self.to_glib_none().0).distance_from_sync }
    }

    #[doc(alias = "get_input_buffer")]
    #[inline]
    pub fn input_buffer(&self) -> Option<&gst::BufferRef> {
        unsafe {
            let ptr = (*self.to_glib_none().0).input_buffer;
            if ptr.is_null() {
                None
            } else {
                Some(gst::BufferRef::from_ptr(ptr))
            }
        }
    }

    #[doc(alias = "get_input_buffer")]
    #[inline]
    pub fn input_buffer_owned(&self) -> Option<gst::Buffer> {
        unsafe {
            let ptr = (*self.to_glib_none().0).input_buffer;
            if ptr.is_null() {
                None
            } else {
                Some(from_glib_none(ptr))
            }
        }
    }

    #[doc(alias = "get_output_buffer")]
    #[inline]
    pub fn output_buffer(&self) -> Option<&gst::BufferRef> {
        unsafe {
            let ptr = (*self.to_glib_none().0).output_buffer;
            if ptr.is_null() {
                None
            } else {
                Some(gst::BufferRef::from_ptr(ptr))
            }
        }
    }

    #[doc(alias = "get_output_buffer_mut")]
    pub fn output_buffer_mut(&mut self) -> Option<&mut gst::BufferRef> {
        unsafe {
            let ptr = (*self.to_glib_none().0).output_buffer;
            if ptr.is_null() {
                None
            } else {
                let writable: bool = from_glib(gst::ffi::gst_mini_object_is_writable(
                    ptr as *const gst::ffi::GstMiniObject,
                ));
                debug_assert!(writable);

                Some(gst::BufferRef::from_mut_ptr(ptr))
            }
        }
    }

    pub fn set_output_buffer(&mut self, output_buffer: gst::Buffer) {
        unsafe {
            assert!(output_buffer.is_writable());
            let prev = (*self.to_glib_none().0).output_buffer;

            if !prev.is_null() {
                gst::ffi::gst_mini_object_unref(prev as *mut gst::ffi::GstMiniObject);
            }

            (*self.to_glib_none().0).output_buffer = output_buffer.into_glib_ptr();
        }
    }

    #[doc(alias = "get_deadline")]
    #[inline]
    pub fn deadline(&self) -> Option<gst::ClockTime> {
        unsafe { from_glib((*self.to_glib_none().0).deadline) }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_video_decoder_get_processed_subframe_index")]
    #[inline]
    pub fn subframes_processed(&self) -> u32 {
        unsafe { (*self.to_glib_none().0).abidata.ABI.subframes_processed }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_video_decoder_get_input_subframe_index")]
    #[inline]
    pub fn num_subframes(&self) -> u32 {
        unsafe { (*self.to_glib_none().0).abidata.ABI.num_subframes }
    }
}

impl<'a> IntoGlibPtr<*mut ffi::GstVideoCodecFrame> for VideoCodecFrame<'a> {
    #[inline]
    unsafe fn into_glib_ptr(self) -> *mut ffi::GstVideoCodecFrame {
        let stream_lock = self.element.stream_lock();
        glib::ffi::g_rec_mutex_unlock(stream_lock);

        let s = mem::ManuallyDrop::new(self);
        s.to_glib_none().0
    }
}

impl<'a> Drop for VideoCodecFrame<'a> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            let stream_lock = self.element.stream_lock();
            glib::ffi::g_rec_mutex_unlock(stream_lock);

            ffi::gst_video_codec_frame_unref(self.frame);
        }
    }
}
