// Take a look at the license at the top of the repository in the LICENSE file.

use std::ptr;

use glib::{prelude::*, subclass::prelude::*, translate::*};

use super::prelude::*;
use crate::{ffi, AllocationParams, Allocator};

pub unsafe trait AllocatorImpl:
    GstObjectImpl + ObjectSubclass<Type: IsA<Allocator>>
{
    unsafe fn alloc(&self, size: usize, params: &AllocationParams) -> *mut ffi::GstMemory {
        self.parent_alloc(size, params)
    }

    unsafe fn free(&self, memory: *mut ffi::GstMemory) {
        unsafe { self.parent_free(memory) }
    }
}

pub trait AllocatorImplExt: AllocatorImpl {
    unsafe fn parent_alloc(&self, size: usize, params: &AllocationParams) -> *mut ffi::GstMemory {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAllocatorClass;

            if let Some(f) = (*parent_class).alloc {
                f(
                    self.obj().unsafe_cast_ref::<Allocator>().to_glib_none().0,
                    size,
                    mut_override(params.to_glib_none().0),
                )
            } else {
                ptr::null_mut()
            }
        }
    }

    unsafe fn parent_free(&self, memory: *mut ffi::GstMemory) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAllocatorClass;

            if let Some(f) = (*parent_class).free {
                f(
                    self.obj().unsafe_cast_ref::<Allocator>().to_glib_none().0,
                    memory,
                )
            }
        }
    }
}

impl<T: AllocatorImpl> AllocatorImplExt for T {}

unsafe impl<T: AllocatorImpl> IsSubclassable<T> for Allocator {
    fn class_init(klass: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(klass);
        let klass = klass.as_mut();
        klass.alloc = Some(alloc::<T>);
        klass.free = Some(free::<T>);
    }
}

unsafe extern "C" fn alloc<T: AllocatorImpl>(
    ptr: *mut ffi::GstAllocator,
    size: usize,
    params: *mut ffi::GstAllocationParams,
) -> *mut ffi::GstMemory {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    let params = &*(params as *mut AllocationParams);

    imp.alloc(size, params)
}

