// Take a look at the license at the top of the repository in the LICENSE file.

use std::{fmt, marker::PhantomData, ptr, str};

use glib::{prelude::*, translate::*, value::ToSendValue};

use crate::{caps_features::*, structure::*, CapsIntersectMode};

mini_object_wrapper!(Caps, CapsRef, ffi::GstCaps, || { ffi::gst_caps_get_type() });

impl Caps {
    #[doc(alias = "gst_caps_new_simple")]
    pub fn builder(name: impl IntoGStr) -> Builder<NoFeature> {
        assert_initialized_main_thread!();
        Builder::new(name)
    }

    #[doc(alias = "gst_caps_new_full")]
    pub fn builder_full() -> BuilderFull<SomeFeatures> {
        assert_initialized_main_thread!();
        BuilderFull::new()
    }

    #[doc(alias = "gst_caps_new_full")]
    pub fn builder_full_with_features(features: CapsFeatures) -> BuilderFull<SomeFeatures> {
        assert_initialized_main_thread!();
        BuilderFull::with_features(features)
    }

    #[doc(alias = "gst_caps_new_full")]
    pub fn builder_full_with_any_features() -> BuilderFull<AnyFeatures> {
        assert_initialized_main_thread!();
        BuilderFull::with_any_features()
    }

    #[doc(alias = "gst_caps_new_empty")]
    pub fn new_empty() -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_caps_new_empty()) }
    }

    #[doc(alias = "gst_caps_new_any")]
    pub fn new_any() -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_caps_new_any()) }
    }

    #[doc(alias = "gst_caps_new_empty_simple")]
    pub fn new_empty_simple(name: impl IntoGStr) -> Self {
        skip_assert_initialized!();
        let mut caps = Caps::new_empty();

        let structure = Structure::new_empty(name);
        caps.get_mut().unwrap().append_structure(structure);

        caps
    }

    #[doc(alias = "gst_caps_fixate")]
    pub fn fixate(&mut self) {
        unsafe {
            // See https://gitlab.freedesktop.org/gstreamer/gstreamer/-/merge_requests/388
            assert!(!self.is_any());
            let ptr = if self.is_empty() {
                ffi::gst_caps_new_empty()
            } else {
                ffi::gst_caps_fixate(self.as_mut_ptr())
            };
            self.replace_ptr(ptr);
        }
    }

    #[doc(alias = "gst_caps_merge")]
    pub fn merge(&mut self, other: Self) {
        unsafe {
            let ptr = ffi::gst_caps_merge(self.as_mut_ptr(), other.into_glib_ptr());
            self.replace_ptr(ptr);
        }
    }

    #[doc(alias = "gst_caps_merge_structure")]
    pub fn merge_structure(&mut self, structure: Structure) {
        unsafe {
            let ptr = ffi::gst_caps_merge_structure(self.as_mut_ptr(), structure.into_glib_ptr());
            self.replace_ptr(ptr);
        }
    }

    #[doc(alias = "gst_caps_merge_structure_full")]
    pub fn merge_structure_full(&mut self, structure: Structure, features: Option<CapsFeatures>) {
        unsafe {
            let ptr = ffi::gst_caps_merge_structure_full(
                self.as_mut_ptr(),
                structure.into_glib_ptr(),
                features
                    .map(|f| f.into_glib_ptr())
                    .unwrap_or(ptr::null_mut()),
            );
            self.replace_ptr(ptr);
        }
    }

    #[doc(alias = "gst_caps_normalize")]
    pub fn normalize(&mut self) {
        unsafe {
            let ptr = ffi::gst_caps_normalize(self.as_mut_ptr());
            self.replace_ptr(ptr);
        }
    }

    #[doc(alias = "gst_caps_simplify")]
    pub fn simplify(&mut self) {
        unsafe {
            let ptr = ffi::gst_caps_simplify(self.as_mut_ptr());
            self.replace_ptr(ptr);
        }
    }

    #[doc(alias = "gst_caps_truncate")]
    pub fn truncate(&mut self) {
        unsafe {
            let ptr = ffi::gst_caps_truncate(self.as_mut_ptr());
            self.replace_ptr(ptr);
        }
    }
}

impl str::FromStr for Caps {
    type Err = glib::BoolError;

    #[doc(alias = "gst_caps_from_string")]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_initialized_main_thread!();
        unsafe {
            s.run_with_gstr(|s| {
                Option::<_>::from_glib_full(ffi::gst_caps_from_string(s.as_ptr()))
                    .ok_or_else(|| glib::bool_error!("Failed to parse caps from string"))
            })
        }
    }
}

impl From<Structure> for Caps {
    fn from(v: Structure) -> Caps {
        skip_assert_initialized!();
        let mut caps = Caps::new_empty();

        {
            let caps = caps.get_mut().unwrap();
            caps.append_structure(v);
        }

        caps
    }
}

