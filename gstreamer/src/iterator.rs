// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib;
use glib::translate::*;
use glib::value::{FromValueOptional, ToValue};
use glib::StaticType;
use glib::Value;
use glib_sys;
use glib_sys::{gconstpointer, gpointer};
use gobject_sys;
use gst_sys;
use std::ffi::CString;
use std::fmt;
use std::iter;
use std::marker::PhantomData;
use std::mem;
use std::ptr;
use std::sync::Arc;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Error)]
pub enum IteratorError {
    #[error("Resync")]
    Resync,
    #[error("Error")]
    Error,
}

// Implemented manually so that we can use generics for the item
pub struct Iterator<T> {
    iter: ptr::NonNull<gst_sys::GstIterator>,
    phantom: PhantomData<T>,
}

impl<T> Iterator<T>
where
    for<'a> T: FromValueOptional<'a> + 'static,
{
    pub unsafe fn into_ptr(self) -> *mut gst_sys::GstIterator {
        let s = mem::ManuallyDrop::new(self);
        let it = s.to_glib_none().0;
        it as *mut _
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Result<Option<T>, IteratorError> {
        unsafe {
            let mut value = Value::uninitialized();
            let res =
                gst_sys::gst_iterator_next(self.to_glib_none_mut().0, value.to_glib_none_mut().0);

            #[allow(clippy::wildcard_in_or_patterns)]
            match res {
                gst_sys::GST_ITERATOR_OK => match value.get::<T>().expect("Iterator::next") {
                    Some(value) => Ok(Some(value)),
                    None => Err(IteratorError::Error),
                },
                gst_sys::GST_ITERATOR_DONE => Ok(None),
                gst_sys::GST_ITERATOR_RESYNC => Err(IteratorError::Resync),
                gst_sys::GST_ITERATOR_ERROR | _ => Err(IteratorError::Error),
            }
        }
    }

    pub fn resync(&mut self) {
        unsafe {
            gst_sys::gst_iterator_resync(self.to_glib_none_mut().0);
        }
    }

    pub fn filter<F>(self, func: F) -> Self
    where
        F: Fn(T) -> bool + Send + Sync + 'static,
    {
        unsafe {
            let func_box: Box<dyn Fn(T) -> bool + Send + Sync + 'static> = Box::new(func);
            let mut closure_value = glib::Value::from_type(from_glib(filter_boxed_get_type::<T>()));
            gobject_sys::g_value_take_boxed(
                closure_value.to_glib_none_mut().0,
                Arc::into_raw(Arc::new(func_box)) as gpointer,
            );

            from_glib_full(gst_sys::gst_iterator_filter(
                self.into_ptr(),
                Some(filter_trampoline::<T>),
                closure_value.to_glib_none().0,
            ))
        }
    }

    pub fn find<F>(&mut self, func: F) -> Option<T>
    where
        F: FnMut(T) -> bool,
    {
        unsafe {
            let mut elem = glib::Value::uninitialized();

            let mut func = func;
            let func_ptr = &mut func as *mut F as gpointer;

            let res = from_glib(gst_sys::gst_iterator_find_custom(
                self.to_glib_none_mut().0,
                Some(find_trampoline::<T, F>),
                elem.to_glib_none_mut().0,
                func_ptr,
            ));
            if res {
                elem.get::<T>().expect("Iterator::find")
            } else {
                None
            }
        }
    }

    pub fn foreach<F>(&mut self, func: F) -> Result<(), IteratorError>
    where
        F: FnMut(T),
    {
        unsafe {
            let mut func = func;
            let func_ptr = &mut func as *mut F as gpointer;

            let res = gst_sys::gst_iterator_foreach(
                self.to_glib_none_mut().0,
                Some(foreach_trampoline::<T, F>),
                func_ptr,
            );

            #[allow(clippy::wildcard_in_or_patterns)]
            match res {
                gst_sys::GST_ITERATOR_OK | gst_sys::GST_ITERATOR_DONE => Ok(()),
                gst_sys::GST_ITERATOR_RESYNC => Err(IteratorError::Resync),
                gst_sys::GST_ITERATOR_ERROR | _ => Err(IteratorError::Error),
            }
        }
    }

    pub fn fold<F, U>(&mut self, init: U, func: F) -> Result<U, IteratorError>
    where
        F: FnMut(U, T) -> Result<U, U>,
    {
        unsafe {
            let mut func = func;
            let func_ptr = &mut func as *mut F as gpointer;

            let mut accum = Some(init);
            let mut ret = glib::Value::from_type(glib::Type::Pointer);
            gobject_sys::g_value_set_pointer(
                ret.to_glib_none_mut().0,
                &mut accum as *mut _ as gpointer,
            );

            let res = gst_sys::gst_iterator_fold(
                self.to_glib_none_mut().0,
                Some(fold_trampoline::<T, U, F>),
                ret.to_glib_none_mut().0,
                func_ptr,
            );

            #[allow(clippy::wildcard_in_or_patterns)]
            match res {
                gst_sys::GST_ITERATOR_OK | gst_sys::GST_ITERATOR_DONE => Ok(accum.unwrap()),
                gst_sys::GST_ITERATOR_RESYNC => Err(IteratorError::Resync),
                gst_sys::GST_ITERATOR_ERROR | _ => Err(IteratorError::Error),
            }
        }
    }
}

impl<T> Iterator<T>
where
    for<'a> T: FromValueOptional<'a> + StaticType + ToValue + Send + 'static,
{
    pub fn new<I: IteratorImpl<T>>(imp: I) -> Self {
        assert_initialized_main_thread!();
        static DUMMY_COOKIE: u32 = 0;

        unsafe {
            let it = gst_sys::gst_iterator_new(
                mem::size_of::<RsIterator<T, I>>() as u32,
                T::static_type().to_glib(),
                ptr::null_mut(),
                &DUMMY_COOKIE as *const _ as *mut _,
                Some(rs_iterator_copy::<T, I>),
                Some(rs_iterator_next::<T, I>),
                None,
                Some(rs_iterator_resync::<T, I>),
                Some(rs_iterator_free::<T, I>),
            );

            {
                let it = it as *mut RsIterator<T, I>;
                ptr::write(&mut (*it).imp, imp);
            }

            from_glib_full(it)
        }
    }
}

impl<T> Iterator<T>
where
    for<'a> T: FromValueOptional<'a> + StaticType + ToValue + Clone + Send + 'static,
{
    pub fn from_vec(items: Vec<T>) -> Self {
        skip_assert_initialized!();
        Self::new(VecIteratorImpl::new(items))
    }
}

#[repr(C)]
struct RsIterator<T, I: IteratorImpl<T>>
where
    for<'a> T: FromValueOptional<'a> + StaticType + ToValue + Send + 'static,
{
    iter: gst_sys::GstIterator,
    imp: I,
    phantom: PhantomData<T>,
}

pub trait IteratorImpl<T>: Clone + Send + 'static
where
    for<'a> T: FromValueOptional<'a> + StaticType + ToValue + Send + 'static,
{
    fn next(&mut self) -> Option<Result<T, IteratorError>>;
    fn resync(&mut self);
}

unsafe extern "C" fn rs_iterator_copy<T, I: IteratorImpl<T>>(
    it: *const gst_sys::GstIterator,
    copy: *mut gst_sys::GstIterator,
) where
    for<'a> T: FromValueOptional<'a> + StaticType + ToValue + Send + 'static,
{
    let it = it as *const RsIterator<T, I>;
    let copy = copy as *mut RsIterator<T, I>;

    ptr::write(&mut (*copy).imp, (*it).imp.clone());
}

unsafe extern "C" fn rs_iterator_free<T, I: IteratorImpl<T>>(it: *mut gst_sys::GstIterator)
where
    for<'a> T: FromValueOptional<'a> + StaticType + ToValue + Send + 'static,
{
    let it = it as *mut RsIterator<T, I>;
    ptr::drop_in_place(&mut (*it).imp);
}

unsafe extern "C" fn rs_iterator_next<T, I: IteratorImpl<T>>(
    it: *mut gst_sys::GstIterator,
    result: *mut gobject_sys::GValue,
) -> gst_sys::GstIteratorResult
where
    for<'a> T: FromValueOptional<'a> + StaticType + ToValue + Send + 'static,
{
    let it = it as *mut RsIterator<T, I>;
    match (*it).imp.next() {
        Some(Ok(value)) => {
            let value = value.to_value();
            ptr::write(result, value.into_raw());
            gst_sys::GST_ITERATOR_OK
        }
        None => gst_sys::GST_ITERATOR_DONE,
        Some(Err(res)) => match res {
            IteratorError::Resync => gst_sys::GST_ITERATOR_RESYNC,
            IteratorError::Error => gst_sys::GST_ITERATOR_ERROR,
        },
    }
}

unsafe extern "C" fn rs_iterator_resync<T, I: IteratorImpl<T>>(it: *mut gst_sys::GstIterator)
where
    for<'a> T: FromValueOptional<'a> + StaticType + ToValue + Send + 'static,
{
    let it = it as *mut RsIterator<T, I>;
    (*it).imp.resync();
}

#[derive(Clone)]
struct VecIteratorImpl<T> {
    pos: usize,
    items: Vec<T>,
}

impl<T> VecIteratorImpl<T>
where
    for<'a> T: StaticType + ToValue + FromValueOptional<'a> + Clone + Send + 'static,
{
    fn new(items: Vec<T>) -> Self {
        skip_assert_initialized!();
        Self { pos: 0, items }
    }
}

impl<T> IteratorImpl<T> for VecIteratorImpl<T>
where
    for<'a> T: StaticType + ToValue + FromValueOptional<'a> + Clone + Send + 'static,
{
    fn next(&mut self) -> Option<Result<T, IteratorError>> {
        if self.pos < self.items.len() {
            let res = Ok(self.items[self.pos].clone());
            self.pos += 1;
            return Some(res);
        }

        None
    }

    fn resync(&mut self) {
        self.pos = 0;
    }
}

unsafe impl<T> Send for Iterator<T> {}
unsafe impl<T> Sync for Iterator<T> {}

unsafe extern "C" fn filter_trampoline<T>(value: gconstpointer, func: gconstpointer) -> i32
where
    for<'a> T: FromValueOptional<'a> + 'static,
{
    let value = value as *const gobject_sys::GValue;

    let func = func as *const gobject_sys::GValue;
    let func = gobject_sys::g_value_get_boxed(func);
    let func = &*(func as *const &(dyn Fn(T) -> bool + Send + Sync + 'static));

    let value = &*(value as *const glib::Value);
    let value = value
        .get::<T>()
        .expect("Iterator filter_trampoline")
        .unwrap();

    if func(value) {
        0
    } else {
        -1
    }
}

unsafe extern "C" fn filter_boxed_ref<T: 'static>(boxed: gpointer) -> gpointer {
    let boxed = Arc::from_raw(boxed as *const Box<dyn Fn(T) -> bool + Send + Sync + 'static>);
    let copy = Arc::clone(&boxed);

    // Forget it and keep it alive, we will still need it later
    let _ = Arc::into_raw(boxed);

    Arc::into_raw(copy) as gpointer
}

unsafe extern "C" fn filter_boxed_unref<T: 'static>(boxed: gpointer) {
    let _ = Arc::from_raw(boxed as *const Box<dyn Fn(T) -> bool + Send + Sync + 'static>);
}

unsafe extern "C" fn filter_boxed_get_type<T: StaticType + 'static>() -> glib_sys::GType {
    use once_cell::sync::Lazy;
    use std::collections::HashMap;
    use std::sync::Mutex;

    static mut TYPES: Lazy<Mutex<HashMap<String, glib_sys::GType>>> =
        Lazy::new(|| Mutex::new(HashMap::new()));

    let mut types = TYPES.lock().unwrap();
    let type_name = T::static_type().name();

    if let Some(type_) = types.get(&type_name) {
        return *type_;
    }

    let type_ = {
        let iter_type_name = {
            let mut idx = 0;

            loop {
                let iter_type_name =
                    CString::new(format!("GstRsIteratorFilterBoxed-{}-{}", type_name, idx))
                        .unwrap();
                if gobject_sys::g_type_from_name(iter_type_name.as_ptr())
                    == gobject_sys::G_TYPE_INVALID
                {
                    break iter_type_name;
                }
                idx += 1;
            }
        };

        let type_ = gobject_sys::g_boxed_type_register_static(
            iter_type_name.as_ptr(),
            Some(filter_boxed_ref::<T>),
            Some(filter_boxed_unref::<T>),
        );

        assert_ne!(type_, gobject_sys::G_TYPE_INVALID);

        type_
    };

    types.insert(type_name, type_);

    type_
}

unsafe extern "C" fn find_trampoline<T, F: FnMut(T) -> bool>(
    value: gconstpointer,
    func: gconstpointer,
) -> i32
where
    for<'a> T: FromValueOptional<'a> + 'static,
{
    let value = value as *const gobject_sys::GValue;

    let func = func as *mut F;
    let value = &*(value as *const glib::Value);
    let value = value.get::<T>().expect("Iterator find_trampoline").unwrap();

    if (*func)(value) {
        0
    } else {
        -1
    }
}

unsafe extern "C" fn foreach_trampoline<T, F: FnMut(T)>(
    value: *const gobject_sys::GValue,
    func: gpointer,
) where
    for<'a> T: FromValueOptional<'a> + 'static,
{
    let func = func as *mut F;
    let value = &*(value as *const glib::Value);
    let value = value
        .get::<T>()
        .expect("Iterator foreach_trampoline")
        .unwrap();

    (*func)(value);
}

unsafe extern "C" fn fold_trampoline<T, U, F: FnMut(U, T) -> Result<U, U>>(
    value: *const gobject_sys::GValue,
    ret: *mut gobject_sys::GValue,
    func: gpointer,
) -> glib_sys::gboolean
where
    for<'a> T: FromValueOptional<'a> + 'static,
{
    let func = func as *mut F;
    let value = &*(value as *const glib::Value);
    let value = value.get::<T>().expect("Iterator fold_trampoline").unwrap();

    let accum = &mut *(gobject_sys::g_value_get_pointer(ret) as *mut Option<U>);

    match (*func)(accum.take().unwrap(), value) {
        Ok(next_accum) => {
            *accum = Some(next_accum);
            glib_sys::GTRUE
        }
        Err(next_accum) => {
            *accum = Some(next_accum);
            glib_sys::GFALSE
        }
    }
}

impl<T: StaticType + 'static> Clone for Iterator<T> {
    fn clone(&self) -> Self {
        unsafe { from_glib_full(gst_sys::gst_iterator_copy(self.to_glib_none().0)) }
    }
}

impl<T> fmt::Debug for Iterator<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Iterator")
            .field("iter", &self.iter)
            .finish()
    }
}

