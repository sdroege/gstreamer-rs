use crate::VulkanSwapper;

use glib::prelude::*;

pub trait VulkanSwapperExtManual: IsA<VulkanSwapper> + 'static {
    #[doc(alias = "pixel-aspect-ratio")]
    fn is_force_aspect_ratio(&self) -> gst::Fraction {
        ObjectExt::property(self.as_ref(), "pixel-aspect-ratio")
    }

    #[doc(alias = "pixel-aspect-ratio")]
    fn set_force_aspect_ratio(&self, pixel_aspect_ratio: gst::Fraction) {
        ObjectExt::set_property(self.as_ref(), "pixel-aspect-ratio", pixel_aspect_ratio)
    }
}
impl<O: IsA<VulkanSwapper>> VulkanSwapperExtManual for O {}
