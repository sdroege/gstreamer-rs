// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;
use std::marker::PhantomData;
use std::ops;

use crate::Buffer;
use crate::BufferRef;
#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
use crate::Caps;
#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
use crate::CapsRef;
#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
use crate::ClockTime;

use glib::translate::*;

pub unsafe trait MetaAPI: Sync + Send + Sized {
    type GstType;

    #[doc(alias = "get_meta_api")]
    fn meta_api() -> glib::Type;

    unsafe fn from_ptr(buffer: &BufferRef, ptr: *const Self::GstType) -> MetaRef<Self> {
        assert!(!ptr.is_null());

        let meta_api = Self::meta_api();
        if meta_api != glib::Type::INVALID {
            assert_eq!(
                meta_api,
                from_glib((*(*(ptr as *const ffi::GstMeta)).info).api)
            )
        }

        MetaRef {
            meta: &*(ptr as *const Self),
            buffer,
        }
    }

    unsafe fn from_mut_ptr<T>(
        buffer: &mut BufferRef,
        ptr: *mut Self::GstType,
    ) -> MetaRefMut<Self, T> {
        assert!(!ptr.is_null());

        let meta_api = Self::meta_api();
        if meta_api != glib::Type::INVALID {
            assert_eq!(
                meta_api,
                from_glib((*(*(ptr as *const ffi::GstMeta)).info).api)
            )
        }

        MetaRefMut {
            meta: &mut *(ptr as *mut Self),
            buffer,
            mode: PhantomData,
        }
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub struct MetaSeqnum(u64);

pub struct MetaRef<'a, T: 'a> {
    meta: &'a T,
    buffer: &'a BufferRef,
}

pub enum Standalone {}
pub enum Iterated {}

pub struct MetaRefMut<'a, T: 'a, U> {
    meta: &'a mut T,
    buffer: &'a mut BufferRef,
    mode: PhantomData<U>,
}

impl<'a, T: fmt::Debug + 'a> fmt::Debug for MetaRef<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MetaRef")
            .field("meta", &self.meta)
            .field("buffer", &self.buffer)
            .finish()
    }
}

impl<'a, T: fmt::Debug + 'a, U> fmt::Debug for MetaRefMut<'a, T, U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MetaRef")
            .field("meta", &self.meta)
            .field("buffer", &self.buffer)
            .field("mode", &self.mode)
            .finish()
    }
}

impl<'a, T> ops::Deref for MetaRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.meta
    }
}

impl<'a, T> AsRef<MetaRef<'a, T>> for MetaRef<'a, T> {
    fn as_ref(&self) -> &MetaRef<'a, T> {
        self
    }
}

impl<'a, T, U> ops::Deref for MetaRefMut<'a, T, U> {
    type Target = T;

    fn deref(&self) -> &T {
        self.meta
    }
}

impl<'a, T, U> ops::DerefMut for MetaRefMut<'a, T, U> {
    fn deref_mut(&mut self) -> &mut T {
        self.meta
    }
}

impl<'a, T, U> AsRef<MetaRef<'a, T>> for MetaRefMut<'a, T, U> {
    fn as_ref(&self) -> &MetaRef<'a, T> {
        unsafe { &*(self as *const MetaRefMut<'a, T, U> as *const MetaRef<'a, T>) }
    }
}

impl<'a, T> MetaRef<'a, T> {
    #[doc(alias = "get_api")]
    pub fn api(&self) -> glib::Type {
        unsafe {
            let meta = self.meta as *const _ as *const ffi::GstMeta;
            let info = (*meta).info;
            glib::Type::from_glib((*info).api)
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "get_seqnum")]
    #[doc(alias = "gst_meta_get_seqnum")]
    pub fn seqnum(&self) -> MetaSeqnum {
        unsafe {
            let meta = self.meta as *const _ as *const ffi::GstMeta;
            MetaSeqnum(ffi::gst_meta_get_seqnum(meta))
        }
    }

    pub fn as_ptr(&self) -> *const T::GstType
    where
        T: MetaAPI,
    {
        self.meta as *const _ as *const <T as MetaAPI>::GstType
    }
}

impl<'a> MetaRef<'a, Meta> {
    pub fn downcast_ref<T: MetaAPI>(&self) -> Option<&MetaRef<'a, T>> {
        let target_type = T::meta_api();
        let type_ = self.api();

