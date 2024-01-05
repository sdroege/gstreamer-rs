// Take a look at the license at the top of the repository in the LICENSE file.

use std::{fmt, marker::PhantomData, mem, ops, ptr, slice};

use glib::translate::*;

use smallvec::SmallVec;

pub enum Readable {}
pub enum Writable {}

pub struct AudioBuffer<T> {
    // Has to be boxed because it contains self-references
    audio_buffer: Box<ffi::GstAudioBuffer>,
    buffer: gst::Buffer,
    phantom: PhantomData<T>,
}

unsafe impl<T> Send for AudioBuffer<T> {}
unsafe impl<T> Sync for AudioBuffer<T> {}

impl<T> fmt::Debug for AudioBuffer<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AudioBuffer")
            .field("n_samples", &self.n_samples())
            .field("n_planes", &self.n_planes())
            .field("buffer", &self.buffer())
            .field("info", &self.info())
            .finish()
    }
}

impl<T> AudioBuffer<T> {
    #[inline]
    pub fn info(&self) -> &crate::AudioInfo {
        unsafe {
            &*(&self.audio_buffer.info as *const ffi::GstAudioInfo as *const crate::AudioInfo)
        }
    }

    #[inline]
    pub fn into_buffer(self) -> gst::Buffer {
        unsafe {
            let mut s = mem::ManuallyDrop::new(self);
            let buffer = ptr::read(&s.buffer);
            ffi::gst_audio_buffer_unmap(&mut *s.audio_buffer);
            ptr::drop_in_place(&mut s.audio_buffer);

            buffer
        }
    }

    #[inline]
    pub fn format(&self) -> crate::AudioFormat {
        self.info().format()
    }

    #[inline]
    pub fn format_info(&self) -> crate::AudioFormatInfo {
        self.info().format_info()
    }

    #[inline]
    pub fn channels(&self) -> u32 {
        self.info().channels()
    }

    #[inline]
    pub fn rate(&self) -> u32 {
        self.info().rate()
    }

    #[inline]
    pub fn layout(&self) -> crate::AudioLayout {
        self.info().layout()
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.info().width()
    }

    #[inline]
    pub fn depth(&self) -> u32 {
        self.info().depth()
    }

    #[inline]
    pub fn sample_stride(&self) -> u32 {
        self.info().width() / 8
    }

    #[inline]
    pub fn bps(&self) -> u32 {
        self.info().bps()
    }

    #[inline]
    pub fn bpf(&self) -> u32 {
        self.info().bpf()
    }

    #[inline]
    pub fn n_samples(&self) -> usize {
        self.audio_buffer.n_samples
    }

    #[inline]
    pub fn n_planes(&self) -> u32 {
        self.audio_buffer.n_planes as u32
    }

    #[inline]
    pub fn plane_size(&self) -> usize {
        (self.n_samples() * self.sample_stride() as usize * self.channels() as usize)
            / self.n_planes() as usize
    }

    #[inline]
    pub fn buffer(&self) -> &gst::BufferRef {
        unsafe { gst::BufferRef::from_ptr(self.audio_buffer.buffer) }
    }

    pub fn plane_data(&self, plane: u32) -> Result<&[u8], glib::BoolError> {
        if plane >= self.n_planes() {
            return Err(glib::bool_error!(
                "Plane index higher than number of planes"
            ));
        }

        unsafe {
            Ok(slice::from_raw_parts(
                (*self.audio_buffer.planes.add(plane as usize)) as *const u8,
                self.plane_size(),
            ))
        }
    }

    pub fn planes_data(&self) -> SmallVec<[&[u8]; 8]> {
        let mut planes = SmallVec::default();

        for plane in 0..self.n_planes() {
            planes[plane as usize] = self.plane_data(plane).unwrap();
        }

        planes
    }

    #[inline]
    pub fn as_audio_buffer_ref(&self) -> AudioBufferRef<&gst::BufferRef> {
        AudioBufferRef {
            audio_buffer: AudioBufferPtr::Borrowed(ptr::NonNull::from(&*self.audio_buffer)),
            unmap: false,
            phantom: PhantomData,
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const ffi::GstAudioBuffer {
        &*self.audio_buffer
    }
}

impl<T> Drop for AudioBuffer<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ffi::gst_audio_buffer_unmap(&mut *self.audio_buffer);
        }
    }
}

