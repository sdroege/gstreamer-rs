// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;
use std::marker::PhantomData;
use std::mem;
use std::ops;
use std::ptr;
use std::slice;

use glib::translate::{from_glib, from_glib_full, from_glib_none, ToGlibPtr};

use crate::AllocationParams;
use crate::Allocator;
use crate::MemoryFlags;

mini_object_wrapper!(Memory, MemoryRef, ffi::GstMemory, || {
    ffi::gst_memory_get_type()
});

pub struct MemoryMap<'a, T> {
    memory: &'a MemoryRef,
    map_info: ffi::GstMapInfo,
    phantom: PhantomData<T>,
}

pub struct MappedMemory<T> {
    memory: Option<Memory>,
    map_info: ffi::GstMapInfo,
    phantom: PhantomData<T>,
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        MemoryRef::fmt(self, f)
    }
}

impl fmt::Debug for MemoryRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Memory")
            .field("ptr", unsafe { &self.as_ptr() })
            .field("allocator", &self.allocator())
            .field("parent", &self.parent())
            .field("maxsize", &self.maxsize())
            .field("align", &self.align())
            .field("offset", &self.offset())
            .field("size", &self.size())
            .field("flags", &self.flags())
            .finish()
    }
}

pub enum Readable {}
pub enum Writable {}

impl Memory {
    unsafe extern "C" fn drop_box<T>(vec: glib::ffi::gpointer) {
        let slice: Box<T> = Box::from_raw(vec as *mut T);
        drop(slice);
    }

    pub fn with_size(size: usize) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_allocator_alloc(
                ptr::null_mut(),
                size,
                ptr::null_mut(),
            ))
        }
    }

    pub fn with_size_and_params(size: usize, params: &AllocationParams) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_allocator_alloc(
                ptr::null_mut(),
                size,
                params.as_ptr() as *mut _,
            ))
        }
    }

    pub fn from_slice<T: AsRef<[u8]> + Send + 'static>(slice: T) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let b = Box::new(slice);
            let (size, data) = {
                let slice = (*b).as_ref();
                (slice.len(), slice.as_ptr())
            };
            let user_data = Box::into_raw(b);
            from_glib_full(ffi::gst_memory_new_wrapped(
                ffi::GST_MEMORY_FLAG_READONLY,
                data as glib::ffi::gpointer,
                size,
                0,
                size,
                user_data as glib::ffi::gpointer,
                Some(Self::drop_box::<T>),
            ))
        }
    }

    pub fn from_mut_slice<T: AsMut<[u8]> + Send + 'static>(slice: T) -> Self {
        assert_initialized_main_thread!();

        unsafe {
            let mut b = Box::new(slice);
            let (size, data) = {
                let slice = (*b).as_mut();
                (slice.len(), slice.as_mut_ptr())
            };
            let user_data = Box::into_raw(b);
            from_glib_full(ffi::gst_memory_new_wrapped(
                0,
                data as glib::ffi::gpointer,
                size,
                0,
                size,
                user_data as glib::ffi::gpointer,
                Some(Self::drop_box::<T>),
            ))
        }
    }

    pub fn into_mapped_memory_readable(self) -> Result<MappedMemory<Readable>, Self> {
        unsafe {
            let mut map_info = mem::MaybeUninit::zeroed();
            let res: bool = from_glib(ffi::gst_memory_map(
                self.as_mut_ptr(),
                map_info.as_mut_ptr(),
                ffi::GST_MAP_READ,
            ));
            if res {
                Ok(MappedMemory {
                    memory: Some(self),
                    map_info: map_info.assume_init(),
                    phantom: PhantomData,
                })
            } else {
                Err(self)
            }
        }
    }

    pub fn into_mapped_memory_writable(self) -> Result<MappedMemory<Writable>, Self> {
        unsafe {
            let mut map_info = mem::MaybeUninit::zeroed();
            let res: bool = from_glib(ffi::gst_memory_map(
                self.as_mut_ptr(),
                map_info.as_mut_ptr(),
                ffi::GST_MAP_READWRITE,
            ));
            if res {
                Ok(MappedMemory {
                    memory: Some(self),
                    map_info: map_info.assume_init(),
                    phantom: PhantomData,
                })
            } else {
                Err(self)
            }
        }
    }
}

impl MemoryRef {
    #[doc(alias = "get_allocator")]
    pub fn allocator(&self) -> Option<Allocator> {
        unsafe { from_glib_none(self.0.allocator) }
    }

