// Take a look at the license at the top of the repository in the LICENSE file.

use std::{
    borrow::{Borrow, BorrowMut},
    fmt,
    ops::{Deref, DerefMut},
};

use crate::ffi;
use crate::*;
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[doc(alias = "GstTensor")]
    pub struct Tensor(Boxed<ffi::GstTensor>);

    match fn {
        copy => |ptr| ffi::gst_tensor_copy(ptr),
        free => |ptr| ffi::gst_tensor_free(ptr),
        type_ => || ffi::gst_tensor_get_type(),
    }
}

unsafe impl Send for Tensor {}
unsafe impl Sync for Tensor {}

impl Tensor {
    #[doc(alias = "gst_tensor_new_simple")]
    pub fn new_simple(
        id: glib::Quark,
        data_type: TensorDataType,
        data: gst::Buffer,
        dims_order: TensorDimOrder,
        dims: &[usize],
    ) -> Tensor {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gst_tensor_new_simple(
                id.into_glib(),
                data_type.into_glib(),
                data.into_glib_ptr(),
                dims_order.into_glib(),
                dims.len(),
                dims.as_ptr() as *mut _,
            ))
        }
    }
}

impl Deref for Tensor {
    type Target = TensorRef;

    #[inline]
    fn deref(&self) -> &TensorRef {
        unsafe { &*(self.as_ptr() as *const TensorRef) }
    }
}

impl DerefMut for Tensor {
    #[inline]
    fn deref_mut(&mut self) -> &mut TensorRef {
        unsafe { &mut *(self.as_ptr() as *mut TensorRef) }
    }
}

impl AsRef<TensorRef> for Tensor {
    #[inline]
    fn as_ref(&self) -> &TensorRef {
        self.deref()
    }
}

impl AsMut<TensorRef> for Tensor {
    #[inline]
    fn as_mut(&mut self) -> &mut TensorRef {
        self.deref_mut()
    }
}

impl Borrow<TensorRef> for Tensor {
    #[inline]
    fn borrow(&self) -> &TensorRef {
        self.as_ref()
    }
}

impl BorrowMut<TensorRef> for Tensor {
    #[inline]
    fn borrow_mut(&mut self) -> &mut TensorRef {
        self.as_mut()
    }
}

impl fmt::Debug for Tensor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        TensorRef::fmt(self.as_ref(), f)
    }
}

impl ToOwned for TensorRef {
    type Owned = Tensor;

    fn to_owned(&self) -> Tensor {
        unsafe { Tensor::from_glib_full(self.as_ptr()) }
    }
}

#[repr(transparent)]
#[doc(alias = "GstTensor")]
pub struct TensorRef(ffi::GstTensor);

unsafe impl Send for TensorRef {}
unsafe impl Sync for TensorRef {}

impl TensorRef {
    #[inline]
    pub unsafe fn from_glib_borrow<'a>(ptr: *const ffi::GstTensor) -> &'a TensorRef {
        debug_assert!(!ptr.is_null());

        unsafe { &*(ptr as *mut TensorRef) }
    }

    #[inline]
    pub unsafe fn from_glib_borrow_mut<'a>(ptr: *mut ffi::GstTensor) -> &'a mut TensorRef {
        debug_assert!(!ptr.is_null());

        unsafe { &mut *(ptr as *mut TensorRef) }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const ffi::GstTensor {
        self as *const Self as *mut ffi::GstTensor
    }

    #[inline]
    pub fn as_mut_ptr(&self) -> *mut ffi::GstTensor {
        self as *const Self as *mut ffi::GstTensor
    }

    #[doc(alias = "gst_tensor_get_dims")]
    #[doc(alias = "get_dims")]
    pub fn dims(&self) -> &[usize] {
        let mut num_dims: usize = 0;
        unsafe {
            let dims = ffi::gst_tensor_get_dims(self.as_mut_ptr(), &mut num_dims);
            std::slice::from_raw_parts(dims as *const _, num_dims)
        }
    }

    #[inline]
    pub fn id(&self) -> glib::Quark {
        unsafe { from_glib(self.0.id) }
    }

    #[inline]
    pub fn data_type(&self) -> TensorDataType {
        unsafe { from_glib(self.0.data_type) }
    }

    #[inline]
    pub fn data(&self) -> &gst::BufferRef {
        unsafe { gst::BufferRef::from_ptr(self.0.data) }
    }

    #[inline]
    pub fn data_mut(&mut self) -> &mut gst::BufferRef {
        unsafe {
            self.0.data = gst::ffi::gst_mini_object_make_writable(self.0.data as _) as _;
            gst::BufferRef::from_mut_ptr(self.0.data)
        }
    }

    #[inline]
    pub fn dims_order(&self) -> TensorDimOrder {
        unsafe { from_glib(self.0.dims_order) }
    }

    #[cfg(feature = "v1_28")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
    #[doc(alias = "gst_tensor_check_type")]
    pub fn check_type(
        &self,
        data_type: crate::TensorDataType,
        order: crate::TensorDimOrder,
        dims: &[usize],
    ) -> bool {
        unsafe {
            from_glib(ffi::gst_tensor_check_type(
                self.as_ptr(),
                data_type.into_glib(),
                order.into_glib(),
                dims.len(),
                dims.as_ptr(),
            ))
        }
    }
}

impl fmt::Debug for TensorRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Tensor")
            .field("id", &self.id())
            .field("data_type", &self.data_type())
            .field("dims_order", &self.dims_order())
            .field("dims", &self.dims())
            .field("data", self.data())
            .finish()
    }
}

impl PartialEq for TensorRef {
    #[inline]
    fn eq(&self, other: &TensorRef) -> bool {
        self.as_ptr() == other.as_ptr()
    }
}

impl Eq for TensorRef {}

impl glib::types::StaticType for TensorRef {
    #[inline]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gst_tensor_get_type()) }
    }
}

unsafe impl<'a> glib::value::FromValue<'a> for &'a TensorRef {
    type Checker = glib::value::GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        unsafe {
            &*(glib::gobject_ffi::g_value_get_boxed(value.to_glib_none().0) as *const TensorRef)
        }
    }
}

impl glib::value::ToValue for TensorRef {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Tensor>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                self.as_ptr() as *mut _,
            );
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl glib::value::ToValueOptional for TensorRef {
    fn to_value_optional(s: Option<&Self>) -> glib::Value {
        skip_assert_initialized!();
        let mut value = glib::Value::for_value_type::<Tensor>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                s.map_or(std::ptr::null(), |s| s.as_ptr() as *mut _),
            );
        }
        value
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn create_tensor() {
        gst::init().unwrap();

        let buf = gst::Buffer::with_size(2 * 3 * 4 * 5).unwrap();
        assert_eq!(buf.size(), 2 * 3 * 4 * 5);

        let mut tensor = Tensor::new_simple(
            glib::Quark::from_str("me"),
            TensorDataType::Int16,
            buf,
            TensorDimOrder::RowMajor,
            &[3, 4, 5],
        );

        assert_eq!(tensor.id(), glib::Quark::from_str("me"));
        assert_eq!(tensor.data_type(), TensorDataType::Int16);
        assert_eq!(tensor.dims_order(), TensorDimOrder::RowMajor);
        assert_eq!(tensor.dims()[0], 3);
        assert_eq!(tensor.dims()[1], 4);
        assert_eq!(tensor.dims()[2], 5);
        assert_eq!(tensor.data().size(), 2 * 3 * 4 * 5);

        tensor.data();
        tensor.data_mut();
    }
}