        if type_ == glib::Type::INVALID || target_type == type_ {
            Some(unsafe { &*(self as *const MetaRef<'a, Meta> as *const MetaRef<'a, T>) })
        } else {
            None
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    pub fn try_as_custom_meta(&self) -> Option<&MetaRef<'a, CustomMeta>> {
        unsafe {
            if ffi::gst_meta_info_is_custom(&*self.0.info) == glib::ffi::GFALSE {
                return None;
            }

            Some(&*(self as *const MetaRef<'a, Meta> as *const MetaRef<'a, CustomMeta>))
        }
    }
}

impl<'a, T, U> MetaRefMut<'a, T, U> {
    #[doc(alias = "get_api")]
    pub fn api(&self) -> glib::Type {
        unsafe {
            let meta = self.meta as *const _ as *const ffi::GstMeta;
            let info = (*meta).info;
            glib::Type::from_glib((*info).api)
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "get_seqnum")]
    #[doc(alias = "gst_meta_get_seqnum")]
    pub fn seqnum(&self) -> u64 {
        unsafe {
            let meta = self.meta as *const _ as *const ffi::GstMeta;
            ffi::gst_meta_get_seqnum(meta)
        }
    }

    pub fn as_ptr(&self) -> *const T::GstType
    where
        T: MetaAPI,
    {
        self.meta as *const _ as *const <T as MetaAPI>::GstType
    }

    pub fn as_mut_ptr(&mut self) -> *mut T::GstType
    where
        T: MetaAPI,
    {
        self.meta as *mut _ as *mut <T as MetaAPI>::GstType
    }
}

impl<'a, T> MetaRefMut<'a, T, Standalone> {
    #[doc(alias = "gst_buffer_remove_meta")]
    pub fn remove(self) {
        unsafe {
            let res = ffi::gst_buffer_remove_meta(
                self.buffer.as_mut_ptr(),
                self.meta as *mut T as *mut ffi::GstMeta,
            );
            assert_ne!(res, glib::ffi::GFALSE);
        }
    }
}

impl<'a, U> MetaRefMut<'a, Meta, U> {
    pub fn downcast_ref<T: MetaAPI>(&mut self) -> Option<&MetaRefMut<'a, T, U>> {
        let target_type = T::meta_api();
        let type_ = self.api();

        if type_ == glib::Type::INVALID || target_type == type_ {
            Some(unsafe { &*(self as *mut MetaRefMut<'a, Meta, U> as *const MetaRefMut<'a, T, U>) })
        } else {
            None
        }
    }

    pub fn downcast_mut<T: MetaAPI>(&mut self) -> Option<&mut MetaRefMut<'a, T, U>> {
        let target_type = T::meta_api();
        let type_ = self.api();

        if type_ == glib::Type::INVALID || target_type == type_ {
            Some(unsafe {
                &mut *(self as *mut MetaRefMut<'a, Meta, U> as *mut MetaRefMut<'a, T, U>)
            })
        } else {
            None
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    pub fn try_as_custom_meta(&self) -> Option<&MetaRefMut<'a, CustomMeta, U>> {
        unsafe {
            if ffi::gst_meta_info_is_custom(&*self.0.info) == glib::ffi::GFALSE {
                return None;
            }

            Some(&*(self as *const MetaRefMut<'a, Meta, U> as *const MetaRefMut<'a, CustomMeta, U>))
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    pub fn try_as_mut_custom_meta(&mut self) -> Option<&mut MetaRefMut<'a, CustomMeta, U>> {
        unsafe {
            if ffi::gst_meta_info_is_custom(&*self.0.info) == glib::ffi::GFALSE {
                return None;
            }

            Some(&mut *(self as *mut MetaRefMut<'a, Meta, U> as *mut MetaRefMut<'a, CustomMeta, U>))
        }
    }
}

#[repr(transparent)]
#[doc(alias = "GstMeta")]
pub struct Meta(ffi::GstMeta);

unsafe impl Send for Meta {}
unsafe impl Sync for Meta {}

impl Meta {
    #[doc(alias = "get_api")]
    fn api(&self) -> glib::Type {
        unsafe { glib::Type::from_glib((*self.0.info).api) }
    }
}

