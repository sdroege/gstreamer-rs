// Take a look at the license at the top of the repository in the LICENSE file.

use std::num::NonZeroI64;

use glib::translate::*;
use gst::prelude::*;

use crate::{Tensor, TensorDataType, TensorDimOrder, TensorRef, ffi};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(alias = "GstTensorProvenance")]
pub struct TensorProvenance<'a> {
    pub key: glib::Quark,
    pub correlation_ids: &'a [NonZeroI64],
}

#[repr(transparent)]
#[doc(alias = "GstTensorListMeta")]
pub struct TensorListMeta(ffi::GstTensorListMeta);

unsafe impl Send for TensorListMeta {}
unsafe impl Sync for TensorListMeta {}

impl TensorListMeta {
    #[doc(alias = "gst_buffer_add_tensor_list_meta")]
    pub fn add(
        buffer: &mut gst::BufferRef,
        group_id: glib::Quark,
    ) -> gst::MetaRefMut<'_, Self, gst::meta::Standalone> {
        skip_assert_initialized!();
        unsafe {
            let ptr =
                ffi::gst_buffer_add_tensor_list_meta(buffer.as_mut_ptr(), group_id.into_glib());
            Self::from_mut_ptr(buffer, ptr)
        }
    }

    pub fn group_id(&self) -> Option<glib::Quark> {
        unsafe { try_from_glib(self.0.group_id).ok() }
    }

    pub fn num_items(&self) -> usize {
        self.0.num_items
    }

    #[doc(alias = "gst_tensor_list_meta_get_typed_tensor")]
    pub fn typed_tensor(
        &self,
        item: usize,
        id: glib::Quark,
        data_type: TensorDataType,
        order: TensorDimOrder,
        dims: &[usize],
    ) -> Option<&TensorRef> {
        unsafe {
            let ptr = ffi::gst_tensor_list_meta_get_typed_tensor(
                self.as_mut_ptr(),
                item,
                id.into_glib(),
                data_type.into_glib(),
                order.into_glib(),
                dims.len(),
                dims.as_ptr(),
            );

            if ptr.is_null() {
                None
            } else {
                Some(TensorRef::from_glib_borrow(ptr))
            }
        }
    }

    #[doc(alias = "gst_tensor_list_meta_get_num_provenance")]
    pub fn num_provenance(&self, item: usize) -> usize {
        assert!(item < self.num_items());
        unsafe { ffi::gst_tensor_list_meta_get_num_provenance(self.as_mut_ptr(), item) }
    }

    #[doc(alias = "gst_tensor_list_meta_get_provenance")]
    pub fn provenance(
        &self,
        item: usize,
    ) -> impl ExactSizeIterator<Item = TensorProvenance<'_>> + '_ {
        (0..self.num_provenance(item)).map(move |index| unsafe {
            let mut provenance = std::mem::MaybeUninit::uninit();
            let success = ffi::gst_tensor_list_meta_get_provenance(
                self.as_mut_ptr(),
                item,
                index,
                provenance.as_mut_ptr(),
            );
            assert_ne!(success, glib::ffi::GFALSE);
            let provenance = provenance.assume_init();
            TensorProvenance {
                key: from_glib(provenance.key),
                correlation_ids: if provenance.num_correlation_ids == 0 {
                    &[]
                } else {
                    let ids = std::slice::from_raw_parts(
                        provenance.correlation_ids,
                        provenance.num_correlation_ids,
                    );

                    // Verify the C API's non-zero guarantee before casting
                    debug_assert!(!ids.contains(&0), "Zero correlation ID");
                    std::slice::from_raw_parts(ids.as_ptr().cast(), ids.len())
                },
            }
        })
    }

    #[doc(alias = "gst_tensor_list_meta_append_item")]
    pub fn append_item(&mut self, tensors: glib::Slice<Tensor>) -> Result<(), glib::BoolError> {
        if tensors.is_empty() {
            return Err(glib::bool_error!("Empty tensor set"));
        }
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_tensor_list_meta_append_item(
                    self.as_mut_ptr(),
                    tensors.len(),
                    tensors.into_raw(),
                ),
                "Failed to append tensor set"
            )
        }
    }

    #[doc(alias = "gst_tensor_list_meta_append_item_full")]
    pub fn append_item_full(
        &mut self,
        tensors: glib::Slice<Tensor>,
        provenance: &[TensorProvenance<'_>],
    ) -> Result<(), glib::BoolError> {
        if tensors.is_empty() {
            return Err(glib::bool_error!("Empty tensor set"));
        }
        for (i, entry) in provenance.iter().enumerate() {
            if provenance[..i].iter().any(|p| p.key == entry.key) {
                return Err(glib::bool_error!("Invalid provenance"));
            }
        }
        let raw: Vec<_> = provenance
            .iter()
            .map(|p| ffi::GstTensorProvenance {
                key: p.key.into_glib(),
                num_correlation_ids: p.correlation_ids.len(),
                correlation_ids: p.correlation_ids.as_ptr().cast(),
            })
            .collect();

        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_tensor_list_meta_append_item_full(
                    self.as_mut_ptr(),
                    tensors.len(),
                    tensors.into_raw(),
                    raw.len(),
                    raw.as_ptr(),
                ),
                "Failed to append tensor set"
            )
        }
    }

    #[doc(alias = "gst_tensor_list_meta_append_item_from")]
    pub fn append_item_from(
        &mut self,
        tensors: glib::Slice<Tensor>,
        source: &Self,
        source_item: usize,
    ) -> Result<(), glib::BoolError> {
        if tensors.is_empty() || source_item >= source.num_items() {
            return Err(glib::bool_error!("Invalid tensor set or source item"));
        }

        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_tensor_list_meta_append_item_from(
                    self.as_mut_ptr(),
                    tensors.len(),
                    tensors.into_raw(),
                    &source.0,
                    source_item,
                ),
                "Failed to append tensor set"
            )
        }
    }

    unsafe fn as_mut_ptr(&self) -> *mut ffi::GstTensorListMeta {
        mut_override(&self.0)
    }
}

