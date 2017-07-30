// Copyright (C) 2016-2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use std::mem;
use std::marker::PhantomData;
use std::ffi::CStr;

use ffi;
use glib;
use glib::StaticType;
use glib::value::{Value, TypedValue, FromValueOptional, SetValue, ToValue};
use glib::translate::{from_glib, from_glib_none, from_glib_full, ToGlib, ToGlibPtr, ToGlibPtrMut};

use miniobject::*;

use TagMergeMode;
use Sample;

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

impl_tag!(Title, &'a str, *TAG_TITLE);
impl_tag!(Album, &'a str, *TAG_ALBUM);
impl_tag!(Artist, &'a str, *TAG_ARTIST);
impl_tag!(AlbumArtist, &'a str, *TAG_ALBUM_ARTIST);
impl_tag!(Encoder, &'a str, *TAG_ENCODER);
impl_tag!(AudioCodec, &'a str, *TAG_AUDIO_CODEC);
impl_tag!(VideoCodec, &'a str, *TAG_VIDEO_CODEC);
impl_tag!(SubtitleCodec, &'a str, *TAG_SUBTITLE_CODEC);
impl_tag!(ContainerFormat, &'a str, *TAG_CONTAINER_FORMAT);
// TODO: Should ideally enforce this to be ISO-639
impl_tag!(LanguageCode, &'a str, *TAG_LANGUAGE_CODE);
impl_tag!(Duration, u64, *TAG_DURATION);
impl_tag!(NominalBitrate, u32, *TAG_NOMINAL_BITRATE);
impl_tag!(Image, Sample, *TAG_IMAGE);

lazy_static!{
    pub static ref TAG_TITLE: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_TITLE).to_str().unwrap() };
    pub static ref TAG_ALBUM: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_ALBUM).to_str().unwrap() };
    pub static ref TAG_ARTIST: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_ARTIST).to_str().unwrap() };
    pub static ref TAG_ALBUM_ARTIST: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_ALBUM_ARTIST).to_str().unwrap() };
    pub static ref TAG_ENCODER: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_ENCODER).to_str().unwrap() };
    pub static ref TAG_AUDIO_CODEC: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_AUDIO_CODEC).to_str().unwrap() };
    pub static ref TAG_VIDEO_CODEC: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_VIDEO_CODEC).to_str().unwrap() };
    pub static ref TAG_SUBTITLE_CODEC: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_SUBTITLE_CODEC).to_str().unwrap() };
    pub static ref TAG_CONTAINER_FORMAT: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_CONTAINER_FORMAT).to_str().unwrap() };
    pub static ref TAG_LANGUAGE_CODE: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_LANGUAGE_CODE).to_str().unwrap() };
    pub static ref TAG_DURATION: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_DURATION).to_str().unwrap() };
    pub static ref TAG_NOMINAL_BITRATE: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_NOMINAL_BITRATE).to_str().unwrap() };
    pub static ref TAG_IMAGE: &'static str = unsafe { CStr::from_ptr(ffi::GST_TAG_IMAGE).to_str().unwrap() };
}

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

impl StaticType for TagListRef {
    fn static_type() -> glib::Type {
        unsafe {
            from_glib(ffi::gst_tag_list_get_type())
        }
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