impl AudioBuffer<Readable> {
    #[inline]
    pub fn from_buffer_readable(
        buffer: gst::Buffer,
        info: &crate::AudioInfo,
    ) -> Result<Self, gst::Buffer> {
        skip_assert_initialized!();

        assert!(info.is_valid());

        unsafe {
            let mut audio_buffer = Box::new(mem::MaybeUninit::zeroed().assume_init());
            let res: bool = from_glib(ffi::gst_audio_buffer_map(
                &mut *audio_buffer,
                info.to_glib_none().0 as *mut _,
                buffer.to_glib_none().0,
                gst::ffi::GST_MAP_READ,
            ));

            if !res {
                Err(buffer)
            } else {
                Ok(Self {
                    audio_buffer,
                    buffer,
                    phantom: PhantomData,
                })
            }
        }
    }

    #[inline]
    pub fn buffer_owned(&self) -> gst::Buffer {
        unsafe { from_glib_none(self.audio_buffer.buffer) }
    }
}

impl AudioBuffer<Writable> {
    #[inline]
    pub fn from_buffer_writable(
        buffer: gst::Buffer,
        info: &crate::AudioInfo,
    ) -> Result<Self, gst::Buffer> {
        skip_assert_initialized!();

        assert!(info.is_valid());

        unsafe {
            let mut audio_buffer = Box::new(mem::MaybeUninit::zeroed().assume_init());
            let res: bool = from_glib(ffi::gst_audio_buffer_map(
                &mut *audio_buffer,
                info.to_glib_none().0 as *mut _,
                buffer.to_glib_none().0,
                gst::ffi::GST_MAP_READ | gst::ffi::GST_MAP_WRITE,
            ));

            if !res {
                Err(buffer)
            } else {
                Ok(Self {
                    audio_buffer,
                    buffer,
                    phantom: PhantomData,
                })
            }
        }
    }

    pub fn plane_data_mut(&mut self, plane: u32) -> Result<&mut [u8], glib::BoolError> {
        if plane >= self.n_planes() {
            return Err(glib::bool_error!(
                "Plane index higher than number of planes"
            ));
        }

        unsafe {
            Ok(slice::from_raw_parts_mut(
                (*self.audio_buffer.planes.add(plane as usize)) as *mut u8,
                self.plane_size(),
            ))
        }
    }

    pub fn planes_data_mut(&mut self) -> SmallVec<[&mut [u8]; 8]> {
        let mut planes = SmallVec::default();

        unsafe {
            for plane in 0..self.n_planes() {
                let slice = self.plane_data_mut(plane).unwrap();
                planes.push(slice::from_raw_parts_mut(slice.as_mut_ptr(), slice.len()));
            }
        }

        planes
    }

    #[inline]
    pub fn as_mut_audio_buffer_ref(&mut self) -> AudioBufferRef<&mut gst::BufferRef> {
        AudioBufferRef {
            audio_buffer: AudioBufferPtr::Borrowed(ptr::NonNull::from(&mut *self.audio_buffer)),
            unmap: false,
            phantom: PhantomData,
        }
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut ffi::GstAudioBuffer {
        &mut *self.audio_buffer
    }
}

#[derive(Debug)]
enum AudioBufferPtr {
    Owned(Box<ffi::GstAudioBuffer>),
    Borrowed(ptr::NonNull<ffi::GstAudioBuffer>),
}

impl ops::Deref for AudioBufferPtr {
    type Target = ffi::GstAudioBuffer;

    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Owned(ref b) => b,
            Self::Borrowed(ref b) => unsafe { b.as_ref() },
        }
    }
}

impl ops::DerefMut for AudioBufferPtr {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Owned(ref mut b) => &mut *b,
            Self::Borrowed(ref mut b) => unsafe { b.as_mut() },
        }
    }
}

pub struct AudioBufferRef<T> {
    // Has to be boxed because it contains self-references
    audio_buffer: AudioBufferPtr,
    unmap: bool,
    phantom: PhantomData<T>,
}

impl<T> fmt::Debug for AudioBufferRef<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AudioBufferRef")
            .field("n_samples", &self.n_samples())
            .field("n_planes", &self.n_planes())
            .field("buffer", &unsafe {
                gst::BufferRef::from_ptr(self.audio_buffer.buffer)
            })
            .field("info", &self.info())
            .finish()
    }
}

impl<T> AudioBufferRef<T> {
    #[inline]
    pub fn info(&self) -> &crate::AudioInfo {
        unsafe {
            &*(&self.audio_buffer.info as *const ffi::GstAudioInfo as *const crate::AudioInfo)
        }
    }

