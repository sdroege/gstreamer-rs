// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::{ffi, relation_meta::*};

#[derive(Debug)]
pub enum AnalyticsTextMtd {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::AnalyticsRelationMetaTextExt> Sealed for T {}
}

pub trait AnalyticsRelationMetaTextExt: sealed::Sealed {
    fn add_text_mtd(
        &mut self,
        text: &str,
        confidence: f32,
    ) -> Result<AnalyticsMtdRef<'_, AnalyticsTextMtd>, glib::BoolError>;
}

impl<'a> AnalyticsRelationMetaTextExt
    for gst::MetaRefMut<'a, AnalyticsRelationMeta, gst::meta::Standalone>
{
    #[doc(alias = "gst_analytics_relation_meta_add_text_mtd")]
    fn add_text_mtd(
        &mut self,
        text: &str,
        confidence: f32,
    ) -> Result<AnalyticsMtdRef<'a, AnalyticsTextMtd>, glib::BoolError> {
        unsafe {
            let mut mtd = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_analytics_relation_meta_add_text_mtd(
                self.as_mut_ptr(),
                text.to_glib_none().0,
                confidence,
                mtd.as_mut_ptr(),
            ));

            if ret {
                let id = mtd.assume_init().id;
                Ok(AnalyticsMtdRef::from_meta(self.as_ref(), id))
            } else {
                Err(glib::bool_error!("Couldn't add text metadata"))
            }
        }
    }
}

unsafe impl AnalyticsMtd for AnalyticsTextMtd {
    #[doc(alias = "gst_analytics_text_mtd_get_mtd_type")]
    fn mtd_type() -> ffi::GstAnalyticsMtdType {
        unsafe { ffi::gst_analytics_text_mtd_get_mtd_type() }
    }
}

impl AnalyticsMtdRef<'_, AnalyticsTextMtd> {
    #[doc(alias = "gst_analytics_text_mtd_get_text")]
    pub fn text(&self) -> &str {
        unsafe {
            let mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            let text = ffi::gst_analytics_text_mtd_get_text(
                &mtd as *const _ as *const ffi::GstAnalyticsTextMtd,
            );

            std::ffi::CStr::from_ptr(text).to_str().unwrap()
        }
    }

    #[doc(alias = "gst_analytics_text_mtd_get_length")]
    pub fn length(&self) -> usize {
        unsafe {
            let mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            ffi::gst_analytics_text_mtd_get_length(
                &mtd as *const _ as *const ffi::GstAnalyticsTextMtd,
            )
        }
    }

    #[doc(alias = "gst_analytics_text_mtd_get_confidence")]
    pub fn confidence(&self) -> f32 {
        unsafe {
            let mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            ffi::gst_analytics_text_mtd_get_confidence(
                &mtd as *const _ as *const ffi::GstAnalyticsTextMtd,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn text() {
        gst::init().unwrap();

        assert_eq!(AnalyticsTextMtd::type_name(), "text");

        let mut buf = gst::Buffer::new();
        let mut meta = AnalyticsRelationMeta::add(buf.make_mut());

        assert!(meta.is_empty());

        let text = meta.add_text_mtd("Hello, world!", 0.9f32).unwrap();

        assert_eq!(text.text(), "Hello, world!");
        assert_eq!(text.length(), "Hello, world!".len());
        assert_eq!(text.confidence(), 0.9f32);
    }
}
