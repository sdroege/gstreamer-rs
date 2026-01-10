use glib::{prelude::*, subclass::prelude::*};

use crate::{DmaBufAllocator, subclass::fd_allocator::FdAllocatorImpl};

pub trait DmaBufAllocatorImpl:
    FdAllocatorImpl + ObjectSubclass<Type: IsA<DmaBufAllocator>>
{
}
unsafe impl<T: DmaBufAllocatorImpl> IsSubclassable<T> for DmaBufAllocator {}