impl<const N: usize> From<[Structure; N]> for Caps {
    fn from(v: [Structure; N]) -> Caps {
        skip_assert_initialized!();
        let mut caps = Caps::new_empty();

        {
            let caps = caps.get_mut().unwrap();
            v.into_iter().for_each(|s| caps.append_structure(s));
        }

        caps
    }
}

impl From<(Structure, CapsFeatures)> for Caps {
    fn from(v: (Structure, CapsFeatures)) -> Caps {
        skip_assert_initialized!();
        let mut caps = Caps::new_empty();

        {
            let caps = caps.get_mut().unwrap();
            caps.append_structure_full(v.0, Some(v.1));
        }

        caps
    }
}

impl<const N: usize> From<[(Structure, CapsFeatures); N]> for Caps {
    fn from(v: [(Structure, CapsFeatures); N]) -> Caps {
        skip_assert_initialized!();
        let mut caps = Caps::new_empty();

        {
            let caps = caps.get_mut().unwrap();
            v.into_iter()
                .for_each(|s| caps.append_structure_full(s.0, Some(s.1)));
        }

        caps
    }
}

impl<const N: usize> From<[(Structure, Option<CapsFeatures>); N]> for Caps {
    fn from(v: [(Structure, Option<CapsFeatures>); N]) -> Caps {
        skip_assert_initialized!();
        let mut caps = Caps::new_empty();

        {
            let caps = caps.get_mut().unwrap();
            v.into_iter()
                .for_each(|s| caps.append_structure_full(s.0, s.1));
        }

        caps
    }
}

impl std::iter::FromIterator<Structure> for Caps {
    fn from_iter<T: IntoIterator<Item = Structure>>(iter: T) -> Self {
        skip_assert_initialized!();
        let mut caps = Caps::new_empty();

        {
            let caps = caps.get_mut().unwrap();
            iter.into_iter().for_each(|s| caps.append_structure(s));
        }

        caps
    }
}

impl std::iter::FromIterator<(Structure, CapsFeatures)> for Caps {
    fn from_iter<T: IntoIterator<Item = (Structure, CapsFeatures)>>(iter: T) -> Self {
        skip_assert_initialized!();
        let mut caps = Caps::new_empty();

        {
            let caps = caps.get_mut().unwrap();
            iter.into_iter()
                .for_each(|(s, f)| caps.append_structure_full(s, Some(f)));
        }

        caps
    }
}

impl std::iter::FromIterator<(Structure, Option<CapsFeatures>)> for Caps {
    fn from_iter<T: IntoIterator<Item = (Structure, Option<CapsFeatures>)>>(iter: T) -> Self {
        skip_assert_initialized!();
        let mut caps = Caps::new_empty();

        {
            let caps = caps.get_mut().unwrap();
            iter.into_iter()
                .for_each(|(s, f)| caps.append_structure_full(s, f));
        }

        caps
    }
}

impl std::iter::FromIterator<Caps> for Caps {
    fn from_iter<T: IntoIterator<Item = Caps>>(iter: T) -> Self {
        skip_assert_initialized!();
        let mut caps = Caps::new_empty();

        {
            let caps = caps.get_mut().unwrap();
            iter.into_iter()
                .for_each(|other_caps| caps.append(other_caps));
        }

        caps
    }
}

impl std::iter::Extend<Structure> for CapsRef {
    fn extend<T: IntoIterator<Item = Structure>>(&mut self, iter: T) {
        iter.into_iter().for_each(|s| self.append_structure(s));
    }
}

impl std::iter::Extend<(Structure, CapsFeatures)> for CapsRef {
    fn extend<T: IntoIterator<Item = (Structure, CapsFeatures)>>(&mut self, iter: T) {
        iter.into_iter()
            .for_each(|(s, f)| self.append_structure_full(s, Some(f)));
    }
}

impl std::iter::Extend<(Structure, Option<CapsFeatures>)> for CapsRef {
    fn extend<T: IntoIterator<Item = (Structure, Option<CapsFeatures>)>>(&mut self, iter: T) {
        iter.into_iter()
            .for_each(|(s, f)| self.append_structure_full(s, f));
    }
}

impl std::iter::Extend<Caps> for CapsRef {
    fn extend<T: IntoIterator<Item = Caps>>(&mut self, iter: T) {
        iter.into_iter().for_each(|caps| self.append(caps));
    }
}

impl CapsRef {
    #[doc(alias = "gst_caps_set_value")]
    #[doc(alias = "gst_caps_set_simple")]
    pub fn set(&mut self, name: impl IntoGStr, value: impl ToSendValue + Sync) {
        let value = value.to_send_value();
        self.set_value(name, value);
    }

    #[doc(alias = "gst_caps_set_value")]
    pub fn set_value(&mut self, name: impl IntoGStr, value: glib::SendValue) {
        unsafe {
            name.run_with_gstr(|name| {
                ffi::gst_caps_set_value(self.as_mut_ptr(), name.as_ptr(), value.to_glib_none().0)
            });
        }
    }

