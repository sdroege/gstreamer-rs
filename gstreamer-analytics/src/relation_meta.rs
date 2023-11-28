// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use gst::prelude::*;
use std::marker::PhantomData;

use crate::RelTypes;

#[repr(transparent)]
#[doc(alias = "GstAnalyticsRelationMeta")]
pub struct AnalyticsRelationMeta(ffi::GstAnalyticsRelationMeta);

unsafe impl Send for AnalyticsRelationMeta {}
unsafe impl Sync for AnalyticsRelationMeta {}

#[derive(Debug, Copy, Clone)]
#[doc(alias = "GstAnalyticsRelationMetaInitParams")]
pub struct AnalyticsRelationMetaInitParams(ffi::GstAnalyticsRelationMetaInitParams);

impl Default for AnalyticsRelationMetaInitParams {
    fn default() -> Self {
        Self(ffi::GstAnalyticsRelationMetaInitParams {
            initial_relation_order: 0,
            initial_buf_size: 0,
        })
    }
}

impl AnalyticsRelationMetaInitParams {
    pub fn new(initial_relation_order: usize, initial_buf_size: usize) -> Self {
        skip_assert_initialized!();
        Self(ffi::GstAnalyticsRelationMetaInitParams {
            initial_relation_order,
            initial_buf_size,
        })
    }
}

#[derive(Debug, Clone)]
pub struct AnalyticsMtdRef<'a, T: AnalyticsMtd> {
    id: u32,
    meta: gst::MetaRef<'a, AnalyticsRelationMeta>,
    mtd_type: PhantomData<&'a T>,
}

#[derive(Debug)]
pub struct AnalyticsMtdRefMut<'a, T: AnalyticsMtd> {
    id: u32,
    meta: &'a mut gst::MetaRefMut<'a, AnalyticsRelationMeta, gst::meta::Standalone>,
    mtd_type: PhantomData<&'a T>,
}

pub struct AnalyticsRelationPath {
    garray: *mut glib::ffi::GArray,
}

impl std::fmt::Debug for AnalyticsRelationMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_struct("AnalyticsRelationMeta")
            .field("len", &self.len())
            .finish()
    }
}

impl AnalyticsRelationMeta {
    #[doc(alias = "gst_buffer_add_analytics_relation_meta")]
    pub fn add(buffer: &mut gst::BufferRef) -> gst::MetaRefMut<Self, gst::meta::Standalone> {
        skip_assert_initialized!();

        unsafe {
            let meta_ptr = ffi::gst_buffer_add_analytics_relation_meta(buffer.as_mut_ptr());
            Self::from_mut_ptr(buffer, meta_ptr)
        }
    }