impl<T> Drop for Iterator<T> {
    fn drop(&mut self) {
        unsafe {
            gst_sys::gst_iterator_free(self.iter.as_ptr());
        }
    }
}

impl<T> iter::IntoIterator for Iterator<T>
where
    for<'a> T: FromValueOptional<'a> + 'static,
{
    type Item = Result<T, IteratorError>;
    type IntoIter = StdIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<T> glib::types::StaticType for Iterator<T> {
    fn static_type() -> glib::types::Type {
        unsafe { glib::translate::from_glib(gst_sys::gst_iterator_get_type()) }
    }
}

#[doc(hidden)]
impl<'a, T: StaticType> glib::value::FromValueOptional<'a> for Iterator<T> {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Option::<Iterator<T>>::from_glib_none(
            gobject_sys::g_value_get_boxed(value.to_glib_none().0) as *mut gst_sys::GstIterator,
        )
    }
}

#[doc(hidden)]
impl<T: 'static> glib::value::SetValue for Iterator<T> {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        gobject_sys::g_value_set_boxed(
            value.to_glib_none_mut().0,
            glib::translate::ToGlibPtr::<*const gst_sys::GstIterator>::to_glib_none(this).0
                as glib_sys::gpointer,
        )
    }
}

#[doc(hidden)]
impl<T: 'static> glib::value::SetValueOptional for Iterator<T> {
    unsafe fn set_value_optional(value: &mut glib::Value, this: Option<&Self>) {
        gobject_sys::g_value_set_boxed(
            value.to_glib_none_mut().0,
            glib::translate::ToGlibPtr::<*const gst_sys::GstIterator>::to_glib_none(&this).0
                as glib_sys::gpointer,
        )
    }
}

