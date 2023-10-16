// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
use std::ptr;
use std::{fmt, marker::PhantomData, ops};

use glib::translate::*;

use crate::{Buffer, BufferRef, Caps, CapsRef, ClockTime};

pub unsafe trait MetaAPI: Sync + Send + Sized {
    type GstType;

    #[doc(alias = "get_meta_api")]
    fn meta_api() -> glib::Type;
}

pub trait MetaAPIExt: MetaAPI {
    #[inline]
    #[doc(alias = "gst_meta_api_type_has_tag")]
    fn has_tag(&self, tag: glib::Quark) -> bool {
        unsafe {
            from_glib(ffi::gst_meta_api_type_has_tag(
                Self::meta_api().into_glib(),
                tag.into_glib(),
            ))
        }
    }

    #[inline]
    #[doc(alias = "gst_meta_api_type_get_tags")]
    fn tags(&self) -> &[glib::GStringPtr] {
        unsafe {
            glib::StrV::from_glib_borrow(ffi::gst_meta_api_type_get_tags(
                Self::meta_api().into_glib(),
            ))
        }
    }

    #[inline]
    unsafe fn from_ptr(buffer: &BufferRef, ptr: *const Self::GstType) -> MetaRef<Self> {
        debug_assert!(!ptr.is_null());

        let meta_api = Self::meta_api();
        if meta_api != glib::Type::INVALID {
            debug_assert_eq!(
                meta_api,
                from_glib((*(*(ptr as *const ffi::GstMeta)).info).api)
            )
        }

        MetaRef {
            meta: &*(ptr as *const Self),
            buffer,
        }
    }

    #[inline]
    unsafe fn from_mut_ptr<T>(
        buffer: &mut BufferRef,
        ptr: *mut Self::GstType,
    ) -> MetaRefMut<Self, T> {
        debug_assert!(!ptr.is_null());

        let meta_api = Self::meta_api();
        if meta_api != glib::Type::INVALID {
            debug_assert_eq!(
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

impl<A: MetaAPI> MetaAPIExt for A {}

#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
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

    #[inline]
    fn deref(&self) -> &T {
        self.meta
    }
}

impl<'a, T> AsRef<MetaRef<'a, T>> for MetaRef<'a, T> {
    #[inline]
    fn as_ref(&self) -> &MetaRef<'a, T> {
        self
    }
}

impl<'a, T> AsRef<T> for MetaRef<'a, T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self.meta
    }
}

impl<'a, T, U> ops::Deref for MetaRefMut<'a, T, U> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        self.meta
    }
}

impl<'a, T, U> ops::DerefMut for MetaRefMut<'a, T, U> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        self.meta
    }
}

impl<'a, T, U> AsRef<MetaRef<'a, T>> for MetaRefMut<'a, T, U> {
    #[inline]
    fn as_ref(&self) -> &MetaRef<'a, T> {
        unsafe { &*(self as *const MetaRefMut<'a, T, U> as *const MetaRef<'a, T>) }
    }
}

impl<'a, T, U> AsMut<T> for MetaRefMut<'a, T, U> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        self.meta
    }
}

impl<'a, T> MetaRef<'a, T> {
    #[doc(alias = "get_api")]
    #[inline]
    pub fn api(&self) -> glib::Type {
        unsafe {
            let meta = self.meta as *const _ as *const ffi::GstMeta;
            let info = (*meta).info;
            glib::Type::from_glib((*info).api)
        }
    }

    #[inline]
    pub fn flags(&self) -> crate::MetaFlags {
        unsafe {
            let meta = self.meta as *const _ as *const ffi::GstMeta;
            from_glib((*meta).flags)
        }
    }

    #[inline]
    pub fn type_(&self) -> glib::Type {
        unsafe {
            let meta = self.meta as *const _ as *const ffi::GstMeta;
            let info = (*meta).info;
            glib::Type::from_glib((*info).type_)
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "get_seqnum")]
    #[doc(alias = "gst_meta_get_seqnum")]
    #[inline]
    pub fn seqnum(&self) -> MetaSeqnum {
        unsafe {
            let meta = self.meta as *const _ as *const ffi::GstMeta;
            MetaSeqnum(ffi::gst_meta_get_seqnum(meta))
        }
    }

    #[inline]
    pub fn upcast_ref(&self) -> &MetaRef<'a, Meta> {
        unsafe { &*(self as *const MetaRef<'a, T> as *const MetaRef<'a, Meta>) }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T::GstType
    where
        T: MetaAPI,
    {
        self.meta as *const _ as *const <T as MetaAPI>::GstType
    }
}

impl<'a> MetaRef<'a, Meta> {
    #[inline]
    pub fn downcast_ref<T: MetaAPI>(&self) -> Option<&MetaRef<'a, T>> {
        let target_type = T::meta_api();
        let type_ = self.api();

