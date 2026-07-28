// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::{ffi, relation_meta::*};

#[derive(Debug)]
pub enum AnalyticsOrientationMtd {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::AnalyticsRelationMetaOrientationExt> Sealed for T {}
}

pub trait AnalyticsRelationMetaOrientationExt: sealed::Sealed {
    fn add_orientation_mtd(
        &mut self,
        yaw: f32,
        pitch: f32,
        confidence: f32,
    ) -> Result<AnalyticsMtdRef<'_, AnalyticsOrientationMtd>, glib::BoolError>;
}

impl AnalyticsRelationMetaOrientationExt
    for gst::MetaRefMut<'_, AnalyticsRelationMeta, gst::meta::Standalone>
{
    #[doc(alias = "gst_analytics_relation_meta_add_orientation_mtd")]
    fn add_orientation_mtd(
        &mut self,
        yaw: f32,
        pitch: f32,
        confidence: f32,
    ) -> Result<AnalyticsMtdRef<'_, AnalyticsOrientationMtd>, glib::BoolError> {
        unsafe {
            let mut mtd = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_analytics_relation_meta_add_orientation_mtd(
                self.as_mut_ptr(),
                yaw,
                pitch,
                confidence,
                mtd.as_mut_ptr(),
            ));
            if ret {
                let id = mtd.assume_init().id;
                Ok(AnalyticsMtdRef::from_meta(self.as_ref(), id))
            } else {
                Err(glib::bool_error!("Couldn't add orientation metadata"))
            }
        }
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct AnalyticsOrientation {
    pub yaw: f32,
    pub pitch: f32,
}

unsafe impl AnalyticsMtd for AnalyticsOrientationMtd {
    #[doc(alias = "gst_analytics_orientation_mtd_get_mtd_type")]
    fn mtd_type() -> ffi::GstAnalyticsMtdType {
        unsafe { ffi::gst_analytics_orientation_mtd_get_mtd_type() }
    }
}

impl AnalyticsMtdRef<'_, AnalyticsOrientationMtd> {
    #[doc(alias = "gst_analytics_orientation_mtd_get_angles")]
    pub fn angles(&self) -> Result<AnalyticsOrientation, glib::BoolError> {
        let mut ori = AnalyticsOrientation::default();

        let ret = unsafe {
            let mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            ffi::gst_analytics_orientation_mtd_get_angles(
                &mtd as *const _ as *const ffi::GstAnalyticsOrientationMtd,
                &mut ori.yaw,
                &mut ori.pitch,
            )
        };

        if unsafe { from_glib(ret) } {
            Ok(ori)
        } else {
            Err(glib::bool_error!("Could not retrieve orientation"))
        }
    }

    #[doc(alias = "gst_analytics_orientation_mtd_get_confidence")]
    pub fn confidence(&self) -> Result<f32, glib::BoolError> {
        let mut confidence: f32 = 0.0;
        let ret = unsafe {
            let mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            ffi::gst_analytics_orientation_mtd_get_confidence(
                &mtd as *const _ as *const ffi::GstAnalyticsOrientationMtd,
                &mut confidence,
            )
        };

        if unsafe { from_glib(ret) } {
            Ok(confidence)
        } else {
            Err(glib::bool_error!(
                "Could not retrieve orientation confidence"
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn orientation_mtd() {
        gst::init().unwrap();

        assert_eq!(AnalyticsOrientationMtd::type_name(), "orientation");

        let mut buf = gst::Buffer::new();
        let mut meta = AnalyticsRelationMeta::add(buf.make_mut());

        assert!(meta.is_empty());

        let orientation = meta.add_orientation_mtd(5.0, 10.0, 0.8).unwrap();

        let ori = orientation.angles().unwrap();

        assert_eq!(ori.yaw, 5.0);
        assert_eq!(ori.pitch, 10.0);
        assert_eq!(orientation.confidence().unwrap(), 0.8);

        let meta = buf.meta::<AnalyticsRelationMeta>().unwrap();

        assert!(meta.mtd::<AnalyticsOrientationMtd>(1).is_none());

        let orientation2 = meta.mtd::<AnalyticsOrientationMtd>(0).unwrap();

        let ori = orientation2.angles().unwrap();

        assert_eq!(ori.yaw, 5.0);
        assert_eq!(ori.pitch, 10.0);
        assert_eq!(orientation2.confidence().unwrap(), 0.8);
    }
}
