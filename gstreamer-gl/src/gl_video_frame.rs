// Take a look at the license at the top of the repository in the LICENSE file.

use std::{marker::PhantomData, mem, ptr};

use crate::GLMemoryRef;
use glib::translate::{from_glib, Borrowed, ToGlibPtr};
use gst_video::{video_frame::IsVideoFrame, VideoFrameExt};

pub enum Readable {}
pub enum Writable {}

// TODO: implement copy for videoframes. This would need to go through all the individual
//   memories and copy them. Some GL textures can be copied, others cannot.

pub trait IsGLVideoFrame: IsVideoFrame + Sized {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsGLVideoFrame> Sealed for T {}
}

pub trait GLVideoFrameExt: sealed::Sealed + IsGLVideoFrame {
    #[inline]
    fn memory(&self, idx: u32) -> Result<&GLMemoryRef, glib::BoolError> {
        if idx >= self.info().n_planes() {
            return Err(glib::bool_error!(
                "Memory index higher than number of memories"
            ));
        }

        unsafe {
            let ptr = self.as_raw().map[idx as usize].memory;
            if ffi::gst_is_gl_memory(ptr) == glib::ffi::GTRUE {
                Ok(GLMemoryRef::from_ptr(ptr as _))
            } else {
                Err(glib::bool_error!("Memory is not a GLMemory"))
            }
        }
    }

    #[inline]
    #[doc(alias = "get_texture_id")]
    fn texture_id(&self, idx: u32) -> Result<u32, glib::BoolError> {
        Ok(self.memory(idx)?.texture_id())
    }

    #[inline]
    #[doc(alias = "get_texture_format")]
    fn texture_format(&self, idx: u32) -> Result<crate::GLFormat, glib::BoolError> {
        Ok(self.memory(idx)?.texture_format())
    }

    #[inline]
    #[doc(alias = "get_texture_height")]
    fn texture_height(&self, idx: u32) -> Result<i32, glib::BoolError> {
        Ok(self.memory(idx)?.texture_height())
    }

    #[inline]
    #[doc(alias = "get_texture_target")]
    fn texture_target(&self, idx: u32) -> Result<crate::GLTextureTarget, glib::BoolError> {
        Ok(self.memory(idx)?.texture_target())
    }

    #[inline]
    #[doc(alias = "get_texture_width")]
    fn texture_width(&self, idx: u32) -> Result<i32, glib::BoolError> {
        Ok(self.memory(idx)?.texture_width())
    }
}

impl<O: IsGLVideoFrame> GLVideoFrameExt for O {}

pub struct GLVideoFrame<T> {
    frame: gst_video::ffi::GstVideoFrame,
    buffer: gst::Buffer,
    phantom: PhantomData<T>,
}

unsafe impl<T> Send for GLVideoFrame<T> {}
unsafe impl<T> Sync for GLVideoFrame<T> {}

// TODO implement Debug for GLVideoFrame

impl<T> IsVideoFrame for GLVideoFrame<T> {
    #[inline]
    fn as_raw(&self) -> &gst_video::ffi::GstVideoFrame {
        &self.frame
    }
}

impl<T> IsGLVideoFrame for GLVideoFrame<T> {}

impl<T> GLVideoFrame<T> {
    #[inline]
    pub fn into_buffer(self) -> gst::Buffer {
        unsafe {
            let mut s = mem::ManuallyDrop::new(self);
            let buffer = ptr::read(&s.buffer);
            gst_video::ffi::gst_video_frame_unmap(&mut s.frame);
            buffer
        }
    }

    #[inline]
    pub unsafe fn from_glib_full(frame: gst_video::ffi::GstVideoFrame) -> Self {
        let buffer = gst::Buffer::from_glib_none(frame.buffer);
        Self {
            frame,
            buffer,
            phantom: PhantomData,
        }
    }

    #[inline]
    pub fn into_raw(self) -> gst_video::ffi::GstVideoFrame {
        unsafe {
            let mut s = mem::ManuallyDrop::new(self);
            ptr::drop_in_place(&mut s.buffer);
            s.frame
        }
    }

    #[inline]
    pub fn as_video_frame_gl_ref(&self) -> GLVideoFrameRef<&gst::BufferRef> {
        let frame = unsafe { ptr::read(&self.frame) };
        GLVideoFrameRef {
            frame,
            unmap: false,
            phantom: PhantomData,
        }
    }
}

impl<T> Drop for GLVideoFrame<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            gst_video::ffi::gst_video_frame_unmap(&mut self.frame);
        }
    }
}

