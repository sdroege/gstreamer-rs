// Take a look at the license at the top of the repository in the LICENSE file.

use std::{fmt, marker::PhantomData, mem, ops, ptr, slice};

use glib::translate::{from_glib, from_glib_none, Borrowed, ToGlibPtr};

pub enum Readable {}
pub enum Writable {}

pub trait IsVideoFrame {
    fn as_raw(&self) -> &ffi::GstVideoFrame;
}

impl<T> IsVideoFrame for VideoFrame<T> {
    #[inline]
    fn as_raw(&self) -> &ffi::GstVideoFrame {
        &self.frame
    }
}

pub struct VideoFrame<T> {
    frame: ffi::GstVideoFrame,
    buffer: gst::Buffer,
    phantom: PhantomData<T>,
}

unsafe impl<T> Send for VideoFrame<T> {}
unsafe impl<T> Sync for VideoFrame<T> {}

impl<T> fmt::Debug for VideoFrame<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoFrame")
            .field("flags", &self.flags())
            .field("id", &self.id())
            .field("buffer", &self.buffer())
            .field("info", &self.info())
            .finish()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsVideoFrame> Sealed for T {}
}

pub trait VideoFrameExt: sealed::Sealed + IsVideoFrame {
    #[inline]
    fn as_ptr(&self) -> *const ffi::GstVideoFrame {
        self.as_raw() as _
    }

    #[inline]
    fn info(&self) -> &crate::VideoInfo {
        unsafe {
            let frame = self.as_raw();
            let info = &frame.info as *const ffi::GstVideoInfo as *const crate::VideoInfo;
            &*info
        }
    }

    #[inline]
    fn flags(&self) -> crate::VideoFrameFlags {
        unsafe { from_glib(self.as_raw().flags) }
    }

    #[inline]
    fn id(&self) -> i32 {
        self.as_raw().id
    }

    #[inline]
    fn buffer(&self) -> &gst::BufferRef {
        unsafe { gst::BufferRef::from_ptr(self.as_raw().buffer) }
    }

    #[inline]
    fn format(&self) -> crate::VideoFormat {
        self.info().format()
    }

    #[inline]
    fn format_info(&self) -> crate::VideoFormatInfo {
        self.info().format_info()
    }

    #[inline]
    fn width(&self) -> u32 {
        self.info().width()
    }

    #[inline]
    fn height(&self) -> u32 {
        self.info().height()
    }

    #[inline]
    fn size(&self) -> usize {
        self.info().size()
    }

    #[inline]
    fn is_interlaced(&self) -> bool {
        self.flags().contains(crate::VideoFrameFlags::INTERLACED)
    }

    #[inline]
    fn is_tff(&self) -> bool {
        self.flags().contains(crate::VideoFrameFlags::TFF)
    }

    #[inline]
    fn is_rff(&self) -> bool {
        self.flags().contains(crate::VideoFrameFlags::RFF)
    }

    #[inline]
    fn is_onefield(&self) -> bool {
        self.flags().contains(crate::VideoFrameFlags::ONEFIELD)
    }

    #[inline]
    fn is_bottom_field(&self) -> bool {
        self.flags().contains(crate::VideoFrameFlags::ONEFIELD)
            && !self.flags().contains(crate::VideoFrameFlags::TFF)
    }

    #[inline]
    fn is_top_field(&self) -> bool {
        self.flags().contains(crate::VideoFrameFlags::ONEFIELD)
            && self.flags().contains(crate::VideoFrameFlags::TFF)
    }

    #[inline]
    fn n_planes(&self) -> u32 {
        self.info().n_planes()
    }

    #[inline]
    fn n_components(&self) -> u32 {
        self.info().n_components()
    }

    #[inline]
    fn plane_stride(&self) -> &[i32] {
        self.info().stride()
    }

    #[inline]
    fn plane_offset(&self) -> &[usize] {
        self.info().offset()
    }

    #[inline]
    fn comp_depth(&self, component: u32) -> u32 {
        self.info().comp_depth(component as u8)
    }

    #[inline]
    fn comp_height(&self, component: u32) -> u32 {
        self.info().comp_height(component as u8)
    }