#[doc(hidden)]
impl<T> glib::translate::GlibPtrDefault for Iterator<T> {
    type GlibType = *mut gst_sys::GstIterator;
}

#[doc(hidden)]
impl<'a, T: 'static> glib::translate::ToGlibPtr<'a, *const gst_sys::GstIterator> for Iterator<T> {
    type Storage = &'a Iterator<T>;

    fn to_glib_none(&'a self) -> glib::translate::Stash<'a, *const gst_sys::GstIterator, Self> {
        glib::translate::Stash(self.iter.as_ptr(), self)
    }

    fn to_glib_full(&self) -> *const gst_sys::GstIterator {
        unimplemented!()
    }
}

#[doc(hidden)]
impl<'a, T: 'static> glib::translate::ToGlibPtrMut<'a, *mut gst_sys::GstIterator> for Iterator<T> {
    type Storage = &'a mut Iterator<T>;

    #[inline]
    fn to_glib_none_mut(
        &'a mut self,
    ) -> glib::translate::StashMut<'a, *mut gst_sys::GstIterator, Self> {
        glib::translate::StashMut(self.iter.as_ptr(), self)
    }
}

#[doc(hidden)]
impl<T: StaticType> glib::translate::FromGlibPtrNone<*const gst_sys::GstIterator> for Iterator<T> {
    #[inline]
    unsafe fn from_glib_none(ptr: *const gst_sys::GstIterator) -> Self {
        assert_ne!(
            gobject_sys::g_type_is_a((*ptr).type_, T::static_type().to_glib()),
            glib_sys::GFALSE
        );
        from_glib_full(gst_sys::gst_iterator_copy(ptr))
    }
}