unsafe extern "C" fn free<T: AllocatorImpl>(
    ptr: *mut ffi::GstAllocator,
    memory: *mut ffi::GstMemory,
) {
    debug_assert_eq!((*memory).mini_object.refcount, 0);

    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.free(memory);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    // The test allocator below is basically replicating GStreamer's default
    // sysmem allocator except that the memory allocation is separate from the
    // memory struct for clarity.

    pub mod imp {
        use glib::translate::*;
        use std::alloc;

        use super::*;

        #[repr(C)]
        struct Memory {
            mem: ffi::GstMemory,
            layout: alloc::Layout,
            data: *mut u8,
        }

        const LAYOUT: alloc::Layout = alloc::Layout::new::<Memory>();

        #[derive(Default)]
        pub struct TestAllocator;

        impl ObjectImpl for TestAllocator {}
        impl GstObjectImpl for TestAllocator {}
        unsafe impl AllocatorImpl for TestAllocator {
            unsafe fn alloc(&self, size: usize, params: &AllocationParams) -> *mut ffi::GstMemory {
                unsafe {
                    let Some(maxsize) = size
                        .checked_add(params.prefix())
                        .and_then(|s| s.checked_add(params.padding()))
                    else {
                        return ptr::null_mut();
                    };
                    let Ok(layout) = alloc::Layout::from_size_align(maxsize, params.align() + 1)
                    else {
                        return ptr::null_mut();
                    };

                    let mem = alloc::alloc(LAYOUT) as *mut Memory;

                    let data = alloc::alloc(layout);

                    if params.prefix() > 0
                        && params.flags().contains(crate::MemoryFlags::ZERO_PREFIXED)
                    {
                        ptr::write_bytes(data, 0, params.prefix());
                    }

                    if params.flags().contains(crate::MemoryFlags::ZERO_PADDED) {
                        ptr::write_bytes(data.add(params.prefix()).add(size), 0, params.padding());
                    }

                    ffi::gst_memory_init(
                        ptr::addr_of_mut!((*mem).mem),
                        params.flags().into_glib(),
                        self.obj().as_ptr() as *mut ffi::GstAllocator,
                        ptr::null_mut(),
                        maxsize,
                        params.align(),
                        params.prefix(),
                        size,
                    );
                    ptr::write(ptr::addr_of_mut!((*mem).layout), layout);
                    ptr::write(ptr::addr_of_mut!((*mem).data), data);

                    mem as *mut ffi::GstMemory
                }
            }

            unsafe fn free(&self, mem: *mut ffi::GstMemory) {
                unsafe {
                    let mem = mem as *mut Memory;

                    if (*mem).mem.parent.is_null() {
                        alloc::dealloc((*mem).data, (*mem).layout);
                        ptr::drop_in_place(ptr::addr_of_mut!((*mem).layout));
                    }
                    alloc::dealloc(mem as *mut u8, LAYOUT);
                }
            }
        }

        #[glib::object_subclass]
        impl ObjectSubclass for TestAllocator {
            const NAME: &'static str = "TestAllocator";
            type Type = super::TestAllocator;
            type ParentType = Allocator;

            fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
                static ALLOCATOR_TYPE: &[u8] = b"TestAllocatorMemory\0";

                unsafe {
                    let allocator = obj.as_ptr() as *mut ffi::GstAllocator;

                    // TODO: This should all be in some kind of trait ideally
                    (*allocator).mem_type = ALLOCATOR_TYPE.as_ptr() as *const _;
                    (*allocator).mem_map = Some(TestAllocator::mem_map);
                    (*allocator).mem_unmap = Some(TestAllocator::mem_unmap);
                    // mem_copy not set because the fallback already does the right thing
                    (*allocator).mem_share = Some(TestAllocator::mem_share);
                    (*allocator).mem_is_span = Some(TestAllocator::mem_is_span);
                }
            }
        }

        impl TestAllocator {
            unsafe extern "C" fn mem_map(
                mem: *mut ffi::GstMemory,
                _maxsize: usize,
                _flags: ffi::GstMapFlags,
            ) -> glib::ffi::gpointer {
                let mem = mem as *mut Memory;

                let parent = if (*mem).mem.parent.is_null() {
                    mem
                } else {
                    (*mem).mem.parent as *mut Memory
                };

                // `(*mem).offset` is added to the pointer by `gst_memory_map()`
                (*parent).data as *mut _
            }

            unsafe extern "C" fn mem_unmap(_mem: *mut ffi::GstMemory) {}

            unsafe extern "C" fn mem_share(
                mem: *mut ffi::GstMemory,
                offset: isize,
                size: isize,
            ) -> *mut ffi::GstMemory {
                let mem = mem as *mut Memory;

                // Basically a re-implementation of _sysmem_share()

                let parent = if (*mem).mem.parent.is_null() {
                    mem
                } else {
                    (*mem).mem.parent as *mut Memory
                };

                // Offset and size are actually usizes and the API assumes that negative values simply wrap
                // around, so let's cast to usizes here and do wrapping arithmetic.
                let offset = offset as usize;
                let mut size = size as usize;

                let new_offset = (*mem).mem.offset.wrapping_add(offset);
                debug_assert!(new_offset < (*mem).mem.maxsize);

                if size == usize::MAX {
                    size = (*mem).mem.size.wrapping_sub(offset);
                }
                debug_assert!(new_offset <= usize::MAX - size);
                debug_assert!(new_offset + size <= (*mem).mem.maxsize);

                let sub = alloc::alloc(LAYOUT) as *mut Memory;

                ffi::gst_memory_init(
                    sub as *mut ffi::GstMemory,
                    (*mem).mem.mini_object.flags | ffi::GST_MINI_OBJECT_FLAG_LOCK_READONLY,
                    (*mem).mem.allocator,
                    parent as *mut ffi::GstMemory,
                    (*mem).mem.maxsize,
                    (*mem).mem.align,
                    new_offset,
                    size,
                );
                // This is never actually accessed
                ptr::write(ptr::addr_of_mut!((*sub).data), ptr::null_mut());

                sub as *mut ffi::GstMemory
            }

            unsafe extern "C" fn mem_is_span(
                mem1: *mut ffi::GstMemory,
                mem2: *mut ffi::GstMemory,
                offset: *mut usize,
            ) -> glib::ffi::gboolean {
                let mem1 = mem1 as *mut Memory;
                let mem2 = mem2 as *mut Memory;

                // Same parent is checked by `gst_memory_is_span()` already
                let parent1 = (*mem1).mem.parent as *mut Memory;
                let parent2 = (*mem2).mem.parent as *mut Memory;
                debug_assert_eq!(parent1, parent2);

                if !offset.is_null() {
                    // Offset that can be used on the parent memory to create a
                    // shared memory that starts with `mem1`.
                    //
                    // This needs to use wrapping arithmetic too as in `mem_share()`.
                    *offset = (*mem1).mem.offset.wrapping_sub((*parent1).mem.offset);
                }

                // Check if both memories are contiguous.
                let is_span = ((*mem1).mem.offset + ((*mem1).mem.size)) == (*mem2).mem.offset;

                is_span.into_glib()
            }
        }
    }

    glib::wrapper! {
        pub struct TestAllocator(ObjectSubclass<imp::TestAllocator>) @extends Allocator, crate::Object;
    }

    impl Default for TestAllocator {
        fn default() -> Self {
            glib::Object::new()
        }
    }

    #[test]
    fn test_allocator_registration() {
        crate::init().unwrap();

        const TEST_ALLOCATOR_NAME: &str = "TestAllocator";

        let allocator = TestAllocator::default();
        Allocator::register(TEST_ALLOCATOR_NAME, allocator);

        let allocator = Allocator::find(Some(TEST_ALLOCATOR_NAME));

        assert!(allocator.is_some());
    }

    #[test]
    fn test_allocator_alloc() {
        crate::init().unwrap();

        const SIZE: usize = 1024;

        let allocator = TestAllocator::default();

        let memory = allocator.alloc(SIZE, None).unwrap();

        assert_eq!(memory.size(), SIZE);
    }

    #[test]
    fn test_allocator_mem_ops() {
        crate::init().unwrap();

        let data = [0, 1, 2, 3, 4, 5, 6, 7];

        let allocator = TestAllocator::default();

        let mut memory = allocator.alloc(data.len(), None).unwrap();
        assert_eq!(memory.size(), data.len());

        {
            let memory = memory.get_mut().unwrap();
            let mut map = memory.map_writable().unwrap();
            map.copy_from_slice(&data);
        }

        let copy = memory.copy();
        assert!(copy.parent().is_none());

        {
            let map1 = memory.map_readable().unwrap();
            let map2 = copy.map_readable().unwrap();
            assert_eq!(map1.as_slice(), map2.as_slice());
        }

        let share = memory.share(..);
        assert_eq!(share.parent().unwrap().as_ptr(), memory.as_ptr());

        {
            let map1 = memory.map_readable().unwrap();
            let map2 = share.map_readable().unwrap();
            assert_eq!(map1.as_slice(), map2.as_slice());
        }

        let sub1 = memory.share(..2);
        assert_eq!(sub1.size(), 2);
        assert_eq!(sub1.parent().unwrap().as_ptr(), memory.as_ptr());

        {
            let map = sub1.map_readable().unwrap();
            assert_eq!(map.as_slice(), &data[..2]);
        }

        let sub2 = memory.share(2..);
        assert_eq!(sub2.size(), 6);
        assert_eq!(sub2.parent().unwrap().as_ptr(), memory.as_ptr());

        {
            let map = sub2.map_readable().unwrap();
            assert_eq!(map.as_slice(), &data[2..]);
        }

        let offset = sub1.is_span(&sub2).unwrap();
        assert_eq!(offset, 0);

        let sub3 = sub2.share(2..);
        assert_eq!(sub3.size(), 4);
        assert_eq!(sub3.parent().unwrap().as_ptr(), memory.as_ptr());

        {
            let map = sub3.map_readable().unwrap();
            assert_eq!(map.as_slice(), &data[4..]);
        }
    }
}
