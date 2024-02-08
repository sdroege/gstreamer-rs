// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;
#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
use std::ptr;
#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
use std::slice;

use glib::translate::{from_glib, IntoGlib};
#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
use glib::translate::{from_glib_none, ToGlibPtr};
use gst::prelude::*;

#[repr(transparent)]
#[doc(alias = "GstAudioClippingMeta")]
pub struct AudioClippingMeta(ffi::GstAudioClippingMeta);

unsafe impl Send for AudioClippingMeta {}
unsafe impl Sync for AudioClippingMeta {}

impl AudioClippingMeta {
    #[doc(alias = "gst_buffer_add_audio_clipping_meta")]
    pub fn add<V: gst::format::FormattedValue>(
        buffer: &mut gst::BufferRef,
        start: V,
        end: V,
    ) -> gst::MetaRefMut<Self, gst::meta::Standalone> {
        skip_assert_initialized!();
        assert_eq!(start.format(), end.format());
        unsafe {
            let meta = ffi::gst_buffer_add_audio_clipping_meta(
                buffer.as_mut_ptr(),
                start.format().into_glib(),
                start.into_raw_value() as u64,
                end.into_raw_value() as u64,
            );

            Self::from_mut_ptr(buffer, meta)
        }
    }

    #[doc(alias = "get_start")]
    #[inline]
    pub fn start(&self) -> gst::GenericFormattedValue {
        unsafe { gst::GenericFormattedValue::new(from_glib(self.0.format), self.0.start as i64) }
    }

    #[doc(alias = "get_end")]
    #[inline]
    pub fn end(&self) -> gst::GenericFormattedValue {
        unsafe { gst::GenericFormattedValue::new(from_glib(self.0.format), self.0.end as i64) }
    }
}

unsafe impl MetaAPI for AudioClippingMeta {
    type GstType = ffi::GstAudioClippingMeta;

    #[doc(alias = "gst_audio_clipping_meta_api_get_type")]
    #[inline]
    fn meta_api() -> glib::Type {
        unsafe { from_glib(ffi::gst_audio_clipping_meta_api_get_type()) }
    }
}

impl fmt::Debug for AudioClippingMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AudioClippingMeta")
            .field("start", &self.start())
            .field("end", &self.end())
            .finish()
    }
}

#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
#[repr(transparent)]
#[doc(alias = "GstAudioMeta")]
pub struct AudioMeta(ffi::GstAudioMeta);

#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
unsafe impl Send for AudioMeta {}
#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
unsafe impl Sync for AudioMeta {}

#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
impl AudioMeta {
    #[doc(alias = "gst_buffer_add_audio_meta")]
    pub fn add<'a>(
        buffer: &'a mut gst::BufferRef,
        info: &crate::AudioInfo,
        samples: usize,
        offsets: &[usize],
    ) -> Result<gst::MetaRefMut<'a, Self, gst::meta::Standalone>, glib::BoolError> {
        skip_assert_initialized!();

        if !info.is_valid() {
            return Err(glib::bool_error!("Invalid audio info"));
        }

        if info.rate() == 0
            || info.channels() == 0
            || info.format() == crate::AudioFormat::Unknown
            || info.format() == crate::AudioFormat::Encoded
        {
            return Err(glib::bool_error!("Unsupported audio format {:?}", info));
        }

        if !offsets.is_empty() && info.layout() != crate::AudioLayout::NonInterleaved {
            return Err(glib::bool_error!(
                "Channel offsets only supported for non-interleaved audio"
            ));
        }

        if !offsets.is_empty() && offsets.len() != info.channels() as usize {
            return Err(glib::bool_error!(
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
                            return Err(glib::bool_error!("Overlapping audio channel offsets: offset {} for channel {} and offset {} for channel {} with a plane size of {}", offset, i, other_offset, j, plane_size));
                        }
                    }
                }

                max_offset.unwrap()
            };

            if max_offset + plane_size > buffer.size() {
                return Err(glib::bool_error!("Audio channel offsets out of bounds: max offset {} with plane size {} and buffer size {}", max_offset, plane_size, buffer.size()));
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
                return Err(glib::bool_error!("Failed to add audio meta"));
            }

            Ok(Self::from_mut_ptr(buffer, meta))
        }
    }

    #[doc(alias = "get_info")]
    #[inline]
    pub fn info(&self) -> crate::AudioInfo {
        unsafe { from_glib_none(&self.0.info as *const _) }
    }

    #[doc(alias = "get_samples")]
    #[inline]
    pub fn samples(&self) -> usize {
        self.0.samples
    }

    #[doc(alias = "get_offsets")]
    #[inline]
    pub fn offsets(&self) -> &[usize] {
        if self.0.offsets.is_null() || self.0.info.channels < 1 {
            return &[];
        }

        unsafe { slice::from_raw_parts(self.0.offsets, self.0.info.channels as usize) }
    }
}

