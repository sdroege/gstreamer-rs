// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use std::ptr;

use crate::{ffi, relation_meta::*};

#[derive(Debug)]
pub enum AnalyticsPropertiesMtd {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::AnalyticsRelationMetaPropertiesExt> Sealed for T {}
}

pub trait AnalyticsRelationMetaPropertiesExt: sealed::Sealed {
    fn add_properties_mtd(
        &mut self,
    ) -> Result<AnalyticsMtdRef<'_, AnalyticsPropertiesMtd>, glib::BoolError>;
}

impl AnalyticsRelationMetaPropertiesExt
    for gst::MetaRefMut<'_, AnalyticsRelationMeta, gst::meta::Standalone>
{
    #[doc(alias = "gst_analytics_relation_meta_add_properties_mtd")]
    fn add_properties_mtd(
        &mut self,
    ) -> Result<AnalyticsMtdRef<'_, AnalyticsPropertiesMtd>, glib::BoolError> {
        unsafe {
            let mut mtd = std::mem::MaybeUninit::uninit();
            glib::result_from_gboolean!(
                ffi::gst_analytics_relation_meta_add_properties_mtd(
                    self.as_mut_ptr(),
                    mtd.as_mut_ptr(),
                ),
                "Couldn't add properties metadata"
            )?;
            Ok(AnalyticsMtdRef::from_meta(
                self.as_ref(),
                mtd.assume_init().id,
            ))
        }
    }
}

unsafe impl AnalyticsMtd for AnalyticsPropertiesMtd {
    #[doc(alias = "gst_analytics_properties_mtd_get_mtd_type")]
    fn mtd_type() -> ffi::GstAnalyticsMtdType {
        unsafe { ffi::gst_analytics_properties_mtd_get_mtd_type() }
    }
}

impl AnalyticsMtdRef<'_, AnalyticsPropertiesMtd> {
    #[doc(alias = "gst_analytics_properties_mtd_get_length")]
    pub fn len(&self) -> usize {
        unsafe {
            let mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            ffi::gst_analytics_properties_mtd_get_length(ptr::from_ref(&mtd).cast()) as usize
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[doc(alias = "gst_analytics_properties_mtd_get_nth_key")]
    pub fn nth_key(&self, index: usize) -> Option<glib::GString> {
        let index = u32::try_from(index).ok()?;
        unsafe {
            let mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            from_glib_none(ffi::gst_analytics_properties_mtd_get_nth_key(
                ptr::from_ref(&mtd).cast(),
                index,
            ))
        }
    }

    #[doc(alias = "gst_analytics_properties_mtd_has_key")]
    pub fn has_key(&self, key: &str) -> bool {
        unsafe {
            let mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            from_glib(ffi::gst_analytics_properties_mtd_has_key(
                ptr::from_ref(&mtd).cast(),
                key.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_analytics_properties_mtd_get_value")]
    pub fn value(&self, key: &str) -> Option<&glib::Value> {
        unsafe {
            let mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            let value = ffi::gst_analytics_properties_mtd_get_value(
                ptr::from_ref(&mtd).cast(),
                key.to_glib_none().0,
            );
            value.cast::<glib::Value>().as_ref()
        }
    }

    #[doc(alias = "gst_analytics_properties_mtd_get_string")]
    pub fn string(&self, key: &str) -> Option<&glib::GStr> {
        unsafe {
            let mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            let value = ffi::gst_analytics_properties_mtd_get_string(
                ptr::from_ref(&mtd).cast(),
                key.to_glib_none().0,
            );

            if value.is_null() {
                None
            } else {
                Some(glib::GStr::from_ptr(value))
            }
        }
    }

    #[doc(alias = "gst_analytics_properties_mtd_get_structure")]
    pub fn structure(&self) -> &gst::StructureRef {
        unsafe {
            let mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            gst::StructureRef::from_glib_borrow(ffi::gst_analytics_properties_mtd_get_structure(
                ptr::from_ref(&mtd).cast(),
            ))
        }
    }
}

impl AnalyticsMtdRefMut<'_, AnalyticsPropertiesMtd> {
    #[doc(alias = "gst_analytics_properties_mtd_set_value")]
    pub fn set_value(&mut self, key: &str, value: &glib::Value) -> Result<(), glib::BoolError> {
        unsafe {
            let mut mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            glib::result_from_gboolean!(
                ffi::gst_analytics_properties_mtd_set_value(
                    ptr::from_mut(&mut mtd).cast(),
                    key.to_glib_none().0,
                    value.to_glib_none().0,
                ),
                "Couldn't set property value"
            )
        }
    }

    #[doc(alias = "gst_analytics_properties_mtd_set_string")]
    pub fn set_string(&mut self, key: &str, value: &str) -> Result<(), glib::BoolError> {
        unsafe {
            let mut mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            glib::result_from_gboolean!(
                ffi::gst_analytics_properties_mtd_set_string(
                    ptr::from_mut(&mut mtd).cast(),
                    key.to_glib_none().0,
                    value.to_glib_none().0,
                ),
                "Couldn't set string property"
            )
        }
    }

    #[doc(alias = "gst_analytics_properties_mtd_remove_key")]
    pub fn remove_key(&mut self, key: &str) -> Result<(), glib::BoolError> {
        unsafe {
            let mut mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            glib::result_from_gboolean!(
                ffi::gst_analytics_properties_mtd_remove_key(
                    ptr::from_mut(&mut mtd).cast(),
                    key.to_glib_none().0,
                ),
                "Couldn't remove property"
            )
        }
    }
}
