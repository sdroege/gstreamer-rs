// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::{Tensor, ffi, relation_meta::*};

#[derive(Debug)]
#[doc(alias = "GstAnalyticsTensorProvenanceMtd")]
pub enum AnalyticsTensorProvenanceMtd {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::AnalyticsRelationMetaTensorProvenanceExt> Sealed for T {}
}

pub trait AnalyticsRelationMetaTensorProvenanceExt: sealed::Sealed {
    fn add_tensor_provenance_mtd(
        &mut self,
        group_id: glib::Quark,
        tensor: &Tensor,
    ) -> Result<AnalyticsMtdRef<'_, AnalyticsTensorProvenanceMtd>, glib::BoolError>;
}

impl AnalyticsRelationMetaTensorProvenanceExt
    for gst::MetaRefMut<'_, AnalyticsRelationMeta, gst::meta::Standalone>
{
    #[doc(alias = "gst_analytics_relation_meta_add_tensor_provenance_mtd")]
    fn add_tensor_provenance_mtd(
        &mut self,
        group_id: glib::Quark,
        tensor: &Tensor,
    ) -> Result<AnalyticsMtdRef<'_, AnalyticsTensorProvenanceMtd>, glib::BoolError> {
        unsafe {
            let mut mtd = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_analytics_relation_meta_add_tensor_provenance_mtd(
                self.as_mut_ptr(),
                group_id.into_glib(),
                tensor.as_ptr(),
                mtd.as_mut_ptr(),
            ));

            if ret {
                let id = mtd.assume_init().id;
                Ok(AnalyticsMtdRef::from_meta(self.as_ref(), id))
            } else {
                Err(glib::bool_error!("Couldn't add tensor provenance metadata"))
            }
        }
    }
}

unsafe impl AnalyticsMtd for AnalyticsTensorProvenanceMtd {
    #[doc(alias = "gst_analytics_tensor_provenance_mtd_get_mtd_type")]
    fn mtd_type() -> ffi::GstAnalyticsMtdType {
        unsafe { ffi::gst_analytics_tensor_provenance_mtd_get_mtd_type() }
    }
}

impl AnalyticsMtdRef<'_, AnalyticsTensorProvenanceMtd> {
    #[doc(alias = "gst_analytics_tensor_provenance_mtd_get_timestamp")]
    pub fn timestamp(
        &self,
    ) -> Result<(Option<gst::ClockTime>, Option<gst::ClockTime>), glib::BoolError> {
        unsafe {
            let mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            let mut timestamp = std::mem::MaybeUninit::uninit();
            let mut duration = std::mem::MaybeUninit::uninit();
            glib::result_from_gboolean!(
                ffi::gst_analytics_tensor_provenance_mtd_get_timestamp(
                    &mtd as *const _ as *const ffi::GstAnalyticsTensorProvenanceMtd,
                    timestamp.as_mut_ptr(),
                    duration.as_mut_ptr(),
                ),
                "Could not get timestamp"
            )?;
            Ok((
                from_glib(timestamp.assume_init()),
                from_glib(duration.assume_init()),
            ))
        }
    }

    #[doc(alias = "gst_analytics_tensor_provenance_mtd_get_correlation_id")]
    pub fn correlation_id(&self) -> i64 {
        unsafe {
            let mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            ffi::gst_analytics_tensor_provenance_mtd_get_correlation_id(
                &mtd as *const _ as *const ffi::GstAnalyticsTensorProvenanceMtd,
            )
        }
    }

    #[doc(alias = "gst_analytics_tensor_provenance_mtd_get_group_id")]
    pub fn group_id(&self) -> Option<glib::Quark> {
        unsafe {
            let mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            let id = ffi::gst_analytics_tensor_provenance_mtd_get_group_id(
                &mtd as *const _ as *const ffi::GstAnalyticsTensorProvenanceMtd,
            );
            if id == 0 { None } else { Some(from_glib(id)) }
        }
    }

    #[doc(alias = "gst_analytics_tensor_provenance_mtd_get_tensor_id")]
    pub fn tensor_id(&self) -> Option<glib::Quark> {
        unsafe {
            let mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            let id = ffi::gst_analytics_tensor_provenance_mtd_get_tensor_id(
                &mtd as *const _ as *const ffi::GstAnalyticsTensorProvenanceMtd,
            );
            if id == 0 { None } else { Some(from_glib(id)) }
        }
    }

    #[doc(alias = "gst_analytics_tensor_provenance_mtd_has_spatial_transform")]
    pub fn has_spatial_transform(&self) -> bool {
        unsafe {
            let mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            from_glib(
                ffi::gst_analytics_tensor_provenance_mtd_has_spatial_transform(
                    &mtd as *const _ as *const ffi::GstAnalyticsTensorProvenanceMtd,
                ),
            )
        }
    }

    #[doc(alias = "gst_analytics_tensor_provenance_mtd_get_spatial_transform")]
    pub fn spatial_transform(&self) -> Option<[f32; 9]> {
        unsafe {
            let mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            let mut transform = std::mem::MaybeUninit::uninit();
            let ret = from_glib(
                ffi::gst_analytics_tensor_provenance_mtd_get_spatial_transform(
                    &mtd as *const _ as *const ffi::GstAnalyticsTensorProvenanceMtd,
                    transform.as_mut_ptr(),
                ),
            );
            if ret {
                Some(transform.assume_init())
            } else {
                None
            }
        }
    }

    #[doc(alias = "gst_analytics_tensor_provenance_mtd_transform_point")]
    pub fn transform_point(&self, mut x: f32, mut y: f32) -> Option<(f32, f32)> {
        let ret: bool = unsafe {
            let mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            from_glib(ffi::gst_analytics_tensor_provenance_mtd_transform_point(
                &mtd as *const _ as *const ffi::GstAnalyticsTensorProvenanceMtd,
                &mut x,
                &mut y,
            ))
        };
        ret.then_some((x, y))
    }

    #[doc(alias = "gst_analytics_tensor_provenance_mtd_transform_rectangle")]
    pub fn transform_rectangle(
        &self,
        mut x: f32,
        mut y: f32,
        mut width: f32,
        mut height: f32,
    ) -> Option<(f32, f32, f32, f32)> {
        let ret: bool = unsafe {
            let mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            from_glib(
                ffi::gst_analytics_tensor_provenance_mtd_transform_rectangle(
                    &mtd as *const _ as *const ffi::GstAnalyticsTensorProvenanceMtd,
                    &mut x,
                    &mut y,
                    &mut width,
                    &mut height,
                ),
            )
        };
        ret.then_some((x, y, width, height))
    }
}

