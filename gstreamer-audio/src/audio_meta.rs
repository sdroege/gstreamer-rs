// Copyright (C) 2018-2020 Sebastian Dröge <sebastian@centricular.com>
// Copyright (C) 2020 Andrew Eikum <aeikum@codeweavers.com> for CodeWeavers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

use glib;
use glib::translate::{from_glib, ToGlib};
use gst;
use gst::prelude::*;
use gst_audio_sys;

#[repr(C)]
pub struct AudioClippingMeta(gst_audio_sys::GstAudioClippingMeta);

unsafe impl Send for AudioClippingMeta {}
unsafe impl Sync for AudioClippingMeta {}

impl AudioClippingMeta {
    pub fn add<V: Into<gst::GenericFormattedValue>>(
        buffer: &mut gst::BufferRef,
        start: V,
        end: V,
    ) -> gst::MetaRefMut<Self, gst::meta::Standalone> {
        let start = start.into();
        let end = end.into();
        assert_eq!(start.get_format(), end.get_format());
        unsafe {
            let meta = gst_audio_sys::gst_buffer_add_audio_clipping_meta(
                buffer.as_mut_ptr(),
                start.get_format().to_glib(),
                start.get_value() as u64,
                end.get_value() as u64,
            );

            Self::from_mut_ptr(buffer, meta)
        }
    }

    pub fn get_start(&self) -> gst::GenericFormattedValue {
        gst::GenericFormattedValue::new(from_glib(self.0.format), self.0.start as i64)
    }

    pub fn get_end(&self) -> gst::GenericFormattedValue {
        gst::GenericFormattedValue::new(from_glib(self.0.format), self.0.end as i64)
    }
}

unsafe impl MetaAPI for AudioClippingMeta {
    type GstType = gst_audio_sys::GstAudioClippingMeta;

    fn get_meta_api() -> glib::Type {
        unsafe { from_glib(gst_audio_sys::gst_audio_clipping_meta_api_get_type()) }
    }
}

impl fmt::Debug for AudioClippingMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AudioClippingMeta")
            .field("start", &self.get_start())
            .field("end", &self.get_end())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_get_meta() {
        gst::init().unwrap();

        let mut buffer = gst::Buffer::with_size(1024).unwrap();

        {
            let cmeta =
                AudioClippingMeta::add(buffer.get_mut().unwrap(), gst::Format::Default, 1, 2);
            assert_eq!(cmeta.get_format(), gst::Format::Default);
            assert_eq!(cmeta.get_start(), 1);
            assert_eq!(cmeta.get_end(), 2);
        }

        {
            let cmeta = buffer.get_meta::<AudioClippingMeta>().unwrap();
            assert_eq!(cmeta.get_format(), gst::Format::Default);
            assert_eq!(cmeta.get_start(), 1);
            assert_eq!(cmeta.get_end(), 2);
        }
    }
}
