// Copyright (C) 2018 François Laignel <fengalin@free.fr>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, SerializeStruct, Serializer};

use toc::*;
use TagList;
use TocEntryType;
use TocLoopType;
use TocScope;

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
        toc_entry.serialize_field("loop", &self.get_loop())?;
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
        skip_assert_initialized!();
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
        skip_assert_initialized!();
        TocDe::deserialize(deserializer).map(|toc_de| toc_de.into())
    }
}

#[derive(Deserialize)]
struct TocEntryDe {
    entry_type: TocEntryType,
    uid: String,
    start_stop: Option<(i64, i64)>,
    tags: Option<TagList>,
    #[serde(rename = "loop")]
    loop_: Option<(TocLoopType, i32)>,
    sub_entries: Vec<TocEntry>,
}

impl From<TocEntryDe> for TocEntry {
    fn from(mut toc_entry_de: TocEntryDe) -> Self {
        skip_assert_initialized!();
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
        skip_assert_initialized!();
        TocEntryDe::deserialize(deserializer).map(|toc_entry_de| toc_entry_de.into())
    }
}

#[cfg(test)]
mod tests {
    extern crate ron;

    use tags::Title;
    use toc::*;
    use TagList;
    use TagMergeMode;
    use TocEntryType;
    use TocScope;