    #[doc(alias = "gst_buffer_add_analytics_relation_meta_full")]
    pub fn add_full<'a>(
        buffer: &'a mut gst::BufferRef,
        init_params: &AnalyticsRelationMetaInitParams,
    ) -> gst::MetaRefMut<'a, Self, gst::meta::Standalone> {
        skip_assert_initialized!();

        unsafe {
            let meta_ptr = ffi::gst_buffer_add_analytics_relation_meta_full(
                buffer.as_mut_ptr(),
                mut_override(&init_params.0),
            );
            Self::from_mut_ptr(buffer, meta_ptr)
        }
    }

    #[doc(alias = "gst_analytics_relation_get_length")]
    pub fn len(&self) -> usize {
        unsafe { ffi::gst_analytics_relation_get_length(self.as_mut_ptr()) }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[doc(alias = "gst_analytics_relation_meta_set_relation")]
    pub fn set_relation(
        &mut self,
        type_: crate::RelTypes,
        an_meta_first_id: u32,
        an_meta_second_id: u32,
    ) -> Result<(), glib::BoolError> {
        let ret = unsafe {
            from_glib(ffi::gst_analytics_relation_meta_set_relation(
                self.as_mut_ptr(),
                type_.into_glib(),
                an_meta_first_id,
                an_meta_second_id,
            ))
        };

        if ret {
            Ok(())
        } else {
            Err(glib::bool_error!(
                "Could not set relation {:}->{:} of type {:?}",
                an_meta_first_id,
                an_meta_second_id,
                type_
            ))
        }
    }

    #[doc(alias = "gst_analytics_relation_meta_get_relation")]
    pub fn relation(&self, an_meta_first_id: u32, an_meta_second_id: u32) -> crate::RelTypes {
        unsafe {
            from_glib(ffi::gst_analytics_relation_meta_get_relation(
                self.as_mut_ptr(),
                an_meta_first_id,
                an_meta_second_id,
            ))
        }
    }

    #[doc(alias = "gst_analytics_relation_meta_exist")]
    pub fn exist(
        &self,
        an_meta_first_id: u32,
        an_meta_second_id: u32,
        relation_span: i32,
        cond_types: crate::RelTypes,
    ) -> bool {
        unsafe {
            from_glib(ffi::gst_analytics_relation_meta_exist(
                self.as_mut_ptr(),
                an_meta_first_id,
                an_meta_second_id,
                relation_span,
                cond_types.into_glib(),
                std::ptr::null_mut(),
            ))
        }
    }

    #[doc(alias = "gst_analytics_relation_meta_exist")]
    pub fn exist_path(
        &self,
        an_meta_first_id: u32,
        an_meta_second_id: u32,
        relation_span: i32,
        cond_types: crate::RelTypes,
    ) -> Result<AnalyticsRelationPath, glib::BoolError> {
        let mut array = std::ptr::null_mut::<glib::ffi::GArray>();
        let ret = unsafe {
            from_glib(ffi::gst_analytics_relation_meta_exist(
                self.as_mut_ptr(),
                an_meta_first_id,
                an_meta_second_id,
                relation_span,
                cond_types.into_glib(),
                &mut array,
            ))
        };

        if ret {
            Ok(AnalyticsRelationPath { garray: array })
        } else {
            Err(glib::bool_error!("Such relation doesn't exist"))
        }
    }

    pub unsafe fn as_mut_ptr(&self) -> *mut ffi::GstAnalyticsRelationMeta {
        mut_override(&self.0)
    }
}

impl UnsafeFrom<&AnalyticsRelationMeta> for ffi::GstAnalyticsMtd {
    unsafe fn unsafe_from(t: &AnalyticsRelationMeta) -> Self {
        ffi::GstAnalyticsMtd {
            id: 0,
            meta: t.as_mut_ptr(),
        }
    }
}

impl AnalyticsRelationPath {
    pub fn as_slice(&self) -> &[u32] {
        unsafe {
            std::slice::from_raw_parts(
                (*self.garray).data as *const u32,
                (*self.garray).len as usize,
            )
        }
    }
}

impl Drop for AnalyticsRelationPath {
    fn drop(&mut self) {
        unsafe {
            glib::ffi::g_array_free(self.garray, glib::ffi::GTRUE);
        }
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T> Sealed for T {}
}

pub trait AnalyticsMetaRefExt<'a>: sealed::Sealed {
    #[doc(alias = "gst_analytics_relation_meta_get_mtd")]
    fn mtd<T: AnalyticsMtd>(&self, an_meta_id: u32) -> Option<AnalyticsMtdRef<'a, T>>;
    fn iter<T: AnalyticsMtd>(&'a self) -> AnalyticsMtdIter<T>;
    fn iter_direct_related<T: AnalyticsMtd>(
        &'a self,
        an_meta_id: u32,
        rel_type: RelTypes,
    ) -> AnalyticsMtdIter<T>;
}

impl<'a> AnalyticsMetaRefExt<'a> for gst::MetaRef<'a, AnalyticsRelationMeta> {
    fn mtd<T: AnalyticsMtd>(&self, an_meta_id: u32) -> Option<AnalyticsMtdRef<'a, T>> {
        unsafe {
            let mut mtd = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_analytics_relation_meta_get_mtd(
                self.as_mut_ptr(),
                an_meta_id,
                T::mtd_type(),
                mtd.as_mut_ptr(),
            ));
            let id = mtd.assume_init().id;

