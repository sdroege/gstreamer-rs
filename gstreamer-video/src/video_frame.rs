// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::{from_glib, from_glib_none, Borrowed, ToGlibPtr};

use std::fmt;
use std::marker::PhantomData;
use std::mem;
use std::ops;
use std::ptr;
use std::slice;

pub enum Readable {}
pub enum Writable {}

pub struct VideoFrame<T> {
    frame: ffi::GstVideoFrame,
    buffer: Option<gst::Buffer>,
    info: crate::VideoInfo,
    phantom: PhantomData<T>,
}

unsafe impl<T> Send for VideoFrame<T> {}
unsafe impl<T> Sync for VideoFrame<T> {}

impl<T> fmt::Debug for VideoFrame<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoFrame")
            .field("frame", &self.frame)
            .field("buffer", &self.buffer)
            .field("info", &self.info)
            .field("phantom", &self.phantom)
            .finish()
    }
}

impl<T> VideoFrame<T> {
    pub fn info(&self) -> &crate::VideoInfo {
        &self.info
    }

    pub fn flags(&self) -> crate::VideoFrameFlags {
        unsafe { from_glib(self.frame.flags) }
    }

    pub fn id(&self) -> i32 {
        self.frame.id
    }

    pub fn into_buffer(mut self) -> gst::Buffer {
        self.buffer.take().unwrap()
    }

    pub fn copy(&self, dest: &mut VideoFrame<Writable>) -> Result<(), glib::BoolError> {
        unsafe {
            let res: bool = from_glib(ffi::gst_video_frame_copy(&mut dest.frame, &self.frame));
            if res {
                Ok(())
            } else {
                Err(glib::bool_error!("Failed to copy video frame"))
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
            let res: bool = from_glib(ffi::gst_video_frame_copy_plane(
                &mut dest.frame,
                &self.frame,
                plane,
            ));
            if res {
                Ok(())
            } else {
                Err(glib::bool_error!("Failed to copy video frame plane"))
            }
        }
    }

    pub fn format(&self) -> crate::VideoFormat {
        self.info().format()
    }

    pub fn format_info(&self) -> crate::VideoFormatInfo {
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
        self.flags().contains(crate::VideoFrameFlags::INTERLACED)
    }

    pub fn is_tff(&self) -> bool {
        self.flags().contains(crate::VideoFrameFlags::TFF)
    }

    pub fn is_rff(&self) -> bool {
        self.flags().contains(crate::VideoFrameFlags::RFF)
    }

    pub fn is_onefield(&self) -> bool {
        self.flags().contains(crate::VideoFrameFlags::ONEFIELD)
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
        unsafe { gst::BufferRef::from_ptr(self.frame.buffer) }
    }

    pub fn plane_data(&self, plane: u32) -> Result<&[u8], glib::BoolError> {
        if plane >= self.n_planes() {
            return Err(glib::bool_error!(
                "Plane index higher than number of planes"
            ));
        }

        let format_info = self.format_info();

        // Just get the palette
        if format_info.has_palette() && plane == 1 {
            unsafe {
                return Ok(slice::from_raw_parts(
                    self.frame.data[1] as *const u8,
                    256 * 4,
                ));
            }
        }

        let w = self.plane_stride()[plane as usize] as u32;
        // FIXME: This assumes that the horizontal subsampling of all
        // components in the plane is the same, which is probably safe
        let h = format_info.scale_height(plane as u8, self.height());

        unsafe {
            Ok(slice::from_raw_parts(
                self.frame.data[plane as usize] as *const u8,
                (w * h) as usize,
            ))
        }
    }

    pub unsafe fn from_glib_full(frame: ffi::GstVideoFrame) -> Self {
        let info = crate::VideoInfo(ptr::read(&frame.info));
        let buffer = gst::Buffer::from_glib_none(frame.buffer);
        Self {
            frame,
            buffer: Some(buffer),
            info,
            phantom: PhantomData,
        }
    }

    pub fn as_video_frame_ref(&self) -> VideoFrameRef<&gst::BufferRef> {
        let frame = unsafe { ptr::read(&self.frame) };
        let info = self.info.clone();
        VideoFrameRef {
            frame,
            buffer: Some(self.buffer()),
            info,
            unmap: false,
        }
    }

    pub fn as_ptr(&self) -> *const ffi::GstVideoFrame {
        &self.frame
    }
}

impl<T> Drop for VideoFrame<T> {
    fn drop(&mut self) {
        unsafe {
            ffi::gst_video_frame_unmap(&mut self.frame);
        }
    }
}

impl VideoFrame<Readable> {
    pub fn from_buffer_readable(
        buffer: gst::Buffer,
        info: &crate::VideoInfo,
    ) -> Result<Self, gst::Buffer> {
        skip_assert_initialized!();

        assert!(info.is_valid());

        unsafe {
            let mut frame = mem::MaybeUninit::zeroed();
            let res: bool = from_glib(ffi::gst_video_frame_map(
                frame.as_mut_ptr(),
                info.to_glib_none().0 as *mut _,
                buffer.to_glib_none().0,
                ffi::GST_VIDEO_FRAME_MAP_FLAG_NO_REF | gst::ffi::GST_MAP_READ,
            ));

            if !res {
                Err(buffer)
            } else {
                let frame = frame.assume_init();
                let info = crate::VideoInfo(ptr::read(&frame.info));
                Ok(Self {
                    frame,
                    buffer: Some(buffer),
                    info,
                    phantom: PhantomData,
                })
            }
        }
    }

