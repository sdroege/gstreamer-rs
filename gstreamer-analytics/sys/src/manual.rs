use gstreamer_sys as gst_sys;
use libc::size_t;

use crate::{GstTensorDataType, GstTensorDimOrder, GstTensorLayout};

#[repr(C)]
pub struct GstTensor {
    pub id: glib_sys::GQuark,
    pub layout: GstTensorLayout,
    pub data_type: GstTensorDataType,
    pub data: *mut gst_sys::GstBuffer,
    pub dims_order: GstTensorDimOrder,
    pub num_dims: size_t,
    pub dims: [size_t; 0],
}

impl ::std::fmt::Debug for GstTensor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstTensor @ {self:p}"))
            .field("id", &self.id)
            .field("layout", &self.layout)
            .field("data_type", &self.data_type)
            .field("data", &self.data)
            .field("dims_order", &self.dims_order)
            .field("num_dims", &self.num_dims)
            .field("dims", &unsafe {
                ::std::slice::from_raw_parts(self.dims.as_ptr(), self.num_dims)
            })
            .finish()
    }
}
