// Take a look at the license at the top of the repository in the LICENSE file.

use crate::tags::*;
use crate::TagMergeMode;
use crate::TagSetter;
use glib::prelude::*;
use glib::translate::*;

pub trait TagSetterExtManual: 'static {
    #[doc(alias = "gst_tag_setter_add_tag_value")]
    fn add<'a, T: Tag<'a>>(&self, value: &T::TagType, mode: TagMergeMode);
}

impl<O: IsA<TagSetter>> TagSetterExtManual for O {
    fn add<'a, T: Tag<'a>>(&self, value: &T::TagType, mode: TagMergeMode) {
        unsafe {
            let v = value.to_send_value();

            ffi::gst_tag_setter_add_tag_value(
                self.as_ref().to_glib_none().0,
                mode.into_glib(),
                T::tag_name().to_glib_none().0,
                v.to_glib_none().0,
            );
        }
    }
}