    pub fn from_buffer_id_readable(
        buffer: gst::Buffer,
        id: i32,
        info: &crate::VideoInfo,
    ) -> Result<Self, gst::Buffer> {
        skip_assert_initialized!();

        assert!(info.is_valid());

        unsafe {
            let mut frame = mem::MaybeUninit::zeroed();
            let res: bool = from_glib(ffi::gst_video_frame_map_id(
                frame.as_mut_ptr(),
                info.to_glib_none().0 as *mut _,
                buffer.to_glib_none().0,
                id,
                ffi::GST_VIDEO_FRAME_MAP_FLAG_NO_REF | gst::ffi::GST_MAP_READ,
            ));

            if !res {
                Err(buffer)
            } else {
                let frame = frame.assume_init();
                let info = crate::VideoInfo(ptr::read(&frame.info));
                Ok(Self {
                    frame,
                    buffer: Some(buffer),
                    info,
                    phantom: PhantomData,
                })
            }
        }
    }

    pub fn buffer_owned(&self) -> gst::Buffer {
        unsafe { from_glib_none(self.frame.buffer) }
    }
}

impl VideoFrame<Writable> {
    pub fn from_buffer_writable(
        buffer: gst::Buffer,
        info: &crate::VideoInfo,
    ) -> Result<Self, gst::Buffer> {
        skip_assert_initialized!();

        assert!(info.is_valid());

        unsafe {
            let mut frame = mem::MaybeUninit::zeroed();
            let res: bool = from_glib(ffi::gst_video_frame_map(
                frame.as_mut_ptr(),
                info.to_glib_none().0 as *mut _,
                buffer.to_glib_none().0,
                ffi::GST_VIDEO_FRAME_MAP_FLAG_NO_REF
                    | gst::ffi::GST_MAP_READ
                    | gst::ffi::GST_MAP_WRITE,
            ));

            if !res {
                Err(buffer)
            } else {
                let frame = frame.assume_init();
                let info = crate::VideoInfo(ptr::read(&frame.info));
                Ok(Self {
                    frame,
                    buffer: Some(buffer),
                    info,
                    phantom: PhantomData,
                })
            }
        }
    }

    pub fn from_buffer_id_writable(
        buffer: gst::Buffer,
        id: i32,
        info: &crate::VideoInfo,
    ) -> Result<Self, gst::Buffer> {
        skip_assert_initialized!();

        assert!(info.is_valid());

        unsafe {
            let mut frame = mem::MaybeUninit::zeroed();
            let res: bool = from_glib(ffi::gst_video_frame_map_id(
                frame.as_mut_ptr(),
                info.to_glib_none().0 as *mut _,
                buffer.to_glib_none().0,
                id,
                ffi::GST_VIDEO_FRAME_MAP_FLAG_NO_REF
                    | gst::ffi::GST_MAP_READ
                    | gst::ffi::GST_MAP_WRITE,
            ));

            if !res {
                Err(buffer)
            } else {
                let frame = frame.assume_init();
                let info = crate::VideoInfo(ptr::read(&frame.info));
                Ok(Self {
                    frame,
                    buffer: Some(buffer),
                    info,
                    phantom: PhantomData,
                })
            }
        }
    }

