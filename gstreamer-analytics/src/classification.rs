// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::relation_meta::*;

#[derive(Debug)]
pub enum AnalyticsClassificationMtd {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::AnalyticsRelationMetaClassificationExt> Sealed for T {}
}

pub trait AnalyticsRelationMetaClassificationExt: sealed::Sealed {
    fn add_one_cls_mtd(
        &mut self,
        confidence_level: f32,
        class_quark: glib::Quark,
    ) -> Result<AnalyticsMtdRef<AnalyticsClassificationMtd>, glib::BoolError>;

    fn add_cls_mtd(
        &mut self,
        confidence_levels: &[f32],
        class_quarks: &[glib::Quark],
    ) -> Result<AnalyticsMtdRef<AnalyticsClassificationMtd>, glib::BoolError>;
}

impl<'a> AnalyticsRelationMetaClassificationExt
    for gst::MetaRefMut<'a, AnalyticsRelationMeta, gst::meta::Standalone>
{
    #[doc(alias = "gst_analytics_relation_meta_add_one_cls_mtd")]
    fn add_one_cls_mtd(
        &mut self,
        confidence_level: f32,
        class_quark: glib::Quark,
    ) -> Result<AnalyticsMtdRef<'a, AnalyticsClassificationMtd>, glib::BoolError> {
        unsafe {
            let mut mtd = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_analytics_relation_meta_add_one_cls_mtd(
                self.as_mut_ptr(),
                confidence_level,
                class_quark.into_glib(),
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

    #[doc(alias = "gst_analytics_relation_meta_add_cls_mtd")]
    fn add_cls_mtd(
        &mut self,
        confidence_levels: &[f32],
        class_quarks: &[glib::Quark],
    ) -> Result<AnalyticsMtdRef<'a, AnalyticsClassificationMtd>, glib::BoolError> {
        let length = std::cmp::min(confidence_levels.len(), class_quarks.len());
        unsafe {
            let mut mtd = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_analytics_relation_meta_add_cls_mtd(
                self.as_mut_ptr(),
                length,
                mut_override(confidence_levels.as_ptr()),
                class_quarks.as_ptr() as *mut _,
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

unsafe impl AnalyticsMtd for AnalyticsClassificationMtd {
    #[doc(alias = "gst_analytics_cls_mtd_get_mtd_type")]
    fn mtd_type() -> ffi::GstAnalyticsMtdType {
        unsafe { ffi::gst_analytics_cls_mtd_get_mtd_type() }
    }
}

unsafe fn from(t: ffi::GstAnalyticsMtd) -> ffi::GstAnalyticsClsMtd {
    std::mem::transmute(t)
}

impl<'a> AnalyticsMtdRef<'a, AnalyticsClassificationMtd> {
    #[doc(alias = "gst_analytics_cls_mtd_get_length")]
    pub fn len(&self) -> usize {
        unsafe {
            let mut mtd = from(ffi::GstAnalyticsMtd::unsafe_from(self));
            ffi::gst_analytics_cls_mtd_get_length(&mut mtd)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[doc(alias = "gst_analytics_cls_mtd_get_level")]
    pub fn level(&self, index: usize) -> f32 {
        assert!(index < self.len());

        unsafe {
            let mut mtd = from(ffi::GstAnalyticsMtd::unsafe_from(self));
            ffi::gst_analytics_cls_mtd_get_level(&mut mtd, index)
        }
    }

    #[doc(alias = "gst_analytics_cls_mtd_get_quark")]
    pub fn quark(&self, index: usize) -> glib::Quark {
        assert!(index < self.len());

        unsafe {
            let mut mtd = from(ffi::GstAnalyticsMtd::unsafe_from(self));
            from_glib(ffi::gst_analytics_cls_mtd_get_quark(&mut mtd, index))
        }
    }

    pub fn iterate(&self) -> AnalyticsClassificationIterator {
        AnalyticsClassificationIterator {
            mtd: self,
            index: 0,
            length: self.len(),
        }
    }
}

pub struct AnalyticsClassificationIterator<'a> {
    mtd: &'a AnalyticsMtdRef<'a, AnalyticsClassificationMtd>,
    index: usize,
    length: usize,
}

impl<'a> Iterator for AnalyticsClassificationIterator<'a> {
    type Item = (glib::Quark, f32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.length {
            None
        } else {
            let ret = Some((self.mtd.quark(self.index), self.mtd.level(self.index)));
            self.index += 1;
            ret
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn one_class() {
        gst::init().unwrap();

        assert_eq!(AnalyticsClassificationMtd::type_name(), "classification");

        let mut buf = gst::Buffer::new();
        let mut meta = AnalyticsRelationMeta::add(buf.make_mut());

        assert!(meta.is_empty());

        let cls = meta
            .add_one_cls_mtd(0.7, glib::Quark::from_str("class1"))
            .unwrap();

        assert_eq!(cls.len(), 1);
        assert_eq!(cls.level(0), 0.7);
        assert_eq!(cls.quark(0), glib::Quark::from_str("class1"));
    }

    #[test]
    fn many_classes() {
        gst::init().unwrap();

        assert_eq!(AnalyticsClassificationMtd::type_name(), "classification");

        let mut buf = gst::Buffer::new();
        let mut meta = AnalyticsRelationMeta::add(buf.make_mut());

        assert!(meta.is_empty());

        let classes = [
            glib::Quark::from_str("a"),
            glib::Quark::from_str("b"),
            glib::Quark::from_str("c"),
            glib::Quark::from_str("d"),
        ];
        let levels = [0.1, 0.2, 0.3, 0.4];

        let cls = meta.add_cls_mtd(&levels, &classes).unwrap();

        assert_eq!(cls.len(), 4);
        for i in 0..4usize {
            assert_eq!(cls.level(i), levels[i]);
            assert_eq!(cls.quark(i), classes[i]);
        }

        for (i, (q, l)) in cls.iterate().enumerate() {
            assert_eq!(l, levels[i]);
            assert_eq!(q, classes[i]);
        }
    }
}
