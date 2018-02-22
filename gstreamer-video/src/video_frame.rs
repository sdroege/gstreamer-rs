// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use gst_ffi;

use gst;
use gst::miniobject::MiniObject;
use glib;
use glib::translate::{from_glib, ToGlibPtr};

use std::mem;
use std::ptr;
use std::marker::PhantomData;
use std::slice;
use std::ops;

pub struct Readable;
pub struct Writable;
pub struct VideoFrame<T>(
    ffi::GstVideoFrame,
    Option<gst::Buffer>,
    ::VideoInfo,
    PhantomData<T>,
);

impl<T> VideoFrame<T> {
    pub fn info(&self) -> &::VideoInfo {
        &self.2
    }

    pub fn flags(&self) -> ::VideoFrameFlags {
        from_glib(self.0.flags)
    }

    pub fn id(&self) -> i32 {
        self.0.id
    }

    pub fn into_buffer(mut self) -> gst::Buffer {
        self.1.take().unwrap()
    }

    pub fn copy(&self, dest: &mut VideoFrame<Writable>) -> Result<(), glib::BoolError> {
        unsafe {
            let res: bool = from_glib(ffi::gst_video_frame_copy(&mut dest.0, &self.0));
            if res {
                Ok(())
            } else {
                Err(glib::BoolError("Failed to copy video frame"))
            }
        }
    }

    pub fn copy_plane(
        &self,
        dest: &mut VideoFrame<Writable>,
        plane: u32,
    ) -> Result<(), glib::BoolError> {
        skip_assert_initialized!();

        unsafe {
            let res: bool = from_glib(ffi::gst_video_frame_copy_plane(&mut dest.0, &self.0, plane));
            if res {
                Ok(())
            } else {
                Err(glib::BoolError("Failed to copy video frame plane"))
            }
        }
    }

    pub fn format(&self) -> ::VideoFormat {
        self.info().format()
    }

    pub fn format_info(&self) -> ::VideoFormatInfo {
        self.info().format_info()
    }

    pub fn width(&self) -> u32 {
        self.info().width()
    }

    pub fn height(&self) -> u32 {
        self.info().height()
    }

    pub fn size(&self) -> usize {
        self.info().size()
    }

    pub fn is_interlaced(&self) -> bool {
        self.flags().contains(::VideoFrameFlags::INTERLACED)
    }

    pub fn is_tff(&self) -> bool {
        self.flags().contains(::VideoFrameFlags::TFF)
    }

    pub fn is_rff(&self) -> bool {
        self.flags().contains(::VideoFrameFlags::RFF)
    }

    pub fn is_onefield(&self) -> bool {
        self.flags().contains(::VideoFrameFlags::ONEFIELD)
    }

    pub fn n_planes(&self) -> u32 {
        self.info().n_planes()
    }

    pub fn n_components(&self) -> u32 {
        self.info().n_components()
    }

    pub fn plane_stride(&self) -> &[i32] {
        self.info().stride()
    }

    pub fn plane_offset(&self) -> &[usize] {
        self.info().offset()
    }

    pub fn buffer(&self) -> &gst::BufferRef {
        unsafe { gst::BufferRef::from_ptr(self.0.buffer) }
    }

    pub fn plane_data(&self, plane: u32) -> Option<&[u8]> {
        if plane >= self.n_planes() {
            return None;
        }

        let format_info = self.format_info();

        // Just get the palette
        if format_info.has_palette() && plane == 1 {
            unsafe {
                return Some(slice::from_raw_parts(self.0.data[1] as *const u8, 256 * 4));
            }
        }

        let w = self.plane_stride()[plane as usize] as u32;
        // FIXME: This assumes that the horizontal subsampling of all
        // components in the plane is the same, which is probably safe
        let h = format_info.scale_height(plane as u8, self.height());

        unsafe {
            Some(slice::from_raw_parts(
                self.0.data[plane as usize] as *const u8,
                (w * h) as usize,
            ))
        }
    }
}

