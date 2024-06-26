// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{ffi, Container, Extractable, MetaContainer, TimelineElement};
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GESGroup")]
    pub struct Group(Object<ffi::GESGroup, ffi::GESGroupClass>) @extends Container, TimelineElement, @implements Extractable, MetaContainer;

    match fn {
        type_ => || ffi::ges_group_get_type(),
    }
}

impl Group {
    pub const NONE: Option<&'static Group> = None;

    #[doc(alias = "ges_group_new")]
    pub fn new() -> Group {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::ges_group_new()) }
    }
}

impl Default for Group {
    fn default() -> Self {
        Self::new()
    }
}