    #[doc(alias = "get_structure")]
    #[doc(alias = "gst_caps_get_structure")]
    pub fn structure(&self, idx: u32) -> Option<&StructureRef> {
        if idx >= self.size() {
            return None;
        }

        unsafe {
            let structure = ffi::gst_caps_get_structure(self.as_ptr(), idx);
            if structure.is_null() {
                return None;
            }

            Some(StructureRef::from_glib_borrow(structure))
        }
    }

    #[doc(alias = "get_mut_structure")]
    #[doc(alias = "gst_caps_get_structure")]
    pub fn structure_mut(&mut self, idx: u32) -> Option<&mut StructureRef> {
        if idx >= self.size() {
            return None;
        }

        unsafe {
            let structure = ffi::gst_caps_get_structure(self.as_ptr(), idx);
            if structure.is_null() {
                return None;
            }

            Some(StructureRef::from_glib_borrow_mut(structure))
        }
    }

    #[doc(alias = "get_features")]
    #[doc(alias = "gst_caps_get_features")]
    pub fn features(&self, idx: u32) -> Option<&CapsFeaturesRef> {
        if idx >= self.size() {
            return None;
        }

        unsafe {
            let features = ffi::gst_caps_get_features(self.as_ptr(), idx);
            Some(CapsFeaturesRef::from_glib_borrow(features))
        }
    }

    #[doc(alias = "get_mut_features")]
    #[doc(alias = "gst_caps_get_features")]
    pub fn features_mut(&mut self, idx: u32) -> Option<&mut CapsFeaturesRef> {
        if idx >= self.size() {
            return None;
        }

        unsafe {
            let features = ffi::gst_caps_get_features(self.as_ptr(), idx);
            Some(CapsFeaturesRef::from_glib_borrow_mut(features))
        }
    }