#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
unsafe impl MetaAPI for AudioMeta {
    type GstType = ffi::GstAudioMeta;

    #[doc(alias = "gst_audio_meta_api_get_type")]
    #[inline]
    fn meta_api() -> glib::Type {
        unsafe { from_glib(ffi::gst_audio_meta_api_get_type()) }
    }
}

#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
impl fmt::Debug for AudioMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AudioMeta")
            .field("info", &self.info())
            .field("samples", &self.samples())
            .field("offsets", &self.offsets())
            .finish()
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
#[repr(transparent)]
#[doc(alias = "GstAudioLevelMeta")]
pub struct AudioLevelMeta(ffi::GstAudioLevelMeta);

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
unsafe impl Send for AudioLevelMeta {}
#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
unsafe impl Sync for AudioLevelMeta {}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
impl AudioLevelMeta {
    #[doc(alias = "gst_buffer_add_audio_level_meta")]
    pub fn add(
        buffer: &mut gst::BufferRef,
        level: u8,
        voice_activity: bool,
    ) -> gst::MetaRefMut<Self, gst::meta::Standalone> {
        skip_assert_initialized!();
        unsafe {
            let meta = ffi::gst_buffer_add_audio_level_meta(
                buffer.as_mut_ptr(),
                level,
                voice_activity.into_glib(),
            );

            Self::from_mut_ptr(buffer, meta)
        }
    }

    #[doc(alias = "get_level")]
    #[inline]
    pub fn level(&self) -> u8 {
        self.0.level
    }

    #[doc(alias = "get_voice_activity")]
    #[inline]
    pub fn voice_activity(&self) -> bool {
        unsafe { from_glib(self.0.voice_activity) }
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
unsafe impl MetaAPI for AudioLevelMeta {
    type GstType = ffi::GstAudioLevelMeta;

    #[doc(alias = "gst_audio_level_meta_api_get_type")]
    #[inline]
    fn meta_api() -> glib::Type {
        unsafe { from_glib(ffi::gst_audio_level_meta_api_get_type()) }
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
impl fmt::Debug for AudioLevelMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AudioLevelMeta")
            .field("level", &self.level())
            .field("voice_activity", &self.voice_activity())
            .finish()
    }
}

pub mod tags {
    gst::impl_meta_tag!(Audio, GST_META_TAG_AUDIO_STR);
    gst::impl_meta_tag!(Channels, GST_META_TAG_AUDIO_CHANNELS_STR);
    gst::impl_meta_tag!(Rate, GST_META_TAG_AUDIO_RATE_STR);
    #[cfg(feature = "v1_24")]
    gst::impl_meta_tag!(DSDPlaneOffsets, GST_META_TAG_DSD_PLANE_OFFSETS_STR);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_get_audio_clipping_meta() {
        use gst::prelude::*;

        gst::init().unwrap();

        let mut buffer = gst::Buffer::with_size(1024).unwrap();

        let start = 1.default_format();
        let stop = 2.default_format();

        {
            let cmeta = AudioClippingMeta::add(buffer.get_mut().unwrap(), start, stop);
            assert_eq!(cmeta.start().try_into(), Ok(Some(start)));
            assert_eq!(cmeta.end().try_into(), Ok(Some(stop)));
        }

        {
            let cmeta = buffer.meta::<AudioClippingMeta>().unwrap();
            assert_eq!(cmeta.start().try_into(), Ok(Some(start)));
            assert_eq!(cmeta.end().try_into(), Ok(Some(stop)));

            assert!(cmeta.has_tag::<tags::Audio>());
        }
    }

    #[cfg(feature = "v1_20")]
    #[test]
    fn test_add_get_audio_level_meta() {
        gst::init().unwrap();

        let mut buffer = gst::Buffer::with_size(1024).unwrap();

        {
            let cmeta = AudioLevelMeta::add(buffer.get_mut().unwrap(), 10, true);
            assert_eq!(cmeta.level(), 10);
            assert!(cmeta.voice_activity());
        }

        {
            let cmeta = buffer.meta::<AudioLevelMeta>().unwrap();
            assert_eq!(cmeta.level(), 10);
            assert!(cmeta.voice_activity());
        }
    }
}
