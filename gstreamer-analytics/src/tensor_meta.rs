// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use gst::prelude::*;

use crate::ffi;
use crate::Tensor;

#[repr(transparent)]
#[doc(alias = "GstTensorMeta")]
pub struct TensorMeta(ffi::GstTensorMeta);

unsafe impl Send for TensorMeta {}
unsafe impl Sync for TensorMeta {}

impl TensorMeta {
    #[doc(alias = "gst_buffer_add_tensor_meta")]
    pub fn add(buffer: &mut gst::BufferRef) -> gst::MetaRefMut<Self, gst::meta::Standalone> {
        skip_assert_initialized!();

        unsafe {
            let meta_ptr = ffi::gst_buffer_add_tensor_meta(buffer.as_mut_ptr());
            Self::from_mut_ptr(buffer, meta_ptr)
        }
    }

    #[doc(alias = "gst_tensor_meta_set")]
    pub fn set(&mut self, tensors: glib::Slice<Tensor>) {
        unsafe {
            ffi::gst_tensor_meta_set(self.as_mut_ptr(), tensors.len() as u32, tensors.into_raw());
        }
    }

    #[doc(alias = "gst_tensor_meta_get_index_from_id")]
    pub fn index_from_id(&self, id: glib::Quark) -> i32 {
        unsafe { ffi::gst_tensor_meta_get_index_from_id(self.as_mut_ptr(), id.into_glib()) }
    }

    pub fn as_slice(&self) -> &[Tensor] {
        unsafe { glib::Slice::from_glib_borrow_num(self.0.tensors, self.0.num_tensors) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [Tensor] {
        unsafe { glib::Slice::from_glib_borrow_num_mut(self.0.tensors, self.0.num_tensors) }
    }

    unsafe fn as_mut_ptr(&self) -> *mut ffi::GstTensorMeta {
        mut_override(&self.0)
    }
}

unsafe impl MetaAPI for TensorMeta {
    type GstType = ffi::GstTensorMeta;

    #[doc(alias = "gst_tensor_meta_api_get_type")]
    #[inline]
    fn meta_api() -> glib::Type {
        unsafe { from_glib(ffi::gst_tensor_meta_api_get_type()) }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn build_tensor_meta() {
        gst::init().unwrap();

        let mut buf = gst::Buffer::new();

        let mut tmeta = TensorMeta::add(buf.make_mut());

        let tensor = Tensor::new_simple(
            glib::Quark::from_str("me"),
            TensorDataType::Int16,
            2,
            gst::Buffer::with_size(2 * 2 * 3 * 4 * 5).unwrap(),
            TensorDimOrder::RowMajor,
            &[3, 4, 5],
        );

        let tptr = tensor.as_ptr();

        tmeta.set([tensor].into());

        let tensors = tmeta.as_slice();

        assert_eq!(tensors.len(), 1);

        // Check that it's the same tensor
        assert_eq!(tptr, tensors[0].as_ptr());
        assert_eq!(tensors[0].dims_order(), TensorDimOrder::RowMajor);
        assert_eq!(tensors[0].dims().len(), 3);
        assert_eq!(tensors[0].dims()[0].size, 3);

        assert_eq!(tmeta.as_slice().len(), 1);

        tmeta.as_mut_slice();
    }
}