impl<T> Drop for VideoFrame<T> {
    fn drop(&mut self) {
        unsafe {
            ffi::gst_video_frame_unmap(&mut self.0);
        }
    }
}

impl VideoFrame<Readable> {
    pub fn from_buffer_readable(
        buffer: gst::Buffer,
        info: &::VideoInfo,
    ) -> Result<VideoFrame<Readable>, gst::Buffer> {
        skip_assert_initialized!();

        unsafe {
            let mut frame = mem::zeroed();
            let res: bool = from_glib(ffi::gst_video_frame_map(
                &mut frame,
                info.to_glib_none().0 as *mut _,
                buffer.to_glib_none().0,
                mem::transmute(
                    ffi::GST_VIDEO_FRAME_MAP_FLAG_NO_REF.bits() | gst_ffi::GST_MAP_READ.bits(),
                ),
            ));

            if !res {
                Err(buffer)
            } else {
                let info = ::VideoInfo(ptr::read(&frame.info));
                Ok(VideoFrame(frame, Some(buffer), info, PhantomData))
            }
        }
    }

    pub fn from_buffer_id_readable(
        buffer: gst::Buffer,
        id: i32,
        info: &::VideoInfo,
    ) -> Result<VideoFrame<Readable>, gst::Buffer> {
        skip_assert_initialized!();

        unsafe {
            let mut frame = mem::zeroed();
            let res: bool = from_glib(ffi::gst_video_frame_map_id(
                &mut frame,
                info.to_glib_none().0 as *mut _,
                buffer.to_glib_none().0,
                id,
                mem::transmute(
                    ffi::GST_VIDEO_FRAME_MAP_FLAG_NO_REF.bits() | gst_ffi::GST_MAP_READ.bits(),
                ),
            ));

            if !res {
                Err(buffer)
            } else {
                let info = ::VideoInfo(ptr::read(&frame.info));
                Ok(VideoFrame(frame, Some(buffer), info, PhantomData))
            }
        }
    }

    pub fn as_video_frame_ref(&self) -> VideoFrameRef<&gst::BufferRef> {
        let vframe = unsafe { ptr::read(&self.0) };
        let info = self.2.clone();
        VideoFrameRef(vframe, Some(self.buffer()), info, true)
    }
}

impl VideoFrame<Writable> {
    pub fn from_buffer_writable(
        buffer: gst::Buffer,
        info: &::VideoInfo,
    ) -> Result<VideoFrame<Writable>, gst::Buffer> {
        skip_assert_initialized!();

        unsafe {
            let mut frame = mem::zeroed();
            let res: bool = from_glib(ffi::gst_video_frame_map(
                &mut frame,
                info.to_glib_none().0 as *mut _,
                buffer.to_glib_none().0,
                mem::transmute(
                    ffi::GST_VIDEO_FRAME_MAP_FLAG_NO_REF.bits() | gst_ffi::GST_MAP_READ.bits()
                        | gst_ffi::GST_MAP_WRITE.bits(),
                ),
            ));

            if !res {
                Err(buffer)
            } else {
                let info = ::VideoInfo(ptr::read(&frame.info));
                Ok(VideoFrame(frame, Some(buffer), info, PhantomData))
            }
        }
    }

    pub fn from_buffer_id_writable(
        buffer: gst::Buffer,
        id: i32,
        info: &::VideoInfo,
    ) -> Result<VideoFrame<Writable>, gst::Buffer> {
        skip_assert_initialized!();

        unsafe {
            let mut frame = mem::zeroed();
            let res: bool = from_glib(ffi::gst_video_frame_map_id(
                &mut frame,
                info.to_glib_none().0 as *mut _,
                buffer.to_glib_none().0,
                id,
                mem::transmute(
                    ffi::GST_VIDEO_FRAME_MAP_FLAG_NO_REF.bits() | gst_ffi::GST_MAP_READ.bits()
                        | gst_ffi::GST_MAP_WRITE.bits(),
                ),
            ));

            if !res {
                Err(buffer)
            } else {
                let info = ::VideoInfo(ptr::read(&frame.info));
                Ok(VideoFrame(frame, Some(buffer), info, PhantomData))
            }
        }
    }

