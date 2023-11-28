// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::relation_meta::*;

#[derive(Debug)]
pub enum AnalyticsTrackingMtd {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::AnalyticsRelationMetaTrackingExt> Sealed for T {}
}

pub trait AnalyticsRelationMetaTrackingExt: sealed::Sealed {
    fn add_tracking_mtd(
        &mut self,
        tracking_id: u64,
        tracking_first_seen: gst::ClockTime,
    ) -> Result<AnalyticsMtdRef<AnalyticsTrackingMtd>, glib::BoolError>;
}

impl<'a> AnalyticsRelationMetaTrackingExt
    for gst::MetaRefMut<'a, AnalyticsRelationMeta, gst::meta::Standalone>
{
    #[doc(alias = "gst_analytics_relation_meta_add_tracking_mtd")]
    fn add_tracking_mtd(
        &mut self,
        tracking_id: u64,
        tracking_first_seen: gst::ClockTime,
    ) -> Result<AnalyticsMtdRef<AnalyticsTrackingMtd>, glib::BoolError> {
        unsafe {
            let mut mtd = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_analytics_relation_meta_add_tracking_mtd(
                self.as_mut_ptr(),
                tracking_id,
                tracking_first_seen.into_glib(),
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

unsafe impl AnalyticsMtd for AnalyticsTrackingMtd {
    #[doc(alias = "gst_analytics_tracking_mtd_get_mtd_type")]
    fn mtd_type() -> ffi::GstAnalyticsMtdType {
        unsafe { ffi::gst_analytics_tracking_mtd_get_mtd_type() }
    }
}

unsafe fn from(t: ffi::GstAnalyticsMtd) -> ffi::GstAnalyticsTrackingMtd {
    std::mem::transmute(t)
}

impl<'a> AnalyticsMtdRef<'a, AnalyticsTrackingMtd> {
    #[doc(alias = "gst_analytics_tracking_mtd_get_info")]
    pub fn info(&self) -> (u64, gst::ClockTime, gst::ClockTime, bool) {
        let mut tracking_id: u64 = 0;
        let mut tracking_first_seen: u64 = 0;
        let mut tracking_last_seen: u64 = 0;
        let mut tracking_lost: i32 = 0;

        unsafe {
            let mut mtd = from(ffi::GstAnalyticsMtd::unsafe_from(self));
            ffi::gst_analytics_tracking_mtd_get_info(
                &mut mtd,
                &mut tracking_id,
                &mut tracking_first_seen,
                &mut tracking_last_seen,
                &mut tracking_lost,
            );
        };

        (
            tracking_id,
            gst::ClockTime::from_nseconds(tracking_first_seen),
            gst::ClockTime::from_nseconds(tracking_last_seen),
            tracking_lost != 0,
        )
    }
}

impl<'a> AnalyticsMtdRefMut<'a, AnalyticsTrackingMtd> {
    #[doc(alias = "gst_analytics_tracking_mtd_update_last_seen")]
    pub fn update_last_seen(&mut self, last_seen: gst::ClockTime) -> Result<(), glib::BoolError> {
        let ret: bool = unsafe {
            let mut mtd = from(ffi::GstAnalyticsMtd::unsafe_from(self));
            from_glib(ffi::gst_analytics_tracking_mtd_update_last_seen(
                &mut mtd,
                last_seen.into_glib(),
            ))
        };
        assert!(ret);
        Ok(())
    }

    #[doc(alias = "gst_analytics_tracking_mtd_set_lost")]
    pub fn set_lost(&mut self) -> Result<(), glib::BoolError> {
        let ret: bool = unsafe {
            let mut mtd = from(ffi::GstAnalyticsMtd::unsafe_from(self));
            from_glib(ffi::gst_analytics_tracking_mtd_set_lost(&mut mtd))
        };
        assert!(ret);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn tracking() {
        gst::init().unwrap();

        assert_eq!(AnalyticsTrackingMtd::type_name(), "object-tracking");

        let mut buf = gst::Buffer::new();
        let mut meta = AnalyticsRelationMeta::add(buf.make_mut());

        assert!(meta.is_empty());

        let track = meta
            .add_tracking_mtd(10, gst::ClockTime::from_seconds(10))
            .unwrap();

        let (tracking_id, tracking_first_seen, tracking_last_seen, tracking_lost) = track.info();

        assert_eq!(tracking_id, 10);
        assert_eq!(tracking_first_seen, gst::ClockTime::from_seconds(10));
        assert_eq!(tracking_last_seen, gst::ClockTime::from_seconds(10));
        assert!(!tracking_lost);

        let track_id = track.id();

        let mut tracking_mut = meta.mtd_mut::<AnalyticsTrackingMtd>(track_id).unwrap();

        tracking_mut
            .update_last_seen(gst::ClockTime::from_seconds(20))
            .unwrap();
        tracking_mut.set_lost().unwrap();

        let tracking: AnalyticsMtdRef<_> = tracking_mut.into();
        let (tracking_id, tracking_first_seen, tracking_last_seen, tracking_lost) = tracking.info();

        assert_eq!(tracking_id, 10);
        assert_eq!(tracking_first_seen, gst::ClockTime::from_seconds(10));
        assert_eq!(tracking_last_seen, gst::ClockTime::from_seconds(20));
        assert!(tracking_lost);
    }
}