    #[test]
    fn test_serialize() {
        ::init().unwrap();

        let mut toc = Toc::new(TocScope::Global);
        {
            let toc = toc.get_mut().unwrap();
            let mut tags = TagList::new();
            tags.get_mut()
                .unwrap()
                .add::<Title>(&"toc", TagMergeMode::Append);
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
                        tags.get_mut()
                            .unwrap()
                            .add::<Title>(&"chapter 1.1", TagMergeMode::Append);
                        toc_chap_1_1.set_tags(tags);
                    }
                    toc_chap_1.append_sub_entry(toc_chap_1_1);

                    let mut toc_chap_1_2 = TocEntry::new(TocEntryType::Chapter, "chapter1.2");
                    {
                        let toc_chap_1_2 = toc_chap_1_2.get_mut().unwrap();
                        toc_chap_1_2.set_start_stop_times(4, 10);
                        let mut tags = TagList::new();
                        tags.get_mut()
                            .unwrap()
                            .add::<Title>(&"chapter 1.2", TagMergeMode::Append);
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
                    tags.get_mut()
                        .unwrap()
                        .add::<Title>(&"chapter 2", TagMergeMode::Append);
                    toc_chap_2.set_tags(tags);
                }
                toc_edition.append_sub_entry(toc_chap_2);
            }
            toc.append_entry(toc_edition);
        }

        let mut pretty_config = ron::ser::PrettyConfig::default();
        pretty_config.new_line = "".to_string();

        let res = ron::ser::to_string_pretty(&toc, pretty_config);
        assert_eq!(
            Ok(concat!(
                "(",
                "    scope: Global,",
                "    tags: Some((",
                "        scope: Stream,",
                "        tags: [",
                "            (\"title\", [",
                "                \"toc\",",
                "            ]),",
                "        ],",
                "    )),",
                "    entries: [",
                "        (",
                "            entry_type: Edition,",
                "            uid: \"edition\",",
                "            start_stop: Some((0, 15)),",
                "            tags: None,",
                "            loop: Some((None, 0)),",
                "            sub_entries: [",
                "                (",
                "                    entry_type: Chapter,",
                "                    uid: \"chapter1\",",
                "                    start_stop: Some((0, 10)),",
                "                    tags: None,",
                "                    loop: Some((None, 0)),",
                "                    sub_entries: [",
                "                        (",
                "                            entry_type: Chapter,",
                "                            uid: \"chapter1.1\",",
                "                            start_stop: Some((0, 4)),",
                "                            tags: Some((",
                "                                scope: Stream,",
                "                                tags: [",
                "                                    (\"title\", [",
                "                                        \"chapter 1.1\",",
                "                                    ]),",
                "                                ],",
                "                            )),",
                "                            loop: Some((None, 0)),",
                "                            sub_entries: [],",
                "                        ),",
                "                        (",
                "                            entry_type: Chapter,",
                "                            uid: \"chapter1.2\",",
                "                            start_stop: Some((4, 10)),",
                "                            tags: Some((",
                "                                scope: Stream,",
                "                                tags: [",
                "                                    (\"title\", [",
                "                                        \"chapter 1.2\",",
                "                                    ]),",
                "                                ],",
                "                            )),",
                "                            loop: Some((None, 0)),",
                "                            sub_entries: [],",
                "                        ),",
                "                    ],",
                "                ),",
                "                (",
                "                    entry_type: Chapter,",
                "                    uid: \"chapter2\",",
                "                    start_stop: Some((10, 15)),",
                "                    tags: Some((",
                "                        scope: Stream,",
                "                        tags: [",
                "                            (\"title\", [",
                "                                \"chapter 2\",",
                "                            ]),",
                "                        ],",
                "                    )),",
                "                    loop: Some((None, 0)),",
                "                    sub_entries: [],",
                "                ),",
                "            ],",
                "        ),",
                "    ],",
                ")",
            )
            .to_owned()),
            res,
        );
    }

    #[allow(clippy::cognitive_complexity)]
    #[test]
    fn test_deserialize() {
        use tags::Title;

        ::init().unwrap();

        let toc_ron = r#"
            (
                scope: Global,
                tags: Some((
                    scope: Stream,
                    tags: [
                        ("title", ["toc"]),
                    ],
                )),
                entries: [
                    (
                        entry_type: Edition,
                        uid: "edition",
                        start_stop: Some((0, 15)),
                        tags: None,
                        loop: Some((None, 0)),
                        sub_entries: [
                            (
                                entry_type: Chapter,
                                uid: "chapter1",
                                start_stop: Some((0, 10)),
                                tags: None,
                                loop: Some((None, 0)),
                                sub_entries: [
                                    (
                                        entry_type: Chapter,
                                        uid: "chapter1.1",
                                        start_stop: Some((0, 4)),
                                        tags: Some((
                                            scope: Stream,
                                            tags: [
                                                ("title", ["chapter 1.1"]),
                                            ],
                                        )),
                                        loop: Some((None, 0)),
                                        sub_entries: [
                                        ],
                                    ),
                                    (
                                        entry_type: Chapter,
                                        uid: "chapter1.2",
                                        start_stop: Some((4, 10)),
                                        tags: Some((
                                            scope: Stream,
                                            tags: [
                                                ("title", ["chapter 1.2"]),
                                            ],
                                        )),
                                        loop: Some((None, 0)),
                                        sub_entries: [
                                        ],
                                    ),
                                ],
                            ),
                            (
                                entry_type: Chapter,
                                uid: "chapter2",
                                start_stop: Some((10, 15)),
                                tags: Some((
                                    scope: Stream,
                                    tags: [
                                        ("title", ["chapter 2"]),
                                    ],
                                )),
                                loop: Some((None, 0)),
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
        assert_eq!(
            Some("chapter 1.1"),
            tags.get_index::<Title>(0).unwrap().get()
        );
        assert_eq!(0, chapter1_1.get_sub_entries().len());

        let chapter1_2 = &chap1_sub_entries[1];
        assert_eq!(TocEntryType::Chapter, chapter1_2.get_entry_type());
        assert_eq!("chapter1.2", chapter1_2.get_uid());
        assert_eq!(Some((4, 10)), chapter1_2.get_start_stop_times());
        let tags = chapter1_2.get_tags().unwrap();
        assert_eq!(
            Some("chapter 1.2"),
            tags.get_index::<Title>(0).unwrap().get()
        );
        assert_eq!(0, chapter1_2.get_sub_entries().len());

        let chapter2 = &sub_entries[1];
        assert_eq!(TocEntryType::Chapter, chapter2.get_entry_type());
        assert_eq!("chapter2", chapter2.get_uid());
        let tags = chapter2.get_tags().unwrap();
        assert_eq!(Some("chapter 2"), tags.get_index::<Title>(0).unwrap().get());
        assert_eq!(Some((10, 15)), chapter2.get_start_stop_times());
        assert_eq!(0, chapter2.get_sub_entries().len());
    }

    #[allow(clippy::cognitive_complexity)]
    #[test]
    fn test_serde_roundtrip() {
        ::init().unwrap();

        let mut toc = Toc::new(TocScope::Global);
        {
            let toc = toc.get_mut().unwrap();
            let mut tags = TagList::new();
            tags.get_mut()
                .unwrap()
                .add::<Title>(&"toc", TagMergeMode::Append);
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
                        tags.get_mut()
                            .unwrap()
                            .add::<Title>(&"chapter 1.1", TagMergeMode::Append);
                        toc_chap_1_1.set_tags(tags);
                    }
                    toc_chap_1.append_sub_entry(toc_chap_1_1);

                    let mut toc_chap_1_2 = TocEntry::new(TocEntryType::Chapter, "chapter1.2");
                    {
                        let toc_chap_1_2 = toc_chap_1_2.get_mut().unwrap();
                        toc_chap_1_2.set_start_stop_times(4, 10);
                        let mut tags = TagList::new();
                        tags.get_mut()
                            .unwrap()
                            .add::<Title>(&"chapter 1.2", TagMergeMode::Append);
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
                    tags.get_mut()
                        .unwrap()
                        .add::<Title>(&"chapter 2", TagMergeMode::Append);
                    toc_chap_2.set_tags(tags);
                }
                toc_edition.append_sub_entry(toc_chap_2);
            }
            toc.append_entry(toc_edition);
        }
        let toc_ser = ron::ser::to_string(&toc).unwrap();

        let toc_de: Toc = ron::de::from_str(toc_ser.as_str()).unwrap();
        assert_eq!(toc_de.get_scope(), toc.get_scope());

        let entries_de = toc_de.get_entries();
        let entries = toc.get_entries();
        assert_eq!(entries_de.len(), entries.len());

        let edition_de = &entries_de[0];
        let edition = &entries[0];
        assert_eq!(edition_de.get_entry_type(), edition.get_entry_type());
        assert_eq!(edition_de.get_uid(), edition.get_uid());
        assert_eq!(edition_de.get_tags(), edition.get_tags());
        assert_eq!(
            edition_de.get_start_stop_times(),
            edition.get_start_stop_times()
        );

        let sub_entries_de = edition_de.get_sub_entries();
        let sub_entries = edition.get_sub_entries();
        assert_eq!(sub_entries_de.len(), sub_entries.len());

        let chapter1_de = &sub_entries_de[0];
        let chapter1 = &sub_entries[0];
        assert_eq!(chapter1_de.get_entry_type(), chapter1.get_entry_type());
        assert_eq!(chapter1_de.get_uid(), chapter1.get_uid());
        assert_eq!(
            chapter1_de.get_tags().is_none(),
            chapter1.get_tags().is_none()
        );
        assert_eq!(
            chapter1_de.get_start_stop_times(),
            chapter1.get_start_stop_times()
        );

        let chap1_sub_entries_de = chapter1_de.get_sub_entries();
        let chap1_sub_entries = chapter1.get_sub_entries();
        assert_eq!(sub_entries_de.len(), sub_entries.len());

        let chapter1_1_de = &chap1_sub_entries_de[0];
        let chapter1_1 = &chap1_sub_entries[0];
        assert_eq!(chapter1_1_de.get_entry_type(), chapter1_1.get_entry_type());
        assert_eq!(chapter1_1_de.get_uid(), chapter1_1.get_uid());
        assert_eq!(
            chapter1_1_de.get_start_stop_times(),
            chapter1_1.get_start_stop_times()
        );
        let tags_de = chapter1_1_de.get_tags().unwrap();
        let tags = chapter1_1.get_tags().unwrap();
        assert_eq!(
            tags_de.get_index::<Title>(0).unwrap().get(),
            tags.get_index::<Title>(0).unwrap().get()
        );
        assert_eq!(
            chapter1_1_de.get_sub_entries().len(),
            chapter1_1.get_sub_entries().len()
        );

        let chapter1_2_de = &chap1_sub_entries_de[1];
        let chapter1_2 = &chap1_sub_entries[1];
        assert_eq!(chapter1_2_de.get_entry_type(), chapter1_2.get_entry_type());
        assert_eq!(chapter1_2_de.get_uid(), chapter1_2.get_uid());
        assert_eq!(
            chapter1_2_de.get_start_stop_times(),
            chapter1_2.get_start_stop_times()
        );
        let tags_de = chapter1_2_de.get_tags().unwrap();
        let tags = chapter1_2.get_tags().unwrap();
        assert_eq!(
            tags_de.get_index::<Title>(0).unwrap().get(),
            tags.get_index::<Title>(0).unwrap().get()
        );
        assert_eq!(
            chapter1_2_de.get_sub_entries().len(),
            chapter1_2.get_sub_entries().len()
        );

        let chapter2_de = &sub_entries_de[1];
        let chapter2 = &sub_entries[1];
        assert_eq!(chapter2_de.get_entry_type(), chapter2.get_entry_type());
        assert_eq!(chapter2_de.get_uid(), chapter2.get_uid());
        let tags_de = chapter2_de.get_tags().unwrap();
        let tags = chapter2.get_tags().unwrap();
        assert_eq!(
            tags_de.get_index::<Title>(0).unwrap().get(),
            tags.get_index::<Title>(0).unwrap().get()
        );
        assert_eq!(
            chapter2_de.get_start_stop_times(),
            chapter2.get_start_stop_times()
        );
        assert_eq!(
            chapter2_de.get_sub_entries().len(),
            chapter2.get_sub_entries().len()
        );
    }
}