    pub fn buffer_mut(&mut self) -> &mut gst::BufferRef {
        unsafe { gst::BufferRef::from_mut_ptr(self.0.buffer) }
    }

    pub fn plane_data_mut(&mut self, plane: u32) -> Option<&mut [u8]> {
        if plane >= self.n_planes() {
            return None;
        }

        let format_info = self.format_info();

        // Just get the palette
        if format_info.has_palette() && plane == 1 {
            unsafe {
                return Some(slice::from_raw_parts_mut(
                    self.0.data[1] as *mut u8,
                    256 * 4,
                ));
            }
        }

        let w = self.plane_stride()[plane as usize] as u32;
        // FIXME: This assumes that the horizontal subsampling of all
        // components in the plane is the same, which is probably safe
        let h = format_info.scale_height(plane as u8, self.height());

        unsafe {
            Some(slice::from_raw_parts_mut(
                self.0.data[plane as usize] as *mut u8,
                (w * h) as usize,
            ))
        }
    }

    pub fn as_mut_video_frame_ref(&mut self) -> VideoFrameRef<&mut gst::BufferRef> {
        let vframe = unsafe { ptr::read(&self.0) };
        let info = self.2.clone();
        VideoFrameRef(vframe, Some(self.buffer_mut()), info, true)
    }
}

pub struct VideoFrameRef<T>(ffi::GstVideoFrame, Option<T>, ::VideoInfo, bool);