    pub fn buffer_mut(&mut self) -> &mut gst::BufferRef {
        unsafe { gst::BufferRef::from_mut_ptr(self.frame.buffer) }
    }

    pub fn plane_data_mut(&mut self, plane: u32) -> Result<&mut [u8], glib::BoolError> {
        if plane >= self.n_planes() {
            return Err(glib::bool_error!(
                "Plane index higher than number of planes"
            ));
        }

        let format_info = self.format_info();

        // Just get the palette
        if format_info.has_palette() && plane == 1 {
            unsafe {
                return Ok(slice::from_raw_parts_mut(
                    self.frame.data[1] as *mut u8,
                    256 * 4,
                ));
            }
        }

        let w = self.plane_stride()[plane as usize] as u32;
        // FIXME: This assumes that the horizontal subsampling of all
        // components in the plane is the same, which is probably safe
        let h = format_info.scale_height(plane as u8, self.height());

        unsafe {
            Ok(slice::from_raw_parts_mut(
                self.frame.data[plane as usize] as *mut u8,
                (w * h) as usize,
            ))
        }
    }

    pub fn as_mut_video_frame_ref(&mut self) -> VideoFrameRef<&mut gst::BufferRef> {
        let frame = unsafe { ptr::read(&self.frame) };
        let info = self.info.clone();
        VideoFrameRef {
            frame,
            buffer: Some(self.buffer_mut()),
            info,
            unmap: false,
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut ffi::GstVideoFrame {
        &mut self.frame
    }
}

#[derive(Debug)]
pub struct VideoFrameRef<T> {
    frame: ffi::GstVideoFrame,
    buffer: Option<T>,
    info: crate::VideoInfo,
    unmap: bool,
}

impl<T> VideoFrameRef<T> {
    pub fn info(&self) -> &crate::VideoInfo {
        &self.info
    }

    pub fn flags(&self) -> crate::VideoFrameFlags {
        unsafe { from_glib(self.frame.flags) }
    }

    pub fn id(&self) -> i32 {
        self.frame.id
    }

    pub fn copy(
        &self,
        dest: &mut VideoFrameRef<&mut gst::BufferRef>,
    ) -> Result<(), glib::BoolError> {
        unsafe {
            let res: bool = from_glib(ffi::gst_video_frame_copy(&mut dest.frame, &self.frame));
            if res {
                Ok(())
            } else {
                Err(glib::bool_error!("Failed to copy video frame"))
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
            let res: bool = from_glib(ffi::gst_video_frame_copy_plane(
                &mut dest.frame,
                &self.frame,
                plane,
            ));
            if res {
                Ok(())
            } else {
                Err(glib::bool_error!("Failed to copy video frame plane"))
            }
        }
    }

    pub fn format(&self) -> crate::VideoFormat {
        self.info().format()
    }

    pub fn format_info(&self) -> crate::VideoFormatInfo {
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
        self.flags().contains(crate::VideoFrameFlags::INTERLACED)
    }

    pub fn is_tff(&self) -> bool {
        self.flags().contains(crate::VideoFrameFlags::TFF)
    }

    pub fn is_rff(&self) -> bool {
        self.flags().contains(crate::VideoFrameFlags::RFF)
    }

    pub fn is_onefield(&self) -> bool {
        self.flags().contains(crate::VideoFrameFlags::ONEFIELD)
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

    pub fn plane_data(&self, plane: u32) -> Result<&[u8], glib::BoolError> {
        if plane >= self.n_planes() {
            return Err(glib::bool_error!(
                "Plane index higher than number of planes"
            ));
        }

        let format_info = self.format_info();

        // Just get the palette
        if format_info.has_palette() && plane == 1 {
            unsafe {
                return Ok(slice::from_raw_parts(
                    self.frame.data[1] as *const u8,
                    256 * 4,
                ));
            }
        }

        let w = self.plane_stride()[plane as usize] as u32;
        // FIXME: This assumes that the horizontal subsampling of all
        // components in the plane is the same, which is probably safe
        let h = format_info.scale_height(plane as u8, self.height());

        unsafe {
            Ok(slice::from_raw_parts(
                self.frame.data[plane as usize] as *const u8,
                (w * h) as usize,
            ))
        }
    }

    pub fn as_ptr(&self) -> *const ffi::GstVideoFrame {
        &self.frame
    }
}

impl<'a> VideoFrameRef<&'a gst::BufferRef> {
    pub unsafe fn from_glib_borrow(frame: *const ffi::GstVideoFrame) -> Borrowed<Self> {
        assert!(!frame.is_null());

        let frame = ptr::read(frame);
        let info = crate::VideoInfo(ptr::read(&frame.info));
        let buffer = gst::BufferRef::from_ptr(frame.buffer);
        Borrowed::new(Self {
            frame,
            buffer: Some(buffer),
            info,
            unmap: false,
        })
    }

    pub unsafe fn from_glib_full(frame: ffi::GstVideoFrame) -> Self {
        let info = crate::VideoInfo(ptr::read(&frame.info));
        let buffer = gst::BufferRef::from_ptr(frame.buffer);
        Self {
            frame,
            buffer: Some(buffer),
            info,
            unmap: true,
        }
    }

    pub fn from_buffer_ref_readable<'b>(
        buffer: &'a gst::BufferRef,
        info: &'b crate::VideoInfo,
    ) -> Result<Self, glib::BoolError> {
        skip_assert_initialized!();

        assert!(info.is_valid());

        unsafe {
            let mut frame = mem::MaybeUninit::zeroed();
            let res: bool = from_glib(ffi::gst_video_frame_map(
                frame.as_mut_ptr(),
                info.to_glib_none().0 as *mut _,
                buffer.as_mut_ptr(),
                ffi::GST_VIDEO_FRAME_MAP_FLAG_NO_REF | gst::ffi::GST_MAP_READ,
            ));

            if !res {
                Err(glib::bool_error!("Failed to map VideoFrame"))
            } else {
                let frame = frame.assume_init();
                let info = crate::VideoInfo(ptr::read(&frame.info));
                Ok(Self {
                    frame,
                    buffer: Some(buffer),
                    info,
                    unmap: true,
                })
            }
        }
    }

    pub fn from_buffer_ref_id_readable<'b>(
        buffer: &'a gst::BufferRef,
        id: i32,
        info: &'b crate::VideoInfo,
    ) -> Result<Self, glib::BoolError> {
        skip_assert_initialized!();

        assert!(info.is_valid());

        unsafe {
            let mut frame = mem::MaybeUninit::zeroed();
            let res: bool = from_glib(ffi::gst_video_frame_map_id(
                frame.as_mut_ptr(),
                info.to_glib_none().0 as *mut _,
                buffer.as_mut_ptr(),
                id,
                ffi::GST_VIDEO_FRAME_MAP_FLAG_NO_REF | gst::ffi::GST_MAP_READ,
            ));

            if !res {
                Err(glib::bool_error!("Failed to map VideoFrame"))
            } else {
                let frame = frame.assume_init();
                let info = crate::VideoInfo(ptr::read(&frame.info));
                Ok(Self {
                    frame,
                    buffer: Some(buffer),
                    info,
                    unmap: true,
                })
            }
        }
    }

    pub fn buffer(&self) -> &gst::BufferRef {
        self.buffer.as_ref().unwrap()
    }
}

impl<'a> VideoFrameRef<&'a mut gst::BufferRef> {
    pub unsafe fn from_glib_borrow_mut(frame: *mut ffi::GstVideoFrame) -> Self {
        assert!(!frame.is_null());

        let frame = ptr::read(frame);
        let info = crate::VideoInfo(ptr::read(&frame.info));
        let buffer = gst::BufferRef::from_mut_ptr(frame.buffer);
        Self {
            frame,
            buffer: Some(buffer),
            info,
            unmap: false,
        }
    }

    pub unsafe fn from_glib_full_mut(frame: ffi::GstVideoFrame) -> Self {
        let info = crate::VideoInfo(ptr::read(&frame.info));
        let buffer = gst::BufferRef::from_mut_ptr(frame.buffer);
        Self {
            frame,
            buffer: Some(buffer),
            info,
            unmap: true,
        }
    }

    pub fn from_buffer_ref_writable<'b>(
        buffer: &'a mut gst::BufferRef,
        info: &'b crate::VideoInfo,
    ) -> Result<Self, glib::BoolError> {
        skip_assert_initialized!();

        assert!(info.is_valid());

        unsafe {
            let mut frame = mem::MaybeUninit::zeroed();
            let res: bool = from_glib(ffi::gst_video_frame_map(
                frame.as_mut_ptr(),
                info.to_glib_none().0 as *mut _,
                buffer.as_mut_ptr(),
                ffi::GST_VIDEO_FRAME_MAP_FLAG_NO_REF
                    | gst::ffi::GST_MAP_READ
                    | gst::ffi::GST_MAP_WRITE,
            ));

            if !res {
                Err(glib::bool_error!("Failed to map VideoFrame"))
            } else {
                let frame = frame.assume_init();
                let info = crate::VideoInfo(ptr::read(&frame.info));
                Ok(Self {
                    frame,
                    buffer: Some(buffer),
                    info,
                    unmap: true,
                })
            }
        }
    }

    pub fn from_buffer_ref_id_writable<'b>(
        buffer: &'a mut gst::BufferRef,
        id: i32,
        info: &'b crate::VideoInfo,
    ) -> Result<Self, glib::BoolError> {
        skip_assert_initialized!();

        assert!(info.is_valid());

        unsafe {
            let mut frame = mem::MaybeUninit::zeroed();
            let res: bool = from_glib(ffi::gst_video_frame_map_id(
                frame.as_mut_ptr(),
                info.to_glib_none().0 as *mut _,
                buffer.as_mut_ptr(),
                id,
                ffi::GST_VIDEO_FRAME_MAP_FLAG_NO_REF
                    | gst::ffi::GST_MAP_READ
                    | gst::ffi::GST_MAP_WRITE,
            ));

            if !res {
                Err(glib::bool_error!("Failed to map VideoFrame"))
            } else {
                let frame = frame.assume_init();
                let info = crate::VideoInfo(ptr::read(&frame.info));
                Ok(Self {
                    frame,
                    buffer: Some(buffer),
                    info,
                    unmap: true,
                })
            }
        }
    }

