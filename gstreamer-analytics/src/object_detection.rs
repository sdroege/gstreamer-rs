// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::relation_meta::*;

#[derive(Debug)]
pub enum AnalyticsODMtd {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::AnalyticsRelationMetaODExt> Sealed for T {}
}

pub trait AnalyticsRelationMetaODExt: sealed::Sealed {
    fn add_od_mtd(
        &mut self,
        type_: glib::Quark,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        loc_conf_lvl: f32,
    ) -> Result<AnalyticsMtdRef<AnalyticsODMtd>, glib::BoolError>;
}

impl<'a> AnalyticsRelationMetaODExt
    for gst::MetaRefMut<'a, AnalyticsRelationMeta, gst::meta::Standalone>
{
    #[doc(alias = "gst_analytics_relation_meta_add_od_mtd")]
    fn add_od_mtd(
        &mut self,
        type_: glib::Quark,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        loc_conf_lvl: f32,
    ) -> Result<AnalyticsMtdRef<AnalyticsODMtd>, glib::BoolError> {
        unsafe {
            let mut mtd = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_analytics_relation_meta_add_od_mtd(
                self.as_mut_ptr(),
                type_.into_glib(),
                x,
                y,
                w,
                h,
                loc_conf_lvl,
                mtd.as_mut_ptr(),
            ));
            let id = mtd.assume_init().id;

            if ret {
                Ok(AnalyticsMtdRef::from_meta(self.as_ref(), id))
            } else {
                Err(glib::bool_error!("Couldn't add more data"))
            }
        }
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct AnalyticsODLocation {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub loc_conf_lvl: f32,
}

unsafe impl AnalyticsMtd for AnalyticsODMtd {
    #[doc(alias = "gst_analytics_od_mtd_get_mtd_type")]
    fn mtd_type() -> ffi::GstAnalyticsMtdType {
        unsafe { ffi::gst_analytics_od_mtd_get_mtd_type() }
    }
}

unsafe fn from(t: ffi::GstAnalyticsMtd) -> ffi::GstAnalyticsODMtd {
    std::mem::transmute(t)
}

impl<'a> AnalyticsMtdRef<'a, AnalyticsODMtd> {
    #[doc(alias = "gst_analytics_od_mtd_get_obj_type")]
    pub fn obj_type(&self) -> glib::Quark {
        unsafe {
            let mut mtd = from(ffi::GstAnalyticsMtd::unsafe_from(self));
            let type_ = ffi::gst_analytics_od_mtd_get_obj_type(&mut mtd);
            glib::Quark::from_glib(type_)
        }
    }

    #[doc(alias = "gst_analytics_od_mtd_get_location")]
    pub fn location(&self) -> Result<AnalyticsODLocation, glib::BoolError> {
        let mut loc = AnalyticsODLocation::default();

        let success = unsafe {
            let mut mtd = from(ffi::GstAnalyticsMtd::unsafe_from(self));
            ffi::gst_analytics_od_mtd_get_location(
                &mut mtd,
                &mut loc.x,
                &mut loc.y,
                &mut loc.w,
                &mut loc.h,
                &mut loc.loc_conf_lvl,
            )
        };

        if success != 0 {
            Ok(loc)
        } else {
            Err(glib::bool_error!("Could retrieve location"))
        }
    }

    #[doc(alias = "gst_analytics_od_mtd_get_confidence_lvl")]
    pub fn confidence_level(&self) -> f32 {
        unsafe {
            let mut mtd = from(ffi::GstAnalyticsMtd::unsafe_from(self));
            let mut lvl: f32 = 0.0;
            ffi::gst_analytics_od_mtd_get_confidence_lvl(&mut mtd, &mut lvl);
            lvl
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn object_detection() {
        gst::init().unwrap();

        assert_eq!(AnalyticsODMtd::type_name(), "object-detection");

        let mut buf = gst::Buffer::new();
        let mut meta = AnalyticsRelationMeta::add(buf.make_mut());

        assert!(meta.is_empty());

        let od = meta
            .add_od_mtd(glib::Quark::from_str("blb"), 0, 1, 10, 20, 0.8)
            .unwrap();

        assert_eq!(od.obj_type(), glib::Quark::from_str("blb"));

        let loc = od.location().unwrap();

        assert_eq!(loc.x, 0);
        assert_eq!(loc.y, 1);
        assert_eq!(loc.w, 10);
        assert_eq!(loc.h, 20);
        assert_eq!(loc.loc_conf_lvl, 0.8);
        let meta = buf.meta::<AnalyticsRelationMeta>().unwrap();

        assert!(meta.mtd::<AnalyticsODMtd>(1).is_none());

        let meta2 = buf.meta::<AnalyticsRelationMeta>().unwrap();
        let od2 = meta2.mtd::<AnalyticsODMtd>(0).unwrap();

        assert_eq!(od2.obj_type(), glib::Quark::from_str("blb"));
        let loc = od2.location().unwrap();

        assert_eq!(loc.x, 0);
        assert_eq!(loc.y, 1);
        assert_eq!(loc.w, 10);
        assert_eq!(loc.h, 20);
        assert_eq!(loc.loc_conf_lvl, 0.8);
    }
}