    #[inline]
    fn comp_width(&self, component: u32) -> u32 {
        self.info().comp_width(component as u8)
    }

    #[inline]
    fn comp_offset(&self, component: u32) -> usize {
        self.info().comp_offset(component as u8)
    }

    #[inline]
    fn comp_poffset(&self, component: u32) -> u32 {
        self.info().comp_poffset(component as u8)
    }

    #[inline]
    fn comp_pstride(&self, component: u32) -> i32 {
        self.info().comp_pstride(component as u8)
    }

    #[inline]
    fn comp_stride(&self, component: u32) -> i32 {
        self.info().comp_stride(component as u8)
    }

    #[inline]
    fn comp_plane(&self, component: u32) -> u32 {
        self.info().comp_plane(component as u8)
    }
}

impl<O: IsVideoFrame> VideoFrameExt for O {}

impl<T> VideoFrame<T> {
    #[inline]
    pub fn into_buffer(self) -> gst::Buffer {
        unsafe {
            let mut s = mem::ManuallyDrop::new(self);
            let buffer = ptr::read(&s.buffer);
            ffi::gst_video_frame_unmap(&mut s.frame);
            buffer
        }
    }

    #[doc(alias = "gst_video_frame_copy")]
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

    #[doc(alias = "gst_video_frame_copy_plane")]
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

    #[inline]
    pub fn comp_data(&self, component: u32) -> Result<&[u8], glib::BoolError> {
        let poffset = self.info().comp_poffset(component as u8) as usize;
        Ok(&self.plane_data(self.format_info().plane()[component as usize])?[poffset..])
    }

    #[inline]
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

        if w == 0 || h == 0 {
            return Ok(&[]);
        }

        unsafe {
            Ok(slice::from_raw_parts(
                self.frame.data[plane as usize] as *const u8,
                (w * h) as usize,
            ))
        }
    }

    #[inline]
    pub unsafe fn from_glib_full(frame: ffi::GstVideoFrame) -> Self {
        let buffer = gst::Buffer::from_glib_none(frame.buffer);
        Self {
            frame,
            buffer,
            phantom: PhantomData,
        }
    }

    #[inline]
    pub fn as_video_frame_ref(&self) -> VideoFrameRef<&gst::BufferRef> {
        let frame = unsafe { ptr::read(&self.frame) };
        VideoFrameRef {
            frame,
            unmap: false,
            phantom: PhantomData,
        }
    }

    #[inline]
    pub fn into_raw(self) -> ffi::GstVideoFrame {
        unsafe {
            let mut s = mem::ManuallyDrop::new(self);
            ptr::drop_in_place(&mut s.buffer);
            s.frame
        }
    }
}

impl<T> Drop for VideoFrame<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ffi::gst_video_frame_unmap(&mut self.frame);
        }
    }
}

impl VideoFrame<Readable> {
    #[inline]
    pub fn from_buffer_readable(
        buffer: gst::Buffer,
        info: &crate::VideoInfo,
    ) -> Result<Self, gst::Buffer> {
        skip_assert_initialized!();

        assert!(info.is_valid());

        unsafe {
            let mut frame = mem::MaybeUninit::uninit();
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
                Ok(Self {
                    frame,
                    buffer,
                    phantom: PhantomData,
                })
            }
        }
    }

    #[inline]
    pub fn from_buffer_id_readable(
        buffer: gst::Buffer,
        id: i32,
        info: &crate::VideoInfo,
    ) -> Result<Self, gst::Buffer> {
        skip_assert_initialized!();

        assert!(info.is_valid());

        unsafe {
            let mut frame = mem::MaybeUninit::uninit();
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
                Ok(Self {
                    frame,
                    buffer,
                    phantom: PhantomData,
                })
            }
        }
    }

    #[inline]
    pub fn buffer_owned(&self) -> gst::Buffer {
        unsafe { from_glib_none(self.frame.buffer) }
    }
}

