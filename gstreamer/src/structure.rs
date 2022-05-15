// Take a look at the license at the top of the repository in the LICENSE file.

use std::borrow::{Borrow, BorrowMut, ToOwned};
use std::ffi::CStr;
use std::fmt;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr;
use std::str;

use crate::Fraction;

use glib::translate::*;
use glib::value::{FromValue, SendValue, ToSendValue};
use glib::StaticType;

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum GetError<E: std::error::Error> {
    #[error("GetError: Structure field with name {name} not found")]
    FieldNotFound { name: &'static str },
    #[error("GetError: Structure field with name {name} not retrieved")]
    ValueGetError {
        name: &'static str,
        #[source]
        error: E,
    },
}

impl<E: std::error::Error> GetError<E> {
    fn new_field_not_found(name: &'static str) -> Self {
        skip_assert_initialized!();
        GetError::FieldNotFound { name }
    }

    fn from_value_get_error(name: &'static str, error: E) -> Self {
        skip_assert_initialized!();
        GetError::ValueGetError { name, error }
    }
}

#[doc(alias = "GstStructure")]
#[repr(transparent)]
pub struct Structure(ptr::NonNull<ffi::GstStructure>);
unsafe impl Send for Structure {}
unsafe impl Sync for Structure {}

impl Structure {
    #[doc(alias = "gst_structure_new")]
    pub fn builder(name: &str) -> Builder {
        assert_initialized_main_thread!();
        Builder::new(name)
    }

    #[doc(alias = "gst_structure_new_empty")]
    pub fn new_empty(name: &str) -> Structure {
        assert_initialized_main_thread!();
        unsafe {
            let ptr = ffi::gst_structure_new_empty(name.to_glib_none().0);
            assert!(!ptr.is_null());
            Structure(ptr::NonNull::new_unchecked(ptr))
        }
    }

    #[doc(alias = "gst_structure_new")]
    pub fn new(name: &str, values: &[(&str, &(dyn ToSendValue + Sync))]) -> Structure {
        assert_initialized_main_thread!();
        let mut structure = Structure::new_empty(name);

        for &(f, v) in values {
            structure.set_value(f, v.to_send_value());
        }

        structure
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_iter<'a>(
        name: &str,
        iter: impl IntoIterator<Item = (&'a str, SendValue)>,
    ) -> Structure {
        assert_initialized_main_thread!();
        let mut structure = Structure::new_empty(name);

        iter.into_iter()
            .for_each(|(f, v)| structure.set_value(f, v));

        structure
    }
}

impl IntoGlibPtr<*mut ffi::GstStructure> for Structure {
    unsafe fn into_glib_ptr(self) -> *mut ffi::GstStructure {
        let s = mem::ManuallyDrop::new(self);
        s.0.as_ptr()
    }
}

impl Deref for Structure {
    type Target = StructureRef;

    fn deref(&self) -> &StructureRef {
        unsafe { &*(self.0.as_ptr() as *const StructureRef) }
    }
}

impl DerefMut for Structure {
    fn deref_mut(&mut self) -> &mut StructureRef {
        unsafe { &mut *(self.0.as_ptr() as *mut StructureRef) }
    }
}

impl AsRef<StructureRef> for Structure {
    fn as_ref(&self) -> &StructureRef {
        self.deref()
    }
}

impl AsMut<StructureRef> for Structure {
    fn as_mut(&mut self) -> &mut StructureRef {
        self.deref_mut()
    }
}

impl Clone for Structure {
    fn clone(&self) -> Self {
        unsafe {
            let ptr = ffi::gst_structure_copy(self.0.as_ref());
            assert!(!ptr.is_null());
            Structure(ptr::NonNull::new_unchecked(ptr))
        }
    }
}

impl Drop for Structure {
    fn drop(&mut self) {
        unsafe { ffi::gst_structure_free(self.0.as_mut()) }
    }
}

impl fmt::Debug for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Structure").field(&self.to_string()).finish()
    }
}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Need to make sure to not call ToString::to_string() here, which
        // we have because of the Display impl. We need StructureRef::to_string()
        f.write_str(&StructureRef::to_string(self.as_ref()))
    }
}