    #[doc(alias = "gst_caps_set_features")]
    pub fn set_features(&mut self, idx: u32, features: Option<CapsFeatures>) {
        assert!(idx < self.size());

        unsafe {
            ffi::gst_caps_set_features(
                self.as_mut_ptr(),
                idx,
                features
                    .map(|f| f.into_glib_ptr())
                    .unwrap_or(ptr::null_mut()),
            )
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_caps_set_features_simple")]
    pub fn set_features_simple(&mut self, features: Option<CapsFeatures>) {
        unsafe {
            ffi::gst_caps_set_features_simple(
                self.as_mut_ptr(),
                features
                    .map(|f| f.into_glib_ptr())
                    .unwrap_or(ptr::null_mut()),
            )
        }
    }

    #[doc(alias = "get_size")]
    #[doc(alias = "gst_caps_get_size")]
    pub fn size(&self) -> u32 {
        unsafe { ffi::gst_caps_get_size(self.as_ptr()) }
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }

    pub fn iter_mut(&mut self) -> IterMut {
        IterMut::new(self)
    }

    pub fn iter_with_features(&self) -> IterFeatures {
        IterFeatures::new(self)
    }

    pub fn iter_with_features_mut(&mut self) -> IterFeaturesMut {
        IterFeaturesMut::new(self)
    }

    #[doc(alias = "gst_caps_append_structure")]
    pub fn append_structure(&mut self, structure: Structure) {
        unsafe { ffi::gst_caps_append_structure(self.as_mut_ptr(), structure.into_glib_ptr()) }
    }

    #[doc(alias = "gst_caps_append_structure_full")]
    pub fn append_structure_full(&mut self, structure: Structure, features: Option<CapsFeatures>) {
        unsafe {
            ffi::gst_caps_append_structure_full(
                self.as_mut_ptr(),
                structure.into_glib_ptr(),
                features
                    .map(|f| f.into_glib_ptr())
                    .unwrap_or(ptr::null_mut()),
            )
        }
    }

    #[doc(alias = "gst_caps_remove_structure")]
    pub fn remove_structure(&mut self, idx: u32) {
        unsafe { ffi::gst_caps_remove_structure(self.as_mut_ptr(), idx) }
    }

    #[doc(alias = "gst_caps_append")]
    pub fn append(&mut self, other: Caps) {
        unsafe { ffi::gst_caps_append(self.as_mut_ptr(), other.into_glib_ptr()) }
    }

    #[doc(alias = "gst_caps_can_intersect")]
    pub fn can_intersect(&self, other: &Self) -> bool {
        unsafe { from_glib(ffi::gst_caps_can_intersect(self.as_ptr(), other.as_ptr())) }
    }

    #[doc(alias = "gst_caps_intersect")]
    pub fn intersect(&self, other: &Self) -> Caps {
        unsafe {
            from_glib_full(ffi::gst_caps_intersect(
                self.as_mut_ptr(),
                other.as_mut_ptr(),
            ))
        }
    }

    #[doc(alias = "gst_caps_intersect_full")]
    pub fn intersect_with_mode(&self, other: &Self, mode: CapsIntersectMode) -> Caps {
        unsafe {
            from_glib_full(ffi::gst_caps_intersect_full(
                self.as_mut_ptr(),
                other.as_mut_ptr(),
                mode.into_glib(),
            ))
        }
    }

    #[doc(alias = "gst_caps_is_always_compatible")]
    pub fn is_always_compatible(&self, other: &Self) -> bool {
        unsafe {
            from_glib(ffi::gst_caps_is_always_compatible(
                self.as_ptr(),
                other.as_ptr(),
            ))
        }
    }

    #[doc(alias = "gst_caps_is_any")]
    pub fn is_any(&self) -> bool {
        unsafe { from_glib(ffi::gst_caps_is_any(self.as_ptr())) }
    }

    #[doc(alias = "gst_caps_is_empty")]
    pub fn is_empty(&self) -> bool {
        unsafe { from_glib(ffi::gst_caps_is_empty(self.as_ptr())) }
    }

    #[doc(alias = "gst_caps_is_fixed")]
    pub fn is_fixed(&self) -> bool {
        unsafe { from_glib(ffi::gst_caps_is_fixed(self.as_ptr())) }
    }

    #[doc(alias = "gst_caps_is_equal_fixed")]
    pub fn is_equal_fixed(&self, other: &Self) -> bool {
        unsafe { from_glib(ffi::gst_caps_is_equal_fixed(self.as_ptr(), other.as_ptr())) }
    }

    #[doc(alias = "gst_caps_is_strictly_equal")]
    pub fn is_strictly_equal(&self, other: &Self) -> bool {
        unsafe {
            from_glib(ffi::gst_caps_is_strictly_equal(
                self.as_ptr(),
                other.as_ptr(),
            ))
        }
    }

    #[doc(alias = "gst_caps_is_subset")]
    pub fn is_subset(&self, superset: &Self) -> bool {
        unsafe { from_glib(ffi::gst_caps_is_subset(self.as_ptr(), superset.as_ptr())) }
    }

    #[doc(alias = "gst_caps_is_subset_structure")]
    pub fn is_subset_structure(&self, structure: &StructureRef) -> bool {
        unsafe {
            from_glib(ffi::gst_caps_is_subset_structure(
                self.as_ptr(),
                structure.as_ptr(),
            ))
        }
    }

    #[doc(alias = "gst_caps_is_subset_structure_full")]
    pub fn is_subset_structure_full(
        &self,
        structure: &StructureRef,
        features: Option<&CapsFeaturesRef>,
    ) -> bool {
        unsafe {
            from_glib(ffi::gst_caps_is_subset_structure_full(
                self.as_ptr(),
                structure.as_ptr(),
                features.map(|f| f.as_ptr()).unwrap_or(ptr::null()),
            ))
        }
    }

    #[doc(alias = "gst_caps_subtract")]
    pub fn subtract(&self, other: &Self) -> Caps {
        unsafe {
            from_glib_full(ffi::gst_caps_subtract(
                self.as_mut_ptr(),
                other.as_mut_ptr(),
            ))
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_caps_serialize")]
    pub fn serialize(&self, flags: crate::SerializeFlags) -> glib::GString {
        unsafe { from_glib_full(ffi::gst_caps_serialize(&self.0, flags.into_glib())) }
    }

    #[doc(alias = "gst_caps_foreach")]
    pub fn foreach<F: FnMut(&CapsFeaturesRef, &StructureRef) -> std::ops::ControlFlow<()>>(
        &self,
        mut func: F,
    ) -> bool {
        unsafe {
            unsafe extern "C" fn trampoline<
                F: FnMut(&CapsFeaturesRef, &StructureRef) -> std::ops::ControlFlow<()>,
            >(
                features: *mut ffi::GstCapsFeatures,
                s: *mut ffi::GstStructure,
                user_data: glib::ffi::gpointer,
            ) -> glib::ffi::gboolean {
                let func = &mut *(user_data as *mut F);
                let res = func(
                    CapsFeaturesRef::from_glib_borrow(features),
                    StructureRef::from_glib_borrow(s),
                );

                matches!(res, std::ops::ControlFlow::Continue(_)).into_glib()
            }
            let func = &mut func as *mut F;
            from_glib(ffi::gst_caps_foreach(
                self.as_ptr(),
                Some(trampoline::<F>),
                func as glib::ffi::gpointer,
            ))
        }
    }

    #[doc(alias = "gst_caps_map_in_place")]
    pub fn map_in_place<
        F: FnMut(&mut CapsFeaturesRef, &mut StructureRef) -> std::ops::ControlFlow<()>,
    >(
        &mut self,
        mut func: F,
    ) -> bool {
        unsafe {
            unsafe extern "C" fn trampoline<
                F: FnMut(&mut CapsFeaturesRef, &mut StructureRef) -> std::ops::ControlFlow<()>,
            >(
                features: *mut ffi::GstCapsFeatures,
                s: *mut ffi::GstStructure,
                user_data: glib::ffi::gpointer,
            ) -> glib::ffi::gboolean {
                let func = &mut *(user_data as *mut F);
                let res = func(
                    CapsFeaturesRef::from_glib_borrow_mut(features),
                    StructureRef::from_glib_borrow_mut(s),
                );

                matches!(res, std::ops::ControlFlow::Continue(_)).into_glib()
            }
            let func = &mut func as *mut F;
            from_glib(ffi::gst_caps_map_in_place(
                self.as_mut_ptr(),
                Some(trampoline::<F>),
                func as glib::ffi::gpointer,
            ))
        }
    }

    #[doc(alias = "gst_caps_filter_and_map_in_place")]
    pub fn filter_map_in_place<
        F: FnMut(&mut CapsFeaturesRef, &mut StructureRef) -> CapsFilterMapAction,
    >(
        &mut self,
        mut func: F,
    ) {
        unsafe {
            unsafe extern "C" fn trampoline<
                F: FnMut(&mut CapsFeaturesRef, &mut StructureRef) -> CapsFilterMapAction,
            >(
                features: *mut ffi::GstCapsFeatures,
                s: *mut ffi::GstStructure,
                user_data: glib::ffi::gpointer,
            ) -> glib::ffi::gboolean {
                let func = &mut *(user_data as *mut F);

                let res = func(
                    CapsFeaturesRef::from_glib_borrow_mut(features),
                    StructureRef::from_glib_borrow_mut(s),
                );

                match res {
                    CapsFilterMapAction::Keep => glib::ffi::GTRUE,
                    CapsFilterMapAction::Remove => glib::ffi::GFALSE,
                }
            }

            let func = &mut func as *mut F;
            ffi::gst_caps_filter_and_map_in_place(
                self.as_mut_ptr(),
                Some(trampoline::<F>),
                func as glib::ffi::gpointer,
            );
        }
    }
}

#[derive(Debug)]
pub enum CapsFilterMapAction {
    Keep,
    Remove,
}

macro_rules! define_iter(
    ($name:ident, $typ:ty, $styp:ty, $get_item:expr) => {
    #[derive(Debug)]
    pub struct $name<'a> {
        caps: $typ,
        idx: usize,
        n_structures: usize,
    }

    impl<'a> $name<'a> {
        fn new(caps: $typ) -> $name<'a> {
            skip_assert_initialized!();
            let n_structures = caps.size();

            $name {
                caps,
                idx: 0,
                n_structures: n_structures as usize,
            }
        }
    }

    #[allow(clippy::redundant_closure_call)]
    impl<'a> Iterator for $name<'a> {
        type Item = $styp;

        fn next(&mut self) -> Option<Self::Item> {
            if self.idx >= self.n_structures {
                return None;
            }

            unsafe {
                let item = $get_item(self.caps, self.idx as u32).unwrap();
                self.idx += 1;
                Some(item)
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let remaining = self.n_structures - self.idx;

            (remaining, Some(remaining))
        }

        fn count(self) -> usize {
            self.n_structures - self.idx
        }

        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            let (end, overflow) = self.idx.overflowing_add(n);
            if end >= self.n_structures || overflow {
                self.idx = self.n_structures;
                None
            } else {
                unsafe {
                    self.idx = end + 1;
                    Some($get_item(self.caps, end as u32).unwrap())
                }
            }
        }

        fn last(self) -> Option<Self::Item> {
            if self.idx == self.n_structures {
                None
            } else {
                unsafe {
                    Some($get_item(self.caps, self.n_structures as u32 - 1).unwrap())
                }
            }
        }
    }

    #[allow(clippy::redundant_closure_call)]
    impl<'a> DoubleEndedIterator for $name<'a> {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.idx == self.n_structures {
                return None;
            }

            self.n_structures -= 1;

            unsafe {
                Some($get_item(self.caps, self.n_structures as u32).unwrap())
            }
        }

        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            let (end, overflow) = self.n_structures.overflowing_sub(n);
            if end <= self.idx || overflow {
                self.idx = self.n_structures;
                None
            } else {
                self.n_structures = end - 1;
                unsafe {
                    Some($get_item(self.caps, self.n_structures as u32).unwrap())
                }
            }
        }
    }

    impl<'a> ExactSizeIterator for $name<'a> {}

    impl<'a> std::iter::FusedIterator for $name<'a> {}
    }
);

