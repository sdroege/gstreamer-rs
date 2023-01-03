// Take a look at the license at the top of the repository in the LICENSE file.

use std::{ffi::CStr, fmt, mem, ptr};

use glib::translate::{
    from_glib, from_glib_full, from_glib_none, FromGlibPtrContainer, IntoGlib, IntoGlibPtr,
    ToGlibPtr,
};

use crate::{TagList, TagMergeMode, TocEntryType, TocLoopType, TocScope};

mini_object_wrapper!(Toc, TocRef, ffi::GstToc, || { ffi::gst_toc_get_type() });

impl Toc {
    #[doc(alias = "gst_toc_new")]
    pub fn new(scope: TocScope) -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_toc_new(scope.into_glib())) }
    }
}

impl TocRef {
    #[doc(alias = "get_scope")]
    #[doc(alias = "gst_toc_get_scope")]
    pub fn scope(&self) -> TocScope {
        unsafe { from_glib(ffi::gst_toc_get_scope(self.as_ptr())) }
    }

    #[doc(alias = "gst_toc_find_entry")]
    pub fn find_entry(&self, uid: &str) -> Option<TocEntry> {
        unsafe { from_glib_none(ffi::gst_toc_find_entry(self.as_ptr(), uid.to_glib_none().0)) }
    }

    #[doc(alias = "get_entries")]
    #[doc(alias = "gst_toc_get_entries")]
    pub fn entries(&self) -> Vec<TocEntry> {
        unsafe { FromGlibPtrContainer::from_glib_none(ffi::gst_toc_get_entries(self.as_ptr())) }
    }

    #[doc(alias = "gst_toc_append_entry")]
    pub fn append_entry(&mut self, entry: TocEntry) {
        unsafe {
            ffi::gst_toc_append_entry(self.as_mut_ptr(), entry.into_glib_ptr());
        }
    }

    #[doc(alias = "get_tags")]
    #[doc(alias = "gst_toc_get_tags")]
    pub fn tags(&self) -> Option<TagList> {
        unsafe { from_glib_none(ffi::gst_toc_get_tags(self.as_ptr())) }
    }

    #[doc(alias = "gst_toc_set_tags")]
    pub fn set_tags(&mut self, tag_list: impl Into<Option<TagList>>) {
        unsafe {
            ffi::gst_toc_set_tags(
                self.as_mut_ptr(),
                tag_list
                    .into()
                    .map(|t| t.into_glib_ptr())
                    .unwrap_or(ptr::null_mut()),
            );
        }
    }

    #[doc(alias = "gst_toc_merge_tags")]
    pub fn merge_tags<'a>(&mut self, tag_list: impl Into<Option<&'a TagList>>, mode: TagMergeMode) {
        unsafe {
            ffi::gst_toc_merge_tags(
                self.as_mut_ptr(),
                tag_list
                    .into()
                    .map(|l| l.as_mut_ptr())
                    .unwrap_or(ptr::null_mut()),
                mode.into_glib(),
            );
        }
    }

    #[doc(alias = "gst_toc_dump")]
    pub fn dump(&self) {
        unsafe {
            ffi::gst_toc_dump(self.as_mut_ptr());
        }
    }
}

impl fmt::Debug for Toc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        TocRef::fmt(self, f)
    }
}

impl fmt::Debug for TocRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Toc")
            .field("scope", &self.scope())
            .field("tags", &self.tags())
            .field("entries", &self.entries())
            .finish()
    }
}

mini_object_wrapper!(TocEntry, TocEntryRef, ffi::GstTocEntry, || {
    ffi::gst_toc_entry_get_type()
});

impl TocEntry {
    #[doc(alias = "gst_toc_entry_new")]
    pub fn new(type_: TocEntryType, uid: &str) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_toc_entry_new(
                type_.into_glib(),
                uid.to_glib_none().0,
            ))
        }
    }
}

impl TocEntryRef {
    #[doc(alias = "get_entry_type")]
    #[doc(alias = "gst_toc_entry_get_entry_type")]
    pub fn entry_type(&self) -> TocEntryType {
        unsafe { from_glib(ffi::gst_toc_entry_get_entry_type(self.as_ptr())) }
    }

    #[doc(alias = "get_uid")]
    #[doc(alias = "gst_toc_entry_get_uid")]
    pub fn uid(&self) -> &str {
        unsafe {
            CStr::from_ptr(ffi::gst_toc_entry_get_uid(self.as_ptr()))
                .to_str()
                .unwrap()
        }
    }

    #[doc(alias = "gst_toc_entry_append_sub_entry")]
    pub fn append_sub_entry(&mut self, subentry: TocEntry) {
        unsafe {
            ffi::gst_toc_entry_append_sub_entry(self.as_mut_ptr(), subentry.into_glib_ptr());
        }
    }

