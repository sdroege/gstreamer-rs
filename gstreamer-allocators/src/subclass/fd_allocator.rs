use glib::subclass::prelude::*;

use crate::FdAllocator;
use gst::subclass::prelude::AllocatorImpl;

pub trait FdAllocatorImpl: AllocatorImpl {}
unsafe impl<T: FdAllocatorImpl> IsSubclassable<T> for FdAllocator {}