unsafe impl MetaAPI for Meta {
    type GstType = ffi::GstMeta;

    fn meta_api() -> glib::Type {
        glib::Type::INVALID
    }
}

impl fmt::Debug for Meta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Meta").field("api", &self.api()).finish()
    }
}

#[repr(transparent)]
#[doc(alias = "GstParentBufferMeta")]
pub struct ParentBufferMeta(ffi::GstParentBufferMeta);

unsafe impl Send for ParentBufferMeta {}
unsafe impl Sync for ParentBufferMeta {}

impl ParentBufferMeta {
    #[doc(alias = "gst_buffer_add_parent_buffer_meta")]
    pub fn add<'a>(buffer: &'a mut BufferRef, parent: &Buffer) -> MetaRefMut<'a, Self, Standalone> {
        skip_assert_initialized!();
        unsafe {
            let meta = ffi::gst_buffer_add_parent_buffer_meta(
                buffer.as_mut_ptr(),
                parent.to_glib_none().0,
            );

            Self::from_mut_ptr(buffer, meta)
        }
    }

    #[doc(alias = "get_parent")]
    pub fn parent(&self) -> &BufferRef {
        unsafe { BufferRef::from_ptr(self.0.buffer) }
    }

    #[doc(alias = "get_parent_owned")]
    pub fn parent_owned(&self) -> Buffer {
        unsafe { from_glib_none(self.0.buffer) }
    }
}

unsafe impl MetaAPI for ParentBufferMeta {
    type GstType = ffi::GstParentBufferMeta;

    #[doc(alias = "gst_parent_buffer_meta_api_get_type")]
    fn meta_api() -> glib::Type {
        unsafe { from_glib(ffi::gst_parent_buffer_meta_api_get_type()) }
    }
}

impl fmt::Debug for ParentBufferMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ParentBufferMeta")
            .field("parent", &self.parent())
            .finish()
    }
}

#[repr(transparent)]
#[doc(alias = "GstProtectionMeta")]
pub struct ProtectionMeta(ffi::GstProtectionMeta);

unsafe impl Send for ProtectionMeta {}
unsafe impl Sync for ProtectionMeta {}

impl ProtectionMeta {
    #[doc(alias = "gst_buffer_add_protection_meta")]
    pub fn add(buffer: &mut BufferRef, info: crate::Structure) -> MetaRefMut<Self, Standalone> {
        skip_assert_initialized!();
        unsafe {
            let meta = ffi::gst_buffer_add_protection_meta(buffer.as_mut_ptr(), info.into_ptr());

            Self::from_mut_ptr(buffer, meta)
        }
    }

    #[doc(alias = "get_info")]
    pub fn info(&self) -> &crate::StructureRef {
        unsafe { crate::StructureRef::from_glib_borrow(self.0.info) }
    }

    #[doc(alias = "get_info_mut")]
    pub fn info_mut(&mut self) -> &mut crate::StructureRef {
        unsafe { crate::StructureRef::from_glib_borrow_mut(self.0.info) }
    }
}

unsafe impl MetaAPI for ProtectionMeta {
    type GstType = ffi::GstProtectionMeta;

    #[doc(alias = "gst_protection_meta_api_get_type")]
    fn meta_api() -> glib::Type {
        unsafe { from_glib(ffi::gst_protection_meta_api_get_type()) }
    }
}

impl fmt::Debug for ProtectionMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ProtectionMeta")
            .field("info", &self.info())
            .finish()
    }
}

#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
#[repr(transparent)]
#[doc(alias = "GstReferenceTimestampMeta")]
pub struct ReferenceTimestampMeta(ffi::GstReferenceTimestampMeta);

#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
unsafe impl Send for ReferenceTimestampMeta {}
#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
unsafe impl Sync for ReferenceTimestampMeta {}