define_iter!(
    Iter,
    &'a CapsRef,
    &'a StructureRef,
    |caps: &CapsRef, idx| {
        let ptr = ffi::gst_caps_get_structure(caps.as_ptr(), idx);
        if ptr.is_null() {
            None
        } else {
            Some(StructureRef::from_glib_borrow(
                ptr as *const ffi::GstStructure,
            ))
        }
    }
);
define_iter!(
    IterMut,
    &'a mut CapsRef,
    &'a mut StructureRef,
    |caps: &CapsRef, idx| {
        let ptr = ffi::gst_caps_get_structure(caps.as_ptr(), idx);
        if ptr.is_null() {
            None
        } else {
            Some(StructureRef::from_glib_borrow_mut(ptr))
        }
    }
);
define_iter!(
    IterFeatures,
    &'a CapsRef,
    (&'a StructureRef, &'a CapsFeaturesRef),
    |caps: &CapsRef, idx| {
        let ptr1 = ffi::gst_caps_get_structure(caps.as_ptr(), idx);
        let ptr2 = ffi::gst_caps_get_features(caps.as_ptr(), idx);
        if ptr1.is_null() || ptr2.is_null() {
            None
        } else {
            Some((
                StructureRef::from_glib_borrow(ptr1),
                CapsFeaturesRef::from_glib_borrow(ptr2),
            ))
        }
    }
);
define_iter!(
    IterFeaturesMut,
    &'a mut CapsRef,
    (&'a mut StructureRef, &'a mut CapsFeaturesRef),
    |caps: &CapsRef, idx| {
        let ptr1 = ffi::gst_caps_get_structure(caps.as_ptr(), idx);
        let ptr2 = ffi::gst_caps_get_features(caps.as_ptr(), idx);
        if ptr1.is_null() || ptr2.is_null() {
            None
        } else {
            Some((
                StructureRef::from_glib_borrow_mut(ptr1),
                CapsFeaturesRef::from_glib_borrow_mut(ptr2),
            ))
        }
    }
);

