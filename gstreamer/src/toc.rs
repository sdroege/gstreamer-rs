// Take a look at the license at the top of the repository in the LICENSE file.

use std::ffi::CStr;
use std::fmt;
use std::mem;

use glib::translate::{
    from_glib, from_glib_full, from_glib_none, FromGlibPtrContainer, ToGlib, ToGlibPtr,
};

use crate::TagList;
use crate::TagMergeMode;
use crate::TocEntryType;
use crate::TocLoopType;
use crate::TocScope;

gst_define_mini_object_wrapper!(Toc, TocRef, ffi::GstToc, || { ffi::gst_toc_get_type() });

impl Toc {
    pub fn new(scope: TocScope) -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_toc_new(scope.to_glib())) }
    }
}

impl TocRef {
    pub fn get_scope(&self) -> TocScope {
        unsafe { from_glib(ffi::gst_toc_get_scope(self.as_ptr())) }
    }

    pub fn find_entry(&self, uid: &str) -> Option<TocEntry> {
        unsafe { from_glib_none(ffi::gst_toc_find_entry(self.as_ptr(), uid.to_glib_none().0)) }
    }

    pub fn get_entries(&self) -> Vec<TocEntry> {
        unsafe { FromGlibPtrContainer::from_glib_none(ffi::gst_toc_get_entries(self.as_ptr())) }
    }

    pub fn append_entry(&mut self, entry: TocEntry) {
        unsafe {
            ffi::gst_toc_append_entry(self.as_mut_ptr(), entry.into_ptr());
        }
    }

    pub fn get_tags(&self) -> Option<TagList> {
        unsafe { from_glib_none(ffi::gst_toc_get_tags(self.as_ptr())) }
    }

    pub fn set_tags(&mut self, tag_list: TagList) {
        unsafe {
            ffi::gst_toc_set_tags(self.as_mut_ptr(), tag_list.into_ptr());
        }
    }

    pub fn merge_tags(&mut self, tag_list: &TagList, mode: TagMergeMode) {
        unsafe {
            ffi::gst_toc_merge_tags(self.as_mut_ptr(), tag_list.as_mut_ptr(), mode.to_glib());
        }
    }

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
            .field("scope", &self.get_scope())
            .field("tags", &self.get_tags())
            .field("entries", &self.get_entries())
            .finish()
    }
}

gst_define_mini_object_wrapper!(TocEntry, TocEntryRef, ffi::GstTocEntry, || {
    ffi::gst_toc_entry_get_type()
});

impl TocEntry {
    pub fn new(type_: TocEntryType, uid: &str) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_toc_entry_new(
                type_.to_glib(),
                uid.to_glib_none().0,
            ))
        }
    }
}

impl TocEntryRef {
    pub fn get_entry_type(&self) -> TocEntryType {
        unsafe { from_glib(ffi::gst_toc_entry_get_entry_type(self.as_ptr())) }
    }

    pub fn get_uid(&self) -> &str {
        unsafe {
            CStr::from_ptr(ffi::gst_toc_entry_get_uid(self.as_ptr()))
                .to_str()
                .unwrap()
        }
    }

    pub fn append_sub_entry(&mut self, subentry: TocEntry) {
        unsafe {
            ffi::gst_toc_entry_append_sub_entry(self.as_mut_ptr(), subentry.into_ptr());
        }
    }

    pub fn get_sub_entries(&self) -> Vec<TocEntry> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gst_toc_entry_get_sub_entries(self.as_ptr()))
        }
    }

    pub fn get_parent(&self) -> Option<TocEntry> {
        unsafe { from_glib_none(ffi::gst_toc_entry_get_parent(self.as_mut_ptr())) }
    }

    pub fn get_start_stop_times(&self) -> Option<(i64, i64)> {
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

    pub fn set_start_stop_times(&mut self, start: i64, stop: i64) {
        unsafe {
            ffi::gst_toc_entry_set_start_stop_times(self.as_mut_ptr(), start, stop);
        }
    }

    pub fn get_tags(&self) -> Option<TagList> {
        unsafe { from_glib_none(ffi::gst_toc_entry_get_tags(self.as_ptr())) }
    }

    pub fn set_tags(&mut self, tag_list: TagList) {
        unsafe {
            ffi::gst_toc_entry_set_tags(self.as_mut_ptr(), tag_list.into_ptr());
        }
    }

    pub fn merge_tags(&mut self, tag_list: &TagList, mode: TagMergeMode) {
        unsafe {
            ffi::gst_toc_entry_merge_tags(self.as_mut_ptr(), tag_list.as_mut_ptr(), mode.to_glib());
        }
    }

    pub fn is_alternative(&self) -> bool {
        unsafe { from_glib(ffi::gst_toc_entry_is_alternative(self.as_ptr())) }
    }

    pub fn is_sequence(&self) -> bool {
        unsafe { from_glib(ffi::gst_toc_entry_is_sequence(self.as_ptr())) }
    }

    pub fn get_loop(&self) -> Option<(TocLoopType, i32)> {
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

    pub fn set_loop(&mut self, loop_type: TocLoopType, repeat_count: i32) {
        unsafe {
            ffi::gst_toc_entry_set_loop(self.as_mut_ptr(), loop_type.to_glib(), repeat_count);
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
            .field("entry_type", &self.get_entry_type())
            .field("uid", &self.get_uid())
            .field("start_stop", &self.get_start_stop_times())
            .field("tags", &self.get_tags())
            .field("is_alternative", &self.is_alternative())
            .field("is_sequence", &self.is_sequence())
            .field("loop", &self.get_loop())
            .field("sub_entries", &self.get_sub_entries())
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
        let parent = toc_sub_entry.get_parent();
        assert!(parent.is_none());

        // Append sub entry
        toc_entry.get_mut().unwrap().append_sub_entry(toc_sub_entry);

        // Toc
        let mut toc = Toc::new(TocScope::Global);
        assert_eq!(toc.get_scope(), TocScope::Global);

        // Append toc entry
        toc.get_mut().unwrap().append_entry(toc_entry);
        assert_eq!(toc.get_scope(), TocScope::Global);

        // Check toc entries
        let toc_entries = toc.get_entries();
        assert_eq!(toc_entries.len(), 1);

        let toc_parent_entry = &toc_entries[0];
        assert_eq!(toc_parent_entry.get_entry_type(), TocEntryType::Chapter);
        assert_eq!(toc_parent_entry.get_uid(), "chapter");
        let start_stop_times = toc_parent_entry.get_start_stop_times();
        assert!(start_stop_times.is_some());
        assert_eq!(start_stop_times.unwrap(), (1, 10));

        // Check sub entry
        let toc_sub_entries = toc_parent_entry.get_sub_entries();
        assert_eq!(toc_sub_entries.len(), 1);
        let toc_sub_entry = &toc_sub_entries[0];
        assert_eq!(toc_sub_entry.get_entry_type(), TocEntryType::Angle);
        let parent = toc_sub_entry.get_parent();
        assert!(parent.is_some());
        assert_eq!(parent.unwrap().get_entry_type(), TocEntryType::Chapter);
    }
}