    #[inline]
    pub fn format(&self) -> crate::AudioFormat {
        self.info().format()
    }

    #[inline]
    pub fn format_info(&self) -> crate::AudioFormatInfo {
        self.info().format_info()
    }

    #[inline]
    pub fn channels(&self) -> u32 {
        self.info().channels()
    }

    #[inline]
    pub fn rate(&self) -> u32 {
        self.info().rate()
    }

    #[inline]
    pub fn layout(&self) -> crate::AudioLayout {
        self.info().layout()
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.info().width()
    }

    #[inline]
    pub fn depth(&self) -> u32 {
        self.info().depth()
    }

    #[inline]
    pub fn sample_stride(&self) -> u32 {
        self.info().width() / 8
    }

    #[inline]
    pub fn bps(&self) -> u32 {
        self.info().bps()
    }

    #[inline]
    pub fn bpf(&self) -> u32 {
        self.info().bpf()
    }

    #[inline]
    pub fn n_samples(&self) -> usize {
        self.audio_buffer.n_samples
    }

    #[inline]
    pub fn n_planes(&self) -> u32 {
        self.audio_buffer.n_planes as u32
    }

    #[inline]
    pub fn plane_size(&self) -> usize {
        (self.n_samples() * self.sample_stride() as usize * self.channels() as usize)
            / self.n_planes() as usize
    }

    pub fn plane_data(&self, plane: u32) -> Result<&[u8], glib::BoolError> {
        if plane >= self.n_planes() {
            return Err(glib::bool_error!(
                "Plane index higher than number of planes"
            ));
        }

        if self.plane_size() == 0 {
            return Ok(&[]);
        }

        unsafe {
            Ok(slice::from_raw_parts(
                (*self.audio_buffer.planes.add(plane as usize)) as *const u8,
                self.plane_size(),
            ))
        }
    }

    pub fn planes_data(&self) -> SmallVec<[&[u8]; 8]> {
        let mut planes = SmallVec::default();

        for plane in 0..self.n_planes() {
            planes[plane as usize] = self.plane_data(plane).unwrap();
        }

        planes
    }

    #[inline]
    pub fn as_ptr(&self) -> *const ffi::GstAudioBuffer {
        &*self.audio_buffer
    }
}

impl<'a> AudioBufferRef<&'a gst::BufferRef> {
    #[inline]
    pub unsafe fn from_glib_borrow(audio_buffer: *const ffi::GstAudioBuffer) -> Borrowed<Self> {
        debug_assert!(!audio_buffer.is_null());

        Borrowed::new(Self {
            audio_buffer: AudioBufferPtr::Borrowed(ptr::NonNull::new_unchecked(
                audio_buffer as *mut _,
            )),
            unmap: false,
            phantom: PhantomData,
        })
    }

    #[inline]
    pub fn from_buffer_ref_readable<'b>(
        buffer: &'a gst::BufferRef,
        info: &'b crate::AudioInfo,
    ) -> Result<Self, glib::BoolError> {
        skip_assert_initialized!();

        assert!(info.is_valid());

        unsafe {
            let mut audio_buffer = Box::new(mem::MaybeUninit::zeroed().assume_init());
            let res: bool = from_glib(ffi::gst_audio_buffer_map(
                &mut *audio_buffer,
                info.to_glib_none().0 as *mut _,
                buffer.as_mut_ptr(),
                gst::ffi::GST_MAP_READ,
            ));

            if !res {
                Err(glib::bool_error!("Failed to map AudioBuffer"))
            } else {
                Ok(Self {
                    audio_buffer: AudioBufferPtr::Owned(audio_buffer),
                    unmap: true,
                    phantom: PhantomData,
                })
            }
        }
    }

    #[inline]
    pub fn buffer(&self) -> &gst::BufferRef {
        unsafe { gst::BufferRef::from_ptr(self.audio_buffer.buffer) }
    }
}

impl<'a> AudioBufferRef<&'a mut gst::BufferRef> {
    #[inline]
    pub unsafe fn from_glib_borrow_mut(audio_buffer: *mut ffi::GstAudioBuffer) -> Borrowed<Self> {
        debug_assert!(!audio_buffer.is_null());

        Borrowed::new(Self {
            audio_buffer: AudioBufferPtr::Borrowed(ptr::NonNull::new_unchecked(audio_buffer)),
            unmap: false,
            phantom: PhantomData,
        })
    }