            if ret {
                Some(AnalyticsMtdRef::from_meta(self, id))
            } else {
                None
            }
        }
    }

    fn iter<T: AnalyticsMtd>(&'a self) -> AnalyticsMtdIter<'a, T> {
        AnalyticsMtdIter::new(self)
    }
    fn iter_direct_related<T: AnalyticsMtd>(
        &'a self,
        an_meta_id: u32,
        rel_type: RelTypes,
    ) -> AnalyticsMtdIter<T> {
        AnalyticsMtdIter::new_direct_related(self, an_meta_id, rel_type.into_glib())
    }
}

impl<'a, T: AnalyticsMtd> AnalyticsMtdRef<'a, T> {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub unsafe fn from_meta(meta: &gst::MetaRef<'a, AnalyticsRelationMeta>, id: u32) -> Self {
        skip_assert_initialized!();
        AnalyticsMtdRef {
            meta: meta.clone(),
            id,
            mtd_type: PhantomData,
        }
    }

    #[doc(alias = "gst_analytics_mtd_get_mtd_type")]
    pub fn mtd_type(&self) -> ffi::GstAnalyticsMtdType {
        unsafe {
            let mut mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            ffi::gst_analytics_mtd_get_mtd_type(&mut mtd)
        }
    }
}
impl<'a> AnalyticsMtdRef<'a, AnalyticsAnyMtd> {
    pub fn downcast<T: AnalyticsMtd>(
        self,
    ) -> Result<AnalyticsMtdRef<'a, T>, AnalyticsMtdRef<'a, AnalyticsAnyMtd>> {
        if self.mtd_type() == T::mtd_type() {
            Ok(AnalyticsMtdRef {
                id: self.id,
                meta: self.meta,
                mtd_type: PhantomData,
            })
        } else {
            Err(self)
        }
    }

    pub fn downcast_ref<T: AnalyticsMtd>(&self) -> Option<&AnalyticsMtdRef<'a, T>> {
        unsafe {
            if self.mtd_type() == T::mtd_type() {
                Some(&*(self as *const _ as *const _))
            } else {
                None
            }
        }
    }
}

impl<'a> AnalyticsMtdRefMut<'a, AnalyticsAnyMtd> {
    pub fn downcast_mut<T: AnalyticsMtd>(&mut self) -> Option<&mut AnalyticsMtdRefMut<'a, T>> {
        unsafe {
            if self.as_ref().mtd_type() == T::mtd_type() {
                Some(&mut *(self as *mut _ as *mut _))
            } else {
                None
            }
        }
    }
}

impl<'a, T: AnalyticsMtd> UnsafeFrom<&AnalyticsMtdRef<'a, T>> for ffi::GstAnalyticsMtd {
    unsafe fn unsafe_from(t: &AnalyticsMtdRef<'a, T>) -> Self {
        ffi::GstAnalyticsMtd {
            id: t.id,
            meta: t.meta.as_mut_ptr(),
        }
    }
}

pub trait AnalyticsMetaRefMutExt<'a>: sealed::Sealed {
    #[doc(alias = "gst_analytics_relation_meta_get_mtd")]
    fn mtd_mut<T: AnalyticsMtd>(&'a mut self, an_meta_id: u32)
        -> Option<AnalyticsMtdRefMut<'a, T>>;

    fn iter_mut<T: AnalyticsMtd>(&'a mut self) -> AnalyticsMtdIterMut<T>;
    fn iter_direct_related_mut<T: AnalyticsMtd>(
        &'a mut self,
        an_meta_id: u32,
        rel_type: RelTypes,
    ) -> AnalyticsMtdIterMut<T>;
}

impl<'a> AnalyticsMetaRefMutExt<'a>
    for gst::MetaRefMut<'a, AnalyticsRelationMeta, gst::meta::Standalone>
{
    fn mtd_mut<T: AnalyticsMtd>(
        &'a mut self,
        an_meta_id: u32,
    ) -> Option<AnalyticsMtdRefMut<'a, T>> {
        unsafe {
            let mut mtd = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_analytics_relation_meta_get_mtd(
                self.as_mut_ptr(),
                an_meta_id,
                T::mtd_type(),
                mtd.as_mut_ptr(),
            ));
            let id = mtd.assume_init().id;

            if ret {
                Some(AnalyticsMtdRefMut::from_meta(self, id))
            } else {
                None
            }
        }
    }

    fn iter_mut<T: AnalyticsMtd>(&'a mut self) -> AnalyticsMtdIterMut<T> {
        AnalyticsMtdIterMut::new(self)
    }
    fn iter_direct_related_mut<T: AnalyticsMtd>(
        &'a mut self,
        an_meta_id: u32,
        rel_type: RelTypes,
    ) -> AnalyticsMtdIterMut<T> {
        AnalyticsMtdIterMut::new_direct_related(self, an_meta_id, rel_type.into_glib())
    }
}

