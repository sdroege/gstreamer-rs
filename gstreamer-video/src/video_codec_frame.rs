// Take a look at the license at the top of the repository in the LICENSE file.

use crate::utils::HasStreamLock;
use crate::VideoCodecFrameFlags;
use glib::translate::*;
use std::fmt;
use std::mem;

pub struct VideoCodecFrame<'a> {
    frame: *mut ffi::GstVideoCodecFrame,
    /* GstVideoCodecFrame API isn't safe so protect the frame using the
     * element (decoder or encoder) stream lock */
    element: &'a dyn HasStreamLock,
}

#[doc(hidden)]
impl<'a> ::glib::translate::ToGlibPtr<'a, *mut ffi::GstVideoCodecFrame> for VideoCodecFrame<'a> {
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> ::glib::translate::Stash<'a, *mut ffi::GstVideoCodecFrame, Self> {
        Stash(self.frame, self)
    }

    fn to_glib_full(&self) -> *mut ffi::GstVideoCodecFrame {
        unimplemented!()
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

    pub fn flags(&self) -> VideoCodecFrameFlags {
        let flags = unsafe { (*self.to_glib_none().0).flags };
        VideoCodecFrameFlags::from_bits_truncate(flags)
    }

    pub fn set_flags(&mut self, flags: VideoCodecFrameFlags) {
        unsafe { (*self.to_glib_none().0).flags |= flags.bits() }
    }

    pub fn unset_flags(&mut self, flags: VideoCodecFrameFlags) {
        unsafe { (*self.to_glib_none().0).flags &= !flags.bits() }
    }

    pub fn system_frame_number(&self) -> u32 {
        unsafe { (*self.to_glib_none().0).system_frame_number }
    }

    pub fn decode_frame_number(&self) -> u32 {
        unsafe { (*self.to_glib_none().0).decode_frame_number }
    }

    pub fn presentation_frame_number(&self) -> u32 {
        unsafe { (*self.to_glib_none().0).presentation_frame_number }
    }

    pub fn dts(&self) -> gst::ClockTime {
        unsafe { from_glib((*self.to_glib_none().0).dts) }
    }

    pub fn set_dts(&mut self, dts: gst::ClockTime) {
        unsafe {
            (*self.to_glib_none().0).dts = dts.to_glib();
        }
    }

    pub fn pts(&self) -> gst::ClockTime {
        unsafe { from_glib((*self.to_glib_none().0).pts) }
    }

    pub fn set_pts(&mut self, pts: gst::ClockTime) {
        unsafe {
            (*self.to_glib_none().0).pts = pts.to_glib();
        }
    }

    pub fn duration(&self) -> gst::ClockTime {
        unsafe { from_glib((*self.to_glib_none().0).duration) }
    }

    pub fn set_duration(&mut self, duration: gst::ClockTime) {
        unsafe {
            (*self.to_glib_none().0).duration = duration.to_glib();
        }
    }

    pub fn distance_from_sync(&self) -> i32 {
        unsafe { (*self.to_glib_none().0).distance_from_sync }
    }

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

    pub fn output_buffer_mut(&mut self) -> Option<&mut gst::BufferRef> {
        unsafe {
            let ptr = (*self.to_glib_none().0).output_buffer;
            if ptr.is_null() {
                None
            } else {
                let writable: bool = from_glib(gst::ffi::gst_mini_object_is_writable(
                    ptr as *const gst::ffi::GstMiniObject,
                ));
                assert!(writable);

                Some(gst::BufferRef::from_mut_ptr(ptr))
            }
        }
    }

    pub fn set_output_buffer(&mut self, output_buffer: gst::Buffer) {
        unsafe {
            let prev = (*self.to_glib_none().0).output_buffer;

            if !prev.is_null() {
                gst::ffi::gst_mini_object_unref(prev as *mut gst::ffi::GstMiniObject);
            }

            let ptr = output_buffer.into_ptr();
            let writable: bool = from_glib(gst::ffi::gst_mini_object_is_writable(
                ptr as *const gst::ffi::GstMiniObject,
            ));
            assert!(writable);

            (*self.to_glib_none().0).output_buffer = ptr;
        }
    }

    pub fn deadline(&self) -> gst::ClockTime {
        unsafe { from_glib((*self.to_glib_none().0).deadline) }
    }

    #[doc(hidden)]
    pub unsafe fn into_ptr(self) -> *mut ffi::GstVideoCodecFrame {
        let stream_lock = self.element.stream_lock();
        glib::ffi::g_rec_mutex_unlock(stream_lock);

        let s = mem::ManuallyDrop::new(self);
        s.to_glib_none().0
    }
}

impl<'a> Drop for VideoCodecFrame<'a> {
    fn drop(&mut self) {
        unsafe {
            let stream_lock = self.element.stream_lock();
            glib::ffi::g_rec_mutex_unlock(stream_lock);

            ffi::gst_video_codec_frame_unref(self.frame);
        }
    }
}
