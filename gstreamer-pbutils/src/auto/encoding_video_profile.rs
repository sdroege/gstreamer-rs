// This file was generated by gir (d933f9a+) from gir-files (???)
// DO NOT EDIT

use EncodingProfile;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gst;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct EncodingVideoProfile(Object<ffi::GstEncodingVideoProfile, ffi::GstEncodingVideoProfileClass>): EncodingProfile;

    match fn {
        get_type => || ffi::gst_encoding_video_profile_get_type(),
    }
}

impl EncodingVideoProfile {
    pub fn new<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b gst::Caps>>>(format: &gst::Caps, preset: P, restriction: Q, presence: u32) -> EncodingVideoProfile {
        assert_initialized_main_thread!();
        let preset = preset.into();
        let preset = preset.to_glib_none();
        let restriction = restriction.into();
        let restriction = restriction.to_glib_none();
        unsafe {
            from_glib_full(ffi::gst_encoding_video_profile_new(format.to_glib_none().0, preset.0, restriction.0, presence))
        }
    }
}

unsafe impl Send for EncodingVideoProfile {}
unsafe impl Sync for EncodingVideoProfile {}

pub trait EncodingVideoProfileExt {
    fn get_pass(&self) -> u32;

    fn get_variableframerate(&self) -> bool;

    fn set_pass(&self, pass: u32);

    fn set_variableframerate(&self, variableframerate: bool);
}

impl<O: IsA<EncodingVideoProfile>> EncodingVideoProfileExt for O {
    fn get_pass(&self) -> u32 {
        unsafe {
            ffi::gst_encoding_video_profile_get_pass(self.to_glib_none().0)
        }
    }

    fn get_variableframerate(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_encoding_video_profile_get_variableframerate(self.to_glib_none().0))
        }
    }

    fn set_pass(&self, pass: u32) {
        unsafe {
            ffi::gst_encoding_video_profile_set_pass(self.to_glib_none().0, pass);
        }
    }

    fn set_variableframerate(&self, variableframerate: bool) {
        unsafe {
            ffi::gst_encoding_video_profile_set_variableframerate(self.to_glib_none().0, variableframerate.to_glib());
        }
    }
}