    #[doc(alias = "get_parent")]
    pub fn parent(&self) -> Option<&MemoryRef> {
        unsafe {
            if self.0.parent.is_null() {
                None
            } else {
                Some(MemoryRef::from_ptr(self.0.parent))
            }
        }
    }

    #[doc(alias = "get_maxsize")]
    pub fn maxsize(&self) -> usize {
        self.0.maxsize
    }

    #[doc(alias = "get_align")]
    pub fn align(&self) -> usize {
        self.0.align
    }

    #[doc(alias = "get_offset")]
    pub fn offset(&self) -> usize {
        self.0.offset
    }

    #[doc(alias = "get_size")]
    pub fn size(&self) -> usize {
        self.0.size
    }

    #[doc(alias = "get_flags")]
    pub fn flags(&self) -> MemoryFlags {
        unsafe { from_glib(self.0.mini_object.flags) }
    }

    pub fn copy_part(&self, offset: isize, size: Option<usize>) -> Memory {
        let pos_sz = match size {
            Some(val) => val as isize,
            None => 0,
        };
        assert!(offset + pos_sz < (self.maxsize() as isize));
        unsafe {
            from_glib_full(ffi::gst_memory_copy(
                self.as_mut_ptr(),
                offset,
                match size {
                    Some(val) => val as isize,
                    None => -1,
                },
            ))
        }
    }

    #[doc(alias = "gst_memory_is_span")]
    pub fn is_span(&self, mem2: &MemoryRef) -> Option<usize> {
        unsafe {
            let mut offset = mem::MaybeUninit::uninit();
            let res = from_glib(ffi::gst_memory_is_span(
                self.as_mut_ptr(),
                mem2.as_mut_ptr(),
                offset.as_mut_ptr(),
            ));
            if res {
                Some(offset.assume_init())
            } else {
                None
            }
        }
    }

    #[doc(alias = "gst_memory_is_type")]
    pub fn is_type(&self, mem_type: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_memory_is_type(
                self.as_mut_ptr(),
                mem_type.to_glib_none().0,
            ))
        }
    }

    pub fn map_readable(&self) -> Result<MemoryMap<Readable>, glib::BoolError> {
        unsafe {
            let mut map_info = mem::MaybeUninit::zeroed();
            let res =
                ffi::gst_memory_map(self.as_mut_ptr(), map_info.as_mut_ptr(), ffi::GST_MAP_READ);
            if res == glib::ffi::GTRUE {
                Ok(MemoryMap {
                    memory: self,
                    map_info: map_info.assume_init(),
                    phantom: PhantomData,
                })
            } else {
                Err(glib::bool_error!("Failed to map memory readable"))
            }
        }
    }

    pub fn map_writable(&mut self) -> Result<MemoryMap<Writable>, glib::BoolError> {
        unsafe {
            let mut map_info = mem::MaybeUninit::zeroed();
            let res = ffi::gst_memory_map(
                self.as_mut_ptr(),
                map_info.as_mut_ptr(),
                ffi::GST_MAP_READWRITE,
            );
            if res == glib::ffi::GTRUE {
                Ok(MemoryMap {
                    memory: self,
                    map_info: map_info.assume_init(),
                    phantom: PhantomData,
                })
            } else {
                Err(glib::bool_error!("Failed to map memory writable"))
            }
        }
    }

    #[doc(alias = "gst_memory_share")]
    pub fn share(&self, offset: isize, size: Option<usize>) -> Memory {
        let pos_sz = match size {
            Some(val) => val as isize,
            None => 0,
        };
        assert!(offset + pos_sz < (self.maxsize() as isize));
        unsafe {
            from_glib_full(ffi::gst_memory_share(
                self.as_ptr() as *mut _,
                offset,
                match size {
                    Some(val) => val as isize,
                    None => -1,
                },
            ))
        }
    }

    #[doc(alias = "gst_memory_resize")]
    pub fn resize(&mut self, offset: isize, size: usize) {
        assert!(offset + (size as isize) < (self.maxsize() as isize));
        unsafe { ffi::gst_memory_resize(self.as_mut_ptr(), offset, size) }
    }

    pub fn dump(&self, size: Option<usize>) -> Dump {
        Dump { memory: self, size }
    }
}

