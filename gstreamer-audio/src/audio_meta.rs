// Copyright (C) 2018-2020 Sebastian Dröge <sebastian@centricular.com>
// Copyright (C) 2020 Andrew Eikum <aeikum@codeweavers.com> for CodeWeavers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
use std::ptr;
#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
use std::slice;

use glib::translate::{from_glib, ToGlib};
#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
use glib::translate::{from_glib_none, ToGlibPtr};
use gst::prelude::*;

#[repr(transparent)]
pub struct AudioClippingMeta(ffi::GstAudioClippingMeta);

unsafe impl Send for AudioClippingMeta {}
unsafe impl Sync for AudioClippingMeta {}

impl AudioClippingMeta {
    pub fn add<V: Into<gst::GenericFormattedValue>>(
        buffer: &mut gst::BufferRef,
        start: V,
        end: V,
    ) -> gst::MetaRefMut<Self, gst::meta::Standalone> {
        skip_assert_initialized!();
        let start = start.into();
        let end = end.into();
        assert_eq!(start.get_format(), end.get_format());
        unsafe {
            let meta = ffi::gst_buffer_add_audio_clipping_meta(
                buffer.as_mut_ptr(),
                start.get_format().to_glib(),
                start.get_value() as u64,
                end.get_value() as u64,
            );

            Self::from_mut_ptr(buffer, meta)
        }
    }

    pub fn get_start(&self) -> gst::GenericFormattedValue {
        unsafe { gst::GenericFormattedValue::new(from_glib(self.0.format), self.0.start as i64) }
    }

    pub fn get_end(&self) -> gst::GenericFormattedValue {
        unsafe { gst::GenericFormattedValue::new(from_glib(self.0.format), self.0.end as i64) }
    }
}

unsafe impl MetaAPI for AudioClippingMeta {
    type GstType = ffi::GstAudioClippingMeta;

    fn get_meta_api() -> glib::Type {
        unsafe { from_glib(ffi::gst_audio_clipping_meta_api_get_type()) }
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

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
#[repr(transparent)]
pub struct AudioMeta(ffi::GstAudioMeta);

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
unsafe impl Send for AudioMeta {}
#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
unsafe impl Sync for AudioMeta {}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
impl AudioMeta {
    pub fn add<'a>(
        buffer: &'a mut gst::BufferRef,
        info: &crate::AudioInfo,
        samples: usize,
        offsets: &[usize],
    ) -> Result<gst::MetaRefMut<'a, Self, gst::meta::Standalone>, glib::BoolError> {
        skip_assert_initialized!();

        if !info.is_valid() {
            return Err(glib::glib_bool_error!("Invalid audio info"));
        }

        if info.rate() == 0
            || info.channels() == 0
            || info.format() == crate::AudioFormat::Unknown
            || info.format() == crate::AudioFormat::Encoded
        {
            return Err(glib::glib_bool_error!(
                "Unsupported audio format {:?}",
                info
            ));
        }

        if !offsets.is_empty() && info.layout() != crate::AudioLayout::NonInterleaved {
            return Err(glib::glib_bool_error!(
                "Channel offsets only supported for non-interleaved audio"
            ));
        }

        if !offsets.is_empty() && offsets.len() != info.channels() as usize {
            return Err(glib::glib_bool_error!(
                "Number of channel offsets different than number of channels ({} != {})",
                offsets.len(),
                info.channels()
            ));
        }

        if info.layout() == crate::AudioLayout::NonInterleaved {
            let plane_size = samples * (info.width() / 8) as usize;
            let max_offset = if offsets.is_empty() {
                plane_size * (info.channels() - 1) as usize
            } else {
                let mut max_offset = None;

                for (i, offset) in offsets.iter().copied().enumerate() {
                    if let Some(current_max_offset) = max_offset {
                        max_offset = Some(std::cmp::max(current_max_offset, offset));
                    } else {
                        max_offset = Some(offset);
                    }

                    for (j, other_offset) in offsets.iter().copied().enumerate() {
                        if i != j
                            && !(other_offset + plane_size <= offset
                                || offset + plane_size <= other_offset)
                        {
                            return Err(glib::glib_bool_error!("Overlapping audio channel offsets: offset {} for channel {} and offset {} for channel {} with a plane size of {}", offset, i, other_offset, j, plane_size));
                        }
                    }
                }

                max_offset.unwrap()
            };

            if max_offset + plane_size > buffer.get_size() {
                return Err(glib::glib_bool_error!("Audio channel offsets out of bounds: max offset {} with plane size {} and buffer size {}", max_offset, plane_size, buffer.get_size()));
            }
        }

        unsafe {
            let meta = ffi::gst_buffer_add_audio_meta(
                buffer.as_mut_ptr(),
                info.to_glib_none().0,
                samples,
                if offsets.is_empty() {
                    ptr::null_mut()
                } else {
                    offsets.as_ptr() as *mut _
                },
            );

            if meta.is_null() {
                return Err(glib::glib_bool_error!("Failed to add audio meta"));
            }

            Ok(Self::from_mut_ptr(buffer, meta))
        }
    }

    pub fn get_info(&self) -> crate::AudioInfo {
        unsafe { from_glib_none(&self.0.info as *const _ as *mut ffi::GstAudioInfo) }
    }

    pub fn get_samples(&self) -> usize {
        self.0.samples
    }

    pub fn get_offsets(&self) -> &[usize] {
        if self.0.offsets.is_null() || self.0.info.channels < 1 {
            return &[];
        }

        unsafe { slice::from_raw_parts(self.0.offsets, self.0.info.channels as usize) }
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
unsafe impl MetaAPI for AudioMeta {
    type GstType = ffi::GstAudioMeta;

    fn get_meta_api() -> glib::Type {
        unsafe { from_glib(ffi::gst_audio_meta_api_get_type()) }
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
impl fmt::Debug for AudioMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AudioMeta")
            .field("info", &self.get_info())
            .field("samples", &self.get_samples())
            .field("offsets", &self.get_offsets())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_get_meta() {
        use std::convert::TryInto;

        gst::init().unwrap();

        let mut buffer = gst::Buffer::with_size(1024).unwrap();

        {
            let cmeta = AudioClippingMeta::add(
                buffer.get_mut().unwrap(),
                gst::format::Default(Some(1)),
                gst::format::Default(Some(2)),
            );
            assert_eq!(
                cmeta.get_start().try_into(),
                Ok(gst::format::Default(Some(1)))
            );
            assert_eq!(
                cmeta.get_end().try_into(),
                Ok(gst::format::Default(Some(2)))
            );
        }

        {
            let cmeta = buffer.get_meta::<AudioClippingMeta>().unwrap();
            assert_eq!(
                cmeta.get_start().try_into(),
                Ok(gst::format::Default(Some(1)))
            );
            assert_eq!(
                cmeta.get_end().try_into(),
                Ok(gst::format::Default(Some(2)))
            );
        }
    }
}
