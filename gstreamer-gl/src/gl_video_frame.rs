// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::{from_glib, ToGlibPtr};
use gst_video::video_frame::Readable;

use byteorder::{NativeEndian, ReadBytesExt};
use std::mem;

pub trait VideoFrameGLExt {
    fn from_buffer_readable_gl(
        buffer: gst::Buffer,
        info: &gst_video::VideoInfo,
    ) -> Result<gst_video::VideoFrame<Readable>, gst::Buffer>;

    fn from_buffer_ref_readable_gl<'a, 'b>(
        buffer: &'a gst::BufferRef,
        info: &'b gst_video::VideoInfo,
    ) -> Result<gst_video::VideoFrameRef<&'a gst::BufferRef>, glib::error::BoolError>;

    fn get_texture_id(&self, idx: u32) -> Option<u32>;
}

impl VideoFrameGLExt for gst_video::VideoFrame<Readable> {
    fn from_buffer_readable_gl(
        buffer: gst::Buffer,
        info: &gst_video::VideoInfo,
    ) -> Result<gst_video::VideoFrame<Readable>, gst::Buffer> {
        skip_assert_initialized!();
        gst_video::VideoFrameRef::<&gst::BufferRef>::from_buffer_readable_gl(buffer, info)
    }

    fn from_buffer_ref_readable_gl<'a, 'b>(
        buffer: &'a gst::BufferRef,
        info: &'b gst_video::VideoInfo,
    ) -> Result<gst_video::VideoFrameRef<&'a gst::BufferRef>, glib::error::BoolError> {
        skip_assert_initialized!();
        gst_video::VideoFrameRef::<&gst::BufferRef>::from_buffer_ref_readable_gl(buffer, info)
    }

    fn get_texture_id(&self, idx: u32) -> Option<u32> {
        self.as_video_frame_ref().get_texture_id(idx)
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
            let mut frame = mem::MaybeUninit::zeroed();
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
                Ok(gst_video::VideoFrame::from_glib_full(frame.assume_init()))
            }
        }
    }

    fn from_buffer_ref_readable_gl<'b, 'c>(
        buffer: &'b gst::BufferRef,
        info: &'c gst_video::VideoInfo,
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
            let mut frame = mem::MaybeUninit::zeroed();
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
                Ok(gst_video::VideoFrameRef::from_glib_full(
                    frame.assume_init(),
                ))
            }
        }
    }

    fn get_texture_id(&self, idx: u32) -> Option<u32> {
        let len = buffer_n_gl_memory(self.buffer())?;

        if idx >= len {
            return None;
        }

        // FIXME: planes are not memories
        if idx > self.n_planes() {
            return None;
        }

        let mut data = self.plane_data(idx).ok()?;
        let id = &data.read_u32::<NativeEndian>().ok()?;

        Some(*id)
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
        Some(num as u32)
    }
}