impl GLVideoFrame<Readable> {
    #[inline]
    pub fn from_buffer_readable(
        buffer: gst::Buffer,
        info: &gst_video::VideoInfo,
    ) -> Result<Self, gst::Buffer> {
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
                Ok(Self {
                    frame,
                    buffer,
                    phantom: PhantomData,
                })
            }
        }
    }
}

impl GLVideoFrame<Writable> {
    #[inline]
    pub fn from_buffer_writable(
        buffer: gst::Buffer,
        info: &gst_video::VideoInfo,
    ) -> Result<Self, gst::Buffer> {
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
                    | gst::ffi::GST_MAP_WRITE
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
                Ok(Self {
                    frame,
                    buffer,
                    phantom: PhantomData,
                })
            }
        }
    }

    #[inline]
    pub fn memory_mut(&self, idx: u32) -> Result<&mut GLMemoryRef, glib::BoolError> {
        unsafe { Ok(GLMemoryRef::from_mut_ptr(self.memory(idx)?.as_ptr() as _)) }
    }

    #[inline]
    pub fn buffer_mut(&mut self) -> &mut gst::BufferRef {
        unsafe { gst::BufferRef::from_mut_ptr(self.frame.buffer) }
    }
}

pub struct GLVideoFrameRef<T> {
    frame: gst_video::ffi::GstVideoFrame,
    unmap: bool,
    phantom: PhantomData<T>,
}

unsafe impl<T> Send for GLVideoFrameRef<T> {}
unsafe impl<T> Sync for GLVideoFrameRef<T> {}

impl<T> IsVideoFrame for GLVideoFrameRef<T> {
    #[inline]
    fn as_raw(&self) -> &gst_video::ffi::GstVideoFrame {
        &self.frame
    }
}

impl<T> IsGLVideoFrame for GLVideoFrameRef<T> {}

// TODO implement Debug for GLVideoFrameRef

impl<'a> GLVideoFrameRef<&'a gst::BufferRef> {
    #[inline]
    pub unsafe fn from_glib_borrow(frame: *const gst_video::ffi::GstVideoFrame) -> Borrowed<Self> {
        debug_assert!(!frame.is_null());

        let frame = ptr::read(frame);
        Borrowed::new(Self {
            frame,
            unmap: false,
            phantom: PhantomData,
        })
    }

    #[inline]
    pub unsafe fn from_glib_full(frame: gst_video::ffi::GstVideoFrame) -> Self {
        Self {
            frame,
            unmap: true,
            phantom: PhantomData,
        }
    }

    #[inline]
    pub fn from_buffer_ref_readable<'b>(
        buffer: &'a gst::BufferRef,
        info: &'b gst_video::VideoInfo,
    ) -> Result<GLVideoFrameRef<&'a gst::BufferRef>, glib::error::BoolError> {
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
                Ok(Self {
                    frame,
                    unmap: true,
                    phantom: PhantomData,
                })
            }
        }
    }
}

impl<'a> GLVideoFrameRef<&'a mut gst::BufferRef> {
    #[inline]
    pub unsafe fn from_glib_borrow_mut(frame: *mut gst_video::ffi::GstVideoFrame) -> Self {
        debug_assert!(!frame.is_null());

        let frame = ptr::read(frame);
        Self {
            frame,
            unmap: false,
            phantom: PhantomData,
        }
    }

    #[inline]
    pub unsafe fn from_glib_full_mut(frame: gst_video::ffi::GstVideoFrame) -> Self {
        Self {
            frame,
            unmap: true,
            phantom: PhantomData,
        }
    }

    #[inline]
    pub fn from_buffer_ref_writable<'b>(
        buffer: &'a mut gst::BufferRef,
        info: &'b gst_video::VideoInfo,
    ) -> Result<GLVideoFrameRef<&'a mut gst::BufferRef>, glib::error::BoolError> {
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
                    | gst::ffi::GST_MAP_WRITE
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

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut gst_video::ffi::GstVideoFrame {
        &mut self.frame
    }

    #[inline]
    pub fn memory_mut(&self, idx: u32) -> Result<&mut GLMemoryRef, glib::BoolError> {
        unsafe { Ok(GLMemoryRef::from_mut_ptr(self.memory(idx)?.as_ptr() as _)) }
    }
}

impl<'a> std::ops::Deref for GLVideoFrameRef<&'a mut gst::BufferRef> {
    type Target = GLVideoFrameRef<&'a gst::BufferRef>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self as *const Self::Target) }
    }
}

impl<T> Drop for GLVideoFrameRef<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            if self.unmap {
                gst_video::ffi::gst_video_frame_unmap(&mut self.frame);
            }
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
