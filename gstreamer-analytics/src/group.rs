// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use std::marker::PhantomData;

use crate::{AnalyticsKeypointDimensions, AnalyticsKeypointPosition, ffi, relation_meta::*};

#[derive(Debug)]
pub enum AnalyticsGroupMtd {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::AnalyticsRelationMetaGroupExt> Sealed for T {}
}

pub trait AnalyticsRelationMetaGroupExt: sealed::Sealed {
    fn add_group_mtd(
        &mut self,
        pre_alloc_size: usize,
    ) -> Result<AnalyticsMtdRef<'_, AnalyticsGroupMtd>, glib::BoolError>;

    fn add_group_mtd_with_size(
        &mut self,
        group_size: usize,
    ) -> Result<AnalyticsMtdRef<'_, AnalyticsGroupMtd>, glib::BoolError>;

    fn add_keypoints_group(
        &mut self,
        semantic_tag: &str,
        dimension: AnalyticsKeypointDimensions,
        positions: &[i32],
        confidences: Option<&[f32]>,
        visibilities: Option<&[u8]>,
        skeleton_pairs: &[i32],
    ) -> Result<AnalyticsMtdRef<'_, AnalyticsGroupMtd>, glib::BoolError>;

    fn add_keypoints_group_from_positions(
        &mut self,
        semantic_tag: &str,
        positions: &[AnalyticsKeypointPosition],
        confidences: Option<&[f32]>,
        visibilities: Option<&[u8]>,
        skeleton_pairs: &[i32],
    ) -> Result<AnalyticsMtdRef<'_, AnalyticsGroupMtd>, glib::BoolError> {
        if positions.is_empty() {
            return Err(glib::bool_error!("No keypoint positions provided"));
        }

        let dimension = positions[0].dimension;

        if positions
            .iter()
            .any(|position| position.dimension != dimension)
        {
            return Err(glib::bool_error!(
                "All keypoint positions must use the same dimension"
            ));
        }

        let coords_per_keypoint = match dimension {
            AnalyticsKeypointDimensions::_2d => 2,
            AnalyticsKeypointDimensions::_3d => 3,
            _ => {
                return Err(glib::bool_error!(
                    "Unsupported keypoint dimension for positions"
                ));
            }
        };

        let mut flattened_positions = Vec::with_capacity(positions.len() * coords_per_keypoint);
        for position in positions {
            flattened_positions.push(position.x);
            flattened_positions.push(position.y);
            if coords_per_keypoint == 3 {
                flattened_positions.push(position.z);
            }
        }

        self.add_keypoints_group(
            semantic_tag,
            dimension,
            &flattened_positions,
            confidences,
            visibilities,
            skeleton_pairs,
        )
    }
}