unsafe impl MetaAPI for TensorListMeta {
    type GstType = ffi::GstTensorListMeta;
    #[doc(alias = "gst_tensor_list_meta_api_get_type")]
    fn meta_api() -> glib::Type {
        unsafe { from_glib(ffi::gst_tensor_list_meta_api_get_type()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tensor(name: &str) -> Tensor {
        skip_assert_initialized!();
        Tensor::new_simple(
            glib::Quark::from_str(name),
            TensorDataType::Uint8,
            gst::Buffer::with_size(2).unwrap(),
            TensorDimOrder::RowMajor,
            &[1, 2],
        )
    }

    #[test]
    fn rejected_append_drops_tensors() {
        gst::init().unwrap();
        let mut buffer = gst::Buffer::new();
        let mut meta = TensorListMeta::add(buffer.make_mut(), glib::Quark::from_str("group"));
        let tensor = tensor("input");
        let data: gst::Buffer = unsafe { from_glib_none(tensor.data().as_ptr()) };
        assert!(!data.is_writable());
        let provenance = TensorProvenance {
            key: glib::Quark::from_str("input"),
            correlation_ids: &[],
        };
        assert!(
            meta.append_item_full([tensor].into(), &[provenance, provenance])
                .is_err()
        );
        assert_eq!(meta.num_items(), 0);
        assert!(data.is_writable());
    }

    #[test]
    fn tensor_list_access() {
        gst::init().unwrap();
        let group = glib::Quark::from_str("group");
        let input = glib::Quark::from_str("input");
        let mut buffer = gst::Buffer::new();
        let mut meta = TensorListMeta::add(buffer.make_mut(), group);
        meta.append_item([tensor("input")].into()).unwrap();

        assert_eq!(meta.group_id(), Some(group));
        assert_eq!(meta.num_items(), 1);
        assert!(
            meta.typed_tensor(
                0,
                input,
                TensorDataType::Uint8,
                TensorDimOrder::RowMajor,
                &[1, 2]
            )
            .is_some()
        );
        assert_eq!(meta.num_provenance(0), 0);
    }

    #[test]
    fn item_provenance() {
        gst::init().unwrap();
        let image = glib::Quark::from_str("image");
        let aux = glib::Quark::from_str("aux");
        let ids = [11, 12].map(|id| NonZeroI64::new(id).unwrap());
        let aux_ids = [NonZeroI64::new(21).unwrap()];
        let mut buffer = gst::Buffer::new();
        {
            let mut meta = TensorListMeta::add(buffer.make_mut(), glib::Quark::from_str("group"));
            meta.append_item_full(
                [tensor("input")].into(),
                &[
                    TensorProvenance {
                        key: image,
                        correlation_ids: &ids,
                    },
                    TensorProvenance {
                        key: aux,
                        correlation_ids: &aux_ids,
                    },
                ],
            )
            .unwrap();
        }

        let meta = buffer.meta::<TensorListMeta>().unwrap();
        assert_eq!(
            meta.provenance(0).collect::<Vec<_>>(),
            vec![
                TensorProvenance {
                    key: image,
                    correlation_ids: &ids
                },
                TensorProvenance {
                    key: aux,
                    correlation_ids: &aux_ids
                },
            ]
        );
    }

    #[test]
    fn copy_provenance_to_new_tensors() {
        gst::init().unwrap();
        let group = glib::Quark::from_str("group");
        let image = glib::Quark::from_str("image");
        let mut source = gst::Buffer::new();
        let mut destination = gst::Buffer::new();
        let mut source_meta = TensorListMeta::add(source.make_mut(), group);
        source_meta
            .append_item_full(
                [tensor("input")].into(),
                &[TensorProvenance {
                    key: image,
                    correlation_ids: &[11, 12].map(|id| NonZeroI64::new(id).unwrap()),
                }],
            )
            .unwrap();

        let mut meta = TensorListMeta::add(destination.make_mut(), group);
        meta.append_item_from([tensor("output")].into(), &source_meta, 0)
            .unwrap();

        assert_eq!(
            source_meta
                .typed_tensor(
                    0,
                    glib::Quark::from_str("input"),
                    TensorDataType::Uint8,
                    TensorDimOrder::RowMajor,
                    &[1, 2]
                )
                .unwrap()
                .id(),
            glib::Quark::from_str("input")
        );

        assert_eq!(
            meta.typed_tensor(
                0,
                glib::Quark::from_str("output"),
                TensorDataType::Uint8,
                TensorDimOrder::RowMajor,
                &[1, 2]
            )
            .unwrap()
            .id(),
            glib::Quark::from_str("output")
        );

        assert_eq!(
            meta.provenance(0).collect::<Vec<_>>(),
            source_meta.provenance(0).collect::<Vec<_>>()
        );

        assert_ne!(
            meta.provenance(0).next().unwrap().correlation_ids.as_ptr(),
            source_meta
                .provenance(0)
                .next()
                .unwrap()
                .correlation_ids
                .as_ptr()
        );
    }
}