    pub fn buffer_mut(&mut self) -> &mut gst::BufferRef {
        self.buffer.as_mut().unwrap()
    }

    pub fn plane_data_mut(&mut self, plane: u32) -> Result<&mut [u8], glib::BoolError> {
        if plane >= self.n_planes() {
            return Err(glib::bool_error!(
                "Plane index higher than number of planes"
            ));
        }

        let format_info = self.format_info();

        // Just get the palette
        if format_info.has_palette() && plane == 1 {
            unsafe {
                return Ok(slice::from_raw_parts_mut(
                    self.frame.data[1] as *mut u8,
                    256 * 4,
                ));
            }
        }

        let w = self.plane_stride()[plane as usize] as u32;
        // FIXME: This assumes that the horizontal subsampling of all
        // components in the plane is the same, which is probably safe
        let h = format_info.scale_height(plane as u8, self.height());

        unsafe {
            Ok(slice::from_raw_parts_mut(
                self.frame.data[plane as usize] as *mut u8,
                (w * h) as usize,
            ))
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut ffi::GstVideoFrame {
        &mut self.frame
    }
}

impl<'a> ops::Deref for VideoFrameRef<&'a mut gst::BufferRef> {
    type Target = VideoFrameRef<&'a gst::BufferRef>;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self as *const Self::Target) }
    }
}

unsafe impl<T> Send for VideoFrameRef<T> {}
unsafe impl<T> Sync for VideoFrameRef<T> {}

impl<T> Drop for VideoFrameRef<T> {
    fn drop(&mut self) {
        unsafe {
            if self.unmap {
                ffi::gst_video_frame_unmap(&mut self.frame);
            }
        }
    }
}

pub trait VideoBufferExt {
    #[doc(alias = "get_video_flags")]
    fn video_flags(&self) -> crate::VideoBufferFlags;
    fn set_video_flags(&mut self, flags: crate::VideoBufferFlags);
    fn unset_video_flags(&mut self, flags: crate::VideoBufferFlags);
}

impl VideoBufferExt for gst::BufferRef {
    fn video_flags(&self) -> crate::VideoBufferFlags {
        unsafe {
            let ptr = self.as_mut_ptr();
            crate::VideoBufferFlags::from_bits_truncate((*ptr).mini_object.flags)
        }
    }

