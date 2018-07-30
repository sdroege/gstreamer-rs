// Copyright (C) 2016-2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::CStr;
use std::fmt;
use std::mem;

use ffi;

use glib;
use glib::translate::{
    from_glib, from_glib_full, from_glib_none, FromGlibPtrContainer, ToGlib, ToGlibPtr,
};

use miniobject::*;
use TagList;
use TagMergeMode;
use TocEntryType;
use TocLoopType;
use TocScope;

pub type Toc = GstRc<TocRef>;
pub struct TocRef(ffi::GstToc);

unsafe impl MiniObject for TocRef {
    type GstType = ffi::GstToc;
}

impl GstRc<TocRef> {
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

impl glib::types::StaticType for TocRef {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(ffi::gst_toc_get_type()) }
    }
}

impl ToOwned for TocRef {
    type Owned = GstRc<TocRef>;

    fn to_owned(&self) -> GstRc<TocRef> {
        #[cfg_attr(feature = "cargo-clippy", allow(cast_ptr_alignment))]
        unsafe {
            from_glib_full(ffi::gst_mini_object_copy(self.as_ptr() as *const _) as *mut _)
        }
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

unsafe impl Sync for TocRef {}
unsafe impl Send for TocRef {}

pub type TocEntry = GstRc<TocEntryRef>;
pub struct TocEntryRef(ffi::GstTocEntry);

unsafe impl MiniObject for TocEntryRef {
    type GstType = ffi::GstTocEntry;
}

impl GstRc<TocEntryRef> {
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
            let mut start = mem::uninitialized();
            let mut stop = mem::uninitialized();

            if from_glib(ffi::gst_toc_entry_get_start_stop_times(
                self.as_ptr(),
                &mut start,
                &mut stop,
            )) {
                Some((start, stop))
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
            let mut loop_type = mem::uninitialized();
            let mut repeat_count = mem::uninitialized();
            if from_glib(ffi::gst_toc_entry_get_loop(
                self.as_ptr(),
                &mut loop_type,
                &mut repeat_count,
            )) {
                Some((from_glib(loop_type), repeat_count))
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

impl glib::types::StaticType for TocEntryRef {
    fn static_type() -> glib::types::Type {
        unsafe { from_glib(ffi::gst_toc_entry_get_type()) }
    }
}

impl ToOwned for TocEntryRef {
    type Owned = GstRc<TocEntryRef>;

    fn to_owned(&self) -> GstRc<TocEntryRef> {
        #[cfg_attr(feature = "cargo-clippy", allow(cast_ptr_alignment))]
        unsafe {
            from_glib_full(ffi::gst_mini_object_copy(self.as_ptr() as *const _) as *mut _)
        }
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

unsafe impl Sync for TocEntryRef {}
unsafe impl Send for TocEntryRef {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        ::init().unwrap();

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
