// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::{ffi, relation_meta::*};

#[derive(Debug)]
pub enum AnalyticsTensorMtd {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::AnalyticsRelationMetaTensorExt> Sealed for T {}
}

pub trait AnalyticsRelationMetaTensorExt: sealed::Sealed {
    fn add_tensor_mtd_simple(
        &mut self,
        id: glib::Quark,
        data_type: crate::TensorDataType,
        data: gst::Buffer,
        dims_order: crate::TensorDimOrder,
        dims: &[usize],
    ) -> Result<AnalyticsMtdRef<'_, AnalyticsTensorMtd>, glib::BoolError>;
}

impl<'a> AnalyticsRelationMetaTensorExt
    for gst::MetaRefMut<'a, AnalyticsRelationMeta, gst::meta::Standalone>
{
    #[doc(alias = "gst_analytics_relation_meta_add_tensor_mtd_simple")]
    fn add_tensor_mtd_simple(
        &mut self,
        id: glib::Quark,
        data_type: crate::TensorDataType,
        data: gst::Buffer,
        dims_order: crate::TensorDimOrder,
        dims: &[usize],
    ) -> Result<AnalyticsMtdRef<'a, AnalyticsTensorMtd>, glib::BoolError> {
        unsafe {
            let mut mtd = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_analytics_relation_meta_add_tensor_mtd_simple(
                self.as_mut_ptr(),
                id.into_glib(),
                data_type.into_glib(),
                data.into_glib_ptr(),
                dims_order.into_glib(),
                dims.len(),
                dims.as_ptr() as *mut _,
                mtd.as_mut_ptr(),
            ));

            if ret {
                let id = mtd.assume_init().id;
                Ok(AnalyticsMtdRef::from_meta(self.as_ref(), id))
            } else {
                Err(glib::bool_error!("Could not add tensor metadata"))
            }
        }
    }
}

unsafe impl AnalyticsMtd for AnalyticsTensorMtd {
    #[doc(alias = "gst_analytics_tensor_mtd_get_mtd_type")]
    fn mtd_type() -> ffi::GstAnalyticsMtdType {
        unsafe { ffi::gst_analytics_tensor_mtd_get_mtd_type() }
    }
}

impl AnalyticsMtdRef<'_, AnalyticsTensorMtd> {
    #[doc(alias = "gst_analytics_tensor_mtd_get_tensor")]
    pub fn tensor(&self) -> &crate::TensorRef {
        unsafe {
            let mtd = ffi::GstAnalyticsMtd::unsafe_from(self);
            let ptr = ffi::gst_analytics_tensor_mtd_get_tensor(
                &mtd as *const _ as *const ffi::GstAnalyticsTensorMtd,
            );
            crate::TensorRef::from_glib_borrow(ptr)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn tensor_mtd() {
        gst::init().unwrap();

        assert_eq!(AnalyticsTensorMtd::type_name(), "tensor");

        let mut buffer = gst::Buffer::new();
        let mut meta = AnalyticsRelationMeta::add(buffer.make_mut());

        assert!(meta.is_empty());

        let dims_0 = &[3, 4, 5];
        let tensor_buffer_0 = gst::Buffer::with_size(3 * 4 * 5).unwrap();
        let tensor_mtd_0 = meta
            .add_tensor_mtd_simple(
                glib::Quark::from_str("foo"),
                TensorDataType::Uint8,
                tensor_buffer_0,
                TensorDimOrder::RowMajor,
                dims_0,
            )
            .unwrap();

        assert!(tensor_mtd_0.tensor().check_type(
            TensorDataType::Uint8,
            TensorDimOrder::RowMajor,
            dims_0
        ));
        assert_eq!(tensor_mtd_0.id(), 0);
        assert_eq!(tensor_mtd_0.mtd_type(), AnalyticsTensorMtd::mtd_type());
        assert_eq!(tensor_mtd_0.tensor().id(), glib::Quark::from_str("foo"));

        assert_eq!(meta.len(), 1);

        let dims_1 = &[4, 5, 6];
        let tensor_buffer_1 = gst::Buffer::with_size(2 * 4 * 5 * 6).unwrap();
        let tensor_mtd_1 = meta
            .add_tensor_mtd_simple(
                glib::Quark::from_str("bar"),
                TensorDataType::Int16,
                tensor_buffer_1,
                TensorDimOrder::ColMajor,
                dims_1,
            )
            .unwrap();

        assert!(tensor_mtd_1.tensor().check_type(
            TensorDataType::Int16,
            TensorDimOrder::ColMajor,
            dims_1
        ));
        assert_eq!(tensor_mtd_1.id(), 1);
        assert_eq!(tensor_mtd_1.mtd_type(), AnalyticsTensorMtd::mtd_type());
        assert_eq!(tensor_mtd_1.tensor().id(), glib::Quark::from_str("bar"));

        assert_eq!(meta.len(), 2);
    }
}