impl PartialEq for Structure {
    fn eq(&self, other: &Structure) -> bool {
        self.as_ref().eq(other)
    }
}

impl PartialEq<StructureRef> for Structure {
    fn eq(&self, other: &StructureRef) -> bool {
        self.as_ref().eq(other)
    }
}

impl Eq for Structure {}

impl str::FromStr for Structure {
    type Err = glib::BoolError;

    #[doc(alias = "gst_structure_from_string")]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_initialized_main_thread!();
        unsafe {
            let structure = ffi::gst_structure_from_string(s.to_glib_none().0, ptr::null_mut());
            if structure.is_null() {
                Err(glib::bool_error!("Failed to parse structure from string"))
            } else {
                Ok(Self(ptr::NonNull::new_unchecked(structure)))
            }
        }
    }
}

impl Borrow<StructureRef> for Structure {
    fn borrow(&self) -> &StructureRef {
        self.as_ref()
    }
}

impl BorrowMut<StructureRef> for Structure {
    fn borrow_mut(&mut self) -> &mut StructureRef {
        self.as_mut()
    }
}

impl ToOwned for StructureRef {
    type Owned = Structure;

    fn to_owned(&self) -> Structure {
        unsafe {
            let ptr = ffi::gst_structure_copy(&self.0);
            assert!(!ptr.is_null());
            Structure(ptr::NonNull::new_unchecked(ptr))
        }
    }
}

impl glib::types::StaticType for Structure {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(ffi::gst_structure_get_type()) }
    }
}

impl<'a> ToGlibPtr<'a, *const ffi::GstStructure> for Structure {
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::GstStructure, Self> {
        unsafe { Stash(self.0.as_ref(), self) }
    }

    fn to_glib_full(&self) -> *const ffi::GstStructure {
        unsafe { ffi::gst_structure_copy(self.0.as_ref()) }
    }
}

impl<'a> ToGlibPtr<'a, *mut ffi::GstStructure> for Structure {
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::GstStructure, Self> {
        unsafe {
            Stash(
                self.0.as_ref() as *const ffi::GstStructure as *mut ffi::GstStructure,
                self,
            )
        }
    }

    fn to_glib_full(&self) -> *mut ffi::GstStructure {
        unsafe { ffi::gst_structure_copy(self.0.as_ref()) }
    }
}

impl<'a> ToGlibPtrMut<'a, *mut ffi::GstStructure> for Structure {
    type Storage = &'a mut Self;

    fn to_glib_none_mut(&'a mut self) -> StashMut<*mut ffi::GstStructure, Self> {
        unsafe { StashMut(self.0.as_mut(), self) }
    }
}

impl FromGlibPtrNone<*const ffi::GstStructure> for Structure {
    unsafe fn from_glib_none(ptr: *const ffi::GstStructure) -> Self {
        assert!(!ptr.is_null());
        let ptr = ffi::gst_structure_copy(ptr);
        assert!(!ptr.is_null());
        Structure(ptr::NonNull::new_unchecked(ptr))
    }
}

impl FromGlibPtrNone<*mut ffi::GstStructure> for Structure {
    unsafe fn from_glib_none(ptr: *mut ffi::GstStructure) -> Self {
        assert!(!ptr.is_null());
        let ptr = ffi::gst_structure_copy(ptr);
        assert!(!ptr.is_null());
        Structure(ptr::NonNull::new_unchecked(ptr))
    }
}

impl FromGlibPtrFull<*const ffi::GstStructure> for Structure {
    unsafe fn from_glib_full(ptr: *const ffi::GstStructure) -> Self {
        assert!(!ptr.is_null());
        Structure(ptr::NonNull::new_unchecked(ptr as *mut ffi::GstStructure))
    }
}

impl FromGlibPtrFull<*mut ffi::GstStructure> for Structure {
    unsafe fn from_glib_full(ptr: *mut ffi::GstStructure) -> Self {
        assert!(!ptr.is_null());
        Structure(ptr::NonNull::new_unchecked(ptr))
    }
}

