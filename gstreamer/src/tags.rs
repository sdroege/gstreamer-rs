// Copyright (C) 2016-2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use std::mem;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;

use glib;
use ffi;
use glib::StaticType;
use glib::value::{Value, TypedValue, FromValueOptional, SetValue, ToValue};
use glib::translate::{from_glib, from_glib_none, from_glib_full, ToGlib, ToGlibPtr, ToGlibPtrMut};

use miniobject::*;

use TagMergeMode;

pub trait Tag<'a> {
    type TagType: FromValueOptional<'a> + SetValue;
    fn tag_name() -> &'static str;
}

macro_rules! impl_tag(
    ($name:ident, $t:ty, $tag:expr) => {
        pub struct $name;
        impl<'a> Tag<'a> for $name {
            type TagType = $t;
            fn tag_name() -> &'static str {
                $tag
            }
        }
    };
);

impl_tag!(Title, &'a str, "title");
impl_tag!(Album, &'a str, "album");
impl_tag!(Artist, &'a str, "artist");
impl_tag!(Encoder, &'a str, "encoder");
impl_tag!(AudioCodec, &'a str, "audio-codec");
impl_tag!(VideoCodec, &'a str, "video-codec");
impl_tag!(SubtitleCodec, &'a str, "subtitle-codec");
impl_tag!(ContainerFormat, &'a str, "container-format");
// TODO: Should ideally enforce this to be ISO-639
impl_tag!(LanguageCode, &'a str, "language-code");
impl_tag!(Duration, u64, "duration");
impl_tag!(NominalBitrate, u32, "nominal-bitrate");

pub type TagList = GstRc<TagListRef>;
pub struct TagListRef(ffi::GstTagList);

unsafe impl MiniObject for TagListRef {
    type GstType = ffi::GstTagList;
}

impl GstRc<TagListRef> {
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_tag_list_new_empty()) }
    }
}

impl TagListRef {
    pub fn add<'a, T: Tag<'a>>(&mut self, value: T::TagType, mode: TagMergeMode)
    where
        T::TagType: ToValue,
    {
        unsafe {
            let v = value.to_value();

            ffi::gst_tag_list_add_value(
                self.as_mut_ptr(),
                mode.to_glib(),
                T::tag_name().to_glib_none().0,
                v.to_glib_none().0,
            );
        }
    }

    pub fn get<'a, T: Tag<'a>>(&self) -> Option<TypedValue<T::TagType>> {
        unsafe {
            let mut value: Value = mem::zeroed();

            let found: bool = from_glib(ffi::gst_tag_list_copy_value(
                value.to_glib_none_mut().0,
                self.as_ptr(),
                T::tag_name().to_glib_none().0,
            ));

            if !found {
                return None;
            }

            value.downcast().ok()
        }
    }

    pub fn get_index<'a, T: Tag<'a>>(&'a self, idx: u32) -> Option<&'a TypedValue<T::TagType>> {
        unsafe {
            let value = ffi::gst_tag_list_get_value_index(
                self.as_ptr(),
                T::tag_name().to_glib_none().0,
                idx,
            );

            if value.is_null() || (*value).g_type != T::TagType::static_type().to_glib() {
                return None;
            }

            Some(&*(value as *const TypedValue<T::TagType>))
        }
    }

    pub fn get_size<'a, T: Tag<'a>>(&'a self) -> u32 {
        unsafe { ffi::gst_tag_list_get_tag_size(self.as_ptr(), T::tag_name().to_glib_none().0) }
    }

    pub fn iter_tag<'a, T: Tag<'a>>(&'a self) -> TagIterator<'a, T> {
        TagIterator::new(self)
    }

    pub fn to_string(&self) -> String {
        unsafe { from_glib_full(ffi::gst_tag_list_to_string(self.as_ptr())) }
    }
}

impl fmt::Debug for TagListRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl PartialEq for TagListRef {
    fn eq(&self, other: &TagListRef) -> bool {
        unsafe { from_glib(ffi::gst_tag_list_is_equal(self.as_ptr(), other.as_ptr())) }
    }
}

impl Eq for TagListRef {}

impl ToOwned for TagListRef {
    type Owned = GstRc<TagListRef>;

    fn to_owned(&self) -> GstRc<TagListRef> {
        unsafe { from_glib_none(self.as_ptr()) }
    }
}

unsafe impl Sync for TagListRef {}
unsafe impl Send for TagListRef {}

pub struct TagIterator<'a, T: Tag<'a>> {
    taglist: &'a TagListRef,
    idx: u32,
    size: u32,
    phantom: PhantomData<T>,
}

impl<'a, T: Tag<'a>> TagIterator<'a, T> {
    fn new(taglist: &'a TagListRef) -> TagIterator<'a, T> {
        TagIterator {
            taglist: taglist,
            idx: 0,
            size: taglist.get_size::<T>(),
            phantom: PhantomData,
        }
    }
}

impl<'a, T: Tag<'a>> Iterator for TagIterator<'a, T>
where
    <T as Tag<'a>>::TagType: 'a,
    T: 'a,
{
    type Item = &'a TypedValue<T::TagType>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.size {
            return None;
        }

        let item = self.taglist.get_index::<T>(self.idx);
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

impl<'a, T: Tag<'a>> DoubleEndedIterator for TagIterator<'a, T>
where
    <T as Tag<'a>>::TagType: 'a,
    T: 'a,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.idx == self.size {
            return None;
        }

        self.size -= 1;
        self.taglist.get_index::<T>(self.size)
    }
}

impl<'a, T: Tag<'a>> ExactSizeIterator for TagIterator<'a, T>
where
    <T as Tag<'a>>::TagType: 'a,
    T: 'a,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        ::init().unwrap();

        let mut tags = TagList::new();
        assert_eq!(tags.to_string(), "taglist;");
        {
            let tags = tags.get_mut().unwrap();
            tags.add::<Title>("some title".into(), TagMergeMode::Append);
            tags.add::<Duration>((1000u64 * 1000 * 1000 * 120).into(), TagMergeMode::Append);
        }
        assert_eq!(
            tags.to_string(),
            "taglist, title=(string)\"some\\ title\", duration=(guint64)120000000000;"
        );
    }

    #[test]
    fn test_get() {
        ::init().unwrap();

        let mut tags = TagList::new();
        assert_eq!(tags.to_string(), "taglist;");
        {
            let tags = tags.get_mut().unwrap();
            tags.add::<Title>("some title".into(), TagMergeMode::Append);
            tags.add::<Duration>((1000u64 * 1000 * 1000 * 120).into(), TagMergeMode::Append);
        }

        assert_eq!(tags.get::<Title>().unwrap().get(), Some("some title"));
        assert_eq!(
            tags.get::<Duration>().unwrap().get_some(),
            (1000u64 * 1000 * 1000 * 120)
        );
        assert_eq!(
            tags.get_index::<Title>(0).unwrap().get(),
            Some("some title")
        );
        assert_eq!(
            tags.get_index::<Duration>(0).unwrap().get_some(),
            (1000u64 * 1000 * 1000 * 120)
        );
    }
}
