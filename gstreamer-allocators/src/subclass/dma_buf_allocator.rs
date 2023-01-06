use glib::subclass::prelude::*;

use crate::{subclass::fd_allocator::FdAllocatorImpl, DmaBufAllocator};

pub trait DmaBufAllocatorImpl: FdAllocatorImpl {}
unsafe impl<T: DmaBufAllocatorImpl> IsSubclassable<T> for DmaBufAllocator {}