impl<'a> IntoIterator for &'a CapsRef {
    type IntoIter = IterFeatures<'a>;
    type Item = (&'a StructureRef, &'a CapsFeaturesRef);

    fn into_iter(self) -> Self::IntoIter {
        self.iter_with_features()
    }
}

impl<'a> IntoIterator for &'a mut CapsRef {
    type IntoIter = IterFeaturesMut<'a>;
    type Item = (&'a mut StructureRef, &'a mut CapsFeaturesRef);

    fn into_iter(self) -> Self::IntoIter {
        self.iter_with_features_mut()
    }
}

impl fmt::Debug for Caps {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <CapsRef as fmt::Debug>::fmt(self, f)
    }
}

impl fmt::Display for Caps {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <CapsRef as fmt::Display>::fmt(self, f)
    }
}

impl PartialEq for Caps {
    fn eq(&self, other: &Caps) -> bool {
        CapsRef::eq(self, other)
    }
}

impl Eq for Caps {}

impl PartialEq<CapsRef> for Caps {
    fn eq(&self, other: &CapsRef) -> bool {
        CapsRef::eq(self, other)
    }
}

impl PartialEq<Caps> for CapsRef {
    fn eq(&self, other: &Caps) -> bool {
        CapsRef::eq(other, self)
    }
}

impl fmt::Debug for CapsRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_any() {
            f.debug_tuple("Caps(\"ANY\")").finish()
        } else if self.is_empty() {
            f.debug_tuple("Caps(\"EMPTY\")").finish()
        } else {
            let mut debug = f.debug_tuple("Caps");

            for (structure, features) in self.iter_with_features() {
                struct WithFeatures<'a> {
                    features: &'a CapsFeaturesRef,
                    structure: &'a StructureRef,
                }

                impl<'a> fmt::Debug for WithFeatures<'a> {
                    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                        let name = format!("{}({})", self.structure.name(), self.features);
                        let mut debug = f.debug_struct(&name);

                        for (id, field) in self.structure.iter() {
                            if field.type_() == Structure::static_type() {
                                let s = field.get::<Structure>().unwrap();
                                debug.field(id, &s);
                            } else if field.type_() == crate::Array::static_type() {
                                let arr = field.get::<crate::Array>().unwrap();
                                debug.field(id, &arr);
                            } else if field.type_() == crate::List::static_type() {
                                let list = field.get::<crate::List>().unwrap();
                                debug.field(id, &list);
                            } else {
                                debug.field(id, &field);
                            }
                        }

                        debug.finish()
                    }
                }

                debug.field(&WithFeatures {
                    structure,
                    features,
                });
            }

            debug.finish()
        }
    }
}

impl fmt::Display for CapsRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = unsafe { glib::GString::from_glib_full(ffi::gst_caps_to_string(self.as_ptr())) };
        f.write_str(&s)
    }
}

impl PartialEq for CapsRef {
    #[doc(alias = "gst_caps_is_equal")]
    fn eq(&self, other: &CapsRef) -> bool {
        unsafe { from_glib(ffi::gst_caps_is_equal(self.as_ptr(), other.as_ptr())) }
    }
}

impl Eq for CapsRef {}

pub enum NoFeature {}
pub enum HasFeatures {}

#[must_use = "The builder must be built to be used"]
pub struct Builder<T> {
    s: crate::Structure,
    features: Option<CapsFeatures>,
    phantom: PhantomData<T>,
}

impl<T> fmt::Debug for Builder<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Builder")
            .field("s", &self.s)
            .field("features", &self.features)
            .field("phantom", &self.phantom)
            .finish()
    }
}

