// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use gst_audio_sys;
use AudioChannelPosition;

use std::mem;

use glib;
use glib::translate::{from_glib, ToGlib};
use gst;

use array_init;

impl AudioChannelPosition {
    pub fn to_mask(self) -> u64 {
        let pos = self.to_glib();
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
            return Err(glib_bool_error!("Invalid number of channels"));
        }

        let positions_raw: [gst_audio_sys::GstAudioChannelPosition; 64] =
            array_init::array_init(|i| {
                if i >= len as usize {
                    gst_audio_sys::GST_AUDIO_CHANNEL_POSITION_INVALID
                } else {
                    positions[i].to_glib()
                }
            });

        unsafe {
            let mut mask = mem::MaybeUninit::uninit();
            let valid: bool = from_glib(gst_audio_sys::gst_audio_channel_positions_to_mask(
                positions_raw.as_ptr() as *mut _,
                len as i32,
                force_order.to_glib(),
                mask.as_mut_ptr(),
            ));
            if valid {
                Ok(mask.assume_init())
            } else {
                Err(glib_bool_error!(
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
            return Err(glib_bool_error!("Invalid number of channels"));
        }

        let len = positions.len();
        let mut positions_raw: [gst_audio_sys::GstAudioChannelPosition; 64] =
            [gst_audio_sys::GST_AUDIO_CHANNEL_POSITION_INVALID; 64];
        let valid: bool = unsafe {
            from_glib(gst_audio_sys::gst_audio_channel_positions_from_mask(
                len as i32,
                mask,
                positions_raw.as_mut_ptr(),
            ))
        };

        if valid {
            for (d, s) in positions.iter_mut().zip(positions_raw.iter()) {
                *d = from_glib(*s);
            }
            Ok(())
        } else {
            Err(glib_bool_error!(
                "Couldn't convert channel positions to mask",
            ))
        }
    }

    pub fn positions_to_valid_order(
        positions: &mut [AudioChannelPosition],
    ) -> Result<(), glib::BoolError> {
        assert_initialized_main_thread!();

        if positions.len() > 64 {
            return Err(glib_bool_error!("Invalid number of channels"));
        }

        let len = positions.len();
        let mut positions_raw: [gst_audio_sys::GstAudioChannelPosition; 64] =
            array_init::array_init(|i| {
                if i >= len as usize {
                    gst_audio_sys::GST_AUDIO_CHANNEL_POSITION_INVALID
                } else {
                    positions[i].to_glib()
                }
            });

        let valid: bool = unsafe {
            from_glib(gst_audio_sys::gst_audio_channel_positions_to_valid_order(
                positions_raw.as_mut_ptr(),
                len as i32,
            ))
        };

        if valid {
            for (d, s) in positions.iter_mut().zip(positions_raw.iter()) {
                *d = from_glib(*s);
            }
            Ok(())
        } else {
            Err(glib_bool_error!(
                "Couldn't convert channel positions to mask",
            ))
        }
    }

    pub fn get_fallback_mask(channels: u32) -> u64 {
        assert_initialized_main_thread!();

        unsafe { gst_audio_sys::gst_audio_channel_get_fallback_mask(channels as i32) }
    }

    pub fn check_valid_channel_positions(
        positions: &[::AudioChannelPosition],
        force_order: bool,
    ) -> bool {
        assert_initialized_main_thread!();

        if positions.len() > 64 {
            return false;
        }

        let len = positions.len();
        let positions_raw: [gst_audio_sys::GstAudioChannelPosition; 64] =
            array_init::array_init(|i| {
                if i >= len as usize {
                    gst_audio_sys::GST_AUDIO_CHANNEL_POSITION_INVALID
                } else {
                    positions[i].to_glib()
                }
            });

        unsafe {
            from_glib(gst_audio_sys::gst_audio_check_valid_channel_positions(
                positions_raw.as_ptr() as *mut _,
                len as i32,
                force_order.to_glib(),
            ))
        }
    }
}

pub fn buffer_reorder_channels(
    buffer: &mut gst::BufferRef,
    format: ::AudioFormat,
    channels: u32,
    from: &[AudioChannelPosition],
    to: &[AudioChannelPosition],
) -> Result<(), glib::BoolError> {
    assert_initialized_main_thread!();

    if from.len() != to.len() || from.len() > 64 {
        return Err(glib_bool_error!("Invalid number of channels"));
    }

    let from_len = from.len();
    let to_len = to.len();

    let from_raw: [gst_audio_sys::GstAudioChannelPosition; 64] = array_init::array_init(|i| {
        if i >= from_len as usize {
            gst_audio_sys::GST_AUDIO_CHANNEL_POSITION_INVALID
        } else {
            from[i].to_glib()
        }
    });

    let to_raw: [gst_audio_sys::GstAudioChannelPosition; 64] = array_init::array_init(|i| {
        if i >= to_len as usize {
            gst_audio_sys::GST_AUDIO_CHANNEL_POSITION_INVALID
        } else {
            to[i].to_glib()
        }
    });

    let valid: bool = unsafe {
        from_glib(gst_audio_sys::gst_audio_buffer_reorder_channels(
            buffer.as_mut_ptr(),
            format.to_glib(),
            channels as i32,
            from_raw.as_ptr() as *mut _,
            to_raw.as_ptr() as *mut _,
        ))
    };

    if valid {
        Ok(())
    } else {
        Err(glib_bool_error!("Failed to reorder channels"))
    }
}

pub fn reorder_channels(
    data: &mut [u8],
    format: ::AudioFormat,
    channels: u32,
    from: &[AudioChannelPosition],
    to: &[AudioChannelPosition],
) -> Result<(), glib::BoolError> {
    assert_initialized_main_thread!();

    if from.len() != to.len() || from.len() > 64 {
        return Err(glib_bool_error!("Invalid number of channels"));
    }

    let from_len = from.len();
    let to_len = to.len();

    let from_raw: [gst_audio_sys::GstAudioChannelPosition; 64] = array_init::array_init(|i| {
        if i >= from_len as usize {
            gst_audio_sys::GST_AUDIO_CHANNEL_POSITION_INVALID
        } else {
            from[i].to_glib()
        }
    });

    let to_raw: [gst_audio_sys::GstAudioChannelPosition; 64] = array_init::array_init(|i| {
        if i >= to_len as usize {
            gst_audio_sys::GST_AUDIO_CHANNEL_POSITION_INVALID
        } else {
            to[i].to_glib()
        }
    });

    let valid: bool = unsafe {
        from_glib(gst_audio_sys::gst_audio_reorder_channels(
            data.as_mut_ptr() as *mut _,
            data.len(),
            format.to_glib(),
            channels as i32,
            from_raw.as_ptr() as *mut _,
            to_raw.as_ptr() as *mut _,
        ))
    };

    if valid {
        Ok(())
    } else {
        Err(glib_bool_error!("Failed to reorder channels"))
    }
}

pub fn get_channel_reorder_map(
    from: &[AudioChannelPosition],
    to: &[AudioChannelPosition],
    reorder_map: &mut [usize],
) -> Result<(), glib::BoolError> {
    assert_initialized_main_thread!();

    if from.len() != to.len() || from.len() != reorder_map.len() || from.len() > 64 {
        return Err(glib_bool_error!("Invalid number of channels"));
    }

    let from_len = from.len();
    let to_len = to.len();

    let from_raw: [gst_audio_sys::GstAudioChannelPosition; 64] = array_init::array_init(|i| {
        if i >= from_len as usize {
            gst_audio_sys::GST_AUDIO_CHANNEL_POSITION_INVALID
        } else {
            from[i].to_glib()
        }
    });

    let to_raw: [gst_audio_sys::GstAudioChannelPosition; 64] = array_init::array_init(|i| {
        if i >= to_len as usize {
            gst_audio_sys::GST_AUDIO_CHANNEL_POSITION_INVALID
        } else {
            to[i].to_glib()
        }
    });

    let mut reorder_map_raw = [0i32, 64];
    let valid: bool = unsafe {
        from_glib(gst_audio_sys::gst_audio_get_channel_reorder_map(
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
        Err(glib_bool_error!("Failed to reorder channels"))
    }
}