    #[inline]
    pub fn from_buffer_ref_writable<'b>(
        buffer: &'a mut gst::BufferRef,
        info: &'b crate::AudioInfo,
    ) -> Result<Self, glib::BoolError> {
        skip_assert_initialized!();

        assert!(info.is_valid());

        unsafe {
            let mut audio_buffer = Box::new(mem::MaybeUninit::zeroed().assume_init());
            let res: bool = from_glib(ffi::gst_audio_buffer_map(
                &mut *audio_buffer,
                info.to_glib_none().0 as *mut _,
                buffer.as_mut_ptr(),
                gst::ffi::GST_MAP_READ | gst::ffi::GST_MAP_WRITE,
            ));

            if !res {
                Err(glib::bool_error!("Failed to map AudioBuffer"))
            } else {
                Ok(Self {
                    audio_buffer: AudioBufferPtr::Owned(audio_buffer),
                    unmap: true,
                    phantom: PhantomData,
                })
            }
        }
    }

    #[inline]
    pub fn plane_data_mut(&mut self, plane: u32) -> Result<&mut [u8], glib::BoolError> {
        if plane >= self.n_planes() {
            return Err(glib::bool_error!(
                "Plane index higher than number of planes"
            ));
        }

        if self.plane_size() == 0 {
            return Ok(&mut []);
        }

        unsafe {
            Ok(slice::from_raw_parts_mut(
                (*self.audio_buffer.planes.add(plane as usize)) as *mut u8,
                self.plane_size(),
            ))
        }
    }

    pub fn planes_data_mut(&mut self) -> SmallVec<[&mut [u8]; 8]> {
        let mut planes = SmallVec::default();

        unsafe {
            for plane in 0..self.n_planes() {
                let slice = self.plane_data_mut(plane).unwrap();
                planes.push(slice::from_raw_parts_mut(slice.as_mut_ptr(), slice.len()));
            }
        }

        planes
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut ffi::GstAudioBuffer {
        &mut *self.audio_buffer
    }
}

impl<'a> ops::Deref for AudioBufferRef<&'a mut gst::BufferRef> {
    type Target = AudioBufferRef<&'a gst::BufferRef>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self as *const Self::Target) }
    }
}

unsafe impl<T> Send for AudioBufferRef<T> {}
unsafe impl<T> Sync for AudioBufferRef<T> {}

impl<T> Drop for AudioBufferRef<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            if self.unmap {
                ffi::gst_audio_buffer_unmap(&mut *self.audio_buffer);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_read() {
        gst::init().unwrap();

        let info = crate::AudioInfo::builder(crate::AUDIO_FORMAT_S16, 48000, 2)
            .build()
            .unwrap();
        let buffer = gst::Buffer::with_size(info.rate() as usize * info.bpf() as usize).unwrap();
        let buffer = AudioBuffer::from_buffer_readable(buffer, &info).unwrap();

        assert!(buffer.plane_data(0).is_ok());
        assert_eq!(buffer.plane_data(0).unwrap().len(), 2 * 2 * 48000);
        assert!(buffer.plane_data(1).is_err());
        assert!(buffer.info() == &info);

        {
            let buffer = buffer.as_audio_buffer_ref();

            assert!(buffer.plane_data(0).is_ok());
            assert_eq!(buffer.plane_data(0).unwrap().len(), 2 * 2 * 48000);
            assert!(buffer.plane_data(1).is_err());
            assert!(buffer.info() == &info);
        }

        assert!(buffer.plane_data(0).is_ok());
        assert_eq!(buffer.plane_data(0).unwrap().len(), 2 * 2 * 48000);
        assert!(buffer.plane_data(1).is_err());
        assert!(buffer.info() == &info);
    }

