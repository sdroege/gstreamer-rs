// Take a look at the license at the top of the repository in the LICENSE file.

use std::cmp::Ordering;
use std::ffi::CStr;
use std::fmt;
use std::str;

use glib::translate::{from_glib, from_glib_none, FromGlib, IntoGlib, ToGlibPtr, ToGlibPtrMut};
use glib::StaticType;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum AudioEndianness {
    Unknown,
    LittleEndian = 1234,
    BigEndian = 4321,
}

impl FromGlib<i32> for AudioEndianness {
    #[allow(unused_unsafe)]
    unsafe fn from_glib(value: i32) -> Self {
        assert_initialized_main_thread!();

        match value {
            1234 => Self::LittleEndian,
            4321 => Self::BigEndian,
            _ => Self::Unknown,
        }
    }
}

impl IntoGlib for AudioEndianness {
    type GlibType = i32;

    fn into_glib(self) -> i32 {
        match self {
            Self::LittleEndian => 1234,
            Self::BigEndian => 4321,
            _ => 0,
        }
    }
}

pub struct AudioFormatInfo(&'static ffi::GstAudioFormatInfo);

impl AudioFormatInfo {
    pub fn from_format(format: crate::AudioFormat) -> Self {
        assert_initialized_main_thread!();

        unsafe {
            let info = ffi::gst_audio_format_get_info(format.into_glib());
            assert!(!info.is_null());

            Self(&*info)
        }
    }

    pub fn format(&self) -> crate::AudioFormat {
        unsafe { from_glib(self.0.format) }
    }

    pub fn name<'a>(&self) -> &'a str {
        unsafe { CStr::from_ptr(self.0.name).to_str().unwrap() }
    }

    pub fn description<'a>(&self) -> &'a str {
        unsafe { CStr::from_ptr(self.0.description).to_str().unwrap() }
    }

    pub fn flags(&self) -> crate::AudioFormatFlags {
        unsafe { from_glib(self.0.flags) }
    }

    pub fn endianness(&self) -> AudioEndianness {
        unsafe { from_glib(self.0.endianness) }
    }

    pub fn width(&self) -> u32 {
        self.0.width as u32
    }

    pub fn depth(&self) -> u32 {
        self.0.depth as u32
    }

    pub fn unpack_format(&self) -> crate::AudioFormat {
        unsafe { from_glib(self.0.unpack_format) }
    }

    pub fn silence<'a>(&self) -> &'a [u8] {
        &self.0.silence
    }

    pub fn unpack(&self, flags: crate::AudioPackFlags, dest: &mut [u8], src: &[u8]) {
        let unpack_format = Self::from_format(self.unpack_format());
        let unpack_width = unpack_format.width() as usize;

        if unpack_width == 0 || self.0.unpack_func.is_none() {
            panic!("No unpack format for {:?}", self);
        }

        let self_width = self.width() as usize;
        if self_width == 0 {
            panic!("No width for {:?}", self);
        }

        if src.len() % (self_width / 8) != 0 {
            panic!("Incomplete number of samples in src");
        }

        let nsamples = src.len() / (self_width / 8);

        if dest.len() != nsamples * (unpack_width / 8) {
            panic!("Invalid dest length");
        }

        unsafe {
            (self.0.unpack_func.as_ref().unwrap())(
                self.0,
                flags.into_glib(),
                dest.as_mut_ptr() as *mut _,
                src.as_ptr() as *const _,
                nsamples as i32,
            );
        }
    }

    pub fn pack(&self, flags: crate::AudioPackFlags, dest: &mut [u8], src: &[u8]) {
        let unpack_format = Self::from_format(self.unpack_format());
        let unpack_width = unpack_format.width() as usize;

        if unpack_width == 0 || self.0.pack_func.is_none() {
            panic!("No unpack format for {:?}", self);
        }

        let self_width = self.width() as usize;
        if self_width == 0 {
            panic!("No width for {:?}", self);
        }

        if src.len() % (unpack_width / 8) != 0 {
            panic!("Incomplete number of samples in src");
        }

        let nsamples = src.len() / (unpack_width / 8);

        if dest.len() != nsamples * (self_width / 8) {
            panic!("Invalid dest length");
        }

        unsafe {
            (self.0.pack_func.as_ref().unwrap())(
                self.0,
                flags.into_glib(),
                src.as_ptr() as *const _,
                dest.as_mut_ptr() as *mut _,
                nsamples as i32,
            );
        }
    }

    #[doc(alias = "gst_audio_format_info_fill_silence")]
    #[doc(alias = "gst_audio_format_fill_silence")]
    pub fn fill_silence(&self, dest: &mut [u8]) {
        let self_width = self.width() as usize;

        if self_width == 0 {
            panic!("Filling with silence unsupported");
        }

        if dest.len() % (self_width / 8) != 0 {
            panic!("Incomplete number of samples in dest");
        }

        unsafe {
            cfg_if::cfg_if! {
                if #[cfg(all(feature = "v1_20", not(feature = "dox")))] {
                    ffi::gst_audio_format_info_fill_silence(self.0, dest.as_mut_ptr() as *mut _, dest.len())
                } else {
                    ffi::gst_audio_format_fill_silence(self.0, dest.as_mut_ptr() as *mut _, dest.len())
                }
            }
        }
    }

    pub fn is_float(&self) -> bool {
        self.flags().contains(crate::AudioFormatFlags::FLOAT)
    }

    pub fn is_integer(&self) -> bool {
        self.flags().contains(crate::AudioFormatFlags::INTEGER)
    }

    pub fn is_signed(&self) -> bool {
        self.flags().contains(crate::AudioFormatFlags::SIGNED)
    }

    pub fn is_little_endian(&self) -> bool {
        self.endianness() == AudioEndianness::LittleEndian
    }

    pub fn is_big_endian(&self) -> bool {
        self.endianness() == AudioEndianness::BigEndian
    }
}