impl<'a> VideoFrameRef<&'a gst::BufferRef> {
    pub fn from_buffer_ref_readable<'b>(
        buffer: &'a gst::BufferRef,
        info: &'b ::VideoInfo,
    ) -> Option<VideoFrameRef<&'a gst::BufferRef>> {
        skip_assert_initialized!();

        unsafe {
            let mut frame = mem::zeroed();
            let res: bool = from_glib(ffi::gst_video_frame_map(
                &mut frame,
                info.to_glib_none().0 as *mut _,
                buffer.as_mut_ptr(),
                mem::transmute(
                    ffi::GST_VIDEO_FRAME_MAP_FLAG_NO_REF.bits() | gst_ffi::GST_MAP_READ.bits(),
                ),
            ));

            if !res {
                None
            } else {
                let info = ::VideoInfo(ptr::read(&frame.info));
                Some(VideoFrameRef(frame, Some(buffer), info, false))
            }
        }
    }

    pub fn from_buffer_ref_id_readable<'b>(
        buffer: &'a gst::BufferRef,
        id: i32,
        info: &'b ::VideoInfo,
    ) -> Option<VideoFrameRef<&'a gst::BufferRef>> {
        skip_assert_initialized!();

        unsafe {
            let mut frame = mem::zeroed();
            let res: bool = from_glib(ffi::gst_video_frame_map_id(
                &mut frame,
                info.to_glib_none().0 as *mut _,
                buffer.as_mut_ptr(),
                id,
                mem::transmute(
                    ffi::GST_VIDEO_FRAME_MAP_FLAG_NO_REF.bits() | gst_ffi::GST_MAP_READ.bits(),
                ),
            ));

            if !res {
                None
            } else {
                let info = ::VideoInfo(ptr::read(&frame.info));
                Some(VideoFrameRef(frame, Some(buffer), info, false))
            }
        }
    }

    pub fn info(&self) -> &::VideoInfo {
        &self.2
    }

    pub fn flags(&self) -> ::VideoFrameFlags {
        from_glib(self.0.flags)
    }

    pub fn id(&self) -> i32 {
        self.0.id
    }

    pub fn copy(
        &self,
        dest: &mut VideoFrameRef<&mut gst::BufferRef>,
    ) -> Result<(), glib::BoolError> {
        unsafe {
            let res: bool = from_glib(ffi::gst_video_frame_copy(&mut dest.0, &self.0));
            if res {
                Ok(())
            } else {
                Err(glib::BoolError("Failed to copy video frame"))
            }
        }
    }

    pub fn copy_plane(
        &self,
        dest: &mut VideoFrameRef<&mut gst::BufferRef>,
        plane: u32,
    ) -> Result<(), glib::BoolError> {
        skip_assert_initialized!();

        unsafe {
            let res: bool = from_glib(ffi::gst_video_frame_copy_plane(&mut dest.0, &self.0, plane));
            if res {
                Ok(())
            } else {
                Err(glib::BoolError("Failed to copy video frame plane"))
            }
        }
    }

    pub fn format(&self) -> ::VideoFormat {
        self.info().format()
    }

    pub fn format_info(&self) -> ::VideoFormatInfo {
        self.info().format_info()
    }

    pub fn width(&self) -> u32 {
        self.info().width()
    }

    pub fn height(&self) -> u32 {
        self.info().height()
    }

    pub fn size(&self) -> usize {
        self.info().size()
    }

    pub fn is_interlaced(&self) -> bool {
        self.flags().contains(::VideoFrameFlags::INTERLACED)
    }

    pub fn is_tff(&self) -> bool {
        self.flags().contains(::VideoFrameFlags::TFF)
    }

    pub fn is_rff(&self) -> bool {
        self.flags().contains(::VideoFrameFlags::RFF)
    }

    pub fn is_onefield(&self) -> bool {
        self.flags().contains(::VideoFrameFlags::ONEFIELD)
    }

    pub fn n_planes(&self) -> u32 {
        self.info().n_planes()
    }

    pub fn n_components(&self) -> u32 {
        self.info().n_components()
    }

    pub fn plane_stride(&self) -> &[i32] {
        self.info().stride()
    }

    pub fn plane_offset(&self) -> &[usize] {
        self.info().offset()
    }

    pub fn buffer(&self) -> &gst::BufferRef {
        self.1.as_ref().unwrap()
    }

    pub fn plane_data(&self, plane: u32) -> Option<&[u8]> {
        if plane >= self.n_planes() {
            return None;
        }

        let format_info = self.format_info();

        // Just get the palette
        if format_info.has_palette() && plane == 1 {
            unsafe {
                return Some(slice::from_raw_parts(self.0.data[1] as *const u8, 256 * 4));
            }
        }

        let w = self.plane_stride()[plane as usize] as u32;
        // FIXME: This assumes that the horizontal subsampling of all
        // components in the plane is the same, which is probably safe
        let h = format_info.scale_height(plane as u8, self.height());

        unsafe {
            Some(slice::from_raw_parts(
                self.0.data[plane as usize] as *const u8,
                (w * h) as usize,
            ))
        }
    }
}

