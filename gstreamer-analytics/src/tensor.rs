// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ffi;
use crate::*;
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    #[doc(alias = "gst_tensor_get_dims")]
    #[doc(alias = "get_dims")]
    pub fn dims(&self) -> &[usize] {
        let mut num_dims: usize = 0;
        unsafe {
            let dims = ffi::gst_tensor_get_dims(self.as_ptr(), &mut num_dims);
            std::slice::from_raw_parts(dims as *const _, num_dims)
        }
    }

    #[inline]
    pub fn id(&self) -> glib::Quark {
        unsafe { from_glib(self.inner.id) }
    }

    #[inline]
    pub fn data_type(&self) -> TensorDataType {
        unsafe { from_glib(self.inner.data_type) }
    }

    #[inline]
    pub fn data(&self) -> &gst::BufferRef {
        unsafe { gst::BufferRef::from_ptr(self.inner.data) }
    }

    #[inline]
    pub fn data_mut(&mut self) -> &mut gst::BufferRef {
        unsafe {
            self.inner.data = gst::ffi::gst_mini_object_make_writable(self.inner.data as _) as _;
            gst::BufferRef::from_mut_ptr(self.inner.data)
        }
    }

    #[inline]
    pub fn dims_order(&self) -> TensorDimOrder {
        unsafe { from_glib(self.inner.dims_order) }
    }

    #[cfg(feature = "v1_28")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_28")))]
    #[doc(alias = "gst_tensor_check_type")]
    pub fn check_type(
        &self,
        order: crate::TensorDimOrder,
        num_dims: usize,
        data_type: crate::TensorDataType,
        data: &gst::BufferRef,
    ) -> bool {
        unsafe {
            from_glib(ffi::gst_tensor_check_type(
                self.to_glib_none().0,
                order.into_glib(),
                num_dims,
                data_type.into_glib(),
                mut_override(data.as_ptr()),
            ))
        }
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