    #[test]
    fn test_map_read_planar() {
        gst::init().unwrap();

        let info = crate::AudioInfo::builder(crate::AUDIO_FORMAT_S16, 48000, 2)
            .layout(crate::AudioLayout::NonInterleaved)
            .build()
            .unwrap();
        let mut buffer =
            gst::Buffer::with_size(info.rate() as usize * info.bpf() as usize).unwrap();
        {
            let buffer = buffer.get_mut().unwrap();
            crate::AudioMeta::add(buffer, &info, 48000, &[]).unwrap();
        }
        let buffer = AudioBuffer::from_buffer_readable(buffer, &info).unwrap();

        assert!(buffer.plane_data(0).is_ok());
        assert_eq!(buffer.plane_data(0).unwrap().len(), 2 * 48000);
        assert!(buffer.plane_data(1).is_ok());
        assert_eq!(buffer.plane_data(1).unwrap().len(), 2 * 48000);
        assert!(buffer.info() == &info);

        {
            let buffer = buffer.as_audio_buffer_ref();

            assert!(buffer.plane_data(0).is_ok());
            assert_eq!(buffer.plane_data(0).unwrap().len(), 2 * 48000);
            assert!(buffer.plane_data(1).is_ok());
            assert_eq!(buffer.plane_data(1).unwrap().len(), 2 * 48000);
            assert!(buffer.info() == &info);
        }

        assert!(buffer.plane_data(0).is_ok());
        assert_eq!(buffer.plane_data(0).unwrap().len(), 2 * 48000);
        assert!(buffer.plane_data(1).is_ok());
        assert_eq!(buffer.plane_data(1).unwrap().len(), 2 * 48000);
        assert!(buffer.info() == &info);
    }

    #[test]
    fn test_map_write() {
        gst::init().unwrap();

        let info = crate::AudioInfo::builder(crate::AUDIO_FORMAT_S16, 48000, 2)
            .build()
            .unwrap();
        let buffer = gst::Buffer::with_size(info.rate() as usize * info.bpf() as usize).unwrap();
        let mut buffer = AudioBuffer::from_buffer_writable(buffer, &info).unwrap();

        assert!(buffer.plane_data_mut(0).is_ok());
        assert_eq!(buffer.plane_data_mut(0).unwrap().len(), 2 * 2 * 48000);
        assert!(buffer.plane_data_mut(1).is_err());
        assert!(buffer.info() == &info);

        {
            let mut buffer = buffer.as_mut_audio_buffer_ref();

            assert!(buffer.plane_data_mut(0).is_ok());
            assert_eq!(buffer.plane_data_mut(0).unwrap().len(), 2 * 2 * 48000);
            assert!(buffer.plane_data_mut(1).is_err());
            assert!(buffer.info() == &info);
        }

        assert!(buffer.plane_data_mut(0).is_ok());
        assert_eq!(buffer.plane_data_mut(0).unwrap().len(), 2 * 2 * 48000);
        assert!(buffer.plane_data_mut(1).is_err());
        assert!(buffer.info() == &info);
    }

    #[test]
    fn test_map_write_planar() {
        gst::init().unwrap();

        let info = crate::AudioInfo::builder(crate::AUDIO_FORMAT_S16, 48000, 2)
            .layout(crate::AudioLayout::NonInterleaved)
            .build()
            .unwrap();
        let mut buffer =
            gst::Buffer::with_size(info.rate() as usize * info.bpf() as usize).unwrap();
        {
            let buffer = buffer.get_mut().unwrap();
            crate::AudioMeta::add(buffer, &info, 48000, &[]).unwrap();
        }
        let mut buffer = AudioBuffer::from_buffer_writable(buffer, &info).unwrap();

        assert!(buffer.plane_data_mut(0).is_ok());
        assert_eq!(buffer.plane_data_mut(0).unwrap().len(), 2 * 48000);
        assert!(buffer.plane_data_mut(1).is_ok());
        assert_eq!(buffer.plane_data_mut(1).unwrap().len(), 2 * 48000);
        assert!(buffer.info() == &info);

        {
            let mut buffer = buffer.as_mut_audio_buffer_ref();

            assert!(buffer.plane_data_mut(0).is_ok());
            assert_eq!(buffer.plane_data_mut(0).unwrap().len(), 2 * 48000);
            assert!(buffer.plane_data_mut(1).is_ok());
            assert_eq!(buffer.plane_data_mut(1).unwrap().len(), 2 * 48000);
            assert!(buffer.info() == &info);
        }

        assert!(buffer.plane_data_mut(0).is_ok());
        assert_eq!(buffer.plane_data_mut(0).unwrap().len(), 2 * 48000);
        assert!(buffer.plane_data_mut(1).is_ok());
        assert_eq!(buffer.plane_data_mut(1).unwrap().len(), 2 * 48000);
        assert!(buffer.info() == &info);
    }