unsafe impl MetaAPI for AnalyticsRelationMeta {
    type GstType = ffi::GstAnalyticsRelationMeta;

    #[doc(alias = "gst_analytics_relation_meta_api_get_type")]
    #[inline]
    fn meta_api() -> glib::Type {
        unsafe { from_glib(ffi::gst_analytics_relation_meta_api_get_type()) }
    }
}

pub unsafe trait AnalyticsMtd {
    fn mtd_type() -> ffi::GstAnalyticsMtdType;
}

pub trait AnalyticsMtdExt: AnalyticsMtd {
    #[doc(alias = "gst_analytics_mtd_type_get_name")]
    fn type_name() -> &'static str {
        unsafe {
            let ptr = ffi::gst_analytics_mtd_type_get_name(Self::mtd_type());
            std::ffi::CStr::from_ptr(ptr).to_str().unwrap()
        }
    }
}

impl<T: AnalyticsMtd> AnalyticsMtdExt for T {}

impl<'a, T: AnalyticsMtd> AnalyticsMtdRefMut<'a, T> {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub unsafe fn from_meta(
        meta: &'a mut gst::MetaRefMut<'a, AnalyticsRelationMeta, gst::meta::Standalone>,
        id: u32,
    ) -> Self {
        skip_assert_initialized!();
        AnalyticsMtdRefMut {
            meta,
            id,
            mtd_type: PhantomData,
        }
    }
}

impl<'a, T: AnalyticsMtd> UnsafeFrom<&mut AnalyticsMtdRefMut<'a, T>> for ffi::GstAnalyticsMtd {
    unsafe fn unsafe_from(t: &mut AnalyticsMtdRefMut<'a, T>) -> Self {
        ffi::GstAnalyticsMtd {
            id: t.id,
            meta: t.meta.as_mut_ptr(),
        }
    }
}

impl<'a, T: AnalyticsMtd> From<AnalyticsMtdRefMut<'a, T>> for AnalyticsMtdRef<'a, T> {
    fn from(value: AnalyticsMtdRefMut<'a, T>) -> Self {
        skip_assert_initialized!();
        AnalyticsMtdRef {
            meta: value.meta.as_ref().clone(),
            id: value.id,
            mtd_type: value.mtd_type,
        }
    }
}

impl<'a, T: AnalyticsMtd> From<&mut AnalyticsMtdRefMut<'a, T>> for AnalyticsMtdRef<'a, T> {
    fn from(value: &mut AnalyticsMtdRefMut<'a, T>) -> Self {
        skip_assert_initialized!();
        AnalyticsMtdRef {
            meta: value.meta.as_ref().clone(),
            id: value.id,
            mtd_type: value.mtd_type,
        }
    }
}

impl<'a, T: AnalyticsMtd> AsRef<AnalyticsMtdRef<'a, T>> for AnalyticsMtdRefMut<'a, T> {
    #[inline]
    fn as_ref(&self) -> &AnalyticsMtdRef<'a, T> {
        unsafe { &*(self as *const AnalyticsMtdRefMut<'a, T> as *const AnalyticsMtdRef<'a, T>) }
    }
}