impl AnalyticsRelationMetaGroupExt
    for gst::MetaRefMut<'_, AnalyticsRelationMeta, gst::meta::Standalone>
{
    #[doc(alias = "gst_analytics_relation_meta_add_group_mtd")]
    fn add_group_mtd(
        &mut self,
        pre_alloc_size: usize,
    ) -> Result<AnalyticsMtdRef<'_, AnalyticsGroupMtd>, glib::BoolError> {
        unsafe {
            let mut mtd = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_analytics_relation_meta_add_group_mtd(
                self.as_mut_ptr(),
                pre_alloc_size,
                mtd.as_mut_ptr(),
            ));
            let id = mtd.assume_init().id;

            if ret {
                Ok(AnalyticsMtdRef::from_meta(self.as_ref(), id))
            } else {
                Err(glib::bool_error!("Couldn't add group metadata"))
            }
        }
    }

    #[doc(alias = "gst_analytics_relation_meta_add_group_mtd_with_size")]
    fn add_group_mtd_with_size(
        &mut self,
        group_size: usize,
    ) -> Result<AnalyticsMtdRef<'_, AnalyticsGroupMtd>, glib::BoolError> {
        unsafe {
            let mut mtd = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_analytics_relation_meta_add_group_mtd_with_size(
                self.as_mut_ptr(),
                group_size,
                mtd.as_mut_ptr(),
            ));
            let id = mtd.assume_init().id;

            if ret {
                Ok(AnalyticsMtdRef::from_meta(self.as_ref(), id))
            } else {
                Err(glib::bool_error!("Couldn't add group metadata"))
            }
        }
    }

    #[doc(alias = "gst_analytics_relation_meta_add_keypoints_group")]
    fn add_keypoints_group(
        &mut self,
        semantic_tag: &str,
        dimension: AnalyticsKeypointDimensions,
        positions: &[i32],
        confidences: Option<&[f32]>,
        visibilities: Option<&[u8]>,
        skeleton_pairs: &[i32],
    ) -> Result<AnalyticsMtdRef<'_, AnalyticsGroupMtd>, glib::BoolError> {
        let coords_per_keypoint = match dimension {
            AnalyticsKeypointDimensions::_2d => 2,
            AnalyticsKeypointDimensions::_3d => 3,
            _ => {
                return Err(glib::bool_error!(
                    "Unsupported keypoint dimension for positions"
                ));
            }
        };

        if positions.is_empty() {
            return Err(glib::bool_error!("No keypoint positions provided"));
        }

        if !positions.len().is_multiple_of(coords_per_keypoint) {
            return Err(glib::bool_error!(
                "Positions length must match the keypoint dimension"
            ));
        }

        let keypoint_count = positions.len() / coords_per_keypoint;

        if let Some(confidences) = confidences
            && confidences.len() != keypoint_count
        {
            return Err(glib::bool_error!(
                "Confidences length must match keypoint count"
            ));
        }

        if let Some(visibilities) = visibilities
            && visibilities.len() != keypoint_count
        {
            return Err(glib::bool_error!(
                "Visibilities length must match keypoint count"
            ));
        }

        unsafe {
            let mut mtd = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_analytics_relation_meta_add_keypoints_group(
                self.as_mut_ptr(),
                semantic_tag.to_glib_none().0,
                dimension.into_glib(),
                positions.len(),
                positions.as_ptr(),
                keypoint_count,
                confidences.map_or(std::ptr::null(), |confidences| confidences.as_ptr()),
                visibilities.map_or(std::ptr::null(), |visibilities| visibilities.as_ptr()),
                skeleton_pairs.len(),
                skeleton_pairs.as_ptr(),
                mtd.as_mut_ptr(),
            ));
            let id = mtd.assume_init().id;

            if ret {
                Ok(AnalyticsMtdRef::from_meta(self.as_ref(), id))
            } else {
                Err(glib::bool_error!("Couldn't add keypoints group metadata"))
            }
        }
    }
}

impl AnalyticsMtdRef<'_, AnalyticsGroupMtd> {
    #[doc(alias = "gst_analytics_group_mtd_has_semantic_tag")]
    pub fn has_semantic_tag(&self, tag: &str) -> Result<bool, glib::BoolError> {
        unsafe {
            let mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            Ok(from_glib(ffi::gst_analytics_group_mtd_has_semantic_tag(
                &mtd as *const _ as *const ffi::GstAnalyticsGroupMtd,
                tag.to_glib_none().0,
            )))
        }
    }

    #[doc(alias = "gst_analytics_group_mtd_semantic_tag_has_prefix")]
    pub fn semantic_tag_has_prefix(&self, prefix: &str) -> Result<bool, glib::BoolError> {
        unsafe {
            let mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            Ok(from_glib(
                ffi::gst_analytics_group_mtd_semantic_tag_has_prefix(
                    &mtd as *const _ as *const ffi::GstAnalyticsGroupMtd,
                    prefix.to_glib_none().0,
                ),
            ))
        }
    }

    #[doc(alias = "gst_analytics_group_mtd_get_member_count")]
    pub fn member_count(&self) -> usize {
        unsafe {
            let mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            ffi::gst_analytics_group_mtd_get_member_count(
                &mtd as *const _ as *const ffi::GstAnalyticsGroupMtd,
            ) as usize
        }
    }

    #[doc(alias = "gst_analytics_group_mtd_get_member")]
    pub fn member(&self, index: usize) -> Option<AnalyticsMtdRef<'_, AnalyticsAnyMtd>> {
        if index >= self.member_count() {
            return None;
        }

        unsafe {
            let mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            let mut member = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_analytics_group_mtd_get_member(
                &mtd as *const _ as *const ffi::GstAnalyticsGroupMtd,
                index,
                member.as_mut_ptr(),
            ));

            if ret {
                let member = member.assume_init();
                let id = ffi::gst_analytics_mtd_get_id(&member);
                Some(AnalyticsMtdRef::from_meta(self.meta_ref(), id))
            } else {
                None
            }
        }
    }

    pub fn member_typed<T: AnalyticsMtd>(&self, index: usize) -> Option<AnalyticsMtdRef<'_, T>> {
        self.member(index)
            .and_then(|member| member.downcast::<T>().ok())
    }

    #[doc(alias = "gst_analytics_group_mtd_iterate")]
    pub fn iter<T: AnalyticsMtd>(&self) -> AnalyticsGroupMtdIter<'_, T> {
        AnalyticsGroupMtdIter::new(self)
    }
}

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct AnalyticsGroupMtdIter<'a, T: AnalyticsMtd> {
    group: &'a AnalyticsMtdRef<'a, AnalyticsGroupMtd>,
    state: glib::ffi::gpointer,
    phantom: PhantomData<T>,
}