unsafe impl Sync for AudioFormatInfo {}
unsafe impl Send for AudioFormatInfo {}

impl PartialEq for AudioFormatInfo {
    fn eq(&self, other: &Self) -> bool {
        self.format() == other.format()
    }
}

impl Eq for AudioFormatInfo {}

impl PartialOrd for AudioFormatInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AudioFormatInfo {
    // See GST_AUDIO_FORMATS_ALL for the sorting algorithm
    fn cmp(&self, other: &Self) -> Ordering {
        self.depth()
            .cmp(&other.depth())
            .then_with(|| self.width().cmp(&other.width()))
            .then_with(|| {
                match (
                    self.flags().contains(crate::AudioFormatFlags::FLOAT),
                    other.flags().contains(crate::AudioFormatFlags::FLOAT),
                ) {
                    (true, false) => Ordering::Greater,
                    (false, true) => Ordering::Less,
                    _ => Ordering::Equal,
                }
            })
            .then_with(|| {
                match (
                    self.flags().contains(crate::AudioFormatFlags::SIGNED),
                    other.flags().contains(crate::AudioFormatFlags::SIGNED),
                ) {
                    (true, false) => Ordering::Greater,
                    (false, true) => Ordering::Less,
                    _ => Ordering::Equal,
                }
            })
            .then_with(|| match (self.endianness(), other.endianness()) {
                (crate::AudioEndianness::LittleEndian, crate::AudioEndianness::BigEndian) => {
                    #[cfg(target_endian = "little")]
                    {
                        Ordering::Greater
                    }
                    #[cfg(target_endian = "big")]
                    {
                        Ordering::Less
                    }
                }
                (crate::AudioEndianness::BigEndian, crate::AudioEndianness::LittleEndian) => {
                    #[cfg(target_endian = "little")]
                    {
                        Ordering::Less
                    }
                    #[cfg(target_endian = "big")]
                    {
                        Ordering::Greater
                    }
                }
                _ => Ordering::Equal,
            })
    }
}