macro_rules! define_mtd_iter {
    ($name:ident, $metaref:ty, $itemref:ty, $copy_meta:expr) => {
        pub struct $name<'a, T: AnalyticsMtd> {
            meta: $metaref,
            state: glib::ffi::gpointer,
            mtd_type: ffi::GstAnalyticsMtdType,
            an_meta_id: u32,
            rel_type: ffi::GstAnalyticsRelTypes,
            phantom: std::marker::PhantomData<T>,
        }

        impl<'a, T: AnalyticsMtd> $name<'a, T> {
            fn new(meta: $metaref) -> Self {
                skip_assert_initialized!();
                $name {
                    meta,
                    state: std::ptr::null_mut(),
                    mtd_type: T::mtd_type(),
                    an_meta_id: std::u32::MAX,
                    rel_type: RelTypes::ANY.into_glib(),
                    phantom: PhantomData,
                }
            }
            fn new_direct_related(
                meta: $metaref,
                an_meta_id: u32,
                rel_type: ffi::GstAnalyticsRelTypes,
            ) -> Self {
                skip_assert_initialized!();
                $name {
                    meta,
                    state: std::ptr::null_mut(),
                    mtd_type: T::mtd_type(),
                    an_meta_id,
                    rel_type,
                    phantom: PhantomData,
                }
            }
        }

        impl<'a, T: AnalyticsMtd + 'a> Iterator for $name<'a, T> {
            type Item = $itemref;

            fn next(&mut self) -> Option<Self::Item> {
                unsafe {
                    let mut mtd = ffi::GstAnalyticsMtd::unsafe_from(&**self.meta);
                    let ret = {
                        if self.an_meta_id == std::u32::MAX {
                            ffi::gst_analytics_relation_meta_iterate(
                                self.meta.as_mut_ptr(),
                                &mut self.state,
                                self.mtd_type,
                                &mut mtd,
                            )
                        } else {
                            ffi::gst_analytics_relation_meta_get_direct_related(
                                self.meta.as_mut_ptr(),
                                self.an_meta_id,
                                self.rel_type,
                                self.mtd_type,
                                &mut self.state,
                                &mut mtd,
                            )
                        }
                    };
                    if from_glib(ret) {
                        // This is a known clippy limitation
                        // https://github.com/rust-lang/rust-clippy/issues/1553
                        #[allow(clippy::redundant_closure_call)]
                        Some(Self::Item::from_meta($copy_meta(self.meta), mtd.id))
                    } else {
                        None
                    }
                }
            }
        }
    };
}

define_mtd_iter!(
    AnalyticsMtdIter,
    &'a gst::MetaRef<'a, AnalyticsRelationMeta>,
    AnalyticsMtdRef<'a, T>,
    |meta| meta
);

define_mtd_iter!(
    AnalyticsMtdIterMut,
    &'a mut gst::MetaRefMut<'a, AnalyticsRelationMeta, gst::meta::Standalone>,
    AnalyticsMtdRefMut<'a, T>,
    |meta: &mut _| &mut *(meta as *mut gst::MetaRefMut<
        'a,
        AnalyticsRelationMeta,
        gst::meta::Standalone,
    >)
);

#[derive(Debug)]
pub enum AnalyticsAnyMtd {}