impl FromGlibPtrBorrow<*const ffi::GstStructure> for Structure {
    unsafe fn from_glib_borrow(ptr: *const ffi::GstStructure) -> Borrowed<Self> {
        Borrowed::new(from_glib_full(ptr))
    }
}

impl FromGlibPtrBorrow<*mut ffi::GstStructure> for Structure {
    unsafe fn from_glib_borrow(ptr: *mut ffi::GstStructure) -> Borrowed<Self> {
        Borrowed::new(from_glib_full(ptr))
    }
}

impl glib::value::ValueType for Structure {
    type Type = Self;
}

impl glib::value::ValueTypeOptional for Structure {}

unsafe impl<'a> glib::value::FromValue<'a> for Structure {
    type Checker = glib::value::GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib_none(
            glib::gobject_ffi::g_value_get_boxed(value.to_glib_none().0) as *mut ffi::GstStructure
        )
    }
}

impl glib::value::ToValue for Structure {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                glib::translate::ToGlibPtr::<*const ffi::GstStructure>::to_glib_none(self).0
                    as *mut _,
            )
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl glib::value::ToValueOptional for Structure {
    fn to_value_optional(s: Option<&Self>) -> glib::Value {
        skip_assert_initialized!();
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                glib::translate::ToGlibPtr::<*const ffi::GstStructure>::to_glib_none(&s).0
                    as *mut _,
            )
        }
        value
    }
}

impl GlibPtrDefault for Structure {
    type GlibType = *mut ffi::GstStructure;
}

#[repr(transparent)]
#[doc(alias = "GstStructure")]
pub struct StructureRef(ffi::GstStructure);

unsafe impl Send for StructureRef {}
unsafe impl Sync for StructureRef {}

impl StructureRef {
    pub unsafe fn from_glib_borrow<'a>(ptr: *const ffi::GstStructure) -> &'a StructureRef {
        assert!(!ptr.is_null());

        &*(ptr as *mut StructureRef)
    }

    pub unsafe fn from_glib_borrow_mut<'a>(ptr: *mut ffi::GstStructure) -> &'a mut StructureRef {
        assert!(!ptr.is_null());

        &mut *(ptr as *mut StructureRef)
    }

    pub unsafe fn as_ptr(&self) -> *const ffi::GstStructure {
        self as *const Self as *const ffi::GstStructure
    }

    pub unsafe fn as_mut_ptr(&self) -> *mut ffi::GstStructure {
        self as *const Self as *mut ffi::GstStructure
    }

    #[doc(alias = "gst_structure_get")]
    pub fn get<'a, T: FromValue<'a>>(
        &'a self,
        name: &str,
    ) -> Result<T, GetError<<<T as FromValue<'a>>::Checker as glib::value::ValueTypeChecker>::Error>>
    {
        let name = glib::Quark::from_str(name);
        self.get_by_quark(name)
    }

    #[doc(alias = "gst_structure_get")]
    pub fn get_optional<'a, T: FromValue<'a>>(
        &'a self,
        name: &str,
    ) -> Result<
        Option<T>,
        GetError<<<T as FromValue<'a>>::Checker as glib::value::ValueTypeChecker>::Error>,
    > {
        let name = glib::Quark::from_str(name);
        self.get_optional_by_quark(name)
    }

    #[doc(alias = "get_value")]
    #[doc(alias = "gst_structure_get_value")]
    pub fn value(&self, name: &str) -> Result<&SendValue, GetError<std::convert::Infallible>> {
        let name = glib::Quark::from_str(name);
        self.value_by_quark(name)
    }

    #[doc(alias = "gst_structure_id_get")]
    pub fn get_by_quark<'a, T: FromValue<'a>>(
        &'a self,
        name: glib::Quark,
    ) -> Result<T, GetError<<<T as FromValue<'a>>::Checker as glib::value::ValueTypeChecker>::Error>>
    {
        self.value_by_quark(name)
            .map_err(|err| match err {
                GetError::FieldNotFound { name } => GetError::FieldNotFound { name },
                _ => unreachable!(),
            })?
            .get()
            .map_err(|err| GetError::from_value_get_error(name.as_str(), err))
    }