#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
impl ReferenceTimestampMeta {
    #[doc(alias = "gst_buffer_add_reference_timestamp_meta")]
    pub fn add<'a>(
        buffer: &'a mut BufferRef,
        reference: &Caps,
        timestamp: ClockTime,
        duration: impl Into<Option<ClockTime>>,
    ) -> MetaRefMut<'a, Self, Standalone> {
        skip_assert_initialized!();
        unsafe {
            let meta = ffi::gst_buffer_add_reference_timestamp_meta(
                buffer.as_mut_ptr(),
                reference.to_glib_none().0,
                timestamp.into_glib(),
                duration.into().into_glib(),
            );

            Self::from_mut_ptr(buffer, meta)
        }
    }

    #[doc(alias = "get_reference")]
    pub fn reference(&self) -> &CapsRef {
        unsafe { CapsRef::from_ptr(self.0.reference) }
    }

    #[doc(alias = "get_parent_owned")]
    pub fn parent_owned(&self) -> Caps {
        unsafe { from_glib_none(self.0.reference) }
    }

    #[doc(alias = "get_timestamp")]
    pub fn timestamp(&self) -> ClockTime {
        unsafe { try_from_glib(self.0.timestamp).expect("undefined timestamp") }
    }

    #[doc(alias = "get_duration")]
    pub fn duration(&self) -> Option<ClockTime> {
        unsafe { from_glib(self.0.duration) }
    }
}

#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
unsafe impl MetaAPI for ReferenceTimestampMeta {
    type GstType = ffi::GstReferenceTimestampMeta;

    #[doc(alias = "gst_reference_timestamp_meta_api_get_type")]
    fn meta_api() -> glib::Type {
        unsafe { from_glib(ffi::gst_reference_timestamp_meta_api_get_type()) }
    }
}

#[cfg(any(feature = "v1_14", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_14")))]
impl fmt::Debug for ReferenceTimestampMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::utils::Displayable;

        f.debug_struct("ReferenceTimestampMeta")
            .field("reference", &self.reference())
            .field("timestamp", &self.timestamp().display().to_string())
            .field("duration", &self.duration().display().to_string())
            .finish()
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
#[repr(transparent)]
#[doc(alias = "GstCustomMeta")]
pub struct CustomMeta(ffi::GstCustomMeta);

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
unsafe impl Send for CustomMeta {}
#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
unsafe impl Sync for CustomMeta {}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
impl CustomMeta {
    #[doc(alias = "gst_meta_register_custom")]
    pub fn register<
        F: Fn(&mut BufferRef, &CustomMeta, &BufferRef, glib::Quark) -> bool + Send + Sync + 'static,
    >(
        name: &str,
        tags: &[&str],
        transform_func: F,
    ) {
        unsafe extern "C" fn transform_func_trampoline<
            F: Fn(&mut BufferRef, &CustomMeta, &BufferRef, glib::Quark) -> bool
                + Send
                + Sync
                + 'static,
        >(
            dest: *mut ffi::GstBuffer,
            meta: *mut ffi::GstCustomMeta,
            src: *mut ffi::GstBuffer,
            type_: glib::ffi::GQuark,
            _data: glib::ffi::gpointer,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let func = &*(user_data as *const F);
            let res = func(
                BufferRef::from_mut_ptr(dest),
                &*(meta as *const CustomMeta),
                BufferRef::from_ptr(src),
                from_glib(type_),
            );
            res.into_glib()
        }

        unsafe extern "C" fn transform_func_free<F>(ptr: glib::ffi::gpointer) {
            let _ = Box::from_raw(ptr as *mut F);
        }

        unsafe {
            ffi::gst_meta_register_custom(
                name.to_glib_none().0,
                tags.to_glib_none().0,
                Some(transform_func_trampoline::<F>),
                Box::into_raw(Box::new(transform_func)) as glib::ffi::gpointer,
                Some(transform_func_free::<F>),
            );
        }
    }