impl Builder<NoFeature> {
    fn new(name: impl IntoGStr) -> Builder<NoFeature> {
        skip_assert_initialized!();
        Builder {
            s: crate::Structure::new_empty(name),
            features: None,
            phantom: PhantomData,
        }
    }

    pub fn features(
        self,
        features: impl IntoIterator<Item = impl IntoGStr>,
    ) -> Builder<HasFeatures> {
        Builder {
            s: self.s,
            features: Some(CapsFeatures::new(features)),
            phantom: PhantomData,
        }
    }

    pub fn any_features(self) -> Builder<HasFeatures> {
        Builder {
            s: self.s,
            features: Some(CapsFeatures::new_any()),
            phantom: PhantomData,
        }
    }
}

impl<T> Builder<T> {
    pub fn field(mut self, name: impl IntoGStr, value: impl Into<glib::Value> + Send) -> Self {
        self.s.set(name, value);
        self
    }

    #[must_use = "Building the caps without using them has no effect"]
    pub fn build(self) -> Caps {
        let mut caps = Caps::new_empty();

        caps.get_mut()
            .unwrap()
            .append_structure_full(self.s, self.features);
        caps
    }

    pub fn structure(&self) -> &crate::Structure {
        &self.s
    }
}

pub enum AnyFeatures {}
pub enum SomeFeatures {}

#[must_use = "The builder must be built to be used"]
pub struct BuilderFull<T> {
    caps: crate::Caps,
    features: Option<CapsFeatures>,
    phantom: PhantomData<T>,
}

impl<T> fmt::Debug for BuilderFull<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Builder")
            .field("caps", &self.caps)
            .field("features", &self.features)
            .field("phantom", &self.phantom)
            .finish()
    }
}

impl BuilderFull<SomeFeatures> {
    fn new() -> Self {
        BuilderFull {
            caps: Caps::new_empty(),
            features: None,
            phantom: PhantomData,
        }
    }

    fn with_features(features: CapsFeatures) -> Self {
        skip_assert_initialized!();
        BuilderFull {
            caps: Caps::new_empty(),
            features: Some(features),
            phantom: PhantomData,
        }
    }

    pub fn structure_with_features(self, structure: Structure, features: CapsFeatures) -> Self {
        self.append_structure(structure, Some(features))
    }

    pub fn structure_with_any_features(self, structure: Structure) -> Self {
        self.append_structure(structure, Some(CapsFeatures::new_any()))
    }
}

impl BuilderFull<AnyFeatures> {
    fn with_any_features() -> Self {
        BuilderFull {
            caps: Caps::new_empty(),
            features: Some(CapsFeatures::new_any()),
            phantom: PhantomData,
        }
    }
}

impl<T> BuilderFull<T> {
    fn append_structure(mut self, structure: Structure, features: Option<CapsFeatures>) -> Self {
        let features = {
            match self.features {
                None => features,
                Some(ref result) => {
                    let mut result = result.clone();
                    match features {
                        None => Some(result),
                        Some(features) => {
                            features.iter().for_each(|feat| result.add(feat));
                            Some(result)
                        }
                    }
                }
            }
        };

        self.caps
            .get_mut()
            .unwrap()
            .append_structure_full(structure, features);
        self
    }

    pub fn structure(self, structure: Structure) -> Self {
        self.append_structure(structure, None)
    }

    #[must_use = "Building the caps without using them has no effect"]
    pub fn build(self) -> Caps {
        self.caps
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Array, Fraction};

    #[test]
    fn test_builder() {
        crate::init().unwrap();

        let mut caps = Caps::builder("foo/bar")
            .field("int", 12)
            .field("bool", true)
            .field("string", "bla")
            .field("fraction", Fraction::new(1, 2))
            .field("array", Array::new([1, 2]))
            .build();
        assert_eq!(
            caps.to_string(),
            "foo/bar, int=(int)12, bool=(boolean)true, string=(string)bla, fraction=(fraction)1/2, array=(int)< 1, 2 >"
        );

        assert!(caps
            .features(0)
            .unwrap()
            .is_equal(crate::CAPS_FEATURES_MEMORY_SYSTEM_MEMORY.as_ref()));

        {
            let caps = caps.get_mut().unwrap();
            caps.set_features(0, Some(CapsFeatures::new(["foo:bla"])));
        }
        assert!(caps
            .features(0)
            .unwrap()
            .is_equal(CapsFeatures::new(["foo:bla"]).as_ref()));

        let caps = Caps::builder("foo/bar")
            .field("int", 12)
            .any_features()
            .build();
        assert_eq!(caps.to_string(), "foo/bar(ANY), int=(int)12");

        let caps = Caps::builder("foo/bar")
            .field("int", 12)
            .features(["foo:bla", "foo:baz"])
            .build();
        assert_eq!(caps.to_string(), "foo/bar(foo:bla, foo:baz), int=(int)12");
    }

