// Take a look at the license at the top of the repository in the LICENSE file.

use crate::AudioChannelPosition;

use std::mem;

use glib::translate::{from_glib, IntoGlib};

impl AudioChannelPosition {
    pub fn to_mask(self) -> u64 {
        let pos = self.into_glib();
        if pos < 0 {
            return 0;
        }

        1 << (pos as u32)
    }

    pub fn positions_to_mask(
        positions: &[AudioChannelPosition],
        force_order: bool,
    ) -> Result<u64, glib::error::BoolError> {
        assert_initialized_main_thread!();

        let len = positions.len();
        if len > 64 {
            return Err(glib::bool_error!("Invalid number of channels"));
        }

        let positions_raw: [ffi::GstAudioChannelPosition; 64] = array_init::array_init(|i| {
            if i >= len as usize {
                ffi::GST_AUDIO_CHANNEL_POSITION_INVALID
            } else {
                positions[i].into_glib()
            }
        });

        unsafe {
            let mut mask = mem::MaybeUninit::uninit();
            let valid: bool = from_glib(ffi::gst_audio_channel_positions_to_mask(
                positions_raw.as_ptr() as *mut _,
                len as i32,
                force_order.into_glib(),
                mask.as_mut_ptr(),
            ));
            if valid {
                Ok(mask.assume_init())
            } else {
                Err(glib::bool_error!(
                    "Couldn't convert channel positions to mask"
                ))
            }
        }
    }

    pub fn positions_from_mask(
        mask: u64,
        positions: &mut [AudioChannelPosition],
    ) -> Result<(), glib::BoolError> {
        assert_initialized_main_thread!();

        if positions.len() > 64 {
            return Err(glib::bool_error!("Invalid number of channels"));
        }

        let len = positions.len();
        let mut positions_raw: [ffi::GstAudioChannelPosition; 64] =
            [ffi::GST_AUDIO_CHANNEL_POSITION_INVALID; 64];
        let valid: bool = unsafe {
            from_glib(ffi::gst_audio_channel_positions_from_mask(
                len as i32,
                mask,
                positions_raw.as_mut_ptr(),
            ))
        };

        if valid {
            for (d, s) in positions.iter_mut().zip(positions_raw.iter()) {
                *d = unsafe { from_glib(*s) };
            }
            Ok(())
        } else {
            Err(glib::bool_error!(
                "Couldn't convert channel positions to mask",
            ))
        }
    }

    pub fn positions_to_valid_order(
        positions: &mut [AudioChannelPosition],
    ) -> Result<(), glib::BoolError> {
        assert_initialized_main_thread!();

        if positions.len() > 64 {
            return Err(glib::bool_error!("Invalid number of channels"));
        }

        let len = positions.len();
        let mut positions_raw: [ffi::GstAudioChannelPosition; 64] = array_init::array_init(|i| {
            if i >= len as usize {
                ffi::GST_AUDIO_CHANNEL_POSITION_INVALID
            } else {
                positions[i].into_glib()
            }
        });

        let valid: bool = unsafe {
            from_glib(ffi::gst_audio_channel_positions_to_valid_order(
                positions_raw.as_mut_ptr(),
                len as i32,
            ))
        };

        if valid {
            for (d, s) in positions.iter_mut().zip(positions_raw.iter()) {
                *d = unsafe { from_glib(*s) };
            }
            Ok(())
        } else {
            Err(glib::bool_error!(
                "Couldn't convert channel positions to mask",
            ))
        }
    }

    pub fn fallback_mask(channels: u32) -> u64 {
        assert_initialized_main_thread!();

        unsafe { ffi::gst_audio_channel_get_fallback_mask(channels as i32) }
    }

    pub fn check_valid_channel_positions(
        positions: &[crate::AudioChannelPosition],
        force_order: bool,
    ) -> bool {
        assert_initialized_main_thread!();

        if positions.len() > 64 {
            return false;
        }

        let len = positions.len();
        let positions_raw: [ffi::GstAudioChannelPosition; 64] = array_init::array_init(|i| {
            if i >= len as usize {
                ffi::GST_AUDIO_CHANNEL_POSITION_INVALID
            } else {
                positions[i].into_glib()
            }
        });

        unsafe {
            from_glib(ffi::gst_audio_check_valid_channel_positions(
                positions_raw.as_ptr() as *mut _,
                len as i32,
                force_order.into_glib(),
            ))
        }
    }
}