impl<'a> VideoFrameRef<&'a mut gst::BufferRef> {
    pub fn from_buffer_ref_writable<'b>(
        buffer: &'a mut gst::BufferRef,
        info: &'b ::VideoInfo,
    ) -> Option<VideoFrameRef<&'a mut gst::BufferRef>> {
        skip_assert_initialized!();

        unsafe {
            let mut frame = mem::zeroed();
            let res: bool = from_glib(ffi::gst_video_frame_map(
                &mut frame,
                info.to_glib_none().0 as *mut _,
                buffer.as_mut_ptr(),
                mem::transmute(
                    ffi::GST_VIDEO_FRAME_MAP_FLAG_NO_REF.bits() | gst_ffi::GST_MAP_READ.bits()
                        | gst_ffi::GST_MAP_WRITE.bits(),
                ),
            ));

            if !res {
                None
            } else {
                let info = ::VideoInfo(ptr::read(&frame.info));
                Some(VideoFrameRef(frame, Some(buffer), info, false))
            }
        }
    }

    pub fn from_buffer_ref_id_writable<'b>(
        buffer: &'a mut gst::BufferRef,
        id: i32,
        info: &'b ::VideoInfo,
    ) -> Option<VideoFrameRef<&'a mut gst::BufferRef>> {
        skip_assert_initialized!();

        unsafe {
            let mut frame = mem::zeroed();
            let res: bool = from_glib(ffi::gst_video_frame_map_id(
                &mut frame,
                info.to_glib_none().0 as *mut _,
                buffer.as_mut_ptr(),
                id,
                mem::transmute(
                    ffi::GST_VIDEO_FRAME_MAP_FLAG_NO_REF.bits() | gst_ffi::GST_MAP_READ.bits()
                        | gst_ffi::GST_MAP_WRITE.bits(),
                ),
            ));

            if !res {
                None
            } else {
                let info = ::VideoInfo(ptr::read(&frame.info));
                Some(VideoFrameRef(frame, Some(buffer), info, false))
            }
        }
    }

    pub fn buffer_mut(&mut self) -> &mut gst::BufferRef {
        self.1.as_mut().unwrap()
    }

    pub fn plane_data_mut(&mut self, plane: u32) -> Option<&mut [u8]> {
        if plane >= self.n_planes() {
            return None;
        }

        let format_info = self.format_info();

        // Just get the palette
        if format_info.has_palette() && plane == 1 {
            unsafe {
                return Some(slice::from_raw_parts_mut(
                    self.0.data[1] as *mut u8,
                    256 * 4,
                ));
            }
        }

        let w = self.plane_stride()[plane as usize] as u32;
        // FIXME: This assumes that the horizontal subsampling of all
        // components in the plane is the same, which is probably safe
        let h = format_info.scale_height(plane as u8, self.height());

        unsafe {
            Some(slice::from_raw_parts_mut(
                self.0.data[plane as usize] as *mut u8,
                (w * h) as usize,
            ))
        }
    }
}

impl<'a> ops::Deref for VideoFrameRef<&'a mut gst::BufferRef> {
    type Target = VideoFrameRef<&'a gst::BufferRef>;

    fn deref(&self) -> &Self::Target {
        use std::mem;
        unsafe { mem::transmute(self) }
    }
}

impl<T> Drop for VideoFrameRef<T> {
    fn drop(&mut self) {
        if !self.3 {
            unsafe {
                ffi::gst_video_frame_unmap(&mut self.0);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gst;

    #[test]
    fn test_map_read() {
        gst::init().unwrap();

        let info = ::VideoInfo::new(::VideoFormat::Gray8, 320, 240)
            .build()
            .unwrap();
        let buffer = gst::Buffer::with_size(info.size()).unwrap();
        let frame = VideoFrame::from_buffer_readable(buffer, &info).unwrap();

        assert_ne!(frame.plane_data(0), None);
        assert_eq!(frame.plane_data(0).unwrap().len(), 320 * 240);
        assert_eq!(frame.plane_data(1), None);
        assert!(frame.info() == &info);
    }

    #[test]
    fn test_map_write() {
        gst::init().unwrap();

        let info = ::VideoInfo::new(::VideoFormat::Gray8, 320, 240)
            .build()
            .unwrap();
        let buffer = gst::Buffer::with_size(info.size()).unwrap();
        let mut frame = VideoFrame::from_buffer_writable(buffer, &info).unwrap();

        assert_ne!(frame.plane_data_mut(0), None);
        assert_eq!(frame.plane_data_mut(0).unwrap().len(), 320 * 240);
        assert_eq!(frame.plane_data_mut(1), None);
        assert!(frame.info() == &info);
    }
}
