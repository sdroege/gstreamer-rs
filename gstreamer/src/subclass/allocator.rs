// Take a look at the license at the top of the repository in the LICENSE file.

use std::ptr;

use glib::{bool_error, prelude::*, subclass::prelude::*, translate::*, BoolError};

use super::prelude::*;
use crate::{AllocationParams, Allocator, Memory};

pub trait AllocatorImpl: AllocatorImplExt + GstObjectImpl + Send + Sync {
    fn alloc(&self, size: usize, params: Option<&AllocationParams>) -> Result<Memory, BoolError> {
        self.parent_alloc(size, params)
    }

    fn free(&self, memory: Memory) {
        self.parent_free(memory)
    }
}

pub trait AllocatorImplExt: ObjectSubclass {
    fn parent_alloc(
        &self,
        size: usize,
        params: Option<&AllocationParams>,
    ) -> Result<Memory, BoolError>;

    fn parent_free(&self, memory: Memory);
}

impl<T: AllocatorImpl> AllocatorImplExt for T {
    fn parent_alloc(
        &self,
        size: usize,
        params: Option<&AllocationParams>,
    ) -> Result<Memory, BoolError> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAllocatorClass;

            if let Some(f) = (*parent_class).alloc {
                from_glib_full::<*mut ffi::GstMemory, Option<_>>(f(
                    self.obj().unsafe_cast_ref::<Allocator>().to_glib_none().0,
                    size,
                    mut_override(params.to_glib_none().0),
                ))
                .ok_or_else(|| bool_error!("Allocation failed"))
            } else {
                Err(bool_error!("No allocation method on parent class"))
            }
        }
    }

    fn parent_free(&self, memory: Memory) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GstAllocatorClass;

            if let Some(f) = (*parent_class).free {
                f(
                    self.obj().unsafe_cast_ref::<Allocator>().to_glib_none().0,
                    memory.into_glib_ptr(),
                )
            }
        }
    }
}

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
    let instance = imp.obj();

    let params = if params.is_null() {
        None
    } else {
        Some(&*(params as *mut AllocationParams))
    };

    imp.alloc(size, params)
        .map(|memory| memory.into_glib_ptr())
        .unwrap_or_else(|error| {
            error!(crate::CAT_RUST, obj: instance, "{:?}", error);

            ptr::null_mut()
        })
}

unsafe extern "C" fn free<T: AllocatorImpl>(
    ptr: *mut ffi::GstAllocator,
    memory: *mut ffi::GstMemory,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let memory = from_glib_full(memory);

    imp.free(memory);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    pub mod imp {
        use super::*;

        #[derive(Default)]
        pub struct TestAllocator;

        impl ObjectImpl for TestAllocator {}
        impl GstObjectImpl for TestAllocator {}
        impl AllocatorImpl for TestAllocator {
            fn alloc(
                &self,
                size: usize,
                _params: Option<&AllocationParams>,
            ) -> Result<Memory, BoolError> {
                Ok(Memory::from_slice(vec![0; size]))
            }

            fn free(&self, memory: Memory) {
                self.parent_free(memory)
            }
        }

        #[glib::object_subclass]
        impl ObjectSubclass for TestAllocator {
            const NAME: &'static str = "TestAllocator";
            type Type = super::TestAllocator;
            type ParentType = Allocator;
        }
    }

    glib::wrapper! {
        pub struct TestAllocator(ObjectSubclass<imp::TestAllocator>) @extends Allocator, crate::Object;
    }

    impl Default for TestAllocator {
        fn default() -> Self {
            glib::Object::new(&[])
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
}