pub fn buffer_reorder_channels(
    buffer: &mut gst::BufferRef,
    format: crate::AudioFormat,
    channels: u32,
    from: &[AudioChannelPosition],
    to: &[AudioChannelPosition],
) -> Result<(), glib::BoolError> {
    assert_initialized_main_thread!();

    if from.len() != to.len() || from.len() > 64 {
        return Err(glib::bool_error!("Invalid number of channels"));
    }

    let from_len = from.len();
    let to_len = to.len();

    let from_raw: [ffi::GstAudioChannelPosition; 64] = array_init::array_init(|i| {
        if i >= from_len as usize {
            ffi::GST_AUDIO_CHANNEL_POSITION_INVALID
        } else {
            from[i].into_glib()
        }
    });

    let to_raw: [ffi::GstAudioChannelPosition; 64] = array_init::array_init(|i| {
        if i >= to_len as usize {
            ffi::GST_AUDIO_CHANNEL_POSITION_INVALID
        } else {
            to[i].into_glib()
        }
    });

    let valid: bool = unsafe {
        from_glib(ffi::gst_audio_buffer_reorder_channels(
            buffer.as_mut_ptr(),
            format.into_glib(),
            channels as i32,
            from_raw.as_ptr() as *mut _,
            to_raw.as_ptr() as *mut _,
        ))
    };

    if valid {
        Ok(())
    } else {
        Err(glib::bool_error!("Failed to reorder channels"))
    }
}

pub fn reorder_channels(
    data: &mut [u8],
    format: crate::AudioFormat,
    channels: u32,
    from: &[AudioChannelPosition],
    to: &[AudioChannelPosition],
) -> Result<(), glib::BoolError> {
    assert_initialized_main_thread!();

    if from.len() != to.len() || from.len() > 64 {
        return Err(glib::bool_error!("Invalid number of channels"));
    }

    let from_len = from.len();
    let to_len = to.len();

    let from_raw: [ffi::GstAudioChannelPosition; 64] = array_init::array_init(|i| {
        if i >= from_len as usize {
            ffi::GST_AUDIO_CHANNEL_POSITION_INVALID
        } else {
            from[i].into_glib()
        }
    });

    let to_raw: [ffi::GstAudioChannelPosition; 64] = array_init::array_init(|i| {
        if i >= to_len as usize {
            ffi::GST_AUDIO_CHANNEL_POSITION_INVALID
        } else {
            to[i].into_glib()
        }
    });

    let valid: bool = unsafe {
        from_glib(ffi::gst_audio_reorder_channels(
            data.as_mut_ptr() as *mut _,
            data.len(),
            format.into_glib(),
            channels as i32,
            from_raw.as_ptr() as *mut _,
            to_raw.as_ptr() as *mut _,
        ))
    };

    if valid {
        Ok(())
    } else {
        Err(glib::bool_error!("Failed to reorder channels"))
    }
}

pub fn channel_reorder_map(
    from: &[AudioChannelPosition],
    to: &[AudioChannelPosition],
    reorder_map: &mut [usize],
) -> Result<(), glib::BoolError> {
    assert_initialized_main_thread!();

    if from.len() != to.len() || from.len() != reorder_map.len() || from.len() > 64 {
        return Err(glib::bool_error!("Invalid number of channels"));
    }

    let from_len = from.len();
    let to_len = to.len();

    let from_raw: [ffi::GstAudioChannelPosition; 64] = array_init::array_init(|i| {
        if i >= from_len as usize {
            ffi::GST_AUDIO_CHANNEL_POSITION_INVALID
        } else {
            from[i].into_glib()
        }
    });

    let to_raw: [ffi::GstAudioChannelPosition; 64] = array_init::array_init(|i| {
        if i >= to_len as usize {
            ffi::GST_AUDIO_CHANNEL_POSITION_INVALID
        } else {
            to[i].into_glib()
        }
    });

    let mut reorder_map_raw = [0i32, 64];
    let valid: bool = unsafe {
        from_glib(ffi::gst_audio_get_channel_reorder_map(
            from_len as i32,
            from_raw.as_ptr() as *mut _,
            to_raw.as_ptr() as *mut _,
            reorder_map_raw.as_mut_ptr(),
        ))
    };

    if valid {
        for (d, s) in reorder_map.iter_mut().zip(reorder_map_raw.iter()) {
            *d = *s as usize;
        }
        Ok(())
    } else {
        Err(glib::bool_error!("Failed to reorder channels"))
    }
}
