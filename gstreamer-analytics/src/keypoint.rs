// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::{ffi, relation_meta::*};

#[derive(Debug)]
pub enum AnalyticsKeypointMtd {}

pub type AnalyticsKeypointDimensions = crate::KeypointDimensions;
pub type AnalyticsKeypointVisibility = crate::KeypointVisibility;

#[derive(Clone, Copy, Debug)]
pub struct AnalyticsKeypointPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub dimension: AnalyticsKeypointDimensions,
}

impl Default for AnalyticsKeypointPosition {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            z: 0,
            dimension: AnalyticsKeypointDimensions::_2d,
        }
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::AnalyticsRelationMetaKeypointExt> Sealed for T {}
}

pub trait AnalyticsRelationMetaKeypointExt: sealed::Sealed {
    fn add_keypoint_mtd(
        &mut self,
        dimension: AnalyticsKeypointDimensions,
        x: i32,
        y: i32,
        z: i32,
        visibility_flags: AnalyticsKeypointVisibility,
        confidence: f32,
    ) -> Result<AnalyticsMtdRef<'_, AnalyticsKeypointMtd>, glib::BoolError>;

    fn add_keypoint_mtd_from_position(
        &mut self,
        position: AnalyticsKeypointPosition,
        visibility_flags: AnalyticsKeypointVisibility,
        confidence: f32,
    ) -> Result<AnalyticsMtdRef<'_, AnalyticsKeypointMtd>, glib::BoolError> {
        self.add_keypoint_mtd(
            position.dimension,
            position.x,
            position.y,
            position.z,
            visibility_flags,
            confidence,
        )
    }
}

impl AnalyticsRelationMetaKeypointExt
    for gst::MetaRefMut<'_, AnalyticsRelationMeta, gst::meta::Standalone>
{
    #[doc(alias = "gst_analytics_relation_meta_add_keypoint_mtd")]
    fn add_keypoint_mtd(
        &mut self,
        dimension: AnalyticsKeypointDimensions,
        x: i32,
        y: i32,
        z: i32,
        visibility_flags: AnalyticsKeypointVisibility,
        confidence: f32,
    ) -> Result<AnalyticsMtdRef<'_, AnalyticsKeypointMtd>, glib::BoolError> {
        unsafe {
            let mut mtd = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_analytics_relation_meta_add_keypoint_mtd(
                self.as_mut_ptr(),
                dimension.into_glib(),
                x,
                y,
                z,
                visibility_flags.into_glib() as u8,
                confidence,
                mtd.as_mut_ptr(),
            ));
            let id = mtd.assume_init().id;

            if ret {
                Ok(AnalyticsMtdRef::from_meta(self.as_ref(), id))
            } else {
                Err(glib::bool_error!("Couldn't add keypoint metadata"))
            }
        }
    }
}

unsafe impl AnalyticsMtd for AnalyticsKeypointMtd {
    #[doc(alias = "gst_analytics_keypoint_mtd_get_mtd_type")]
    fn mtd_type() -> ffi::GstAnalyticsMtdType {
        unsafe { ffi::gst_analytics_keypoint_mtd_get_mtd_type() }
    }
}

impl AnalyticsMtdRef<'_, AnalyticsKeypointMtd> {
    #[doc(alias = "gst_analytics_keypoint_mtd_get_position")]
    pub fn position(&self) -> Result<AnalyticsKeypointPosition, glib::BoolError> {
        let mut pos = AnalyticsKeypointPosition::default();
        let mut dimension = AnalyticsKeypointDimensions::_2d.into_glib();

        let ret = unsafe {
            let mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            ffi::gst_analytics_keypoint_mtd_get_position(
                &mtd as *const _ as *const ffi::GstAnalyticsKeypointMtd,
                &mut pos.x,
                &mut pos.y,
                &mut pos.z,
                &mut dimension,
            )
        };

        if unsafe { from_glib(ret) } {
            pos.dimension = unsafe { from_glib(dimension) };
            Ok(pos)
        } else {
            Err(glib::bool_error!("Could not retrieve keypoint position"))
        }
    }

    #[doc(alias = "gst_analytics_keypoint_mtd_get_confidence")]
    pub fn confidence(&self) -> Result<f32, glib::BoolError> {
        let mut confidence: f32 = 0.0;
        let ret = unsafe {
            let mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            ffi::gst_analytics_keypoint_mtd_get_confidence(
                &mtd as *const _ as *const ffi::GstAnalyticsKeypointMtd,
                &mut confidence,
            )
        };

        if unsafe { from_glib(ret) } {
            Ok(confidence)
        } else {
            Err(glib::bool_error!("Could not retrieve keypoint confidence"))
        }
    }

    #[doc(alias = "gst_analytics_keypoint_mtd_get_visibility_flags")]
    pub fn visibility_flags(&self) -> Result<AnalyticsKeypointVisibility, glib::BoolError> {
        let mut visibility_flags: u8 = 0;
        let ret = unsafe {
            let mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            ffi::gst_analytics_keypoint_mtd_get_visibility_flags(
                &mtd as *const _ as *const ffi::GstAnalyticsKeypointMtd,
                &mut visibility_flags,
            )
        };

        if unsafe { from_glib(ret) } {
            Ok(unsafe { from_glib(visibility_flags as ffi::GstAnalyticsKeypointVisibility) })
        } else {
            Err(glib::bool_error!(
                "Could not retrieve keypoint visibility flags"
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn keypoint_mtd() {
        gst::init().unwrap();

        let type_name = AnalyticsKeypointMtd::type_name();
        assert_eq!(type_name, "keypoint-mtd");

        let mut buf = gst::Buffer::new();
        let mut meta = AnalyticsRelationMeta::add(buf.make_mut());

        let keypoint = meta
            .add_keypoint_mtd_from_position(
                AnalyticsKeypointPosition {
                    x: 12,
                    y: 34,
                    z: 0,
                    dimension: AnalyticsKeypointDimensions::_2d,
                },
                AnalyticsKeypointVisibility::VISIBLE,
                0.75,
            )
            .unwrap();

        let position = keypoint.position().unwrap();
        assert_eq!(position.dimension, AnalyticsKeypointDimensions::_2d);
        assert_eq!(position.x, 12);
        assert_eq!(position.y, 34);
        assert_eq!(position.z, 0);

        assert_eq!(keypoint.confidence().unwrap(), 0.75);
        assert_eq!(
            keypoint.visibility_flags().unwrap(),
            AnalyticsKeypointVisibility::VISIBLE
        );
    }
}