impl<'a, T> MemoryMap<'a, T> {
    #[doc(alias = "get_size")]
    pub fn size(&self) -> usize {
        self.map_info.size
    }

    #[doc(alias = "get_memory")]
    pub fn memory(&self) -> &MemoryRef {
        self.memory
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.map_info.data as *const u8, self.map_info.size) }
    }
}

impl<'a> MemoryMap<'a, Writable> {
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.map_info.data as *mut u8, self.map_info.size) }
    }
}

impl<'a, T> AsRef<[u8]> for MemoryMap<'a, T> {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl<'a> AsMut<[u8]> for MemoryMap<'a, Writable> {
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
}

impl<'a, T> ops::Deref for MemoryMap<'a, T> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl<'a> ops::DerefMut for MemoryMap<'a, Writable> {
    fn deref_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
}

impl<'a, T> fmt::Debug for MemoryMap<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("MemoryMap").field(&self.memory()).finish()
    }
}

impl<'a, T> PartialEq for MemoryMap<'a, T> {
    fn eq(&self, other: &MemoryMap<'a, T>) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}

impl<'a, T> Eq for MemoryMap<'a, T> {}

impl<'a, T> Drop for MemoryMap<'a, T> {
    fn drop(&mut self) {
        unsafe {
            ffi::gst_memory_unmap(self.memory.as_mut_ptr(), &mut self.map_info);
        }
    }
}

unsafe impl<'a, T> Send for MemoryMap<'a, T> {}
unsafe impl<'a, T> Sync for MemoryMap<'a, T> {}

impl<T> MappedMemory<T> {
    pub fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.map_info.data as *const u8, self.map_info.size) }
    }

    #[doc(alias = "get_size")]
    pub fn size(&self) -> usize {
        self.map_info.size
    }

    #[doc(alias = "get_memory")]
    pub fn memory(&self) -> &MemoryRef {
        self.memory.as_ref().unwrap().as_ref()
    }

    pub fn into_memory(mut self) -> Memory {
        let memory = self.memory.take().unwrap();
        unsafe {
            ffi::gst_memory_unmap(memory.as_mut_ptr(), &mut self.map_info);
        }

        memory
    }
}

impl MappedMemory<Writable> {
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.map_info.data as *mut u8, self.map_info.size) }
    }
}

impl<T> AsRef<[u8]> for MappedMemory<T> {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl AsMut<[u8]> for MappedMemory<Writable> {
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
}

impl<T> ops::Deref for MappedMemory<T> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl ops::DerefMut for MappedMemory<Writable> {
    fn deref_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
}

impl<T> Drop for MappedMemory<T> {
    fn drop(&mut self) {
        if let Some(ref memory) = self.memory {
            unsafe {
                ffi::gst_memory_unmap(memory.as_mut_ptr(), &mut self.map_info);
            }
        }
    }
}

impl<T> fmt::Debug for MappedMemory<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("MappedMemory").field(&self.memory()).finish()
    }
}

impl<T> PartialEq for MappedMemory<T> {
    fn eq(&self, other: &MappedMemory<T>) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}

impl<T> Eq for MappedMemory<T> {}

unsafe impl<T> Send for MappedMemory<T> {}
unsafe impl<T> Sync for MappedMemory<T> {}

pub struct Dump<'a> {
    memory: &'a MemoryRef,
    size: Option<usize>,
}

impl<'a> Dump<'a> {
    fn fmt(&self, f: &mut fmt::Formatter, debug: bool) -> fmt::Result {
        use pretty_hex::*;

        let map = self.memory.map_readable().expect("Failed to map memory");
        let data = map.as_slice();
        let size = self.size.unwrap_or_else(|| self.memory.size());
        let data = &data[0..size];

        if debug {
            write!(f, "{:?}", data.hex_dump())
        } else {
            write!(f, "{}", data.hex_dump())
        }
    }
}

impl<'a> fmt::Display for Dump<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt(f, false)
    }
}

impl<'a> fmt::Debug for Dump<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f, true)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_dump() {
        crate::init().unwrap();

        let mem = crate::Memory::from_slice(vec![1, 2, 3, 4]);
        println!("{}", mem.dump(Some(mem.size())));

        let mem = crate::Memory::from_slice(vec![1, 2, 3, 4]);
        println!("{:?}", mem.dump(Some(2)));

        let mem = crate::Memory::from_slice(vec![0; 64]);
        dbg!(mem.dump(None));
    }
}