impl AnalyticsMtdRefMut<'_, AnalyticsTensorProvenanceMtd> {
    #[doc(alias = "gst_analytics_tensor_provenance_mtd_set_timestamp")]
    pub fn set_timestamp(
        &mut self,
        timestamp: impl Into<Option<gst::ClockTime>>,
        duration: impl Into<Option<gst::ClockTime>>,
    ) -> Result<(), glib::BoolError> {
        unsafe {
            let mut mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            glib::result_from_gboolean!(
                ffi::gst_analytics_tensor_provenance_mtd_set_timestamp(
                    &mut mtd as *mut _ as *mut ffi::GstAnalyticsTensorProvenanceMtd,
                    timestamp.into().into_glib(),
                    duration.into().into_glib(),
                ),
                "Could not set timestamp"
            )
        }
    }

    #[doc(alias = "gst_analytics_tensor_provenance_mtd_set_spatial_transform_from_rectangles")]
    pub fn set_spatial_transform_from_rectangles(
        &mut self,
        tensor_rect: &gst_video::VideoRectangle,
        video_rect: &gst_video::VideoRectangle,
    ) -> Result<(), glib::BoolError> {
        unsafe {
            let mut mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            glib::result_from_gboolean!(
                ffi::gst_analytics_tensor_provenance_mtd_set_spatial_transform_from_rectangles(
                    &mut mtd as *mut _ as *mut ffi::GstAnalyticsTensorProvenanceMtd,
                    tensor_rect.to_glib_none().0,
                    video_rect.to_glib_none().0,
                ),
                "Could not set spatial transform from rectangles"
            )
        }
    }

    #[doc(alias = "gst_analytics_tensor_provenance_mtd_set_spatial_transform")]
    pub fn set_spatial_transform(&mut self, transform: &[f32; 9]) -> Result<(), glib::BoolError> {
        unsafe {
            let mut mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            glib::result_from_gboolean!(
                ffi::gst_analytics_tensor_provenance_mtd_set_spatial_transform(
                    &mut mtd as *mut _ as *mut ffi::GstAnalyticsTensorProvenanceMtd,
                    transform,
                ),
                "Could not set spatial transform"
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn tensor_provenance() {
        gst::init().unwrap();

        assert_eq!(
            AnalyticsTensorProvenanceMtd::type_name(),
            "tensor-provenance"
        );

        let group_id = glib::Quark::from_str("test-group");
        let tensor_id = glib::Quark::from_str("test-tensor");
        let tensor = Tensor::new_simple(
            tensor_id,
            TensorDataType::Uint8,
            gst::Buffer::with_size(4).unwrap(),
            TensorDimOrder::RowMajor,
            &[2, 2],
        );
        let mut buf = gst::Buffer::new();
        let mut meta = AnalyticsRelationMeta::add(buf.make_mut());
        let provenance = meta.add_tensor_provenance_mtd(group_id, &tensor).unwrap();
        drop(tensor);

        assert_eq!(provenance.group_id(), Some(group_id));
        assert_eq!(provenance.tensor_id(), Some(tensor_id));
        assert!(!provenance.has_spatial_transform());
        assert_eq!(provenance.spatial_transform(), None);
        assert!(provenance.transform_point(1.0, 2.0).is_none());

        let id = provenance.id();
        let mut provenance_mut = meta.mtd_mut::<AnalyticsTensorProvenanceMtd>(id).unwrap();
        let transform = [2.0, 0.0, 10.0, 0.0, 3.0, 20.0, 0.0, 0.0, 1.0];
        provenance_mut.set_spatial_transform(&transform).unwrap();

        let provenance: AnalyticsMtdRef<_> = provenance_mut.into();
        assert!(provenance.has_spatial_transform());
        assert_eq!(provenance.spatial_transform(), Some(transform));
        assert_eq!(provenance.transform_point(1.0, 2.0).unwrap(), (12.0, 26.0));
        assert_eq!(
            provenance.transform_rectangle(1.0, 2.0, 4.0, 5.0).unwrap(),
            (12.0, 26.0, 8.0, 15.0)
        );
    }
}
