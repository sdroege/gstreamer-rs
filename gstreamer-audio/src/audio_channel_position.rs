// Take a look at the license at the top of the repository in the LICENSE file.

use std::{mem, slice};

use glib::{translate::*, value::FromValue, StaticType, ToValue, Type};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GstAudioChannelPosition")]
#[repr(i32)]
pub enum AudioChannelPosition {
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_NONE")]
    None = ffi::GST_AUDIO_CHANNEL_POSITION_NONE,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_MONO")]
    Mono,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_INVALID")]
    Invalid,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_FRONT_LEFT")]
    FrontLeft,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_FRONT_RIGHT")]
    FrontRight,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_FRONT_CENTER")]
    FrontCenter,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_LFE1")]
    Lfe1,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_REAR_LEFT")]
    RearLeft,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_REAR_RIGHT")]
    RearRight,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_FRONT_LEFT_OF_CENTER")]
    FrontLeftOfCenter,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_FRONT_RIGHT_OF_CENTER")]
    FrontRightOfCenter,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_REAR_CENTER")]
    RearCenter,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_LFE2")]
    Lfe2,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_SIDE_LEFT")]
    SideLeft,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_SIDE_RIGHT")]
    SideRight,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_TOP_FRONT_LEFT")]
    TopFrontLeft,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_TOP_FRONT_RIGHT")]
    TopFrontRight,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_TOP_FRONT_CENTER")]
    TopFrontCenter,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_TOP_CENTER")]
    TopCenter,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_TOP_REAR_LEFT")]
    TopRearLeft,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_TOP_REAR_RIGHT")]
    TopRearRight,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_TOP_SIDE_LEFT")]
    TopSideLeft,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_TOP_SIDE_RIGHT")]
    TopSideRight,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_TOP_REAR_CENTER")]
    TopRearCenter,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_BOTTOM_FRONT_CENTER")]
    BottomFrontCenter,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_BOTTOM_FRONT_LEFT")]
    BottomFrontLeft,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_BOTTOM_FRONT_RIGHT")]
    BottomFrontRight,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_WIDE_LEFT")]
    WideLeft,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_WIDE_RIGHT")]
    WideRight,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_SURROUND_LEFT")]
    SurroundLeft,
    #[doc(alias = "GST_AUDIO_CHANNEL_POSITION_SURROUND_RIGHT")]
    SurroundRight = ffi::GST_AUDIO_CHANNEL_POSITION_SURROUND_RIGHT,
    #[doc(hidden)]
    UnknownChannel28 = 28,
    #[doc(hidden)]
    UnknownChannel29 = 29,
    #[doc(hidden)]
    UnknownChannel30 = 30,
    #[doc(hidden)]
    UnknownChannel31 = 31,
    #[doc(hidden)]
    UnknownChannel32 = 32,
    #[doc(hidden)]
    UnknownChannel33 = 33,
    #[doc(hidden)]
    UnknownChannel34 = 34,
    #[doc(hidden)]
    UnknownChannel35 = 35,
    #[doc(hidden)]
    UnknownChannel36 = 36,
    #[doc(hidden)]
    UnknownChannel37 = 37,
    #[doc(hidden)]
    UnknownChannel38 = 38,
    #[doc(hidden)]
    UnknownChannel39 = 39,
    #[doc(hidden)]
    UnknownChannel40 = 40,
    #[doc(hidden)]
    UnknownChannel41 = 41,
    #[doc(hidden)]
    UnknownChannel42 = 42,
    #[doc(hidden)]
    UnknownChannel43 = 43,
    #[doc(hidden)]
    UnknownChannel44 = 44,
    #[doc(hidden)]
    UnknownChannel45 = 45,
    #[doc(hidden)]
    UnknownChannel46 = 46,
    #[doc(hidden)]
    UnknownChannel47 = 47,
    #[doc(hidden)]
    UnknownChannel48 = 48,
    #[doc(hidden)]
    UnknownChannel49 = 49,
    #[doc(hidden)]
    UnknownChannel50 = 50,
    #[doc(hidden)]
    UnknownChannel51 = 51,
    #[doc(hidden)]
    UnknownChannel52 = 52,
    #[doc(hidden)]
    UnknownChannel53 = 53,
    #[doc(hidden)]
    UnknownChannel54 = 54,
    #[doc(hidden)]
    UnknownChannel55 = 55,
    #[doc(hidden)]
    UnknownChannel56 = 56,
    #[doc(hidden)]
    UnknownChannel57 = 57,
    #[doc(hidden)]
    UnknownChannel58 = 58,
    #[doc(hidden)]
    UnknownChannel59 = 59,
    #[doc(hidden)]
    UnknownChannel60 = 60,
    #[doc(hidden)]
    UnknownChannel61 = 61,
    #[doc(hidden)]
    UnknownChannel62 = 62,
    #[doc(hidden)]
    UnknownChannel63 = 63,
    #[doc(hidden)]
    UnknownChannel64 = 64,
}

