use crate::VulkanFullScreenQuad;

use crate::traits::VulkanFullScreenQuadExt;

use glib::prelude::*;
use glib::translate::*;

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::VulkanFullScreenQuad>> Sealed for T {}
}

pub trait VulkanFullScreenQuadExtManual:
    sealed::Sealed + IsA<VulkanFullScreenQuad> + 'static
{
    fn draw_into_output(&self, outbuf: &mut gst::BufferRef) -> Result<(), glib::Error> {
        let out = unsafe { gst::Buffer::from_glib_none(outbuf.as_ptr()) };
        self.set_output_buffer(Some(&out))?;
        let ret = self.draw();
        self.set_output_buffer(None)?;
        ret
    }
}
impl<O: IsA<VulkanFullScreenQuad>> VulkanFullScreenQuadExtManual for O {}