impl fmt::Debug for AudioFormatInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AudioFormatInfo")
            .field("format", &self.format())
            .field("name", &self.name())
            .field("description", &self.description())
            .field("flags", &self.flags())
            .field("endianness", &self.endianness())
            .field("width", &self.width())
            .field("depth", &self.depth())
            .field("silence", &self.silence())
            .finish()
    }
}

impl fmt::Display for AudioFormatInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl str::FromStr for crate::AudioFormatInfo {
    type Err = glib::BoolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        skip_assert_initialized!();
        let format = s.parse()?;
        Ok(Self::from_format(format))
    }
}

impl From<crate::AudioFormat> for AudioFormatInfo {
    fn from(f: crate::AudioFormat) -> Self {
        skip_assert_initialized!();
        Self::from_format(f)
    }
}

impl glib::types::StaticType for AudioFormatInfo {
    fn static_type() -> glib::types::Type {
        unsafe { glib::translate::from_glib(ffi::gst_audio_format_info_get_type()) }
    }
}

impl glib::value::ValueType for AudioFormatInfo {
    type Type = Self;
}

#[doc(hidden)]
unsafe impl<'a> glib::value::FromValue<'a> for AudioFormatInfo {
    type Checker = glib::value::GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib_none(glib::gobject_ffi::g_value_get_boxed(value.to_glib_none().0)
            as *mut ffi::GstAudioFormatInfo)
    }
}

#[doc(hidden)]
impl glib::value::ToValue for AudioFormatInfo {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                self.to_glib_none().0 as *mut _,
            )
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[doc(hidden)]
impl glib::value::ToValueOptional for AudioFormatInfo {
    fn to_value_optional(s: Option<&Self>) -> glib::Value {
        skip_assert_initialized!();
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_boxed(
                value.to_glib_none_mut().0,
                s.to_glib_none().0 as *mut _,
            )
        }
        value
    }
}

#[doc(hidden)]
impl glib::translate::GlibPtrDefault for AudioFormatInfo {
    type GlibType = *mut ffi::GstAudioFormatInfo;
}

#[doc(hidden)]
impl<'a> glib::translate::ToGlibPtr<'a, *const ffi::GstAudioFormatInfo> for AudioFormatInfo {
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> glib::translate::Stash<'a, *const ffi::GstAudioFormatInfo, Self> {
        glib::translate::Stash(self.0, self)
    }

    fn to_glib_full(&self) -> *const ffi::GstAudioFormatInfo {
        unimplemented!()
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrNone<*mut ffi::GstAudioFormatInfo> for AudioFormatInfo {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::GstAudioFormatInfo) -> Self {
        Self(&*ptr)
    }
}

#[doc(hidden)]
impl glib::translate::FromGlibPtrNone<*const ffi::GstAudioFormatInfo> for AudioFormatInfo {
    #[inline]
    unsafe fn from_glib_none(ptr: *const ffi::GstAudioFormatInfo) -> Self {
        Self(&*ptr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        gst::init().unwrap();

        let info = AudioFormatInfo::from_format(crate::AudioFormat::S16le);
        assert_eq!(info.name(), "S16LE");

        let other_info = "S16LE".parse().unwrap();
        assert_eq!(info, other_info);
    }

    #[test]
    fn pack_unpack() {
        gst::init().unwrap();

        let info = AudioFormatInfo::from_format(crate::AudioFormat::S16le);
        let unpack_info = AudioFormatInfo::from_format(info.unpack_format());

        assert!(unpack_info.width() > 0);

        let input = [0, 0, 255, 255, 128, 128, 64, 64];
        let mut unpacked = [0; 16];
        let mut output = [0; 8];

        info.unpack(crate::AudioPackFlags::empty(), &mut unpacked, &input);
        info.pack(crate::AudioPackFlags::empty(), &mut output, &unpacked);

        assert_eq!(input, output);
    }
}