impl VideoFrame<Writable> {
    #[inline]
    pub fn from_buffer_writable(
        buffer: gst::Buffer,
        info: &crate::VideoInfo,
    ) -> Result<Self, gst::Buffer> {
        skip_assert_initialized!();

        assert!(info.is_valid());

        unsafe {
            let mut frame = mem::MaybeUninit::uninit();
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
                Ok(Self {
                    frame,
                    buffer,
                    phantom: PhantomData,
                })
            }
        }
    }

    #[inline]
    pub fn from_buffer_id_writable(
        buffer: gst::Buffer,
        id: i32,
        info: &crate::VideoInfo,
    ) -> Result<Self, gst::Buffer> {
        skip_assert_initialized!();

        assert!(info.is_valid());

        unsafe {
            let mut frame = mem::MaybeUninit::uninit();
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
                Ok(Self {
                    frame,
                    buffer,
                    phantom: PhantomData,
                })
            }
        }
    }

    #[inline]
    pub fn buffer_mut(&mut self) -> &mut gst::BufferRef {
        unsafe { gst::BufferRef::from_mut_ptr(self.frame.buffer) }
    }

    pub fn comp_data_mut(&mut self, component: u32) -> Result<&mut [u8], glib::BoolError> {
        let poffset = self.info().comp_poffset(component as u8) as usize;
        Ok(&mut self.plane_data_mut(self.format_info().plane()[component as usize])?[poffset..])
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

        if w == 0 || h == 0 {
            return Ok(&mut []);
        }

        unsafe {
            Ok(slice::from_raw_parts_mut(
                self.frame.data[plane as usize] as *mut u8,
                (w * h) as usize,
            ))
        }
    }

    #[inline]
    pub fn as_mut_video_frame_ref(&mut self) -> VideoFrameRef<&mut gst::BufferRef> {
        let frame = unsafe { ptr::read(&self.frame) };
        VideoFrameRef {
            frame,
            unmap: false,
            phantom: PhantomData,
        }
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut ffi::GstVideoFrame {
        &mut self.frame
    }
}

pub struct VideoFrameRef<T> {
    frame: ffi::GstVideoFrame,
    unmap: bool,
    phantom: PhantomData<T>,
}

impl<T> IsVideoFrame for VideoFrameRef<T> {
    #[inline]
    fn as_raw(&self) -> &ffi::GstVideoFrame {
        &self.frame
    }
}

impl<T> fmt::Debug for VideoFrameRef<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoFrameRef")
            .field("flags", &self.flags())
            .field("id", &self.id())
            .field("buffer", &unsafe {
                gst::BufferRef::from_ptr(self.frame.buffer)
            })
            .field("info", &self.info())
            .finish()
    }
}

impl<T> VideoFrameRef<T> {
    #[doc(alias = "gst_video_frame_copy")]
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

    #[doc(alias = "gst_video_frame_copy_plane")]
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

    pub fn comp_data(&self, component: u32) -> Result<&[u8], glib::BoolError> {
        let poffset = self.info().comp_poffset(component as u8) as usize;
        Ok(&self.plane_data(self.format_info().plane()[component as usize])?[poffset..])
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

        if w == 0 || h == 0 {
            return Ok(&[]);
        }

        unsafe {
            Ok(slice::from_raw_parts(
                self.frame.data[plane as usize] as *const u8,
                (w * h) as usize,
            ))
        }
    }
}

impl<'a> VideoFrameRef<&'a gst::BufferRef> {
    #[inline]
    pub unsafe fn from_glib_borrow(frame: *const ffi::GstVideoFrame) -> Borrowed<Self> {
        debug_assert!(!frame.is_null());

        let frame = ptr::read(frame);
        Borrowed::new(Self {
            frame,
            unmap: false,
            phantom: PhantomData,
        })
    }

    #[inline]
    pub unsafe fn from_glib_full(frame: ffi::GstVideoFrame) -> Self {
        Self {
            frame,
            unmap: true,
            phantom: PhantomData,
        }
    }

