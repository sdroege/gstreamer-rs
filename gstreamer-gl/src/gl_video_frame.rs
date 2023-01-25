// Take a look at the license at the top of the repository in the LICENSE file.

use std::mem;

use glib::translate::{from_glib, ToGlibPtr};
use gst_video::video_frame::Readable;

pub trait VideoFrameGLExt {
    fn from_buffer_readable_gl(
        buffer: gst::Buffer,
        info: &gst_video::VideoInfo,
    ) -> Result<gst_video::VideoFrame<Readable>, gst::Buffer>;

    fn from_buffer_ref_readable_gl<'a>(
        buffer: &'a gst::BufferRef,
        info: &gst_video::VideoInfo,
    ) -> Result<gst_video::VideoFrameRef<&'a gst::BufferRef>, glib::error::BoolError>;

    #[doc(alias = "get_texture_id")]
    fn texture_id(&self, idx: u32) -> Option<u32>;
}

impl VideoFrameGLExt for gst_video::VideoFrame<Readable> {
    fn from_buffer_readable_gl(
        buffer: gst::Buffer,
        info: &gst_video::VideoInfo,
    ) -> Result<gst_video::VideoFrame<Readable>, gst::Buffer> {
        skip_assert_initialized!();
        gst_video::VideoFrameRef::<&gst::BufferRef>::from_buffer_readable_gl(buffer, info)
    }

    fn from_buffer_ref_readable_gl<'a>(
        buffer: &'a gst::BufferRef,
        info: &gst_video::VideoInfo,
    ) -> Result<gst_video::VideoFrameRef<&'a gst::BufferRef>, glib::error::BoolError> {
        skip_assert_initialized!();
        gst_video::VideoFrameRef::<&gst::BufferRef>::from_buffer_ref_readable_gl(buffer, info)
    }

    fn texture_id(&self, idx: u32) -> Option<u32> {
        self.as_video_frame_ref().texture_id(idx)
    }
}

impl<'a> VideoFrameGLExt for gst_video::VideoFrameRef<&'a gst::BufferRef> {
    fn from_buffer_readable_gl(
        buffer: gst::Buffer,
        info: &gst_video::VideoInfo,
    ) -> Result<gst_video::VideoFrame<Readable>, gst::Buffer> {
        skip_assert_initialized!();

        let n_mem = match buffer_n_gl_memory(buffer.as_ref()) {
            Some(n) => n,
            None => return Err(buffer),
        };

        // FIXME: planes are not memories, in multiview use case,
        // number of memories = planes * views, but the raw memory is
        // not exposed in videoframe
        if n_mem != info.n_planes() {
            return Err(buffer);
        }

        unsafe {
            let mut frame = mem::MaybeUninit::uninit();
            let res: bool = from_glib(gst_video::ffi::gst_video_frame_map(
                frame.as_mut_ptr(),
                info.to_glib_none().0 as *mut _,
                buffer.to_glib_none().0,
                gst_video::ffi::GST_VIDEO_FRAME_MAP_FLAG_NO_REF
                    | gst::ffi::GST_MAP_READ
                    | ffi::GST_MAP_GL as u32,
            ));

            if !res {
                Err(buffer)
            } else {
                let mut frame = frame.assume_init();
                // Reset size/stride/offset to 0 as the memory pointers
                // are the GL texture ID and accessing them would read
                // random memory.
                frame.info.size = 0;
                frame.info.stride.fill(0);
                frame.info.offset.fill(0);
                Ok(gst_video::VideoFrame::from_glib_full(frame))
            }
        }
    }

    fn from_buffer_ref_readable_gl<'b>(
        buffer: &'b gst::BufferRef,
        info: &gst_video::VideoInfo,
    ) -> Result<gst_video::VideoFrameRef<&'b gst::BufferRef>, glib::error::BoolError> {
        skip_assert_initialized!();

        let n_mem = match buffer_n_gl_memory(buffer) {
            Some(n) => n,
            None => return Err(glib::bool_error!("Memory is not a GstGLMemory")),
        };

        // FIXME: planes are not memories, in multiview use case,
        // number of memories = planes * views, but the raw memory is
        // not exposed in videoframe
        if n_mem != info.n_planes() {
            return Err(glib::bool_error!(
                "Number of planes and memories is not matching"
            ));
        }

        unsafe {
            let mut frame = mem::MaybeUninit::uninit();
            let res: bool = from_glib(gst_video::ffi::gst_video_frame_map(
                frame.as_mut_ptr(),
                info.to_glib_none().0 as *mut _,
                buffer.as_mut_ptr(),
                gst_video::ffi::GST_VIDEO_FRAME_MAP_FLAG_NO_REF
                    | gst::ffi::GST_MAP_READ
                    | ffi::GST_MAP_GL as u32,
            ));

            if !res {
                Err(glib::bool_error!(
                    "Failed to fill in the values of GstVideoFrame"
                ))
            } else {
                let mut frame = frame.assume_init();
                // Reset size/stride/offset to 0 as the memory pointers
                // are the GL texture ID and accessing them would read
                // random memory.
                frame.info.size = 0;
                frame.info.stride.fill(0);
                frame.info.offset.fill(0);
                Ok(gst_video::VideoFrameRef::from_glib_full(frame))
            }
        }
    }

    fn texture_id(&self, idx: u32) -> Option<u32> {
        let len = buffer_n_gl_memory(self.buffer())?;

        if idx >= len {
            return None;
        }

        // FIXME: planes are not memories
        if idx > self.n_planes() {
            return None;
        }

        unsafe {
            let ptr = (*self.as_ptr()).data[idx as usize] as *const u32;
            Some(*ptr)
        }
    }
}

fn buffer_n_gl_memory(buffer: &gst::BufferRef) -> Option<u32> {
    skip_assert_initialized!();
    unsafe {
        let buf = buffer.as_mut_ptr();
        let num = gst::ffi::gst_buffer_n_memory(buf);
        for i in 0..num - 1 {
            let mem = gst::ffi::gst_buffer_peek_memory(buf, i);
            if ffi::gst_is_gl_memory(mem) != glib::ffi::GTRUE {
                return None;
            }
        }
        Some(num)
    }
}