unsafe impl AnalyticsMtd for AnalyticsAnyMtd {
    fn mtd_type() -> ffi::GstAnalyticsMtdType {
        ffi::GST_ANALYTICS_MTD_TYPE_ANY as ffi::GstAnalyticsMtdType
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn build_relation_meta() {
        gst::init().unwrap();

        let mut buf = gst::Buffer::new();

        let meta = AnalyticsRelationMeta::add(buf.make_mut());

        assert!(meta.is_empty());
    }

    #[test]
    fn build_relation_meta_full() {
        gst::init().unwrap();

        let mut buf = gst::Buffer::new();

        let params = AnalyticsRelationMetaInitParams::new(10, 10);
        let meta = AnalyticsRelationMeta::add_full(buf.make_mut(), &params);

        assert!(meta.is_empty());
    }

    #[test]
    fn relations() {
        gst::init().unwrap();

        let mut buf = gst::Buffer::new();
        let _ = AnalyticsRelationMeta::add(buf.make_mut());

        let mut meta = buf.make_mut().meta_mut::<AnalyticsRelationMeta>().unwrap();
        let od = meta
            .add_od_mtd(glib::Quark::from_str("blb"), 0, 1, 10, 20, 0.8)
            .unwrap();
        let od1_id = od.id();

        let od = meta
            .add_od_mtd(glib::Quark::from_str("blb"), 0, 1, 10, 20, 0.8)
            .unwrap();
        let od2_id = od.id();

        let od: AnalyticsMtdRef<'_, AnalyticsODMtd> = meta
            .add_od_mtd(glib::Quark::from_str("blb"), 0, 1, 10, 20, 0.8)
            .unwrap();
        let od3_id = od.id();

        meta.set_relation(RelTypes::IS_PART_OF, od1_id, od2_id)
            .unwrap();
        meta.set_relation(RelTypes::IS_PART_OF, od2_id, od3_id)
            .unwrap();

        meta.set_relation(RelTypes::IS_PART_OF, 8888, 9999)
            .expect_err("Invalid id");

        let meta = buf.meta::<AnalyticsRelationMeta>().unwrap();
        assert!(meta.relation(od1_id, od2_id) == crate::RelTypes::IS_PART_OF);
        assert!(meta.relation(od2_id, od3_id) == crate::RelTypes::IS_PART_OF);

        assert!(meta.exist(od1_id, od2_id, 1, crate::RelTypes::IS_PART_OF));
        assert!(meta.exist(od1_id, od3_id, 2, crate::RelTypes::IS_PART_OF));
        assert!(!meta.exist(od2_id, od1_id, 1, crate::RelTypes::IS_PART_OF));
        assert!(!meta.exist(od1_id, od3_id, 1, crate::RelTypes::IS_PART_OF));
        assert!(!meta.exist(od1_id, od2_id, 1, crate::RelTypes::CONTAIN));

        let path = meta
            .exist_path(od1_id, od3_id, 3, crate::RelTypes::ANY)
            .unwrap();

        assert_eq!(path.as_slice().len(), 3);
        assert_eq!(path.as_slice()[0], od1_id);
        assert_eq!(path.as_slice()[1], od2_id);
        assert_eq!(path.as_slice()[2], od3_id);

        assert_eq!(meta.len(), meta.iter::<AnalyticsAnyMtd>().count());
        assert_eq!(meta.len(), meta.iter::<AnalyticsODMtd>().count());
        for mtd in meta.iter::<AnalyticsODMtd>() {
            assert_eq!(mtd.obj_type(), glib::Quark::from_str("blb"))
        }

        assert_eq!(meta.len(), meta.iter::<AnalyticsAnyMtd>().count());
        for mtd in meta.iter::<AnalyticsAnyMtd>() {
            if let Ok(mtd) = mtd.downcast::<AnalyticsODMtd>() {
                assert_eq!(mtd.obj_type(), glib::Quark::from_str("blb"))
            }
        }

        assert_eq!(
            meta.iter_direct_related::<AnalyticsODMtd>(od1_id, crate::RelTypes::IS_PART_OF)
                .count(),
            1
        );
        assert_eq!(
            meta.iter_direct_related::<AnalyticsODMtd>(od2_id, crate::RelTypes::IS_PART_OF)
                .count(),
            1
        );
        assert_eq!(
            meta.iter_direct_related::<AnalyticsODMtd>(od3_id, crate::RelTypes::IS_PART_OF)
                .count(),
            0
        );
        assert_eq!(
            meta.iter_direct_related::<AnalyticsODMtd>(od1_id, crate::RelTypes::CONTAIN)
                .count(),
            0
        );

        assert_eq!(
            meta.iter_direct_related::<AnalyticsAnyMtd>(od1_id, crate::RelTypes::CONTAIN)
                .count(),
            0
        );
        for mtd in meta.iter_direct_related::<AnalyticsODMtd>(od1_id, crate::RelTypes::IS_PART_OF) {
            assert_eq!(mtd.obj_type(), glib::Quark::from_str("blb"))
        }

        let mut meta = buf.make_mut().meta_mut::<AnalyticsRelationMeta>().unwrap();
        assert_eq!(meta.len(), meta.iter_mut::<AnalyticsAnyMtd>().count());

        let mut meta = buf.make_mut().meta_mut::<AnalyticsRelationMeta>().unwrap();
        let _ = meta.add_tracking_mtd(10, gst::ClockTime::from_seconds(10));
        let _ = meta.add_tracking_mtd(10, gst::ClockTime::from_seconds(10));

        for mut item in meta.iter_mut::<AnalyticsTrackingMtd>() {
            item.set_lost().unwrap();
        }
    }
}
