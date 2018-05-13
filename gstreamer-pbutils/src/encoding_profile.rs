
use gst;
use glib;
use ffi;

use std::error;
use std::fmt;
use std::collections::LinkedList;

use glib::Cast;
use glib::object::IsA;
use glib::translate::*;

use auto::EncodingProfile;
use auto::EncodingAudioProfile;
use auto::EncodingVideoProfile;
use auto::EncodingContainerProfile;

trait EncodingProfileEdit {
    fn set_allow_dynamic_output(&self, allow_dynamic_output: bool);

    fn set_description<'a, P: Into<Option<&'a str>>>(&self, description: P);

    fn set_enabled(&self, enabled: bool);

    fn set_format(&self, format: &gst::Caps);

    fn set_name<'a, P: Into<Option<&'a str>>>(&self, name: P);

    fn set_presence(&self, presence: u32);

    fn set_preset<'a, P: Into<Option<&'a str>>>(&self, preset: P);

    fn set_preset_name<'a, P: Into<Option<&'a str>>>(&self, preset_name: P);

    fn set_restriction<'a, P: Into<Option<&'a gst::Caps>>>(&self, restriction: P);
}

impl<O: IsA<EncodingProfile> + IsA<glib::object::Object>> EncodingProfileEdit for O {

    fn set_allow_dynamic_output(&self, allow_dynamic_output: bool) {
        unsafe {
            ffi::gst_encoding_profile_set_allow_dynamic_output(self.to_glib_none().0, allow_dynamic_output.to_glib());
        }
    }

    fn set_description<'a, P: Into<Option<&'a str>>>(&self, description: P) {
        let description = description.into();
        let description = description.to_glib_none();
        unsafe {
            ffi::gst_encoding_profile_set_description(self.to_glib_none().0, description.0);
        }
    }

    fn set_enabled(&self, enabled: bool) {
        unsafe {
            ffi::gst_encoding_profile_set_enabled(self.to_glib_none().0, enabled.to_glib());
        }
    }

    fn set_format(&self, format: &gst::Caps) {
        unsafe {
            ffi::gst_encoding_profile_set_format(self.to_glib_none().0, format.to_glib_none().0);
        }
    }

    fn set_name<'a, P: Into<Option<&'a str>>>(&self, name: P) {
        let name = name.into();
        let name = name.to_glib_none();
        unsafe {
            ffi::gst_encoding_profile_set_name(self.to_glib_none().0, name.0);
        }
    }

    fn set_presence(&self, presence: u32) {
        unsafe {
            ffi::gst_encoding_profile_set_presence(self.to_glib_none().0, presence);
        }
    }

    fn set_preset<'a, P: Into<Option<&'a str>>>(&self, preset: P) {
        let preset = preset.into();
        let preset = preset.to_glib_none();
        unsafe {
            ffi::gst_encoding_profile_set_preset(self.to_glib_none().0, preset.0);
        }
    }

    fn set_preset_name<'a, P: Into<Option<&'a str>>>(&self, preset_name: P) {
        let preset_name = preset_name.into();
        let preset_name = preset_name.to_glib_none();
        unsafe {
            ffi::gst_encoding_profile_set_preset_name(self.to_glib_none().0, preset_name.0);
        }
    }

    fn set_restriction<'a, P: Into<Option<&'a gst::Caps>>>(&self, restriction: P) {
        let restriction = restriction.into();
        let restriction = restriction.to_glib_none();
        unsafe {
            ffi::gst_encoding_profile_set_restriction(self.to_glib_none().0, restriction.0);
        }
    }
}

impl EncodingAudioProfile {
    fn new<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b gst::Caps>>>(format: &gst::Caps, preset: P, restriction: Q, presence: u32) -> EncodingAudioProfile {
        assert_initialized_main_thread!();
        let preset = preset.into();
        let preset = preset.to_glib_none();
        let restriction = restriction.into();
        let restriction = restriction.to_glib_none();
        unsafe {
            from_glib_full(ffi::gst_encoding_audio_profile_new(format.to_glib_none().0, preset.0, restriction.0, presence))
        }
    }
}