    #[test]
    fn test_map_ref_read() {
        gst::init().unwrap();

        let info = crate::AudioInfo::builder(crate::AUDIO_FORMAT_S16, 48000, 2)
            .build()
            .unwrap();
        let buffer = gst::Buffer::with_size(info.rate() as usize * info.bpf() as usize).unwrap();
        let buffer = AudioBufferRef::from_buffer_ref_readable(&buffer, &info).unwrap();

        assert!(buffer.plane_data(0).is_ok());
        assert_eq!(buffer.plane_data(0).unwrap().len(), 2 * 2 * 48000);
        assert!(buffer.plane_data(1).is_err());
        assert!(buffer.info() == &info);

        assert!(buffer.plane_data(0).is_ok());
        assert_eq!(buffer.plane_data(0).unwrap().len(), 2 * 2 * 48000);
        assert!(buffer.plane_data(1).is_err());
        assert!(buffer.info() == &info);
    }

    #[test]
    fn test_map_ref_read_planar() {
        gst::init().unwrap();

        let info = crate::AudioInfo::builder(crate::AUDIO_FORMAT_S16, 48000, 2)
            .layout(crate::AudioLayout::NonInterleaved)
            .build()
            .unwrap();
        let mut buffer =
            gst::Buffer::with_size(info.rate() as usize * info.bpf() as usize).unwrap();
        {
            let buffer = buffer.get_mut().unwrap();
            crate::AudioMeta::add(buffer, &info, 48000, &[]).unwrap();
        }
        let buffer = AudioBufferRef::from_buffer_ref_readable(&buffer, &info).unwrap();

        assert!(buffer.plane_data(0).is_ok());
        assert_eq!(buffer.plane_data(0).unwrap().len(), 2 * 48000);
        assert!(buffer.plane_data(1).is_ok());
        assert_eq!(buffer.plane_data(1).unwrap().len(), 2 * 48000);
        assert!(buffer.info() == &info);

        assert!(buffer.plane_data(0).is_ok());
        assert_eq!(buffer.plane_data(0).unwrap().len(), 2 * 48000);
        assert!(buffer.plane_data(1).is_ok());
        assert_eq!(buffer.plane_data(1).unwrap().len(), 2 * 48000);
        assert!(buffer.info() == &info);
    }

    #[test]
    fn test_map_ref_write() {
        gst::init().unwrap();

        let info = crate::AudioInfo::builder(crate::AUDIO_FORMAT_S16, 48000, 2)
            .build()
            .unwrap();
        let mut buffer =
            gst::Buffer::with_size(info.rate() as usize * info.bpf() as usize).unwrap();

        {
            let buffer = buffer.get_mut().unwrap();
            let mut buffer = AudioBufferRef::from_buffer_ref_writable(buffer, &info).unwrap();

            assert!(buffer.plane_data_mut(0).is_ok());
            assert_eq!(buffer.plane_data_mut(0).unwrap().len(), 2 * 2 * 48000);
            assert!(buffer.plane_data_mut(1).is_err());
            assert!(buffer.info() == &info);

            assert!(buffer.plane_data_mut(0).is_ok());
            assert_eq!(buffer.plane_data_mut(0).unwrap().len(), 2 * 2 * 48000);
            assert!(buffer.plane_data_mut(1).is_err());
            assert!(buffer.info() == &info);
        }
    }

    #[test]
    fn test_map_ref_write_planar() {
        gst::init().unwrap();

        let info = crate::AudioInfo::builder(crate::AUDIO_FORMAT_S16, 48000, 2)
            .layout(crate::AudioLayout::NonInterleaved)
            .build()
            .unwrap();
        let mut buffer =
            gst::Buffer::with_size(info.rate() as usize * info.bpf() as usize).unwrap();
        {
            let buffer = buffer.get_mut().unwrap();
            crate::AudioMeta::add(buffer, &info, 48000, &[]).unwrap();
        }

        {
            let buffer = buffer.get_mut().unwrap();
            let mut buffer = AudioBufferRef::from_buffer_ref_writable(buffer, &info).unwrap();

            assert!(buffer.plane_data_mut(0).is_ok());
            assert_eq!(buffer.plane_data_mut(0).unwrap().len(), 2 * 48000);
            assert!(buffer.plane_data_mut(1).is_ok());
            assert_eq!(buffer.plane_data_mut(1).unwrap().len(), 2 * 48000);
            assert!(buffer.info() == &info);

            assert!(buffer.plane_data_mut(0).is_ok());
            assert_eq!(buffer.plane_data_mut(0).unwrap().len(), 2 * 48000);
            assert!(buffer.plane_data_mut(1).is_ok());
            assert_eq!(buffer.plane_data_mut(1).unwrap().len(), 2 * 48000);
            assert!(buffer.info() == &info);
        }
    }
}