unsafe impl TransparentType for AudioChannelPosition {
    type GlibType = ffi::GstAudioChannelPosition;
}

#[doc(hidden)]
impl IntoGlib for AudioChannelPosition {
    type GlibType = ffi::GstAudioChannelPosition;

    #[inline]
    fn into_glib(self) -> ffi::GstAudioChannelPosition {
        self as ffi::GstAudioChannelPosition
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstAudioChannelPosition> for AudioChannelPosition {
    #[inline]
    unsafe fn from_glib(value: ffi::GstAudioChannelPosition) -> Self {
        skip_assert_initialized!();
        debug_assert!((ffi::GST_AUDIO_CHANNEL_POSITION_NONE..=64).contains(&value));
        mem::transmute::<ffi::GstAudioChannelPosition, AudioChannelPosition>(value)
    }
}

impl StaticType for AudioChannelPosition {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_audio_channel_position_get_type()) }
    }
}

impl glib::value::ValueType for AudioChannelPosition {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for AudioChannelPosition {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for AudioChannelPosition {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<AudioChannelPosition> for glib::Value {
    #[inline]
    fn from(v: AudioChannelPosition) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

impl AudioChannelPosition {
    pub fn to_mask(self) -> u64 {
        let pos = self.into_glib();
        if pos < 0 {
            return 0;
        }

        1 << (pos as u32)
    }