    #[doc(alias = "gst_structure_id_get")]
    pub fn get_optional_by_quark<'a, T: FromValue<'a>>(
        &'a self,
        name: glib::Quark,
    ) -> Result<
        Option<T>,
        GetError<<<T as FromValue<'a>>::Checker as glib::value::ValueTypeChecker>::Error>,
    > {
        self.value_by_quark(name)
            .ok()
            .map(|v| v.get())
            .transpose()
            .map_err(|err| GetError::from_value_get_error(name.as_str(), err))
    }

    #[doc(alias = "gst_structure_id_get_value")]
    pub fn value_by_quark(
        &self,
        name: glib::Quark,
    ) -> Result<&SendValue, GetError<std::convert::Infallible>> {
        unsafe {
            let value = ffi::gst_structure_id_get_value(&self.0, name.into_glib());

            if value.is_null() {
                return Err(GetError::new_field_not_found(name.as_str()));
            }

            Ok(&*(value as *const SendValue))
        }
    }

    #[doc(alias = "gst_structure_set")]
    pub fn set<T: ToSendValue + Sync>(&mut self, name: &str, value: T) {
        let value = value.to_send_value();
        self.set_value(name, value);
    }

    #[doc(alias = "gst_structure_set_value")]
    pub fn set_value(&mut self, name: &str, value: SendValue) {
        unsafe {
            ffi::gst_structure_take_value(
                &mut self.0,
                name.to_glib_none().0,
                &mut value.into_raw(),
            );
        }
    }

    #[doc(alias = "gst_structure_id_set")]
    pub fn set_by_quark<T: ToSendValue + Sync>(&mut self, name: glib::Quark, value: T) {
        let value = value.to_send_value();
        self.set_value_by_quark(name, value);
    }

    #[doc(alias = "gst_structure_id_set_value")]
    pub fn set_value_by_quark(&mut self, name: glib::Quark, value: SendValue) {
        unsafe {
            ffi::gst_structure_id_take_value(&mut self.0, name.into_glib(), &mut value.into_raw());
        }
    }

    #[doc(alias = "get_name")]
    #[doc(alias = "gst_structure_get_name")]
    pub fn name<'a>(&self) -> &'a str {
        unsafe {
            CStr::from_ptr(ffi::gst_structure_get_name(&self.0))
                .to_str()
                .unwrap()
        }
    }

    #[doc(alias = "gst_structure_get_name_id")]
    pub fn name_quark(&self) -> glib::Quark {
        unsafe { from_glib(ffi::gst_structure_get_name_id(&self.0)) }
    }

    #[doc(alias = "gst_structure_set_name")]
    pub fn set_name(&mut self, name: &str) {
        unsafe { ffi::gst_structure_set_name(&mut self.0, name.to_glib_none().0) }
    }

    #[doc(alias = "gst_structure_has_name")]
    pub fn has_name(&self, name: &str) -> bool {
        self.name() == name
    }