impl<'a, T: AnalyticsMtd> AnalyticsGroupMtdIter<'a, T> {
    fn new(group: &'a AnalyticsMtdRef<'a, AnalyticsGroupMtd>) -> Self {
        skip_assert_initialized!();
        AnalyticsGroupMtdIter {
            group,
            state: std::ptr::null_mut(),
            phantom: PhantomData,
        }
    }
}

impl<'a, T: AnalyticsMtd + 'a> Iterator for AnalyticsGroupMtdIter<'a, T> {
    type Item = AnalyticsMtdRef<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let mtd = ffi::GstAnalyticsMtd::unsafe_from(self.group);
            let mut member = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_analytics_group_mtd_iterate(
                &mtd as *const _ as *const ffi::GstAnalyticsGroupMtd,
                &mut self.state,
                T::mtd_type(),
                member.as_mut_ptr(),
            ));

            if ret {
                let member = member.assume_init();
                let id = ffi::gst_analytics_mtd_get_id(&member);
                Some(AnalyticsMtdRef::from_meta(self.group.meta_ref(), id))
            } else {
                None
            }
        }
    }
}

unsafe impl AnalyticsMtd for AnalyticsGroupMtd {
    #[doc(alias = "gst_analytics_group_mtd_get_mtd_type")]
    fn mtd_type() -> ffi::GstAnalyticsMtdType {
        unsafe { ffi::gst_analytics_group_mtd_get_mtd_type() }
    }
}

