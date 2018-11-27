// Copyright (C) 2018 Víctor Jáquez <vjaquez@igalia.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib_ffi;
use gst_ffi;
use gst_video_ffi;

use glib::translate::{from_glib, ToGlibPtr};
use gst;
use gst::MiniObject;
use gst_video::video_frame::Readable;
use gst_video::*;

use byteorder::{NativeEndian, ReadBytesExt};
use std::mem;

pub trait VideoFrameGLExt {
    fn from_buffer_readable_gl(
        buffer: gst::Buffer,
        info: &VideoInfo,
    ) -> Result<VideoFrame<Readable>, gst::Buffer>;

    fn get_texture_id(&self, idx: u32) -> Option<u32>;
}

impl VideoFrameGLExt for VideoFrame<Readable> {
    fn from_buffer_readable_gl(
        buffer: gst::Buffer,
        info: &VideoInfo,
    ) -> Result<VideoFrame<Readable>, gst::Buffer> {
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
            let mut frame = mem::zeroed();
            let res: bool = from_glib(gst_video_ffi::gst_video_frame_map(
                &mut frame,
                info.to_glib_none().0 as *mut _,
                buffer.to_glib_none().0,
                gst_video_ffi::GST_VIDEO_FRAME_MAP_FLAG_NO_REF
                    | gst_ffi::GST_MAP_READ
                    | ffi::GST_MAP_GL as u32,
            ));

            if !res {
                Err(buffer)
            } else {
                Ok(VideoFrame::from_glib_full(frame))
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

        let mut data = self.plane_data(idx)?;
        let id = &data.read_u32::<NativeEndian>().ok()?;

        Some(*id)
    }
}

fn buffer_n_gl_memory(buffer: &gst::BufferRef) -> Option<u32> {
    unsafe {
        let buf = buffer.as_mut_ptr();
        let num = gst_ffi::gst_buffer_n_memory(buf);
        for i in 0..num - 1 {
            let mem = gst_ffi::gst_buffer_peek_memory(buf, i);
            if ffi::gst_is_gl_memory(mem) != glib_ffi::GTRUE {
                return None;
            }
        }
        Some(num as u32)
    }
}