    #[test]
    fn test_display() {
        crate::init().unwrap();

        let caps = Caps::builder("foo/bar").build();
        format!("{caps}");
    }

    #[test]
    fn test_builder_full() {
        crate::init().unwrap();

        let caps = Caps::builder_full()
            .structure(Structure::builder("audio/x-raw").build())
            .structure(Structure::builder("video/x-raw").build())
            .build();
        assert_eq!(caps.to_string(), "audio/x-raw; video/x-raw");

        let caps = Caps::builder_full()
            .structure(
                Structure::builder("audio/x-raw")
                    .field("format", "S16LE")
                    .build(),
            )
            .structure(Structure::builder("video/x-raw").build())
            .build();
        assert_eq!(
            caps.to_string(),
            "audio/x-raw, format=(string)S16LE; video/x-raw"
        );

        let caps = Caps::builder_full()
            .structure_with_any_features(Structure::builder("audio/x-raw").build())
            .structure_with_features(
                Structure::builder("video/x-raw").build(),
                CapsFeatures::new(["foo:bla", "foo:baz"]),
            )
            .build();
        assert_eq!(
            caps.to_string(),
            "audio/x-raw(ANY); video/x-raw(foo:bla, foo:baz)"
        );
    }

    #[test]
    fn test_builder_full_with_features() {
        crate::init().unwrap();

        let caps = Caps::builder_full_with_features(CapsFeatures::new(["foo:bla"]))
            .structure(Structure::builder("audio/x-raw").build())
            .structure_with_features(
                Structure::builder("video/x-raw").build(),
                CapsFeatures::new(["foo:baz"]),
            )
            .build();
        assert_eq!(
            caps.to_string(),
            "audio/x-raw(foo:bla); video/x-raw(foo:bla, foo:baz)"
        );
    }

    #[test]
    fn test_builder_full_with_any_features() {
        crate::init().unwrap();

        let caps = Caps::builder_full_with_any_features()
            .structure(Structure::builder("audio/x-raw").build())
            .structure(Structure::builder("video/x-raw").build())
            .build();
        assert_eq!(caps.to_string(), "audio/x-raw(ANY); video/x-raw(ANY)");

        let caps = Caps::builder_full_with_any_features()
            .structure(Structure::builder("audio/x-raw").build())
            .build();
        assert_eq!(caps.to_string(), "audio/x-raw(ANY)");
    }

    #[test]
    fn test_new_from_iter() {
        crate::init().unwrap();

        let caps = Caps::builder_full_with_any_features()
            .structure(Structure::builder("audio/x-raw").build())
            .structure(Structure::builder("video/x-raw").build())
            .build();

        let audio = caps
            .iter()
            .filter(|s| s.name() == "audio/x-raw")
            .map(|s| s.to_owned())
            .collect::<Caps>();
        assert_eq!(audio.to_string(), "audio/x-raw");

        let audio = caps
            .iter_with_features()
            .filter(|(s, _)| s.name() == "audio/x-raw")
            .map(|(s, c)| (s.to_owned(), c.to_owned()))
            .collect::<Caps>();
        assert_eq!(audio.to_string(), "audio/x-raw(ANY)");
    }

    #[test]
    fn test_debug() {
        crate::init().unwrap();

        let caps = Caps::new_any();
        assert_eq!(format!("{caps:?}"), "Caps(\"ANY\")");

        let caps = Caps::new_empty();
        assert_eq!(format!("{caps:?}"), "Caps(\"EMPTY\")");

        let caps = Caps::builder_full_with_any_features()
            .structure(Structure::builder("audio/x-raw").build())
            .build();
        assert_eq!(format!("{caps:?}"), "Caps(audio/x-raw(ANY))");

        let caps = Caps::builder_full_with_features(CapsFeatures::new(["foo:bla"]))
            .structure(
                Structure::builder("audio/x-raw")
                    .field(
                        "struct",
                        Structure::builder("nested").field("badger", true).build(),
                    )
                    .build(),
            )
            .structure(
                Structure::builder("video/x-raw")
                    .field("width", 800u32)
                    .build(),
            )
            .build();

        assert_eq!(format!("{caps:?}"), "Caps(audio/x-raw(foo:bla) { struct: Structure(nested { badger: (gboolean) TRUE }) }, video/x-raw(foo:bla) { width: (guint) 800 })");

        let caps = Caps::builder_full()
            .structure(
                Structure::builder("video/x-raw")
                    .field("array", crate::Array::new(["a", "b", "c"]))
                    .field("list", crate::List::new(["d", "e", "f"]))
                    .build(),
            )
            .build();

        assert_eq!(format!("{caps:?}"), "Caps(video/x-raw(memory:SystemMemory) { array: Array([(gchararray) \"a\", (gchararray) \"b\", (gchararray) \"c\"]), list: List([(gchararray) \"d\", (gchararray) \"e\", (gchararray) \"f\"]) })");
    }
}
