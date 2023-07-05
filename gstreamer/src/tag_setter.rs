// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, translate::*};

use crate::{tags::*, TagMergeMode, TagSetter};

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::TagSetter>> Sealed for T {}
}

pub trait TagSetterExtManual: sealed::Sealed + IsA<TagSetter> + 'static {
    #[doc(alias = "gst_tag_setter_add_tag_value")]
    fn add_tag<'a, T: Tag<'a>>(&self, value: &T::TagType, mode: TagMergeMode) {
        unsafe {
            let v = value.to_send_value();

            ffi::gst_tag_setter_add_tag_value(
                self.as_ref().to_glib_none().0,
                mode.into_glib(),
                T::TAG_NAME.as_ptr(),
                v.to_glib_none().0,
            );
        }
    }
}

impl<O: IsA<TagSetter>> TagSetterExtManual for O {}
