// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{
    TensorDataType, TensorDimOrder, TensorListMeta, TensorMeta, TensorProvenance, TensorRef,
    TypedTensorDesc,
};

enum TensorMetaSource<'a> {
    List(gst::MetaRef<'a, TensorListMeta>),
    Legacy(gst::MetaRef<'a, TensorMeta>),
}

pub struct TensorMetaView<'a>(TensorMetaSource<'a>);

impl<'a> TensorMetaView<'a> {
    // rustdoc-stripper-ignore-next
    /// Find a non-empty tensor list with the requested group ID, falling back
    /// to the first tensor meta accepted by `matches_legacy`.
    pub fn find<F>(
        buffer: &'a gst::BufferRef,
        group_id: glib::Quark,
        mut matches_legacy: F,
    ) -> Option<Self>
    where
        F: FnMut(&TensorMeta) -> bool,
    {
        skip_assert_initialized!();

        if let Some(list) = buffer
            .iter_meta::<TensorListMeta>()
            .find(|meta| meta.group_id() == Some(group_id) && meta.num_items() > 0)
        {
            return Some(Self(TensorMetaSource::List(list)));
        }

        buffer
            .iter_meta::<TensorMeta>()
            .find(|meta| matches_legacy(meta))
            .map(|meta| Self(TensorMetaSource::Legacy(meta)))
    }

    pub fn is_list(&self) -> bool {
        matches!(&self.0, TensorMetaSource::List(_))
    }

    pub fn group_id(&self) -> Option<glib::Quark> {
        match &self.0 {
            TensorMetaSource::List(list) => list.group_id(),
            TensorMetaSource::Legacy(_) => None,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Find a tensor group as in [`Self::find`], matching legacy metadata against `desc`.
    pub fn find_typed(
        buffer: &'a gst::BufferRef,
        group_id: glib::Quark,
        desc: &TypedTensorDesc<'_>,
    ) -> Option<Self> {
        skip_assert_initialized!();

        Self::find(buffer, group_id, |meta| meta.get(desc).is_some())
    }

    // rustdoc-stripper-ignore-next
    /// A legacy `TensorMeta` is exposed as a single item.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = TensorItem<'_>> + '_ {
        let len = match &self.0 {
            TensorMetaSource::List(list) => list.num_items(),
            TensorMetaSource::Legacy(_) => 1,
        };

        (0..len).map(|index| {
            TensorItem(match &self.0 {
                TensorMetaSource::List(list) => TensorItemSource::List(list, index),
                TensorMetaSource::Legacy(meta) => TensorItemSource::Legacy(meta),
            })
        })
    }
}

enum TensorItemSource<'a> {
    List(&'a TensorListMeta, usize),
    Legacy(&'a TensorMeta),
}

pub struct TensorItem<'a>(TensorItemSource<'a>);

impl<'a> TensorItem<'a> {
    pub fn typed_tensor(
        &self,
        id: glib::Quark,
        data_type: TensorDataType,
        order: TensorDimOrder,
        dims: &[usize],
    ) -> Option<&'a TensorRef> {
        self.get(&TypedTensorDesc::new(id, data_type, order, dims))
    }

    pub fn get(&self, desc: &TypedTensorDesc<'_>) -> Option<&'a TensorRef> {
        match self.0 {
            TensorItemSource::List(list, index) => {
                list.typed_tensor(index, desc.id, desc.data_type, desc.order, desc.dims)
            }
            TensorItemSource::Legacy(meta) => meta.get(desc),
        }
    }

    pub fn provenance(&self) -> impl Iterator<Item = TensorProvenance<'a>> + 'a {
        match self.0 {
            TensorItemSource::List(list, index) => Some(list.provenance(index)),
            TensorItemSource::Legacy(_) => None,
        }
        .into_iter()
        .flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Tensor;

    fn make_tensor(id: glib::Quark, size: usize) -> Tensor {
        skip_assert_initialized!();

        Tensor::new_simple(
            id,
            TensorDataType::Uint8,
            gst::Buffer::with_size(size).unwrap(),
            TensorDimOrder::RowMajor,
            &[size],
        )
    }

    #[test]
    fn find_falls_back_to_legacy() {
        gst::init().unwrap();

        let group = glib::Quark::from_str("group");
        let other_group = glib::Quark::from_str("other-group");
        let tensor_id = glib::Quark::from_str("input");

        let mut buffer = gst::Buffer::new();
        let buffer = buffer.get_mut().unwrap();

        TensorMeta::add(buffer).set([make_tensor(tensor_id, 1)].into());
        TensorMeta::add(buffer).set([make_tensor(tensor_id, 2)].into());
        TensorListMeta::add(buffer, group);

        assert!(TensorMetaView::find(buffer, group, |_| false).is_none());

        let desc = TypedTensorDesc::new(
            tensor_id,
            TensorDataType::Uint8,
            TensorDimOrder::RowMajor,
            &[2],
        );

        for group_id in [group, other_group] {
            let view = TensorMetaView::find_typed(buffer, group_id, &desc).unwrap();
            assert!(!view.is_list());
            assert_eq!(view.group_id(), None);
            assert_eq!(view.iter().len(), 1);

            let item = view.iter().next().unwrap();
            assert_eq!(item.provenance().count(), 0);
            assert_eq!(item.get(&desc).unwrap().dims(), &[2]);
        }
    }

    #[test]
    fn find_prefers_tensor_list() {
        gst::init().unwrap();

        let sizes = [3, 4];
        let group = glib::Quark::from_str("group");
        let tensor_id = glib::Quark::from_str("input");
        let mut buffer = gst::Buffer::new();
        let buffer = buffer.get_mut().unwrap();

        TensorMeta::add(buffer).set([make_tensor(tensor_id, 2)].into());

        let correlations = [std::num::NonZeroI64::new(7).unwrap()];
        let provenance = TensorProvenance {
            key: tensor_id,
            correlation_ids: &correlations,
        };

        let mut list = TensorListMeta::add(buffer, group);
        list.append_item_full([make_tensor(tensor_id, sizes[0])].into(), &[provenance])
            .unwrap();
        list.append_item([make_tensor(tensor_id, sizes[1])].into())
            .unwrap();

        let view = TensorMetaView::find(buffer, group, |_| panic!("unexpected fallback")).unwrap();
        assert!(view.is_list());
        assert_eq!(view.group_id(), Some(group));
        assert_eq!(view.iter().len(), sizes.len());

        let desc = TypedTensorDesc::new(
            tensor_id,
            TensorDataType::Uint8,
            TensorDimOrder::RowMajor,
            &[usize::MAX],
        );

        for (item, expected_size) in view.iter().zip(sizes) {
            assert_eq!(item.get(&desc).unwrap().dims(), &[expected_size]);
        }

        assert_eq!(
            view.iter().next().unwrap().provenance().next(),
            Some(provenance)
        );
    }
}
