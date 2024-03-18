// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

use glib::translate::from_glib;
use gst::prelude::*;

#[repr(transparent)]
#[doc(alias = "GESFrameCompositionMeta")]
pub struct FrameCompositionMeta(ffi::GESFrameCompositionMeta);

unsafe impl Send for FrameCompositionMeta {}

unsafe impl Sync for FrameCompositionMeta {}

impl FrameCompositionMeta {
    #[inline]
    pub fn alpha(&self) -> f64 {
        self.0.alpha
    }

    #[inline]
    pub fn position(&self) -> (f64, f64) {
        (self.0.posx, self.0.posy)
    }

    #[inline]
    pub fn pos_x(&self) -> f64 {
        self.0.posx
    }

    #[inline]
    pub fn pos_y(&self) -> f64 {
        self.0.posy
    }

    #[inline]
    pub fn size(&self) -> (f64, f64) {
        (self.0.width, self.0.height)
    }

    #[inline]
    pub fn width(&self) -> f64 {
        self.0.width
    }

    #[inline]
    pub fn height(&self) -> f64 {
        self.0.height
    }

    #[inline]
    pub fn zorder(&self) -> u32 {
        self.0.zorder
    }

    #[inline]
    pub fn operator(&self) -> i32 {
        self.0.operator
    }
}

unsafe impl MetaAPI for FrameCompositionMeta {
    type GstType = ffi::GESFrameCompositionMeta;

    #[doc(alias = "ges_frame_composition_meta_api_get_type")]
    #[inline]
    fn meta_api() -> glib::Type {
        unsafe { from_glib(ffi::ges_frame_composition_meta_api_get_type()) }
    }
}

impl fmt::Debug for FrameCompositionMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FrameCompositionMeta")
            .field("pos-x", &self.pos_x())
            .field("pos-y", &self.pos_y())
            .field("width", &self.width())
            .field("height", &self.height())
            .field("zorder", &self.zorder())
            .field("alpha", &self.alpha())
            .field("operator", &self.operator())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn add_composition_meta(
        buffer: &mut gst::BufferRef,
        position: (f64, f64),
        size: (f64, f64),
        alpha: f64,
        zorder: u32,
        operator: i32,
    ) -> Result<gst::MetaRefMut<FrameCompositionMeta, gst::meta::Standalone>, glib::BoolError> {
        assert_initialized_main_thread!();

        unsafe {
            let meta = ffi::ges_buffer_add_frame_composition_meta(buffer.as_mut_ptr());

            if meta.is_null() {
                return Err(glib::bool_error!("Failed to add frame composition meta"));
            }

            let mut result = FrameCompositionMeta::from_mut_ptr(buffer, meta);
            result.0.posx = position.0;
            result.0.posy = position.1;
            result.0.width = size.0;
            result.0.height = size.1;
            result.0.alpha = alpha;
            result.0.zorder = zorder;
            result.0.operator = operator;
            Ok(result)
        }
    }

    #[test]
    fn test_add_get_meta() {
        gst::init().unwrap();
        crate::init().unwrap();

        let mut buffer = gst::Buffer::with_size(320 * 240 * 4).unwrap();
        {
            let _meta =
                add_composition_meta(buffer.get_mut().unwrap(), (42., 42.), (20., 22.), 0.42, 2, 42)
                    .unwrap();
        }

        {
            let meta = buffer.meta::<FrameCompositionMeta>().unwrap();
            assert_eq!(meta.position(), (42., 42.));
            assert_eq!(meta.size(), (20., 22.));
            assert_eq!(meta.alpha(), 0.42);
            assert_eq!(meta.zorder(), 2);
            assert_eq!(meta.operator(), 42);
        }
    }
}
