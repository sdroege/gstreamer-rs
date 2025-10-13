use crate::VulkanVideoFilter;

use gst::subclass::prelude::*;
use gst_base::subclass::prelude::*;

pub trait VulkanVideoFilterImpl: VulkanVideoFilterImplExt + BaseTransformImpl {}

pub trait VulkanVideoFilterImplExt: ObjectSubclass {}

impl<T: VulkanVideoFilterImpl> VulkanVideoFilterImplExt for T {}

unsafe impl<T: VulkanVideoFilterImpl> IsSubclassable<T> for VulkanVideoFilter {}