    #[doc(alias = "get_sub_entries")]
    #[doc(alias = "gst_toc_entry_get_sub_entries")]
    pub fn sub_entries(&self) -> Vec<TocEntry> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gst_toc_entry_get_sub_entries(self.as_ptr()))
        }
    }

    #[doc(alias = "get_parent")]
    #[doc(alias = "gst_toc_entry_get_parent")]
    pub fn parent(&self) -> Option<TocEntry> {
        unsafe { from_glib_none(ffi::gst_toc_entry_get_parent(self.as_mut_ptr())) }
    }

    #[doc(alias = "get_start_stop_times")]
    #[doc(alias = "gst_toc_entry_get_start_stop_times")]
    pub fn start_stop_times(&self) -> Option<(i64, i64)> {
        unsafe {
            let mut start = mem::MaybeUninit::uninit();
            let mut stop = mem::MaybeUninit::uninit();

            if from_glib(ffi::gst_toc_entry_get_start_stop_times(
                self.as_ptr(),
                start.as_mut_ptr(),
                stop.as_mut_ptr(),
            )) {
                Some((start.assume_init(), stop.assume_init()))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gst_toc_entry_set_start_stop_times")]
    pub fn set_start_stop_times(&mut self, start: i64, stop: i64) {
        unsafe {
            ffi::gst_toc_entry_set_start_stop_times(self.as_mut_ptr(), start, stop);
        }
    }

    #[doc(alias = "get_tags")]
    #[doc(alias = "gst_toc_entry_get_tags")]
    pub fn tags(&self) -> Option<TagList> {
        unsafe { from_glib_none(ffi::gst_toc_entry_get_tags(self.as_ptr())) }
    }

    #[doc(alias = "gst_toc_entry_set_tags")]
    pub fn set_tags(&mut self, tag_list: impl Into<Option<TagList>>) {
        unsafe {
            ffi::gst_toc_entry_set_tags(
                self.as_mut_ptr(),
                tag_list
                    .into()
                    .map(|t| t.into_glib_ptr())
                    .unwrap_or(ptr::null_mut()),
            );
        }
    }

    #[doc(alias = "gst_toc_entry_merge_tags")]
    pub fn merge_tags<'a>(&mut self, tag_list: impl Into<Option<&'a TagList>>, mode: TagMergeMode) {
        unsafe {
            ffi::gst_toc_entry_merge_tags(
                self.as_mut_ptr(),
                tag_list
                    .into()
                    .map(|l| l.as_mut_ptr())
                    .unwrap_or(ptr::null_mut()),
                mode.into_glib(),
            );
        }
    }

    #[doc(alias = "gst_toc_entry_is_alternative")]
    pub fn is_alternative(&self) -> bool {
        unsafe { from_glib(ffi::gst_toc_entry_is_alternative(self.as_ptr())) }
    }

    #[doc(alias = "gst_toc_entry_is_sequence")]
    pub fn is_sequence(&self) -> bool {
        unsafe { from_glib(ffi::gst_toc_entry_is_sequence(self.as_ptr())) }
    }

    #[doc(alias = "get_loop")]
    pub fn loop_(&self) -> Option<(TocLoopType, i32)> {
        unsafe {
            let mut loop_type = mem::MaybeUninit::uninit();
            let mut repeat_count = mem::MaybeUninit::uninit();
            if from_glib(ffi::gst_toc_entry_get_loop(
                self.as_ptr(),
                loop_type.as_mut_ptr(),
                repeat_count.as_mut_ptr(),
            )) {
                Some((
                    from_glib(loop_type.assume_init()),
                    repeat_count.assume_init(),
                ))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gst_toc_entry_set_loop")]
    pub fn set_loop(&mut self, loop_type: TocLoopType, repeat_count: i32) {
        unsafe {
            ffi::gst_toc_entry_set_loop(self.as_mut_ptr(), loop_type.into_glib(), repeat_count);
        }
    }
}

impl fmt::Debug for TocEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        TocEntryRef::fmt(self, f)
    }
}

impl fmt::Debug for TocEntryRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TocEntry")
            .field("entry_type", &self.entry_type())
            .field("uid", &self.uid())
            .field("start_stop", &self.start_stop_times())
            .field("tags", &self.tags())
            .field("is_alternative", &self.is_alternative())
            .field("is_sequence", &self.is_sequence())
            .field("loop", &self.loop_())
            .field("sub_entries", &self.sub_entries())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        crate::init().unwrap();

        // Top level toc entry
        let mut toc_entry = TocEntry::new(TocEntryType::Chapter, "chapter");
        toc_entry.get_mut().unwrap().set_start_stop_times(1, 10);

        // Toc sub entry
        let toc_sub_entry = TocEntry::new(TocEntryType::Angle, "angle");
        let parent = toc_sub_entry.parent();
        assert!(parent.is_none());

        // Append sub entry
        toc_entry.get_mut().unwrap().append_sub_entry(toc_sub_entry);

        // Toc
        let mut toc = Toc::new(TocScope::Global);
        assert_eq!(toc.scope(), TocScope::Global);

        // Append toc entry
        toc.get_mut().unwrap().append_entry(toc_entry);
        assert_eq!(toc.scope(), TocScope::Global);

        // Check toc entries
        let toc_entries = toc.entries();
        assert_eq!(toc_entries.len(), 1);

        let toc_parent_entry = &toc_entries[0];
        assert_eq!(toc_parent_entry.entry_type(), TocEntryType::Chapter);
        assert_eq!(toc_parent_entry.uid(), "chapter");
        let start_stop_times = toc_parent_entry.start_stop_times();
        assert!(start_stop_times.is_some());
        assert_eq!(start_stop_times.unwrap(), (1, 10));

        // Check sub entry
        let toc_sub_entries = toc_parent_entry.sub_entries();
        assert_eq!(toc_sub_entries.len(), 1);
        let toc_sub_entry = &toc_sub_entries[0];
        assert_eq!(toc_sub_entry.entry_type(), TocEntryType::Angle);
        let parent = toc_sub_entry.parent();
        assert!(parent.is_some());
        assert_eq!(parent.unwrap().entry_type(), TocEntryType::Chapter);
    }
}
