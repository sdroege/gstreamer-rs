// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use std::ptr;

use gst_sys;

use glib;
use glib::translate::{from_glib_full, from_glib_none, mut_override, ToGlibPtr};

use Buffer;
use BufferList;
use BufferListRef;
use BufferRef;
use Caps;
use CapsRef;
use FormattedSegment;
use FormattedValue;
use Segment;
use Structure;
use StructureRef;

gst_define_mini_object_wrapper!(Sample, SampleRef, gst_sys::GstSample, || {
    gst_sys::gst_sample_get_type()
});

#[derive(Debug, Clone)]
pub struct SampleBuilder<'a> {
    buffer: Option<&'a Buffer>,
    buffer_list: Option<&'a BufferList>,
    caps: Option<&'a Caps>,
    segment: Option<&'a Segment>,
    info: Option<Structure>,
}

impl<'a> SampleBuilder<'a> {
    pub fn buffer(self, buffer: &'a Buffer) -> Self {
        Self {
            buffer: Some(buffer),
            buffer_list: None,
            ..self
        }
    }

    pub fn buffer_list(self, buffer_list: &'a BufferList) -> Self {
        Self {
            buffer: None,
            buffer_list: Some(buffer_list),
            ..self
        }
    }

    pub fn caps(self, caps: &'a Caps) -> Self {
        Self {
            caps: Some(caps),
            ..self
        }
    }

    pub fn segment<F: FormattedValue>(self, segment: &'a FormattedSegment<F>) -> Self {
        Self {
            segment: Some(segment.upcast_ref()),
            ..self
        }
    }

    pub fn info(self, info: Structure) -> Self {
        Self {
            info: Some(info),
            ..self
        }
    }

    pub fn build(self) -> Sample {
        assert_initialized_main_thread!();

        unsafe {
            let info = self.info.map(|i| i.into_ptr()).unwrap_or(ptr::null_mut());

            let sample: Sample = from_glib_full(gst_sys::gst_sample_new(
                self.buffer.to_glib_none().0,
                self.caps.to_glib_none().0,
                self.segment.to_glib_none().0,
                mut_override(info),
            ));

            if let Some(buffer_list) = self.buffer_list {
                gst_sys::gst_sample_set_buffer_list(
                    sample.to_glib_none().0,
                    buffer_list.to_glib_none().0,
                );
            }

            sample
        }
    }
}

impl Sample {
    pub fn builder<'a>() -> SampleBuilder<'a> {
        SampleBuilder {
            buffer: None,
            buffer_list: None,
            caps: None,
            segment: None,
            info: None,
        }
    }
}

impl SampleRef {
    pub fn get_buffer(&self) -> Option<&BufferRef> {
        unsafe {
            let ptr = gst_sys::gst_sample_get_buffer(self.as_mut_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(BufferRef::from_ptr(ptr))
            }
        }
    }

    pub fn get_buffer_owned(&self) -> Option<Buffer> {
        unsafe {
            self.get_buffer()
                .map(|buffer| from_glib_none(buffer.as_ptr()))
        }
    }

    pub fn get_buffer_list(&self) -> Option<&BufferListRef> {
        unsafe {
            let ptr = gst_sys::gst_sample_get_buffer_list(self.as_mut_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(BufferListRef::from_ptr(ptr))
            }
        }
    }

    pub fn get_buffer_list_owned(&self) -> Option<BufferList> {
        unsafe {
            self.get_buffer_list()
                .map(|list| from_glib_none(list.as_ptr()))
        }
    }

    pub fn get_caps(&self) -> Option<&CapsRef> {
        unsafe {
            let ptr = gst_sys::gst_sample_get_caps(self.as_mut_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CapsRef::from_ptr(ptr))
            }
        }
    }

    pub fn get_caps_owned(&self) -> Option<Caps> {
        unsafe { self.get_caps().map(|caps| from_glib_none(caps.as_ptr())) }
    }

    pub fn get_segment(&self) -> Option<Segment> {
        unsafe { from_glib_none(gst_sys::gst_sample_get_segment(self.as_mut_ptr())) }
    }

    pub fn get_info(&self) -> Option<&StructureRef> {
        unsafe {
            let ptr = gst_sys::gst_sample_get_info(self.as_mut_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(StructureRef::from_glib_borrow(ptr))
            }
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn set_buffer(&mut self, buffer: Option<&Buffer>) {
        unsafe { gst_sys::gst_sample_set_buffer(self.as_mut_ptr(), buffer.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn set_buffer_list(&mut self, buffer_list: Option<&BufferList>) {
        unsafe {
            gst_sys::gst_sample_set_buffer_list(self.as_mut_ptr(), buffer_list.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn set_caps(&mut self, caps: Option<&Caps>) {
        unsafe { gst_sys::gst_sample_set_caps(self.as_mut_ptr(), caps.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn set_segment(&mut self, segment: Option<&Segment>) {
        unsafe { gst_sys::gst_sample_set_segment(self.as_mut_ptr(), segment.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn set_info(&mut self, info: Option<Structure>) {
        unsafe {
            gst_sys::gst_sample_set_info(
                self.as_mut_ptr(),
                info.map(|i| i.into_ptr()).unwrap_or(ptr::null_mut()),
            );
        }
    }
}

impl fmt::Debug for Sample {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        SampleRef::fmt(self, f)
    }
}

impl fmt::Debug for SampleRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Sample")
            .field("buffer", &self.get_buffer())
            .field("caps", &self.get_caps())
            .field("segment", &self.get_segment())
            .field("info", &self.get_info())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sample_new_with_info() {
        use Sample;
        use Structure;

        ::init().unwrap();

        let info = Structure::builder("sample.info")
            .field("f3", &123i32)
            .build();
        let sample = Sample::builder().info(info).build();

        assert!(sample.get_info().is_some());
    }
}
