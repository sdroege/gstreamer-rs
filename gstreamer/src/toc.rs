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

#[cfg(feature = "ser_de")]
mod serde {
    use serde::de::{Deserialize, Deserializer};
    use serde::ser::{Serialize, Serializer, SerializeStruct};

    use tags::*;
    use super::*;

    impl Serialize for TocRef {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut toc = serializer.serialize_struct("Toc", 3)?;
            toc.serialize_field("scope", &self.get_scope())?;
            toc.serialize_field("tags", &self.get_tags())?;
            toc.serialize_field("entries", &self.get_entries())?;
            toc.end()
        }
    }

    impl Serialize for Toc {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            self.as_ref().serialize(serializer)
        }
    }

    impl Serialize for TocEntryRef {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut toc_entry = serializer.serialize_struct("TocEntry", 6)?;
            toc_entry.serialize_field("entry_type", &self.get_entry_type())?;
            toc_entry.serialize_field("uid", &self.get_uid())?;
            toc_entry.serialize_field("start_stop", &self.get_start_stop_times())?;
            toc_entry.serialize_field("tags", &self.get_tags())?;
            toc_entry.serialize_field("loop_", &self.get_loop())?;
            toc_entry.serialize_field("sub_entries", &self.get_sub_entries())?;
            toc_entry.end()
        }
    }

    impl Serialize for TocEntry {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            self.as_ref().serialize(serializer)
        }
    }

    #[derive(Deserialize)]
    struct TocDe {
        scope: TocScope,
        tags: Option<TagList>,
        entries: Vec<TocEntry>,
    }

    impl From<TocDe> for Toc {
        fn from(mut toc_de: TocDe) -> Self {
            let mut toc = Toc::new(toc_de.scope);
            {
                let toc = toc.get_mut().unwrap();
                if let Some(tags) = toc_de.tags.take() {
                    toc.set_tags(tags);
                }
                let entry_iter = toc_de.entries.drain(..);
                for entry in entry_iter {
                    toc.append_entry(entry);
                }
            }
            toc
        }
    }

    impl<'de> Deserialize<'de> for Toc {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            TocDe::deserialize(deserializer)
                .and_then(|toc_de| Ok(toc_de.into()))
        }
    }

    #[derive(Deserialize)]
    struct TocEntryDe {
        entry_type: TocEntryType,
        uid: String,
        start_stop: Option<(i64, i64)>,
        tags: Option<TagList>,
        loop_: Option<(TocLoopType, i32)>,
        sub_entries: Vec<TocEntry>,
    }

    impl From<TocEntryDe> for TocEntry {
        fn from(mut toc_entry_de: TocEntryDe) -> Self {
            let mut toc_entry = TocEntry::new(toc_entry_de.entry_type, toc_entry_de.uid.as_str());
            {
                let toc_entry = toc_entry.get_mut().unwrap();
                if let Some(start_stop) = toc_entry_de.start_stop.take() {
                    toc_entry.set_start_stop_times(start_stop.0, start_stop.1);
                }
                if let Some(tags) = toc_entry_de.tags.take() {
                    toc_entry.set_tags(tags);
                }
                if let Some(loop_) = toc_entry_de.loop_.take() {
                    toc_entry.set_loop(loop_.0, loop_.1);
                }

                let entry_iter = toc_entry_de.sub_entries.drain(..);
                for sub_entries in entry_iter {
                    toc_entry.append_sub_entry(sub_entries);
                }
            }
            toc_entry
        }
    }

    impl<'de> Deserialize<'de> for TocEntry {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            TocEntryDe::deserialize(deserializer)
                .and_then(|toc_entry_de| Ok(toc_entry_de.into()))
        }
    }
}

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

    #[cfg(feature = "ser_de")]
    #[test]
    fn test_serialize() {
        extern crate ron;

        use tags::Title;
        use super::*;

        ::init().unwrap();

        let mut toc = Toc::new(TocScope::Global);
        {
            let toc = toc.get_mut().unwrap();
            let mut tags = TagList::new();
            tags.get_mut().unwrap().add::<Title>(&"toc", TagMergeMode::Append);
            toc.set_tags(tags);

            let mut toc_edition = TocEntry::new(TocEntryType::Edition, "edition");
            {
                let toc_edition = toc_edition.get_mut().unwrap();
                toc_edition.set_start_stop_times(0, 15);

                let mut toc_chap_1 = TocEntry::new(TocEntryType::Chapter, "chapter1");
                {
                    let toc_chap_1 = toc_chap_1.get_mut().unwrap();
                    toc_chap_1.set_start_stop_times(0, 10);
                    let mut toc_chap_1_1 = TocEntry::new(TocEntryType::Chapter, "chapter1.1");
                    {
                        let toc_chap_1_1 = toc_chap_1_1.get_mut().unwrap();
                        toc_chap_1_1.set_start_stop_times(0, 4);
                        let mut tags = TagList::new();
                        tags.get_mut().unwrap().add::<Title>(&"chapter 1.1", TagMergeMode::Append);
                        toc_chap_1_1.set_tags(tags);
                    }
                    toc_chap_1.append_sub_entry(toc_chap_1_1);

                    let mut toc_chap_1_2 = TocEntry::new(TocEntryType::Chapter, "chapter1.2");
                    {
                        let toc_chap_1_2 = toc_chap_1_2.get_mut().unwrap();
                        toc_chap_1_2.set_start_stop_times(4, 10);
                        let mut tags = TagList::new();
                        tags.get_mut().unwrap().add::<Title>(&"chapter 1.2", TagMergeMode::Append);
                        toc_chap_1_2.set_tags(tags);
                    }
                    toc_chap_1.append_sub_entry(toc_chap_1_2);
                }
                toc_edition.append_sub_entry(toc_chap_1);

                let mut toc_chap_2 = TocEntry::new(TocEntryType::Chapter, "chapter2");
                {
                    let toc_chap_2 = toc_chap_2.get_mut().unwrap();
                    toc_chap_2.set_start_stop_times(10, 15);
                    let mut tags = TagList::new();
                    tags.get_mut().unwrap().add::<Title>(&"chapter 2", TagMergeMode::Append);
                    toc_chap_2.set_tags(tags);
                }
                toc_edition.append_sub_entry(toc_chap_2);
            }
            toc.append_entry(toc_edition);
        }

        // don't use newlines
        let mut pretty_config = ron::ser::PrettyConfig::default();
        pretty_config.new_line = "".to_string();

        let res = ron::ser::to_string_pretty(&toc, pretty_config);
        assert_eq!(
            Ok(
                concat!(
                    "(",
                    "    scope: Global,",
                    "    tags: Some([",
                    "        (\"title\", \"toc\"),",
                    "    ]),",
                    "    entries: [",
                    "        (",
                    "            entry_type: Edition,",
                    "            uid: \"edition\",",
                    "            start_stop: Some((0, 15)),",
                    "            tags: None,",
                    "            loop_: Some((None, 0)),",
                    "            sub_entries: [",
                    "                (",
                    "                    entry_type: Chapter,",
                    "                    uid: \"chapter1\",",
                    "                    start_stop: Some((0, 10)),",
                    "                    tags: None,",
                    "                    loop_: Some((None, 0)),",
                    "                    sub_entries: [",
                    "                        (",
                    "                            entry_type: Chapter,",
                    "                            uid: \"chapter1.1\",",
                    "                            start_stop: Some((0, 4)),",
                    "                            tags: Some([",
                    "                                (\"title\", \"chapter 1.1\"),",
                    "                            ]),",
                    "                            loop_: Some((None, 0)),",
                    "                            sub_entries: [",
                    "                            ],",
                    "                        ),",
                    "                        (",
                    "                            entry_type: Chapter,",
                    "                            uid: \"chapter1.2\",",
                    "                            start_stop: Some((4, 10)),",
                    "                            tags: Some([",
                    "                                (\"title\", \"chapter 1.2\"),",
                    "                            ]),",
                    "                            loop_: Some((None, 0)),",
                    "                            sub_entries: [",
                    "                            ],",
                    "                        ),",
                    "                    ],",
                    "                ),",
                    "                (",
                    "                    entry_type: Chapter,",
                    "                    uid: \"chapter2\",",
                    "                    start_stop: Some((10, 15)),",
                    "                    tags: Some([",
                    "                        (\"title\", \"chapter 2\"),",
                    "                    ]),",
                    "                    loop_: Some((None, 0)),",
                    "                    sub_entries: [",
                    "                    ],",
                    "                ),",
                    "            ],",
                    "        ),",
                    "    ],",
                    ")",
                ).to_owned()
            ),
            res,
        );
    }

    #[cfg(feature = "ser_de")]
    #[test]
    fn test_deserialize() {
        extern crate ron;

        use tags::Title;

        ::init().unwrap();

        let toc_ron = r#"
            (
                scope: Global,
                tags: Some([
                    ("title", "toc"),
                ]),
                entries: [
                    (
                        entry_type: Edition,
                        uid: "edition",
                        start_stop: Some((0, 15)),
                        tags: None,
                        loop_: Some((None, 0)),
                        sub_entries: [
                            (
                                entry_type: Chapter,
                                uid: "chapter1",
                                start_stop: Some((0, 10)),
                                tags: None,
                                loop_: Some((None, 0)),
                                sub_entries: [
                                    (
                                        entry_type: Chapter,
                                        uid: "chapter1.1",
                                        start_stop: Some((0, 4)),
                                        tags: Some([
                                            ("title", "chapter 1.1"),
                                        ]),
                                        loop_: Some((None, 0)),
                                        sub_entries: [
                                        ],
                                    ),
                                    (
                                        entry_type: Chapter,
                                        uid: "chapter1.2",
                                        start_stop: Some((4, 10)),
                                        tags: Some([
                                            ("title", "chapter 1.2"),
                                        ]),
                                        loop_: Some((None, 0)),
                                        sub_entries: [
                                        ],
                                    ),
                                ],
                            ),
                            (
                                entry_type: Chapter,
                                uid: "chapter2",
                                start_stop: Some((10, 15)),
                                tags: Some([
                                    ("title", "chapter 2"),
                                ]),
                                loop_: Some((None, 0)),
                                sub_entries: [
                                ],
                            ),
                        ],
                    ),
                ],
            )
        "#;
        let toc: Toc = ron::de::from_str(toc_ron).unwrap();
        assert_eq!(toc.get_scope(), TocScope::Global);

        let entries = toc.get_entries();
        assert_eq!(1, entries.len());

        let edition = &entries[0];
        assert_eq!(TocEntryType::Edition, edition.get_entry_type());
        assert_eq!("edition", edition.get_uid());
        assert!(edition.get_tags().is_none());
        assert_eq!(Some((0, 15)), edition.get_start_stop_times());

        let sub_entries = edition.get_sub_entries();
        assert_eq!(2, sub_entries.len());

        let chapter1 = &sub_entries[0];
        assert_eq!(TocEntryType::Chapter, chapter1.get_entry_type());
        assert_eq!("chapter1", chapter1.get_uid());
        assert!(chapter1.get_tags().is_none());
        assert_eq!(Some((0, 10)), chapter1.get_start_stop_times());

        let chap1_sub_entries = chapter1.get_sub_entries();
        assert_eq!(2, sub_entries.len());

        let chapter1_1 = &chap1_sub_entries[0];
        assert_eq!(TocEntryType::Chapter, chapter1_1.get_entry_type());
        assert_eq!("chapter1.1", chapter1_1.get_uid());
        assert_eq!(Some((0, 4)), chapter1_1.get_start_stop_times());
        let tags = chapter1_1.get_tags().unwrap();
        assert_eq!(Some("chapter 1.1"), tags.get_index::<Title>(0).unwrap().get());
        assert_eq!(0, chapter1_1.get_sub_entries().len());

        let chapter1_2 = &chap1_sub_entries[1];
        assert_eq!(TocEntryType::Chapter, chapter1_2.get_entry_type());
        assert_eq!("chapter1.2", chapter1_2.get_uid());
        assert_eq!(Some((4, 10)), chapter1_2.get_start_stop_times());
        let tags = chapter1_2.get_tags().unwrap();
        assert_eq!(Some("chapter 1.2"), tags.get_index::<Title>(0).unwrap().get());
        assert_eq!(0, chapter1_2.get_sub_entries().len());

        let chapter2 = &sub_entries[1];
        assert_eq!(TocEntryType::Chapter, chapter2.get_entry_type());
        assert_eq!("chapter2", chapter2.get_uid());
        let tags = chapter2.get_tags().unwrap();
        assert_eq!(Some("chapter 2"), tags.get_index::<Title>(0).unwrap().get());
        assert_eq!(Some((10, 15)), chapter2.get_start_stop_times());
        assert_eq!(0, chapter2.get_sub_entries().len());
    }
}
