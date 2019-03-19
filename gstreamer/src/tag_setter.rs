// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::object::IsA;
use glib::translate::*;
use glib::value::ToSendValue;
use gst_sys;
use tags::*;
use TagMergeMode;
use TagSetter;

pub trait TagSetterExtManual: 'static {
    fn add<'a, T: Tag<'a>>(&self, value: T::TagType, mode: TagMergeMode)
    where
        T::TagType: ToSendValue;
}

impl<O: IsA<TagSetter>> TagSetterExtManual for O {
    fn add<'a, T: Tag<'a>>(&self, value: T::TagType, mode: TagMergeMode)
    where
        T::TagType: ToSendValue,
    {
        unsafe {
            let v = value.to_send_value();

            gst_sys::gst_tag_setter_add_tag_value(
                self.as_ref().to_glib_none().0,
                mode.to_glib(),
                T::tag_name().to_glib_none().0,
                v.to_glib_none().0,
            );
        }
    }
}
