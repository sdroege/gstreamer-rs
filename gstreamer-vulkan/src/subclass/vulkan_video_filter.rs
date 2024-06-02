use crate::VulkanVideoFilter;

use gst::subclass::prelude::*;
use gst_base::subclass::prelude::*;

pub trait VulkanVideoFilterImpl: VulkanVideoFilterImplExt + BaseTransformImpl {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::VulkanVideoFilterImplExt> Sealed for T {}
}

pub trait VulkanVideoFilterImplExt: sealed::Sealed + ObjectSubclass {}

impl<T: VulkanVideoFilterImpl> VulkanVideoFilterImplExt for T {}

unsafe impl<T: VulkanVideoFilterImpl> IsSubclassable<T> for VulkanVideoFilter {}