        if type_ == glib::Type::INVALID || target_type == type_ {
            Some(unsafe { &*(self as *const MetaRef<'a, Meta> as *const MetaRef<'a, T>) })
        } else {
            None
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[inline]
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
    #[inline]
    pub fn api(&self) -> glib::Type {
        unsafe {
            let meta = self.meta as *const _ as *const ffi::GstMeta;
            let info = (*meta).info;
            glib::Type::from_glib((*info).api)
        }
    }

    #[inline]
    pub fn flags(&self) -> crate::MetaFlags {
        unsafe {
            let meta = self.meta as *const _ as *const ffi::GstMeta;
            from_glib((*meta).flags)
        }
    }

    #[inline]
    pub fn type_(&self) -> glib::Type {
        unsafe {
            let meta = self.meta as *const _ as *const ffi::GstMeta;
            let info = (*meta).info;
            glib::Type::from_glib((*info).type_)
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "get_seqnum")]
    #[doc(alias = "gst_meta_get_seqnum")]
    #[inline]
    pub fn seqnum(&self) -> u64 {
        unsafe {
            let meta = self.meta as *const _ as *const ffi::GstMeta;
            ffi::gst_meta_get_seqnum(meta)
        }
    }

    #[inline]
    pub fn upcast_ref(&self) -> &MetaRef<'a, Meta> {
        unsafe { &*(self as *const MetaRefMut<'a, T, U> as *const MetaRef<'a, Meta>) }
    }

    #[inline]
    pub fn upcast_mut(&mut self) -> &MetaRefMut<'a, Meta, U> {
        unsafe { &mut *(self as *mut MetaRefMut<'a, T, U> as *mut MetaRefMut<'a, Meta, U>) }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T::GstType
    where
        T: MetaAPI,
    {
        self.meta as *const _ as *const <T as MetaAPI>::GstType
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T::GstType
    where
        T: MetaAPI,
    {
        self.meta as *mut _ as *mut <T as MetaAPI>::GstType
    }
}

impl<'a, T> MetaRefMut<'a, T, Standalone> {
    #[doc(alias = "gst_buffer_remove_meta")]
    pub fn remove(self) -> Result<(), glib::BoolError> {
        if self.flags().contains(crate::MetaFlags::LOCKED) {
            return Err(glib::bool_error!("Can't remove locked meta"));
        }

        unsafe {
            let res = ffi::gst_buffer_remove_meta(
                self.buffer.as_mut_ptr(),
                self.meta as *mut T as *mut ffi::GstMeta,
            );
            debug_assert_ne!(res, glib::ffi::GFALSE);

            Ok(())
        }
    }
}

impl<'a, U> MetaRefMut<'a, Meta, U> {
    #[inline]
    pub fn downcast_ref<T: MetaAPI>(&mut self) -> Option<&MetaRefMut<'a, T, U>> {
        let target_type = T::meta_api();
        let type_ = self.api();

        if type_ == glib::Type::INVALID || target_type == type_ {
            Some(unsafe { &*(self as *mut MetaRefMut<'a, Meta, U> as *const MetaRefMut<'a, T, U>) })
        } else {
            None
        }
    }

    #[inline]
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

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[inline]
    pub fn try_as_custom_meta(&self) -> Option<&MetaRefMut<'a, CustomMeta, U>> {
        unsafe {
            if ffi::gst_meta_info_is_custom(&*self.0.info) == glib::ffi::GFALSE {
                return None;
            }

            Some(&*(self as *const MetaRefMut<'a, Meta, U> as *const MetaRefMut<'a, CustomMeta, U>))
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[inline]
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

unsafe impl MetaAPI for Meta {
    type GstType = ffi::GstMeta;

    #[inline]
    fn meta_api() -> glib::Type {
        glib::Type::INVALID
    }
}

impl fmt::Debug for Meta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Meta")
            .field("api", &unsafe { glib::Type::from_glib((*self.0.info).api) })
            .field("type", &unsafe {
                glib::Type::from_glib((*self.0.info).type_)
            })
            .field("flags", &unsafe {
                crate::MetaFlags::from_glib(self.0.flags)
            })
            .finish()
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
    #[inline]
    pub fn parent(&self) -> &BufferRef {
        unsafe { BufferRef::from_ptr(self.0.buffer) }
    }

    #[doc(alias = "get_parent_owned")]
    #[inline]
    pub fn parent_owned(&self) -> Buffer {
        unsafe { from_glib_none(self.0.buffer) }
    }
}

unsafe impl MetaAPI for ParentBufferMeta {
    type GstType = ffi::GstParentBufferMeta;

    #[doc(alias = "gst_parent_buffer_meta_api_get_type")]
    #[inline]
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
            let meta =
                ffi::gst_buffer_add_protection_meta(buffer.as_mut_ptr(), info.into_glib_ptr());

