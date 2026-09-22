// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::{ffi, relation_meta::*};

#[derive(Debug)]
pub enum AnalyticsSegmentationMtd {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::AnalyticsRelationMetaSegmentationExt> Sealed for T {}
}

pub trait AnalyticsRelationMetaSegmentationExt: sealed::Sealed {
    #[allow(clippy::too_many_arguments)]
    fn add_segmentation_mtd(
        &mut self,
        mask_buffer: gst::Buffer,
        _type: crate::SegmentationType,
        region_ids: &[u32],
        masks_loc_x: i32,
        masks_loc_y: i32,
        masks_loc_w: u32,
        masks_loc_h: u32,
    ) -> Result<AnalyticsMtdRef<'_, AnalyticsSegmentationMtd>, glib::BoolError>;
}

impl<'a> AnalyticsRelationMetaSegmentationExt
    for gst::MetaRefMut<'a, AnalyticsRelationMeta, gst::meta::Standalone>
{
    #[doc(alias = "gst_analytics_relation_meta_add_segmentation_mtd")]
    fn add_segmentation_mtd(
        &mut self,
        mask_buffer: gst::Buffer,
        _type: crate::SegmentationType,
        region_ids: &[u32],
        masks_loc_x: i32,
        masks_loc_y: i32,
        masks_loc_w: u32,
        masks_loc_h: u32,
    ) -> Result<AnalyticsMtdRef<'a, AnalyticsSegmentationMtd>, glib::BoolError> {
        unsafe {
            let mut mtd = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_analytics_relation_meta_add_segmentation_mtd(
                self.as_mut_ptr(),
                mask_buffer.into_glib_ptr(),
                _type.into_glib(),
                region_ids.len(),
                region_ids.as_ptr() as *mut _,
                masks_loc_x,
                masks_loc_y,
                masks_loc_w,
                masks_loc_h,
                mtd.as_mut_ptr(),
            ));

            if ret {
                let id = mtd.assume_init().id;
                Ok(AnalyticsMtdRef::from_meta(self.as_ref(), id))
            } else {
                Err(glib::bool_error!("Couldn't add segmentation metadata"))
            }
        }
    }
}

#[derive(Debug)]
pub struct AnalyticsSegmentationMask {
    pub buffer: gst::Buffer,
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

unsafe impl AnalyticsMtd for AnalyticsSegmentationMtd {
    #[doc(alias = "gst_analytics_segmentation_mtd_get_mtd_type")]
    fn mtd_type() -> ffi::GstAnalyticsMtdType {
        unsafe { ffi::gst_analytics_segmentation_mtd_get_mtd_type() }
    }
}

impl AnalyticsMtdRef<'_, AnalyticsSegmentationMtd> {
    #[doc(alias = "gst_analytics_segmentation_mtd_get_mask")]
    pub fn mask(&self) -> AnalyticsSegmentationMask {
        unsafe {
            let mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            let mut mask_x = 0;
            let mut mask_y = 0;
            let mut mask_w = 0;
            let mut mask_h = 0;

            let ptr = ffi::gst_analytics_segmentation_mtd_get_mask(
                &mtd as *const _ as *const ffi::GstAnalyticsSegmentationMtd,
                &mut mask_x,
                &mut mask_y,
                &mut mask_w,
                &mut mask_h,
            );
            AnalyticsSegmentationMask {
                buffer: gst::Buffer::from_glib_full(ptr),
                x: mask_x,
                y: mask_y,
                w: mask_w,
                h: mask_h,
            }
        }
    }

    #[doc(alias = "gst_analytics_segmentation_mtd_get_region_count")]
    pub fn region_count(&self) -> usize {
        unsafe {
            let mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            ffi::gst_analytics_segmentation_mtd_get_region_count(
                &mtd as *const _ as *const ffi::GstAnalyticsSegmentationMtd,
            )
        }
    }

    #[doc(alias = "gst_analytics_segmentation_mtd_get_region_id")]
    pub fn region_id(&self, index: usize) -> u32 {
        assert!(index < self.region_count());

        unsafe {
            let mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            ffi::gst_analytics_segmentation_mtd_get_region_id(
                &mtd as *const _ as *const ffi::GstAnalyticsSegmentationMtd,
                index,
            )
        }
    }

    #[doc(alias = "gst_analytics_segmentation_mtd_get_region_index")]
    pub fn find_region_id(&self, id: u32) -> Option<usize> {
        unsafe {
            let mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            let mut index = std::mem::MaybeUninit::uninit();

            let ret = from_glib(ffi::gst_analytics_segmentation_mtd_get_region_index(
                &mtd as *const _ as *const ffi::GstAnalyticsSegmentationMtd,
                index.as_mut_ptr(),
                id,
            ));

            if ret { Some(index.assume_init()) } else { None }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn segmentation_mtd() {
        gst::init().unwrap();

        assert_eq!(AnalyticsSegmentationMtd::type_name(), "segmentation");

        let mut buf = gst::Buffer::new();
        let mut meta = AnalyticsRelationMeta::add(buf.get_mut().unwrap());

        assert!(meta.is_empty());

        let mut mask_buf = gst::Buffer::with_size(4).unwrap();
        let vmeta = gst_video::VideoMeta::add(
            mask_buf.get_mut().unwrap(),
            gst_video::VideoFrameFlags::empty(),
            gst_video::VideoFormat::Gray8,
            1,
            1,
        );
        assert!(vmeta.is_ok());

        let region_ids = [0];
        let seg = meta
            .add_segmentation_mtd(
                mask_buf,
                crate::SegmentationType::Semantic,
                &region_ids,
                0,
                0,
                1,
                1,
            )
            .unwrap();

        assert_eq!(seg.region_count(), 1);
        assert_eq!(seg.region_id(0), 0);
    }
}
