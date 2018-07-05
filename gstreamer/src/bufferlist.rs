// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ffi;
use glib;
use glib::translate::{from_glib, from_glib_full};
use glib::StaticType;
use std::fmt;

use miniobject::*;
use Buffer;
use BufferRef;

pub type BufferList = GstRc<BufferListRef>;
pub struct BufferListRef(ffi::GstBufferList);

unsafe impl MiniObject for BufferListRef {
    type GstType = ffi::GstBufferList;
}

impl GstRc<BufferListRef> {
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_buffer_list_new()) }
    }

    pub fn new_sized(size: usize) -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_buffer_list_new_sized(size as u32)) }
    }
}

impl BufferListRef {
    pub fn insert(&mut self, idx: i32, buffer: Buffer) {
        unsafe {
            ffi::gst_buffer_list_insert(self.as_mut_ptr(), idx, buffer.into_ptr());
        }
    }

    pub fn add(&mut self, buffer: Buffer) {
        self.insert(-1, buffer);
    }

    pub fn copy_deep(&self) -> BufferList {
        unsafe { from_glib_full(ffi::gst_buffer_list_copy_deep(self.as_ptr())) }
    }

    pub fn remove(&mut self, idx: u32, len: u32) {
        unsafe { ffi::gst_buffer_list_remove(self.as_mut_ptr(), idx, len) }
    }

    pub fn get(&self, idx: u32) -> Option<&BufferRef> {
        unsafe {
            let ptr = ffi::gst_buffer_list_get(self.as_mut_ptr(), idx);
            if ptr.is_null() {
                None
            } else {
                Some(BufferRef::from_ptr(ptr))
            }
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn get_writable(&mut self, idx: u32) -> Option<&mut BufferRef> {
        unsafe {
            let ptr = ffi::gst_buffer_list_get_writable(self.as_mut_ptr(), idx);
            if ptr.is_null() {
                None
            } else {
                Some(BufferRef::from_mut_ptr(ptr))
            }
        }
    }

    pub fn len(&self) -> usize {
        unsafe { ffi::gst_buffer_list_length(self.as_mut_ptr()) as usize }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn calculate_size(&self) -> usize {
        unsafe { ffi::gst_buffer_list_calculate_size(self.as_mut_ptr()) as usize }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }
}

impl Default for GstRc<BufferListRef> {
    fn default() -> Self {
        Self::new()
    }
}

impl ToOwned for BufferListRef {
    type Owned = GstRc<BufferListRef>;

    fn to_owned(&self) -> GstRc<BufferListRef> {
        #[cfg_attr(feature = "cargo-clippy", allow(cast_ptr_alignment))]
        unsafe {
            from_glib_full(ffi::gst_mini_object_copy(self.as_ptr() as *const _) as *mut _)
        }
    }
}

impl fmt::Debug for BufferListRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let size = self.iter().map(|b| b.get_size()).sum::<usize>();
        let (pts, dts) = self
            .get(0)
            .map(|b| (b.get_pts(), b.get_dts()))
            .unwrap_or((::ClockTime::none(), ::ClockTime::none()));

        f.debug_struct("BufferList")
            .field("ptr", unsafe { &self.as_ptr() })
            .field("buffers", &self.len())
            .field("pts", &pts.to_string())
            .field("dts", &dts.to_string())
            .field("size", &size)
            .finish()
    }
}

impl StaticType for BufferListRef {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gst_buffer_list_get_type()) }
    }
}

unsafe impl Sync for BufferListRef {}
unsafe impl Send for BufferListRef {}

pub struct Iter<'a> {
    list: &'a BufferListRef,
    idx: u32,
    size: u32,
}

impl<'a> Iter<'a> {
    fn new(list: &'a BufferListRef) -> Iter<'a> {
        skip_assert_initialized!();
        Iter {
            list,
            idx: 0,
            size: list.len() as u32,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a BufferRef;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.size {
            return None;
        }

        let item = self.list.get(self.idx);
        self.idx += 1;

        item
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.idx == self.size {
            return (0, Some(0));
        }

        let remaining = (self.size - self.idx) as usize;

        (remaining, Some(remaining))
    }
}

impl<'a> DoubleEndedIterator for Iter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.idx == self.size {
            return None;
        }

        self.size -= 1;
        self.list.get(self.size)
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {}

#[cfg(feature = "ser_de")]
pub(crate) mod serde {
    use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};
    use serde::ser::{Serialize, Serializer, SerializeSeq};

    use std::fmt;