    fn set_video_flags(&mut self, flags: crate::VideoBufferFlags) {
        unsafe {
            let ptr = self.as_mut_ptr();
            (*ptr).mini_object.flags |= flags.bits();
        }
    }

    fn unset_video_flags(&mut self, flags: crate::VideoBufferFlags) {
        unsafe {
            let ptr = self.as_mut_ptr();
            (*ptr).mini_object.flags &= !flags.bits();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_read() {
        gst::init().unwrap();

        let info = crate::VideoInfo::builder(crate::VideoFormat::Gray8, 320, 240)
            .build()
            .unwrap();
        let buffer = gst::Buffer::with_size(info.size()).unwrap();
        let frame = VideoFrame::from_buffer_readable(buffer, &info).unwrap();

        assert!(frame.plane_data(0).is_ok());
        assert_eq!(frame.plane_data(0).unwrap().len(), 320 * 240);
        assert!(frame.plane_data(1).is_err());
        assert!(frame.info() == &info);

        {
            let frame = frame.as_video_frame_ref();

            assert!(frame.plane_data(0).is_ok());
            assert_eq!(frame.plane_data(0).unwrap().len(), 320 * 240);
            assert!(frame.plane_data(1).is_err());
            assert!(frame.info() == &info);
        }

        assert!(frame.plane_data(0).is_ok());
        assert_eq!(frame.plane_data(0).unwrap().len(), 320 * 240);
        assert!(frame.plane_data(1).is_err());
        assert!(frame.info() == &info);
    }

    #[test]
    fn test_map_write() {
        gst::init().unwrap();

        let info = crate::VideoInfo::builder(crate::VideoFormat::Gray8, 320, 240)
            .build()
            .unwrap();
        let buffer = gst::Buffer::with_size(info.size()).unwrap();
        let mut frame = VideoFrame::from_buffer_writable(buffer, &info).unwrap();

        assert!(frame.plane_data_mut(0).is_ok());
        assert_eq!(frame.plane_data_mut(0).unwrap().len(), 320 * 240);
        assert!(frame.plane_data_mut(1).is_err());
        assert!(frame.info() == &info);

        {
            let mut frame = frame.as_mut_video_frame_ref();

            assert!(frame.plane_data_mut(0).is_ok());
            assert_eq!(frame.plane_data_mut(0).unwrap().len(), 320 * 240);
            assert!(frame.plane_data_mut(1).is_err());
            assert!(frame.info() == &info);
        }

        assert!(frame.plane_data_mut(0).is_ok());
        assert_eq!(frame.plane_data_mut(0).unwrap().len(), 320 * 240);
        assert!(frame.plane_data_mut(1).is_err());
        assert!(frame.info() == &info);
    }

    #[test]
    fn test_map_ref_read() {
        gst::init().unwrap();

        let info = crate::VideoInfo::builder(crate::VideoFormat::Gray8, 320, 240)
            .build()
            .unwrap();
        let buffer = gst::Buffer::with_size(info.size()).unwrap();
        let frame = VideoFrameRef::from_buffer_ref_readable(&buffer, &info).unwrap();

        assert!(frame.plane_data(0).is_ok());
        assert_eq!(frame.plane_data(0).unwrap().len(), 320 * 240);
        assert!(frame.plane_data(1).is_err());
        assert!(frame.info() == &info);
    }

    #[test]
    fn test_map_ref_write() {
        gst::init().unwrap();

        let info = crate::VideoInfo::builder(crate::VideoFormat::Gray8, 320, 240)
            .build()
            .unwrap();
        let mut buffer = gst::Buffer::with_size(info.size()).unwrap();
        {
            let buffer = buffer.get_mut().unwrap();
            let mut frame = VideoFrameRef::from_buffer_ref_writable(buffer, &info).unwrap();

            assert!(frame.plane_data_mut(0).is_ok());
            assert_eq!(frame.plane_data_mut(0).unwrap().len(), 320 * 240);
            assert!(frame.plane_data_mut(1).is_err());
            assert!(frame.info() == &info);
        }
    }
}