    #[doc(alias = "gst_structure_has_field")]
    pub fn has_field(&self, field: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_structure_has_field(
                &self.0,
                field.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_structure_has_field_typed")]
    pub fn has_field_with_type(&self, field: &str, type_: glib::Type) -> bool {
        unsafe {
            from_glib(ffi::gst_structure_has_field_typed(
                &self.0,
                field.to_glib_none().0,
                type_.into_glib(),
            ))
        }
    }

    #[doc(alias = "gst_structure_id_has_field")]
    pub fn has_field_by_quark(&self, field: glib::Quark) -> bool {
        unsafe { from_glib(ffi::gst_structure_id_has_field(&self.0, field.into_glib())) }
    }

    #[doc(alias = "gst_structure_id_has_field_typed")]
    pub fn has_field_with_type_by_quark(&self, field: glib::Quark, type_: glib::Type) -> bool {
        unsafe {
            from_glib(ffi::gst_structure_id_has_field_typed(
                &self.0,
                field.into_glib(),
                type_.into_glib(),
            ))
        }
    }

    #[doc(alias = "gst_structure_remove_field")]
    pub fn remove_field(&mut self, field: &str) {
        unsafe {
            ffi::gst_structure_remove_field(&mut self.0, field.to_glib_none().0);
        }
    }

    #[doc(alias = "gst_structure_remove_fields")]
    pub fn remove_fields(&mut self, fields: &[&str]) {
        for f in fields {
            self.remove_field(f)
        }
    }

    #[doc(alias = "gst_structure_remove_all_fields")]
    pub fn remove_all_fields(&mut self) {
        unsafe {
            ffi::gst_structure_remove_all_fields(&mut self.0);
        }
    }

    pub fn fields(&self) -> FieldIterator {
        FieldIterator::new(self)
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }

    #[doc(alias = "get_nth_field_name")]
    #[doc(alias = "gst_structure_nth_field_name")]
    pub fn nth_field_name<'a>(&self, idx: u32) -> Option<&'a str> {
        unsafe {
            let field_name = ffi::gst_structure_nth_field_name(&self.0, idx);
            if field_name.is_null() {
                return None;
            }

            Some(CStr::from_ptr(field_name).to_str().unwrap())
        }
    }

    #[doc(alias = "gst_structure_n_fields")]
    pub fn n_fields(&self) -> u32 {
        unsafe { ffi::gst_structure_n_fields(&self.0) as u32 }
    }

    #[doc(alias = "gst_structure_can_intersect")]
    pub fn can_intersect(&self, other: &StructureRef) -> bool {
        unsafe { from_glib(ffi::gst_structure_can_intersect(&self.0, &other.0)) }
    }

    #[doc(alias = "gst_structure_intersect")]
    pub fn intersect(&self, other: &StructureRef) -> Option<Structure> {
        unsafe { from_glib_full(ffi::gst_structure_intersect(&self.0, &other.0)) }
    }

    #[doc(alias = "gst_structure_is_subset")]
    pub fn is_subset(&self, superset: &StructureRef) -> bool {
        unsafe { from_glib(ffi::gst_structure_is_subset(&self.0, &superset.0)) }
    }

    #[doc(alias = "gst_structure_fixate")]
    pub fn fixate(&mut self) {
        unsafe { ffi::gst_structure_fixate(&mut self.0) }
    }

    #[doc(alias = "gst_structure_fixate_field")]
    pub fn fixate_field(&mut self, name: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_structure_fixate_field(
                &mut self.0,
                name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_structure_fixate_field_boolean")]
    pub fn fixate_field_bool(&mut self, name: &str, target: bool) -> bool {
        unsafe {
            from_glib(ffi::gst_structure_fixate_field_boolean(
                &mut self.0,
                name.to_glib_none().0,
                target.into_glib(),
            ))
        }
    }

    #[doc(alias = "gst_structure_fixate_field_string")]
    pub fn fixate_field_str(&mut self, name: &str, target: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_structure_fixate_field_string(
                &mut self.0,
                name.to_glib_none().0,
                target.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_structure_fixate_field_nearest_double")]
    pub fn fixate_field_nearest_double(&mut self, name: &str, target: f64) -> bool {
        unsafe {
            from_glib(ffi::gst_structure_fixate_field_nearest_double(
                &mut self.0,
                name.to_glib_none().0,
                target,
            ))
        }
    }

    #[doc(alias = "gst_structure_fixate_field_nearest_fraction")]
    pub fn fixate_field_nearest_fraction<T: Into<Fraction>>(
        &mut self,
        name: &str,
        target: T,
    ) -> bool {
        skip_assert_initialized!();

        let target = target.into();
        unsafe {
            from_glib(ffi::gst_structure_fixate_field_nearest_fraction(
                &mut self.0,
                name.to_glib_none().0,
                target.numer(),
                target.denom(),
            ))
        }
    }

    #[doc(alias = "gst_structure_fixate_field_nearest_int")]
    pub fn fixate_field_nearest_int(&mut self, name: &str, target: i32) -> bool {
        unsafe {
            from_glib(ffi::gst_structure_fixate_field_nearest_int(
                &mut self.0,
                name.to_glib_none().0,
                target,
            ))
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_structure_serialize")]
    pub fn serialize(&self, flags: crate::SerializeFlags) -> glib::GString {
        unsafe { from_glib_full(ffi::gst_structure_serialize(&self.0, flags.into_glib())) }
    }

    #[doc(alias = "gst_structure_foreach")]
    pub fn foreach<F: FnMut(glib::Quark, &glib::Value) -> std::ops::ControlFlow<()>>(
        &self,
        mut func: F,
    ) -> bool {
        unsafe {
            unsafe extern "C" fn trampoline<
                F: FnMut(glib::Quark, &glib::Value) -> std::ops::ControlFlow<()>,
            >(
                quark: glib::ffi::GQuark,
                value: *const glib::gobject_ffi::GValue,
                user_data: glib::ffi::gpointer,
            ) -> glib::ffi::gboolean {
                let func = &mut *(user_data as *mut F);
                let res = func(from_glib(quark), &*(value as *const glib::Value));

                matches!(res, std::ops::ControlFlow::Continue(_)).into_glib()
            }
            let func = &mut func as *mut F;
            from_glib(ffi::gst_structure_foreach(
                self.as_ptr(),
                Some(trampoline::<F>),
                func as glib::ffi::gpointer,
            ))
        }
    }

    #[doc(alias = "gst_structure_map_in_place")]
    pub fn map_in_place<F: FnMut(glib::Quark, &mut glib::Value) -> std::ops::ControlFlow<()>>(
        &mut self,
        mut func: F,
    ) -> bool {
        unsafe {
            unsafe extern "C" fn trampoline<
                F: FnMut(glib::Quark, &mut glib::Value) -> std::ops::ControlFlow<()>,
            >(
                quark: glib::ffi::GQuark,
                value: *mut glib::gobject_ffi::GValue,
                user_data: glib::ffi::gpointer,
            ) -> glib::ffi::gboolean {
                let func = &mut *(user_data as *mut F);
                let res = func(from_glib(quark), &mut *(value as *mut glib::Value));

                matches!(res, std::ops::ControlFlow::Continue(_)).into_glib()
            }
            let func = &mut func as *mut F;
            from_glib(ffi::gst_structure_map_in_place(
                self.as_mut_ptr(),
                Some(trampoline::<F>),
                func as glib::ffi::gpointer,
            ))
        }
    }

    #[doc(alias = "gst_structure_filter_and_map_in_place")]
    pub fn filter_map_in_place<F: FnMut(glib::Quark, glib::Value) -> Option<glib::Value>>(
        &mut self,
        mut func: F,
    ) {
        unsafe {
            unsafe extern "C" fn trampoline<
                F: FnMut(glib::Quark, glib::Value) -> Option<glib::Value>,
            >(
                quark: glib::ffi::GQuark,
                value: *mut glib::gobject_ffi::GValue,
                user_data: glib::ffi::gpointer,
            ) -> glib::ffi::gboolean {
                let func = &mut *(user_data as *mut F);

                let v = mem::replace(
                    &mut *(value as *mut glib::Value),
                    glib::Value::uninitialized(),
                );
                match func(from_glib(quark), v) {
                    None => glib::ffi::GFALSE,
                    Some(v) => {
                        *value = v.into_raw();
                        glib::ffi::GTRUE
                    }
                }
            }

            let func = &mut func as *mut F;
            ffi::gst_structure_filter_and_map_in_place(
                self.as_mut_ptr(),
                Some(trampoline::<F>),
                func as glib::ffi::gpointer,
            );
        }
    }
}

impl fmt::Display for StructureRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = unsafe { glib::GString::from_glib_full(ffi::gst_structure_to_string(&self.0)) };
        f.write_str(&s)
    }
}

impl fmt::Debug for StructureRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl PartialEq for StructureRef {
    #[doc(alias = "gst_structure_is_equal")]
    fn eq(&self, other: &StructureRef) -> bool {
        unsafe { from_glib(ffi::gst_structure_is_equal(&self.0, &other.0)) }
    }
}

impl Eq for StructureRef {}

impl glib::types::StaticType for StructureRef {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(ffi::gst_structure_get_type()) }
    }
}

unsafe impl<'a> glib::value::FromValue<'a> for &'a StructureRef {
    type Checker = glib::value::GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        &*(glib::gobject_ffi::g_value_get_boxed(value.to_glib_none().0) as *const StructureRef)
    }
}

impl glib::value::ToValue for StructureRef {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Structure>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                self.as_ptr() as *mut _,
            )
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl glib::value::ToValueOptional for StructureRef {
    fn to_value_optional(s: Option<&Self>) -> glib::Value {
        skip_assert_initialized!();
        let mut value = glib::Value::for_value_type::<Structure>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                s.map(|s| s.as_ptr()).unwrap_or(ptr::null()) as *mut _,
            )
        }
        value
    }
}

