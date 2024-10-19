use glib::{prelude::*, subclass::prelude::*};

use crate::{subclass::fd_allocator::FdAllocatorImpl, DmaBufAllocator};

pub trait DmaBufAllocatorImpl:
    FdAllocatorImpl + ObjectSubclass<Type: IsA<DmaBufAllocator>>
{
}
unsafe impl<T: DmaBufAllocatorImpl> IsSubclassable<T> for DmaBufAllocator {}