            Self::from_mut_ptr(buffer, meta)
        }
    }

    #[doc(alias = "get_info")]
    #[inline]
    pub fn info(&self) -> &crate::StructureRef {
        unsafe { crate::StructureRef::from_glib_borrow(self.0.info) }
    }

    #[doc(alias = "get_info_mut")]
    #[inline]
    pub fn info_mut(&mut self) -> &mut crate::StructureRef {
        unsafe { crate::StructureRef::from_glib_borrow_mut(self.0.info) }
    }
}

unsafe impl MetaAPI for ProtectionMeta {
    type GstType = ffi::GstProtectionMeta;

    #[doc(alias = "gst_protection_meta_api_get_type")]
    #[inline]
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

#[repr(transparent)]
#[doc(alias = "GstReferenceTimestampMeta")]
pub struct ReferenceTimestampMeta(ffi::GstReferenceTimestampMeta);

unsafe impl Send for ReferenceTimestampMeta {}
unsafe impl Sync for ReferenceTimestampMeta {}

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
    #[inline]
    pub fn reference(&self) -> &CapsRef {
        unsafe { CapsRef::from_ptr(self.0.reference) }
    }

    #[doc(alias = "get_parent_owned")]
    #[inline]
    pub fn parent_owned(&self) -> Caps {
        unsafe { from_glib_none(self.0.reference) }
    }

    #[doc(alias = "get_timestamp")]
    #[inline]
    pub fn timestamp(&self) -> ClockTime {
        unsafe { try_from_glib(self.0.timestamp).expect("undefined timestamp") }
    }

    #[doc(alias = "get_duration")]
    #[inline]
    pub fn duration(&self) -> Option<ClockTime> {
        unsafe { from_glib(self.0.duration) }
    }
}

unsafe impl MetaAPI for ReferenceTimestampMeta {
    type GstType = ffi::GstReferenceTimestampMeta;

    #[doc(alias = "gst_reference_timestamp_meta_api_get_type")]
    #[inline]
    fn meta_api() -> glib::Type {
        unsafe { from_glib(ffi::gst_reference_timestamp_meta_api_get_type()) }
    }
}

impl fmt::Debug for ReferenceTimestampMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::utils::Displayable;

        f.debug_struct("ReferenceTimestampMeta")
            .field("reference", &self.reference())
            .field("timestamp", &self.timestamp().display())
            .field("duration", &self.duration().display())
            .finish()
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
#[repr(transparent)]
#[doc(alias = "GstCustomMeta")]
pub struct CustomMeta(ffi::GstCustomMeta);

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
unsafe impl Send for CustomMeta {}
#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
unsafe impl Sync for CustomMeta {}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
impl CustomMeta {
    #[doc(alias = "gst_meta_register_custom")]
    pub fn register(name: &str, tags: &[&str]) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gst_meta_register_custom(
                name.to_glib_none().0,
                tags.to_glib_none().0,
                None,
                ptr::null_mut(),
                None,
            );
        }
    }

    #[doc(alias = "gst_meta_register_custom")]
    pub fn register_with_transform<
        F: Fn(&mut BufferRef, &CustomMeta, &BufferRef, glib::Quark) -> bool + Send + Sync + 'static,
    >(
        name: &str,
        tags: &[&str],
        transform_func: F,
    ) {
        assert_initialized_main_thread!();
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

    #[doc(alias = "gst_meta_register_simple")]
    pub fn register_simple(name: &str) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gst_meta_register_custom(
                name.to_glib_none().0,
                ptr::null_mut(),
                None,
                ptr::null_mut(),
                None,
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
    #[inline]
    pub fn structure(&self) -> &crate::StructureRef {
        unsafe {
            crate::StructureRef::from_glib_borrow(ffi::gst_custom_meta_get_structure(mut_override(
                &self.0,
            )))
        }
    }

    #[doc(alias = "gst_custom_meta_get_structure")]
    #[inline]
    pub fn mut_structure(&mut self) -> &mut crate::StructureRef {
        unsafe {
            crate::StructureRef::from_glib_borrow_mut(ffi::gst_custom_meta_get_structure(
                &mut self.0,
            ))
        }
    }

    #[doc(alias = "gst_custom_meta_has_name")]
    #[inline]
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
            assert_eq!(meta.parent().as_ptr(), parent.as_ptr());
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
            assert_eq!(metas[0].parent().as_ptr(), parent.as_ptr());
        }
        {
            let metas = buffer
                .get_mut()
                .unwrap()
                .iter_meta_mut::<ParentBufferMeta>()
                .collect::<Vec<_>>();
            assert_eq!(metas.len(), 1);
            assert_eq!(metas[0].parent().as_ptr(), parent.as_ptr());
            assert!(!metas[0].has_tag(glib::Quark::from_str("video")));
            assert!(metas[0].has_tag(glib::Quark::from_str("memory-reference")));
            assert_eq!(metas[0].tags().len(), 1);
        }

        {
            let meta = buffer
                .get_mut()
                .unwrap()
                .meta_mut::<ParentBufferMeta>()
                .unwrap();
            assert_eq!(meta.parent().as_ptr(), parent.as_ptr());
            meta.remove().unwrap();
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