#[doc(hidden)]
impl<T: StaticType> glib::translate::FromGlibPtrNone<*mut gst_sys::GstIterator> for Iterator<T> {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut gst_sys::GstIterator) -> Self {
        assert_ne!(
            gobject_sys::g_type_is_a((*ptr).type_, T::static_type().to_glib()),
            glib_sys::GFALSE
        );
        from_glib_full(gst_sys::gst_iterator_copy(ptr))
    }
}

#[doc(hidden)]
impl<T: StaticType> glib::translate::FromGlibPtrBorrow<*mut gst_sys::GstIterator> for Iterator<T> {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut gst_sys::GstIterator) -> Borrowed<Self> {
        assert!(!ptr.is_null());
        assert_ne!(
            gobject_sys::g_type_is_a((*ptr).type_, T::static_type().to_glib()),
            glib_sys::GFALSE
        );
        Borrowed::new(Self {
            iter: ptr::NonNull::new_unchecked(ptr),
            phantom: PhantomData,
        })
    }
}

#[doc(hidden)]
impl<T: StaticType> glib::translate::FromGlibPtrFull<*mut gst_sys::GstIterator> for Iterator<T> {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut gst_sys::GstIterator) -> Self {
        assert!(!ptr.is_null());
        assert_ne!(
            gobject_sys::g_type_is_a((*ptr).type_, T::static_type().to_glib()),
            glib_sys::GFALSE
        );
        Self {
            iter: ptr::NonNull::new_unchecked(ptr),
            phantom: PhantomData,
        }
    }
}

