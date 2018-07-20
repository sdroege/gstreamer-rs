// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use std::ptr;

use ffi;

use glib;
use glib::translate::{from_glib, from_glib_full, from_glib_none, mut_override, ToGlibPtr};
use glib::StaticType;

use miniobject::*;
use Buffer;
use BufferList;
use Caps;
use FormattedSegment;
use FormattedValue;
use Segment;
use Structure;
use StructureRef;

pub type Sample = GstRc<SampleRef>;
pub struct SampleRef(ffi::GstSample);

unsafe impl MiniObject for SampleRef {
    type GstType = ffi::GstSample;
}

impl GstRc<SampleRef> {
    pub fn new<F: FormattedValue>(
        buffer: Option<&Buffer>,
        caps: Option<&Caps>,
        segment: Option<&FormattedSegment<F>>,
        info: Option<Structure>,
    ) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let info = info.map(|i| i.into_ptr()).unwrap_or(ptr::null_mut());

            from_glib_full(ffi::gst_sample_new(
                buffer.to_glib_none().0,
                caps.to_glib_none().0,
                segment.to_glib_none().0,
                mut_override(info),
            ))
        }
    }

    pub fn with_buffer_list<F: FormattedValue>(
        buffer_list: Option<&BufferList>,
        caps: Option<&Caps>,
        segment: Option<&FormattedSegment<F>>,
        info: Option<Structure>,
    ) -> Self {
        assert_initialized_main_thread!();
        let sample = Self::new(None, caps, segment, info);
        unsafe {
            ffi::gst_sample_set_buffer_list(sample.to_glib_none().0, buffer_list.to_glib_none().0);
        }
        sample
    }
}

impl SampleRef {
    pub fn get_buffer(&self) -> Option<Buffer> {
        unsafe { from_glib_none(ffi::gst_sample_get_buffer(self.as_mut_ptr())) }
    }

    pub fn get_buffer_list(&self) -> Option<BufferList> {
        unsafe { from_glib_none(ffi::gst_sample_get_buffer_list(self.as_mut_ptr())) }
    }

    pub fn get_caps(&self) -> Option<Caps> {
        unsafe { from_glib_none(ffi::gst_sample_get_caps(self.as_mut_ptr())) }
    }

    pub fn get_segment(&self) -> Option<Segment> {
        unsafe { from_glib_none(ffi::gst_sample_get_segment(self.as_mut_ptr())) }
    }

    pub fn get_info(&self) -> Option<&StructureRef> {
        unsafe {
            let ptr = ffi::gst_sample_get_info(self.as_mut_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(StructureRef::from_glib_borrow(ptr))
            }
        }
    }
}

impl StaticType for SampleRef {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gst_sample_get_type()) }
    }
}

impl ToOwned for SampleRef {
    type Owned = GstRc<SampleRef>;

    fn to_owned(&self) -> GstRc<SampleRef> {
        #[cfg_attr(feature = "cargo-clippy", allow(cast_ptr_alignment))]
        unsafe { from_glib_full(ffi::gst_mini_object_copy(self.as_ptr() as *const _) as *mut _) }
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

unsafe impl Sync for SampleRef {}
unsafe impl Send for SampleRef {}

#[cfg(test)]
mod tests {

    #[test]
    fn test_sample_new_with_info() {
        use GenericFormattedValue;
        use Sample;
        use Structure;

        ::init().unwrap();

        let info = Structure::builder("sample.info")
            .field("f3", &123i32)
            .build();
        let sample = Sample::new::<GenericFormattedValue>(None, None, None, Some(info));

        assert!(sample.get_info().is_some());
    }
}
