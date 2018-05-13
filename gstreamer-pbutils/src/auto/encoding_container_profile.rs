// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use EncodingProfile;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct EncodingContainerProfile(Object<ffi::GstEncodingContainerProfile, ffi::GstEncodingContainerProfileClass>): EncodingProfile;

    match fn {
        get_type => || ffi::gst_encoding_container_profile_get_type(),
    }
}

unsafe impl Send for EncodingContainerProfile {}
unsafe impl Sync for EncodingContainerProfile {}

pub trait EncodingContainerProfileExt {
    fn contains_profile<P: IsA<EncodingProfile>>(&self, profile: &P) -> bool;

    fn get_profiles(&self) -> Vec<EncodingProfile>;
}

impl<O: IsA<EncodingContainerProfile>> EncodingContainerProfileExt for O {
    fn contains_profile<P: IsA<EncodingProfile>>(&self, profile: &P) -> bool {
        unsafe {
            from_glib(ffi::gst_encoding_container_profile_contains_profile(self.to_glib_none().0, profile.to_glib_none().0))
        }
    }

    fn get_profiles(&self) -> Vec<EncodingProfile> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gst_encoding_container_profile_get_profiles(self.to_glib_none().0))
        }
    }
}
