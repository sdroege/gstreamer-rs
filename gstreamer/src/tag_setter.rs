// Take a look at the license at the top of the repository in the LICENSE file.

use crate::tags::*;
use crate::TagMergeMode;
use crate::TagSetter;
use glib::object::IsA;
use glib::translate::*;
use glib::value::ToSendValue;

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

            ffi::gst_tag_setter_add_tag_value(
                self.as_ref().to_glib_none().0,
                mode.to_glib(),
                T::tag_name().to_glib_none().0,
                v.to_glib_none().0,
            );
        }
    }
}