pub struct StdIterator<T> {
    inner: Iterator<T>,
    error: Option<IteratorError>,
}

impl<T> StdIterator<T> {
    fn new(inner: Iterator<T>) -> Self {
        skip_assert_initialized!();
        Self { inner, error: None }
    }
}

impl<T: StaticType + 'static> Clone for StdIterator<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            error: self.error,
        }
    }
}

impl<T> fmt::Debug for StdIterator<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("StdIterator")
            .field("inner", &self.inner)
            .field("error", &self.error)
            .finish()
    }
}

impl<T> iter::Iterator for StdIterator<T>
where
    for<'a> T: FromValueOptional<'a> + 'static,
{
    type Item = Result<T, IteratorError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.error {
            // Fuse the iterator after returning IteratorError::Error
            Some(IteratorError::Error) => return None,

            // The iterator needs a resync
            Some(IteratorError::Resync) => self.inner.resync(),

            None => {}
        }

        let res = self.inner.next();
        self.error = res.as_ref().err().copied();
        res.transpose()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec() {
        ::init().unwrap();

        let vec = vec![1i32, 2, 3];
        let mut it = Iterator::from_vec(vec);
        let val = it.next();
        assert_eq!(val, Ok(Some(1)));
        let val = it.next();
        assert_eq!(val, Ok(Some(2)));
        let val = it.next();
        assert_eq!(val, Ok(Some(3)));
        assert_eq!(it.next(), Ok(None));

        let vec = vec![1i32, 2, 3];
        let mut it = Iterator::from_vec(vec);
        let mut vals = Vec::new();
        while let Ok(Some(res)) = it.next() {
            vals.push(res);
        }
        assert_eq!(vals, [1, 2, 3]);
    }

    #[test]
    fn test_filter() {
        ::init().unwrap();

        let vec = vec![1i32, 2, 3];
        let mut it = Iterator::from_vec(vec).filter(|val| val % 2 == 1);

        let mut vals = Vec::new();
        while let Ok(Some(res)) = it.next() {
            vals.push(res);
        }
        assert_eq!(vals, [1, 3]);
    }

    #[test]
    fn test_find() {
        ::init().unwrap();

        // Our find
        let vec = vec![1i32, 2, 3];
        let val = Iterator::from_vec(vec).find(|val| val == 2);
        assert_eq!(val.unwrap(), 2);
    }

    #[test]
    fn test_foreach() {
        ::init().unwrap();

        let vec = vec![1i32, 2, 3];
        let mut sum = 0;
        let res = Iterator::from_vec(vec).foreach(|val| sum += val);
        assert_eq!(res, Ok(()));
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_fold() {
        ::init().unwrap();

        // Our fold
        let vec = vec![1i32, 2, 3];
        let res = Iterator::from_vec(vec).fold(0, |mut sum, val| {
            sum += val;
            Ok(sum)
        });
        assert_eq!(res.unwrap(), 6);
    }

    #[test]
    fn test_std() {
        ::init().unwrap();

        let mut it = Iterator::from_vec(vec![1i32, 2, 3]).into_iter();
        assert_eq!(it.next(), Some(Ok(1)));
        assert_eq!(it.next(), Some(Ok(2)));
        assert_eq!(it.next(), Some(Ok(3)));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_into_iter() {
        ::init().unwrap();

        let mut v = vec![1i32, 2, 3].into_iter();
        for x in Iterator::from_vec(vec![1i32, 2, 3]) {
            assert_eq!(x.unwrap(), v.next().unwrap());
        }
        assert_eq!(v.next(), None);
    }

    #[test]
    fn test_std_resync_collect() {
        use prelude::*;
        use std::collections::BTreeSet;

        ::init().unwrap();

        let bin = ::Bin::new(None);
        let id1 = ::ElementFactory::make("identity", None).unwrap();
        let id2 = ::ElementFactory::make("identity", None).unwrap();

        bin.add(&id1).unwrap();

        let mut it = bin.iterate_elements().into_iter();
        assert_eq!(it.next().unwrap().unwrap(), id1);

        bin.add(&id2).unwrap();

        let res = it.by_ref().collect::<Result<Vec<_>, _>>().unwrap_err();
        assert_eq!(res, IteratorError::Resync);

        let mut elems = BTreeSet::new();
        elems.insert(id1);
        elems.insert(id2);

        let res = it.by_ref().collect::<Result<BTreeSet<_>, _>>().unwrap();
        assert_eq!(res, elems);

        let res = it.collect::<Result<Vec<_>, _>>().unwrap();
        assert!(res.is_empty());
    }

    #[test]
    fn test_std_resync_find() {
        use prelude::*;

        ::init().unwrap();

        let bin = ::Bin::new(None);
        let id1 = ::ElementFactory::make("identity", None).unwrap();
        let id2 = ::ElementFactory::make("identity", None).unwrap();

        bin.add(&id1).unwrap();

        let mut it = bin.iterate_elements().into_iter();
        assert_eq!(it.next().unwrap().unwrap(), id1);

        bin.add(&id2).unwrap();

        let res = it.find(|x| x.as_ref() == Ok(&id1));
        assert_eq!(res.unwrap().unwrap(), id1);
    }
}