    use Buffer;
    use BufferList;
    use BufferListRef;

    impl Serialize for BufferListRef {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut iter = self.iter();
            let (remaining, _) = iter.size_hint();
            if remaining > 0 {
                let mut seq = serializer.serialize_seq(Some(remaining))?;
                while let Some(ref buffer) = iter.next() {
                    seq.serialize_element(buffer)?;
                }
                seq.end()
            } else {
                let seq = serializer.serialize_seq(None)?;
                seq.end()
            }
        }
    }

    impl Serialize for BufferList {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            self.as_ref().serialize(serializer)
        }
    }

    struct BufferListVisitor;
    impl<'de> Visitor<'de> for BufferListVisitor {
        type Value = BufferList;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a sequence of Buffers")
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let mut buffer_list = BufferList::new();
            {
                let buffer_list = buffer_list.get_mut().unwrap();
                while let Some(buffer) = seq.next_element::<Buffer>()? {
                    buffer_list.add(buffer);
                }
            }
            Ok(buffer_list)
        }
    }

    impl<'de> Deserialize<'de> for BufferList {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            deserializer.deserialize_seq(BufferListVisitor)
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "ser_de")]
    #[test]
    fn test_serialize() {
        extern crate ron;

        use Buffer;
        use BufferList;

        ::init().unwrap();

        let mut buffer_list = BufferList::new();
        {
            let buffer_list = buffer_list.get_mut().unwrap();

            let mut buffer = Buffer::from_slice(vec![1, 2, 3, 4]).unwrap();
            {
                let buffer = buffer.get_mut().unwrap();
                buffer.set_pts(1.into());
                buffer.set_offset(0);
                buffer.set_offset_end(4);
                buffer.set_duration(4.into());
            }
            buffer_list.add(buffer);

            let mut buffer = Buffer::from_slice(vec![5, 6]).unwrap();
            {
                let buffer = buffer.get_mut().unwrap();
                buffer.set_pts(5.into());
                buffer.set_offset(4);
                buffer.set_offset_end(6);
                buffer.set_duration(2.into());
            }
            buffer_list.add(buffer);
        }

        // don't use newlines
        let mut pretty_config = ron::ser::PrettyConfig::default();
        pretty_config.new_line = "".to_string();

        let res = ron::ser::to_string_pretty(&buffer_list, pretty_config);
        assert_eq!(
            Ok(
                concat!(
                    "[",
                    "    (",
                    "        pts: Some(1),",
                    "        dts: None,",
                    "        duration: Some(4),",
                    "        offset: 0,",
                    "        offset_end: 4,",
                    "        flags: (",
                    "            bits: 0,",
                    "        ),",
                    "        buffer: \"AQIDBA==\",",
                    "    ),",
                    "    (",
                    "        pts: Some(5),",
                    "        dts: None,",
                    "        duration: Some(2),",
                    "        offset: 4,",
                    "        offset_end: 6,",
                    "        flags: (",
                    "            bits: 0,",
                    "        ),",
                    "        buffer: \"BQY=\",",
                    "    ),",
                    "]"
                )
                    .to_owned()
            ),
            res,
        );
    }

    #[cfg(feature = "ser_de")]
    #[test]
    fn test_deserialize() {
        extern crate ron;

        use BufferList;

        ::init().unwrap();

        let buffer_list_ron = r#"
            [
                (
                    pts: Some(1),
                    dts: None,
                    duration: Some(4),
                    offset: 0,
                    offset_end: 4,
                    flags: (
                        bits: 0,
                    ),
                    buffer: "AQIDBA==",
                ),
                (
                    pts: Some(5),
                    dts: None,
                    duration: Some(2),
                    offset: 4,
                    offset_end: 6,
                    flags: (
                        bits: 0,
                    ),
                    buffer: "BQY=",
                ),
            ]
        "#;

        let buffer_list: BufferList = ron::de::from_str(buffer_list_ron).unwrap();
        let mut iter = buffer_list.iter();
        let buffer = iter.next().unwrap();
        assert_eq!(buffer.get_pts(), 1.into());
        assert_eq!(buffer.get_dts(), None.into());
        {
            let data = buffer.map_readable().unwrap();
            assert_eq!(data.as_slice(), vec![1, 2, 3, 4].as_slice());
        }

        let buffer = iter.next().unwrap();
        assert_eq!(buffer.get_pts(), 5.into());
        {
            let data = buffer.map_readable().unwrap();
            assert_eq!(data.as_slice(), vec![5, 6].as_slice());
        }
    }
}