#[derive(Debug)]
pub struct FieldIterator<'a> {
    structure: &'a StructureRef,
    idx: u32,
    n_fields: u32,
}

impl<'a> FieldIterator<'a> {
    fn new(structure: &'a StructureRef) -> FieldIterator<'a> {
        skip_assert_initialized!();
        let n_fields = structure.n_fields();

        FieldIterator {
            structure,
            idx: 0,
            n_fields,
        }
    }
}

impl<'a> Iterator for FieldIterator<'a> {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.n_fields {
            return None;
        }

        if let Some(field_name) = self.structure.nth_field_name(self.idx) {
            self.idx += 1;
            Some(field_name)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.idx == self.n_fields {
            return (0, Some(0));
        }

        let remaining = (self.n_fields - self.idx) as usize;

        (remaining, Some(remaining))
    }
}

impl<'a> DoubleEndedIterator for FieldIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.idx == self.n_fields {
            return None;
        }

        self.n_fields -= 1;
        self.structure.nth_field_name(self.n_fields)
    }
}

impl<'a> ExactSizeIterator for FieldIterator<'a> {}

#[derive(Debug)]
pub struct Iter<'a> {
    iter: FieldIterator<'a>,
}

impl<'a> Iter<'a> {
    fn new(structure: &'a StructureRef) -> Iter<'a> {
        skip_assert_initialized!();
        Iter {
            iter: FieldIterator::new(structure),
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'static str, &'a SendValue);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(f) = self.iter.next() {
            let v = self.iter.structure.value(f);
            Some((f, v.unwrap()))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> DoubleEndedIterator for Iter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(f) = self.iter.next_back() {
            let v = self.iter.structure.value(f);
            Some((f, v.unwrap()))
        } else {
            None
        }
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {}

impl<'a> IntoIterator for &'a StructureRef {
    type IntoIter = Iter<'a>;
    type Item = (&'static str, &'a SendValue);

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> std::iter::Extend<(&'a str, SendValue)> for StructureRef {
    fn extend<T: IntoIterator<Item = (&'a str, SendValue)>>(&mut self, iter: T) {
        iter.into_iter().for_each(|(f, v)| self.set_value(f, v));
    }
}

impl std::iter::Extend<(String, SendValue)> for StructureRef {
    fn extend<T: IntoIterator<Item = (String, SendValue)>>(&mut self, iter: T) {
        iter.into_iter().for_each(|(f, v)| self.set_value(&f, v));
    }
}

impl std::iter::Extend<(glib::Quark, SendValue)> for StructureRef {
    fn extend<T: IntoIterator<Item = (glib::Quark, SendValue)>>(&mut self, iter: T) {
        iter.into_iter()
            .for_each(|(f, v)| self.set_value_by_quark(f, v));
    }
}

#[derive(Debug)]
#[must_use = "The builder must be built to be used"]
pub struct Builder {
    s: Structure,
}

impl Builder {
    fn new(name: &str) -> Self {
        skip_assert_initialized!();
        Builder {
            s: Structure::new_empty(name),
        }
    }

    pub fn field<V: ToSendValue + Sync>(mut self, name: &str, value: V) -> Self {
        self.s.set(name, value);
        self
    }

    #[must_use = "Building the structure without using it has no effect"]
    pub fn build(self) -> Structure {
        self.s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_set_get() {
        use glib::{value, Type};

        crate::init().unwrap();

        let mut s = Structure::new_empty("test");
        assert_eq!(s.name(), "test");

        s.set("f1", "abc");
        s.set("f2", &String::from("bcd"));
        s.set("f3", 123i32);

        assert_eq!(s.get::<&str>("f1"), Ok("abc"));
        assert_eq!(s.get::<Option<&str>>("f2"), Ok(Some("bcd")));
        assert_eq!(s.get::<i32>("f3"), Ok(123i32));
        assert_eq!(s.get_optional::<&str>("f1"), Ok(Some("abc")));
        assert_eq!(s.get_optional::<&str>("f4"), Ok(None));
        assert_eq!(s.get_optional::<i32>("f3"), Ok(Some(123i32)));
        assert_eq!(s.get_optional::<i32>("f4"), Ok(None));

        assert_eq!(
            s.get::<i32>("f2"),
            Err(GetError::from_value_get_error(
                "f2",
                value::ValueTypeMismatchError::new(Type::STRING, Type::I32),
            ))
        );
        assert_eq!(
            s.get::<bool>("f3"),
            Err(GetError::from_value_get_error(
                "f3",
                value::ValueTypeMismatchError::new(Type::I32, Type::BOOL),
            ))
        );
        assert_eq!(
            s.get::<&str>("f4"),
            Err(GetError::new_field_not_found("f4"))
        );
        assert_eq!(s.get::<i32>("f4"), Err(GetError::new_field_not_found("f4")));

        assert_eq!(s.fields().collect::<Vec<_>>(), vec!["f1", "f2", "f3"]);

        let v = s.iter().map(|(f, v)| (f, v.clone())).collect::<Vec<_>>();
        assert_eq!(v.len(), 3);
        assert_eq!(v[0].0, "f1");
        assert_eq!(v[0].1.get::<&str>(), Ok("abc"));
        assert_eq!(v[1].0, "f2");
        assert_eq!(v[1].1.get::<&str>(), Ok("bcd"));
        assert_eq!(v[2].0, "f3");
        assert_eq!(v[2].1.get::<i32>(), Ok(123i32));

        let s2 = Structure::new("test", &[("f1", &"abc"), ("f2", &"bcd"), ("f3", &123i32)]);
        assert_eq!(s, s2);
    }

    #[test]
    fn test_builder() {
        crate::init().unwrap();

        let s = Structure::builder("test")
            .field("f1", "abc")
            .field("f2", &String::from("bcd"))
            .field("f3", 123i32)
            .build();

        assert_eq!(s.name(), "test");
        assert_eq!(s.get::<&str>("f1"), Ok("abc"));
        assert_eq!(s.get::<&str>("f2"), Ok("bcd"));
        assert_eq!(s.get::<i32>("f3"), Ok(123i32));
    }

    #[test]
    fn test_string_conversion() {
        crate::init().unwrap();

        let a = "Test, f1=(string)abc, f2=(uint)123;";

        let s = a.parse::<Structure>().unwrap();
        assert_eq!(s.get::<&str>("f1"), Ok("abc"));
        assert_eq!(s.get::<u32>("f2"), Ok(123));

        assert_eq!(a, s.to_string());
    }

    #[test]
    fn test_from_value_optional() {
        use glib::ToValue;

        crate::init().unwrap();

        let a = None::<&Structure>.to_value();
        assert!(a.get::<Option<Structure>>().unwrap().is_none());
        let b = "foo".parse::<Structure>().unwrap().to_value();
        assert!(b.get::<Option<Structure>>().unwrap().is_some());
    }

    #[test]
    fn test_new_from_iter() {
        crate::init().unwrap();

        let s = Structure::builder("test")
            .field("f1", "abc")
            .field("f2", &String::from("bcd"))
            .field("f3", 123i32)
            .build();

        let s2 = Structure::from_iter(
            s.name(),
            s.iter()
                .filter(|(f, _)| *f == "f1")
                .map(|(f, v)| (f, v.clone())),
        );

        assert_eq!(s2.name(), "test");
        assert_eq!(s2.get::<&str>("f1"), Ok("abc"));
        assert!(s2.get::<&str>("f2").is_err());
        assert!(s2.get::<&str>("f3").is_err());
    }
}