impl EncodingVideoProfile {
    fn new<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b gst::Caps>>>(format: &gst::Caps, preset: P, restriction: Q, presence: u32) -> EncodingVideoProfile {
        assert_initialized_main_thread!();
        let preset = preset.into();
        let preset = preset.to_glib_none();
        let restriction = restriction.into();
        let restriction = restriction.to_glib_none();
        unsafe {
            from_glib_full(ffi::gst_encoding_video_profile_new(format.to_glib_none().0, preset.0, restriction.0, presence))
        }
    }

    fn set_pass(&self, pass: u32) {
        unsafe {
            ffi::gst_encoding_video_profile_set_pass(self.to_glib_none().0, pass);
        }
    }

    fn set_variableframerate(&self, variableframerate: bool) {
        unsafe {
            ffi::gst_encoding_video_profile_set_variableframerate(self.to_glib_none().0, variableframerate.to_glib());
        }
    }
}

impl EncodingContainerProfile {
    fn new<'a, 'b, 'c, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>, R: Into<Option<&'c str>>>(name: P, description: Q, format: &gst::Caps, preset: R) -> EncodingContainerProfile {
        assert_initialized_main_thread!();
        let name = name.into();
        let name = name.to_glib_none();
        let description = description.into();
        let description = description.to_glib_none();
        let preset = preset.into();
        let preset = preset.to_glib_none();
        unsafe {
            from_glib_full(ffi::gst_encoding_container_profile_new(name.0, description.0, format.to_glib_none().0, preset.0))
        }
    }

    fn add_profile<P: IsA<EncodingProfile>>(&self, profile: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_encoding_container_profile_add_profile(self.to_glib_none().0, profile.to_glib_full()), "Failed to add profile")
        }
    }
}

#[derive(Debug, Clone)]
pub struct EncodingProfileBuilderError;

impl fmt::Display for EncodingProfileBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to build encoding profile")
    }
}

impl error::Error for EncodingProfileBuilderError {
    fn description(&self) -> &str {
        "invalid parameters to build encoding profile"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

struct EncodingProfileBuilderData<'a> {
    name: Option<& 'a str>,
    description: Option<& 'a str>,
    format: Option<& 'a gst::Caps>,
    preset: Option<& 'a str>,
    preset_name: Option<& 'a str>,
    restriction: Option<& 'a gst::Caps>,
    presence: u32,
    allow_dynamic_output: bool,
    enabled: bool
}

impl<'a> EncodingProfileBuilderData<'a> {
    fn new() -> EncodingProfileBuilderData<'a> {
        EncodingProfileBuilderData {
            name: None,
            description: None,
            format: None,
            preset: None,
            preset_name : None,
            restriction: None,
            presence: 0,
            allow_dynamic_output: true,
            enabled: true
        }
    }
}

