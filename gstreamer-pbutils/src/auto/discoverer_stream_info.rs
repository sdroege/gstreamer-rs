// This file was generated by gir (d933f9a+) from gir-files (???)
// DO NOT EDIT

use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gst;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct DiscovererStreamInfo(Object<ffi::GstDiscovererStreamInfo>);

    match fn {
        get_type => || ffi::gst_discoverer_stream_info_get_type(),
    }
}

impl DiscovererStreamInfo {
    pub fn list_free(infos: &[DiscovererStreamInfo]) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gst_discoverer_stream_info_list_free(infos.to_glib_none().0);
        }
    }
}

unsafe impl Send for DiscovererStreamInfo {}
unsafe impl Sync for DiscovererStreamInfo {}

pub trait DiscovererStreamInfoExt {
    fn get_caps(&self) -> Option<gst::Caps>;

    fn get_misc(&self) -> Option<gst::Structure>;

    fn get_next(&self) -> Option<DiscovererStreamInfo>;

    fn get_previous(&self) -> Option<DiscovererStreamInfo>;

    fn get_stream_id(&self) -> Option<String>;

    fn get_stream_type_nick(&self) -> Option<String>;

    fn get_tags(&self) -> Option<gst::TagList>;

    fn get_toc(&self) -> Option<gst::Toc>;
}

impl<O: IsA<DiscovererStreamInfo>> DiscovererStreamInfoExt for O {
    fn get_caps(&self) -> Option<gst::Caps> {
        unsafe {
            from_glib_full(ffi::gst_discoverer_stream_info_get_caps(self.to_glib_none().0))
        }
    }

    fn get_misc(&self) -> Option<gst::Structure> {
        unsafe {
            from_glib_none(ffi::gst_discoverer_stream_info_get_misc(self.to_glib_none().0))
        }
    }

    fn get_next(&self) -> Option<DiscovererStreamInfo> {
        unsafe {
            from_glib_full(ffi::gst_discoverer_stream_info_get_next(self.to_glib_none().0))
        }
    }

    fn get_previous(&self) -> Option<DiscovererStreamInfo> {
        unsafe {
            from_glib_full(ffi::gst_discoverer_stream_info_get_previous(self.to_glib_none().0))
        }
    }

    fn get_stream_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_discoverer_stream_info_get_stream_id(self.to_glib_none().0))
        }
    }

    fn get_stream_type_nick(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_discoverer_stream_info_get_stream_type_nick(self.to_glib_none().0))
        }
    }

    fn get_tags(&self) -> Option<gst::TagList> {
        unsafe {
            from_glib_none(ffi::gst_discoverer_stream_info_get_tags(self.to_glib_none().0))
        }
    }

    fn get_toc(&self) -> Option<gst::Toc> {
        unsafe {
            from_glib_none(ffi::gst_discoverer_stream_info_get_toc(self.to_glib_none().0))
        }
    }
}