    #[doc(alias = "gst_audio_channel_positions_to_mask")]
    pub fn positions_to_mask(
        positions: &[Self],
        force_order: bool,
    ) -> Result<u64, glib::error::BoolError> {
        assert_initialized_main_thread!();

        let len = positions.len();
        if len > 64 {
            return Err(glib::bool_error!("Invalid number of channels"));
        }

        unsafe {
            let mut mask = mem::MaybeUninit::uninit();
            let valid: bool = from_glib(ffi::gst_audio_channel_positions_to_mask(
                positions.as_ptr() as *mut _,
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

    #[doc(alias = "gst_audio_channel_positions_from_mask")]
    pub fn positions_from_mask(mask: u64, positions: &mut [Self]) -> Result<(), glib::BoolError> {
        assert_initialized_main_thread!();

        if positions.len() > 64 {
            return Err(glib::bool_error!("Invalid number of channels"));
        }

        let len = positions.len();
        let valid: bool = unsafe {
            from_glib(ffi::gst_audio_channel_positions_from_mask(
                len as i32,
                mask,
                positions.as_mut_ptr() as *mut _,
            ))
        };

        if valid {
            Ok(())
        } else {
            Err(glib::bool_error!(
                "Couldn't convert channel positions to mask",
            ))
        }
    }

    #[doc(alias = "gst_audio_channel_positions_to_valid_order")]
    pub fn positions_to_valid_order(positions: &mut [Self]) -> Result<(), glib::BoolError> {
        assert_initialized_main_thread!();

        if positions.len() > 64 {
            return Err(glib::bool_error!("Invalid number of channels"));
        }

        let len = positions.len();
        let valid: bool = unsafe {
            from_glib(ffi::gst_audio_channel_positions_to_valid_order(
                positions.as_mut_ptr() as *mut _,
                len as i32,
            ))
        };

        if valid {
            Ok(())
        } else {
            Err(glib::bool_error!(
                "Couldn't convert channel positions to mask",
            ))
        }
    }

    #[doc(alias = "get_fallback_mask")]
    #[doc(alias = "gst_audio_channel_get_fallback_mask")]
    pub fn fallback_mask(channels: u32) -> u64 {
        assert_initialized_main_thread!();

        unsafe { ffi::gst_audio_channel_get_fallback_mask(channels as i32) }
    }

    #[doc(alias = "gst_audio_check_valid_channel_positions")]
    pub fn check_valid_channel_positions(positions: &[Self], force_order: bool) -> bool {
        assert_initialized_main_thread!();

        if positions.len() > 64 {
            return false;
        }

        let len = positions.len();
        unsafe {
            from_glib(ffi::gst_audio_check_valid_channel_positions(
                positions.as_ptr() as *mut _,
                len as i32,
                force_order.into_glib(),
            ))
        }
    }
}

#[doc(alias = "gst_audio_buffer_reorder_channels")]
pub fn buffer_reorder_channels(
    buffer: &mut gst::BufferRef,
    format: crate::AudioFormat,
    channels: u32,
    from: &[AudioChannelPosition],
    to: &[AudioChannelPosition],
) -> Result<(), glib::BoolError> {
    skip_assert_initialized!();

    if from.len() != to.len() || from.len() > 64 {
        return Err(glib::bool_error!("Invalid number of channels"));
    }

    let valid: bool = unsafe {
        from_glib(ffi::gst_audio_buffer_reorder_channels(
            buffer.as_mut_ptr(),
            format.into_glib(),
            channels as i32,
            from.as_ptr() as *mut _,
            to.as_ptr() as *mut _,
        ))
    };

    if valid {
        Ok(())
    } else {
        Err(glib::bool_error!("Failed to reorder channels"))
    }
}

#[doc(alias = "gst_audio_reorder_channels")]
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

    let valid: bool = unsafe {
        from_glib(ffi::gst_audio_reorder_channels(
            data.as_mut_ptr() as *mut _,
            data.len(),
            format.into_glib(),
            channels as i32,
            from.as_ptr() as *mut _,
            to.as_ptr() as *mut _,
        ))
    };

    if valid {
        Ok(())
    } else {
        Err(glib::bool_error!("Failed to reorder channels"))
    }
}

#[doc(alias = "get_channel_reorder_map")]
#[doc(alias = "gst_audio_get_channel_reorder_map")]
pub fn channel_reorder_map(
    from: &[AudioChannelPosition],
    to: &[AudioChannelPosition],
    reorder_map: &mut [usize],
) -> Result<(), glib::BoolError> {
    assert_initialized_main_thread!();

    if from.len() != to.len() || from.len() != reorder_map.len() || from.len() > 64 {
        return Err(glib::bool_error!("Invalid number of channels"));
    }

    let mut reorder_map_raw = mem::MaybeUninit::<[i32; 64]>::uninit();
    let valid: bool = unsafe {
        from_glib(ffi::gst_audio_get_channel_reorder_map(
            from.len() as i32,
            from.as_ptr() as *mut _,
            to.as_ptr() as *mut _,
            reorder_map_raw.as_mut_ptr() as *mut i32,
        ))
    };

    if valid {
        let reorder_map_raw =
            unsafe { slice::from_raw_parts(reorder_map_raw.as_ptr() as *const i32, from.len()) };
        for (d, s) in reorder_map.iter_mut().zip(reorder_map_raw.iter()) {
            *d = *s as usize;
        }
        Ok(())
    } else {
        Err(glib::bool_error!("Failed to reorder channels"))
    }
}