pub trait EncodingProfileBuilder<'a>: Sized {
    fn name(self, name: & 'a str) -> Self;
    fn description(self, description: & 'a str) -> Self;
    fn format(self, format: & 'a gst::Caps) -> Self;
    fn preset(self, preset: & 'a str) -> Self;
    fn preset_name(self, preset_name: & 'a str) -> Self;
    fn restriction(self, format: & 'a gst::Caps) -> Self;
    fn presence(self, presence: u32) -> Self;
    fn allow_dynamic_output(self, allow: bool) -> Self;
    fn enabled(self, enabled: bool) -> Self;
}

macro_rules! declare_encoding_profile_builder(
    ($name:ident) => {
        impl<'a> EncodingProfileBuilder<'a> for $name<'a> {
            fn name(mut self, name: &'a str) -> $name<'a> {
                self.base.name = Some(name);
                self
            }

            fn description(mut self, description: &'a str) -> $name<'a> {
                self.base.description = Some(description);
                self
            }

            fn format(mut self, format: &'a gst::Caps) -> $name<'a> {
                self.base.format = Some(format);
                self
            }

            fn preset(mut self, preset: &'a str) -> $name<'a> {
                self.base.preset = Some(preset);
                self
            }

            fn preset_name(mut self, preset_name: &'a str) -> $name<'a> {
                self.base.preset_name = Some(preset_name);
                self
            }

            fn restriction(mut self, restriction: &'a gst::Caps) -> $name<'a> {
                self.base.restriction = Some(restriction);
                self
            }

            fn presence(mut self, presence: u32) -> $name<'a> {
                self.base.presence = presence;
                self
            }

            fn allow_dynamic_output(mut self, allow: bool) -> $name<'a> {
                self.base.allow_dynamic_output = allow;
                self
            }

            fn enabled(mut self, enabled: bool) -> $name<'a> {
                self.base.enabled = enabled;
                self
            }
        }
    }
);

fn set_common_fields<T: EncodingProfileEdit>(profile: &mut T, base_data: &EncodingProfileBuilderData) {
    profile.set_name(base_data.name);
    profile.set_description(base_data.description);
    profile.set_preset(base_data.preset);
    profile.set_preset_name(base_data.preset_name);
    profile.set_allow_dynamic_output(base_data.allow_dynamic_output);
    profile.set_enabled(base_data.enabled);

    profile.set_restriction(base_data.restriction);
    profile.set_presence(base_data.presence);
}

pub struct EncodingAudioProfileBuilder<'a> {
    base : EncodingProfileBuilderData<'a>
}

declare_encoding_profile_builder!(EncodingAudioProfileBuilder);

impl<'a> EncodingAudioProfileBuilder<'a> {
    pub fn new() -> Self {
        EncodingAudioProfileBuilder {
            base: EncodingProfileBuilderData::new(),
        }
    }

    pub fn build(self) -> Result<EncodingProfile, EncodingProfileBuilderError> {
        if self.base.format.is_none() {
            return Err(EncodingProfileBuilderError);
        }

        let profile = EncodingAudioProfile::new(
            self.base.format.unwrap(), self.base.preset,
            self.base.restriction, self.base.presence);

        let mut profile = profile.upcast();
        set_common_fields(&mut profile, &self.base);
        Ok(profile)
    }
}

pub struct EncodingVideoProfileBuilder<'a> {
    base : EncodingProfileBuilderData<'a>,
    pass: u32,
    variable_framerate: bool,
}

declare_encoding_profile_builder!(EncodingVideoProfileBuilder);

impl<'a> EncodingVideoProfileBuilder<'a> {
    pub fn new() -> Self {
        EncodingVideoProfileBuilder {
            base: EncodingProfileBuilderData::new(),
            pass: 0,
            variable_framerate: false
        }
    }

    pub fn pass(mut self, pass: u32) -> Self {
        self.pass = pass;
        self
    }

    pub fn variable_framerate(mut self, variable_framerate: bool) -> Self {
        self.variable_framerate = variable_framerate;
        self
    }

    pub fn build(self) -> Result<EncodingProfile, EncodingProfileBuilderError> {
        if self.base.format.is_none() {
            return Err(EncodingProfileBuilderError);
        }

        let video_profile = EncodingVideoProfile::new(
            self.base.format.unwrap(), self.base.preset, self.base.restriction, self.base.presence);

        video_profile.set_pass(self.pass);
        video_profile.set_variableframerate(self.variable_framerate);

        let mut profile = video_profile.upcast();
        set_common_fields(&mut profile, &self.base);
        Ok(profile)
    }
}

pub struct EncodingContainerProfileBuilder<'a> {
    base : EncodingProfileBuilderData<'a>,
    profiles: LinkedList<& 'a EncodingProfile>
}

declare_encoding_profile_builder!(EncodingContainerProfileBuilder);

impl<'a> EncodingContainerProfileBuilder<'a> {
    pub fn new() -> Self {
        EncodingContainerProfileBuilder {
            base: EncodingProfileBuilderData::new(),
            profiles: LinkedList::new()
        }
    }

    pub fn build(self) -> Result<EncodingProfile, EncodingProfileBuilderError> {
        if self.base.format.is_none() {
            return Err(EncodingProfileBuilderError);
        }

        let container_profile = EncodingContainerProfile::new(
            self.base.name, self.base.description, self.base.format.unwrap(), self.base.preset);

        for profile in self.profiles {
            container_profile.add_profile(profile).or_else(|_error| Err(EncodingProfileBuilderError))?;
        }

        let mut profile = container_profile.upcast();
        set_common_fields(&mut profile, &self.base);
        Ok(profile)
    }

    pub fn add_profile(mut self, profile: & 'a EncodingProfile) -> Self {
        self.profiles.push_back(profile);
        self
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use gst;
    use auto::EncodingProfileExt;
    use auto::EncodingVideoProfile;
    use auto::EncodingContainerProfile;
    use auto::EncodingContainerProfileExt;

    const AUDIO_PROFILE_NAME: &'static str = "audio-profile";
    const AUDIO_PROFILE_DESCRIPTION: &'static str = "audio-profile-description";
    const PRESET: &'static str = "preset";
    const PRESET_NAME: &'static str = "preset-name";
    const PRESENCE: u32 = 5;
    const ALLOW_DYNAMIC_OUTPUT: bool = false;
    const ENABLED: bool = false;

    const VIDEO_PROFILE_NAME: &'static str = "video-profile";
    const VIDEO_PROFILE_DESCRIPTION: &'static str = "video-profile-description";

    const CONTAINER_PROFILE_NAME: &'static str = "container-profile";
    const CONTAINER_PROFILE_DESCRIPTION: &'static str = "container-profile-description";

    // Video profile exclusive attributes
    const PASS: u32 = 8;
    const VARIABLE_FRAMERATE: bool = true;

    #[test]
    fn test_encoding_audio_profile_builder() {
        gst::init().unwrap();

        let caps = gst::Caps::new_simple(
            "audio/x-raw",
            &[],
        );

        let restriction = gst::Caps::new_simple(
            "audio/x-raw",
            &[
                ("format", &"S32LE"),
            ],
        );

        let audio_profile = EncodingAudioProfileBuilder::new()
            .name(AUDIO_PROFILE_NAME)
            .description(AUDIO_PROFILE_DESCRIPTION)
            .format(&caps)
            .preset(PRESET)
            .preset_name(PRESET_NAME)
            .restriction(&restriction)
            .presence(PRESENCE)
            .allow_dynamic_output(ALLOW_DYNAMIC_OUTPUT)
            .enabled(ENABLED)
            .build().unwrap();

        assert_eq!(audio_profile.get_name().unwrap(), AUDIO_PROFILE_NAME);
        assert_eq!(audio_profile.get_description().unwrap(), AUDIO_PROFILE_DESCRIPTION);
        assert_eq!(audio_profile.get_format(), caps);
        assert_eq!(audio_profile.get_preset().unwrap(), PRESET);
        assert_eq!(audio_profile.get_preset_name().unwrap(), PRESET_NAME);
        assert_eq!(audio_profile.get_restriction().unwrap(), restriction);
        assert_eq!(audio_profile.get_presence(), PRESENCE);
        assert_eq!(audio_profile.get_allow_dynamic_output(), ALLOW_DYNAMIC_OUTPUT);
        assert_eq!(audio_profile.is_enabled(), ENABLED);
    }

    #[test]
    fn test_encoding_video_profile_builder() {
        gst::init().unwrap();

        let caps = gst::Caps::new_simple(
            "video/x-raw",
            &[],
        );

        let restriction = gst::Caps::new_simple(
            "video/x-raw",
            &[
                ("format", &"RGBA"),
            ],
        );

        let video_profile = EncodingVideoProfileBuilder::new()
            .name(VIDEO_PROFILE_NAME)
            .description(VIDEO_PROFILE_DESCRIPTION)
            .format(&caps)
            .preset(PRESET)
            .preset_name(PRESET_NAME)
            .restriction(&restriction)
            .presence(PRESENCE)
            .allow_dynamic_output(ALLOW_DYNAMIC_OUTPUT)
            .enabled(ENABLED)
            .pass(PASS)
            .variable_framerate(VARIABLE_FRAMERATE)
            .build().unwrap();

        assert_eq!(video_profile.get_name().unwrap(), VIDEO_PROFILE_NAME);
        assert_eq!(video_profile.get_description().unwrap(), VIDEO_PROFILE_DESCRIPTION);
        assert_eq!(video_profile.get_format(), caps);
        assert_eq!(video_profile.get_preset().unwrap(), PRESET);
        assert_eq!(video_profile.get_preset_name().unwrap(), PRESET_NAME);
        assert_eq!(video_profile.get_restriction().unwrap(), restriction);
        assert_eq!(video_profile.get_presence(), PRESENCE);
        assert_eq!(video_profile.get_allow_dynamic_output(), ALLOW_DYNAMIC_OUTPUT);
        assert_eq!(video_profile.is_enabled(), ENABLED);

        let video_profile: EncodingVideoProfile =
            glib::object::Downcast::downcast(video_profile).ok().unwrap();
        assert_eq!(video_profile.get_variableframerate(), VARIABLE_FRAMERATE);
        assert_eq!(video_profile.get_pass(), PASS);
    }

    #[test]
    fn test_encoding_container_profile_builder() {
        gst::init().unwrap();

        let container_caps = gst::Caps::new_simple(
            "container/x-caps",
            &[],
        );
        let restriction = gst::Caps::new_simple(
            "container/x-caps",
            &[
                ("field", &"somevalue")
            ],
        );
        let video_caps = gst::Caps::new_simple(
            "video/x-raw",
            &[],
        );
        let audio_caps = gst::Caps::new_simple(
            "audio/x-raw",
            &[],
        );

        let video_profile = EncodingVideoProfileBuilder::new()
            .name(VIDEO_PROFILE_NAME)
            .description(VIDEO_PROFILE_DESCRIPTION)
            .format(&video_caps)
            .build().unwrap();
        let audio_profile = EncodingAudioProfileBuilder::new()
            .name(AUDIO_PROFILE_NAME)
            .description(AUDIO_PROFILE_DESCRIPTION)
            .format(&audio_caps)
            .build().unwrap();

        let profile = EncodingContainerProfileBuilder::new()
            .name(CONTAINER_PROFILE_NAME)
            .description(CONTAINER_PROFILE_DESCRIPTION)
            .format(&container_caps)
            .preset(PRESET)
            .preset_name(PRESET_NAME)
            .restriction(&restriction)
            .presence(PRESENCE)
            .allow_dynamic_output(ALLOW_DYNAMIC_OUTPUT)
            .enabled(ENABLED)
            .add_profile(&audio_profile)
            .add_profile(&video_profile)
            .build().unwrap();

        assert_eq!(profile.get_name().unwrap(), CONTAINER_PROFILE_NAME);
        assert_eq!(profile.get_description().unwrap(), CONTAINER_PROFILE_DESCRIPTION);
        assert_eq!(profile.get_format(), container_caps);
        assert_eq!(profile.get_preset().unwrap(), PRESET);
        assert_eq!(profile.get_preset_name().unwrap(), PRESET_NAME);
        assert_eq!(profile.get_restriction().unwrap(), restriction);
        assert_eq!(profile.get_presence(), PRESENCE);
        assert_eq!(profile.get_allow_dynamic_output(), ALLOW_DYNAMIC_OUTPUT);
        assert_eq!(profile.is_enabled(), ENABLED);

        let container_profile: EncodingContainerProfile =
            glib::object::Downcast::downcast(profile).ok().unwrap();

        assert!(container_profile.contains_profile(&video_profile));
        assert!(container_profile.contains_profile(&audio_profile));
    }
}