    #[inline]
    pub fn from_buffer_ref_readable<'b>(
        buffer: &'a gst::BufferRef,
        info: &'b crate::VideoInfo,
    ) -> Result<Self, glib::BoolError> {
        skip_assert_initialized!();

        assert!(info.is_valid());

        unsafe {
            let mut frame = mem::MaybeUninit::uninit();
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
                Ok(Self {
                    frame,
                    unmap: true,
                    phantom: PhantomData,
                })
            }
        }
    }

    #[inline]
    pub fn from_buffer_ref_id_readable<'b>(
        buffer: &'a gst::BufferRef,
        id: i32,
        info: &'b crate::VideoInfo,
    ) -> Result<Self, glib::BoolError> {
        skip_assert_initialized!();

        assert!(info.is_valid());

        unsafe {
            let mut frame = mem::MaybeUninit::uninit();
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
                Ok(Self {
                    frame,
                    unmap: true,
                    phantom: PhantomData,
                })
            }
        }
    }
}

impl<'a> VideoFrameRef<&'a mut gst::BufferRef> {
    #[inline]
    pub unsafe fn from_glib_borrow_mut(frame: *mut ffi::GstVideoFrame) -> Self {
        debug_assert!(!frame.is_null());

        let frame = ptr::read(frame);
        Self {
            frame,
            unmap: false,
            phantom: PhantomData,
        }
    }

    #[inline]
    pub unsafe fn from_glib_full_mut(frame: ffi::GstVideoFrame) -> Self {
        Self {
            frame,
            unmap: true,
            phantom: PhantomData,
        }
    }

    #[inline]
    pub fn from_buffer_ref_writable<'b>(
        buffer: &'a mut gst::BufferRef,
        info: &'b crate::VideoInfo,
    ) -> Result<Self, glib::BoolError> {
        skip_assert_initialized!();

        assert!(info.is_valid());

        unsafe {
            let mut frame = mem::MaybeUninit::uninit();
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
                Ok(Self {
                    frame,
                    unmap: true,
                    phantom: PhantomData,
                })
            }
        }
    }

    #[inline]
    pub fn from_buffer_ref_id_writable<'b>(
        buffer: &'a mut gst::BufferRef,
        id: i32,
        info: &'b crate::VideoInfo,
    ) -> Result<Self, glib::BoolError> {
        skip_assert_initialized!();

        assert!(info.is_valid());

        unsafe {
            let mut frame = mem::MaybeUninit::uninit();
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
                Ok(Self {
                    frame,
                    unmap: true,
                    phantom: PhantomData,
                })
            }
        }
    }

    #[inline]
    pub fn buffer_mut(&mut self) -> &mut gst::BufferRef {
        unsafe { gst::BufferRef::from_mut_ptr(self.frame.buffer) }
    }

    pub fn comp_data_mut(&mut self, component: u32) -> Result<&mut [u8], glib::BoolError> {
        let poffset = self.info().comp_poffset(component as u8) as usize;
        Ok(&mut self.plane_data_mut(self.format_info().plane()[component as usize])?[poffset..])
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

        if w == 0 || h == 0 {
            return Ok(&mut []);
        }

        unsafe {
            Ok(slice::from_raw_parts_mut(
                self.frame.data[plane as usize] as *mut u8,
                (w * h) as usize,
            ))
        }
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut ffi::GstVideoFrame {
        &mut self.frame
    }
}

impl<'a> ops::Deref for VideoFrameRef<&'a mut gst::BufferRef> {
    type Target = VideoFrameRef<&'a gst::BufferRef>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self as *const Self::Target) }
    }
}

unsafe impl<T> Send for VideoFrameRef<T> {}
unsafe impl<T> Sync for VideoFrameRef<T> {}

impl<T> Drop for VideoFrameRef<T> {
    #[inline]
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
    #[inline]
    fn video_flags(&self) -> crate::VideoBufferFlags {
        unsafe {
            let ptr = self.as_mut_ptr();
            crate::VideoBufferFlags::from_bits_truncate((*ptr).mini_object.flags)
        }
    }

    #[inline]
    fn set_video_flags(&mut self, flags: crate::VideoBufferFlags) {
        unsafe {
            let ptr = self.as_mut_ptr();
            (*ptr).mini_object.flags |= flags.bits();
        }
    }

    #[inline]
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
