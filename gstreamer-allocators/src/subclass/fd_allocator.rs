use glib::{prelude::*, subclass::prelude::*};

use crate::FdAllocator;
use gst::subclass::prelude::AllocatorImpl;

pub trait FdAllocatorImpl: AllocatorImpl + ObjectSubclass<Type: IsA<FdAllocator>> {}
unsafe impl<T: FdAllocatorImpl> IsSubclassable<T> for FdAllocator {}