impl AnalyticsMtdRefMut<'_, AnalyticsGroupMtd> {
    #[doc(alias = "gst_analytics_group_mtd_add_member")]
    pub fn add_member(&mut self, an_meta_id: u32) -> Result<(), glib::BoolError> {
        let ret = unsafe {
            let mut mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            from_glib(ffi::gst_analytics_group_mtd_add_member(
                &mut mtd as *mut _ as *mut ffi::GstAnalyticsGroupMtd,
                an_meta_id,
            ))
        };

        if ret {
            Ok(())
        } else {
            Err(glib::bool_error!("Couldn't add group member"))
        }
    }

    #[doc(alias = "gst_analytics_group_mtd_set_semantic_tag")]
    pub fn set_semantic_tag(&mut self, tag: &str) -> Result<(), glib::BoolError> {
        let ret = unsafe {
            let mut mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            from_glib(ffi::gst_analytics_group_mtd_set_semantic_tag(
                &mut mtd as *mut _ as *mut ffi::GstAnalyticsGroupMtd,
                tag.to_glib_none().0,
            ))
        };

        if ret {
            Ok(())
        } else {
            Err(glib::bool_error!("Couldn't set semantic tag"))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn group_members() {
        gst::init().unwrap();

        let type_name = AnalyticsGroupMtd::type_name();
        assert_eq!(type_name, "grouping-mtd");

        let mut buf = gst::Buffer::new();
        let mut meta = AnalyticsRelationMeta::add(buf.make_mut());

        let keypoint_id = {
            let keypoint = meta
                .add_keypoint_mtd_from_position(
                    AnalyticsKeypointPosition {
                        x: 1,
                        y: 2,
                        z: 0,
                        dimension: AnalyticsKeypointDimensions::_2d,
                    },
                    AnalyticsKeypointVisibility::VISIBLE,
                    0.5,
                )
                .unwrap();
            keypoint.id()
        };

        let group = meta.add_group_mtd_with_size(1).unwrap();
        let group_id = group.id();

        let mut group_mut = meta.mtd_mut::<AnalyticsGroupMtd>(group_id).unwrap();
        group_mut.set_semantic_tag("pose").unwrap();
        group_mut.add_member(keypoint_id).unwrap();

        let group = AnalyticsMtdRef::from(group_mut);
        assert!(group.has_semantic_tag("pose").unwrap());
        assert!(group.semantic_tag_has_prefix("po").unwrap());
        assert_eq!(group.member_count(), 1);

        let member = group.member_typed::<AnalyticsKeypointMtd>(0).unwrap();
        let position = member.position().unwrap();
        assert_eq!(position.x, 1);
        assert_eq!(position.y, 2);
    }

    #[test]
    fn keypoints_group() {
        gst::init().unwrap();

        let mut buf = gst::Buffer::new();
        let mut meta = AnalyticsRelationMeta::add(buf.make_mut());

        let positions = [
            AnalyticsKeypointPosition {
                x: 10,
                y: 20,
                z: 0,
                dimension: AnalyticsKeypointDimensions::_2d,
            },
            AnalyticsKeypointPosition {
                x: 30,
                y: 40,
                z: 0,
                dimension: AnalyticsKeypointDimensions::_2d,
            },
        ];
        let confidences = [0.9, 0.8];
        let visibilities = [1, 0];

        let group = meta
            .add_keypoints_group_from_positions(
                "pose",
                &positions,
                Some(&confidences),
                Some(&visibilities),
                &[],
            )
            .unwrap();

        assert!(group.has_semantic_tag("pose").unwrap());
        assert!(group.semantic_tag_has_prefix("po").unwrap());
        assert_eq!(group.member_count(), 2);
    }

    #[test]
    fn keypoints_group_rejects_mismatched_confidences() {
        gst::init().unwrap();

        let mut buf = gst::Buffer::new();
        let mut meta = AnalyticsRelationMeta::add(buf.make_mut());

        let positions = [
            AnalyticsKeypointPosition {
                x: 10,
                y: 20,
                z: 0,
                dimension: AnalyticsKeypointDimensions::_2d,
            },
            AnalyticsKeypointPosition {
                x: 30,
                y: 40,
                z: 0,
                dimension: AnalyticsKeypointDimensions::_2d,
            },
        ];
        let confidences = [0.9];

        let result = meta.add_keypoints_group_from_positions(
            "pose",
            &positions,
            Some(&confidences),
            None,
            &[],
        );

        assert!(result.is_err());
    }

    #[test]
    fn keypoints_group_rejects_mismatched_visibilities() {
        gst::init().unwrap();

        let mut buf = gst::Buffer::new();
        let mut meta = AnalyticsRelationMeta::add(buf.make_mut());

        let positions = [
            AnalyticsKeypointPosition {
                x: 10,
                y: 20,
                z: 0,
                dimension: AnalyticsKeypointDimensions::_2d,
            },
            AnalyticsKeypointPosition {
                x: 30,
                y: 40,
                z: 0,
                dimension: AnalyticsKeypointDimensions::_2d,
            },
        ];
        let visibilities = [1];

        let result = meta.add_keypoints_group_from_positions(
            "pose",
            &positions,
            None,
            Some(&visibilities),
            &[],
        );

        assert!(result.is_err());
    }
}