    #[doc(alias = "gst_buffer_add_custom_meta")]
    pub fn add<'a>(
        buffer: &'a mut BufferRef,
        name: &str,
    ) -> Result<MetaRefMut<'a, Self, Standalone>, glib::BoolError> {
        skip_assert_initialized!();
        unsafe {
            let meta = ffi::gst_buffer_add_custom_meta(buffer.as_mut_ptr(), name.to_glib_none().0);

            if meta.is_null() {
                return Err(glib::bool_error!("Failed to add custom meta"));
            }

            Ok(MetaRefMut {
                meta: &mut *(meta as *mut Self),
                buffer,
                mode: PhantomData,
            })
        }
    }

    #[doc(alias = "gst_buffer_get_custom_meta")]
    pub fn from_buffer<'a>(
        buffer: &'a BufferRef,
        name: &str,
    ) -> Result<MetaRef<'a, Self>, glib::BoolError> {
        skip_assert_initialized!();
        unsafe {
            let meta = ffi::gst_buffer_get_custom_meta(buffer.as_mut_ptr(), name.to_glib_none().0);

            if meta.is_null() {
                return Err(glib::bool_error!("Failed to get custom meta"));
            }

            Ok(MetaRef {
                meta: &*(meta as *const Self),
                buffer,
            })
        }
    }

    #[doc(alias = "gst_buffer_get_custom_meta")]
    pub fn from_mut_buffer<'a>(
        buffer: &'a mut BufferRef,
        name: &str,
    ) -> Result<MetaRefMut<'a, Self, Standalone>, glib::BoolError> {
        skip_assert_initialized!();
        unsafe {
            let meta = ffi::gst_buffer_get_custom_meta(buffer.as_mut_ptr(), name.to_glib_none().0);

            if meta.is_null() {
                return Err(glib::bool_error!("Failed to get custom meta"));
            }

            Ok(MetaRefMut {
                meta: &mut *(meta as *mut Self),
                buffer,
                mode: PhantomData,
            })
        }
    }

    #[doc(alias = "gst_custom_meta_get_structure")]
    pub fn structure(&self) -> &crate::StructureRef {
        unsafe {
            crate::StructureRef::from_glib_borrow(ffi::gst_custom_meta_get_structure(mut_override(
                &self.0,
            )))
        }
    }

    #[doc(alias = "gst_custom_meta_get_structure")]
    pub fn mut_structure(&mut self) -> &mut crate::StructureRef {
        unsafe {
            crate::StructureRef::from_glib_borrow_mut(ffi::gst_custom_meta_get_structure(
                &mut self.0,
            ))
        }
    }

    #[doc(alias = "gst_custom_meta_has_name")]
    pub fn has_name(&self, name: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_custom_meta_has_name(
                mut_override(&self.0),
                name.to_glib_none().0,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_get_iterate_meta() {
        crate::init().unwrap();

        let mut buffer = crate::Buffer::new();
        let parent = crate::Buffer::new();
        {
            let meta = ParentBufferMeta::add(buffer.get_mut().unwrap(), &parent);
            unsafe {
                assert_eq!(meta.parent().as_ptr(), parent.as_ptr());
            }
        }

        {
            let metas = buffer.iter_meta::<Meta>();
            assert_eq!(metas.count(), 1);
        }
        {
            let metas = buffer.get_mut().unwrap().iter_meta_mut::<Meta>();
            assert_eq!(metas.count(), 1);
        }
        {
            let metas = buffer.iter_meta::<ParentBufferMeta>().collect::<Vec<_>>();
            assert_eq!(metas.len(), 1);
            unsafe {
                assert_eq!(metas[0].parent().as_ptr(), parent.as_ptr());
            }
        }
        {
            let metas = buffer
                .get_mut()
                .unwrap()
                .iter_meta_mut::<ParentBufferMeta>()
                .collect::<Vec<_>>();
            assert_eq!(metas.len(), 1);
            unsafe {
                assert_eq!(metas[0].parent().as_ptr(), parent.as_ptr());
            }
        }

        {
            let meta = buffer
                .get_mut()
                .unwrap()
                .meta_mut::<ParentBufferMeta>()
                .unwrap();
            unsafe {
                assert_eq!(meta.parent().as_ptr(), parent.as_ptr());
            }
            meta.remove();
        }

        {
            let metas = buffer.iter_meta::<Meta>();
            assert_eq!(metas.count(), 0);
        }
        {
            let metas = buffer.get_mut().unwrap().iter_meta_mut::<Meta>();
            assert_eq!(metas.count(), 0);
        }
        {
            let metas = buffer.iter_meta::<ParentBufferMeta>();
            assert_eq!(metas.count(), 0);
        }
        {
            let metas = buffer
                .get_mut()
                .unwrap()
                .iter_meta_mut::<ParentBufferMeta>();
            assert_eq!(metas.count(), 0);
        }

        assert!(buffer.meta::<ParentBufferMeta>().is_none());
    }
}
