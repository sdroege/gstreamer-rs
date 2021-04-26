// Take a look at the license at the top of the repository in the LICENSE file.

use thiserror::Error;

use glib::prelude::*;
use glib::translate::*;

use crate::auto::EncodingAudioProfile;
use crate::auto::EncodingContainerProfile;
use crate::auto::EncodingProfile;
use crate::auto::EncodingVideoProfile;

trait EncodingProfileBuilderCommon {
    fn set_allow_dynamic_output(&self, allow_dynamic_output: bool);

    fn set_description(&self, description: Option<&str>);

    fn set_enabled(&self, enabled: bool);

    fn set_format(&self, format: &gst::Caps);

    fn set_name(&self, name: Option<&str>);

    fn set_presence(&self, presence: u32);

    fn set_preset(&self, preset: Option<&str>);

    fn set_preset_name(&self, preset_name: Option<&str>);
}

impl<O: IsA<EncodingProfile>> EncodingProfileBuilderCommon for O {
    fn set_allow_dynamic_output(&self, allow_dynamic_output: bool) {
        unsafe {
            ffi::gst_encoding_profile_set_allow_dynamic_output(
                self.as_ref().to_glib_none().0,
                allow_dynamic_output.to_glib(),
            );
        }
    }

    fn set_description(&self, description: Option<&str>) {
        let description = description.to_glib_none();
        unsafe {
            ffi::gst_encoding_profile_set_description(
                self.as_ref().to_glib_none().0,
                description.0,
            );
        }
    }

    fn set_enabled(&self, enabled: bool) {
        unsafe {
            ffi::gst_encoding_profile_set_enabled(
                self.as_ref().to_glib_none().0,
                enabled.to_glib(),
            );
        }
    }

    fn set_format(&self, format: &gst::Caps) {
        unsafe {
            ffi::gst_encoding_profile_set_format(
                self.as_ref().to_glib_none().0,
                format.to_glib_none().0,
            );
        }
    }

    fn set_name(&self, name: Option<&str>) {
        let name = name.to_glib_none();
        unsafe {
            ffi::gst_encoding_profile_set_name(self.as_ref().to_glib_none().0, name.0);
        }
    }

    fn set_presence(&self, presence: u32) {
        unsafe {
            ffi::gst_encoding_profile_set_presence(self.as_ref().to_glib_none().0, presence);
        }
    }

    fn set_preset(&self, preset: Option<&str>) {
        let preset = preset.to_glib_none();
        unsafe {
            ffi::gst_encoding_profile_set_preset(self.as_ref().to_glib_none().0, preset.0);
        }
    }

    fn set_preset_name(&self, preset_name: Option<&str>) {
        let preset_name = preset_name.to_glib_none();
        unsafe {
            ffi::gst_encoding_profile_set_preset_name(
                self.as_ref().to_glib_none().0,
                preset_name.0,
            );
        }
    }
}

// Split the trait as only the getter is public
trait EncodingProfileHasRestrictionSetter {
    fn set_restriction(&self, restriction: Option<&gst::Caps>);
}

pub trait EncodingProfileHasRestrictionGetter {
    fn restriction(&self) -> Option<gst::Caps>;
}

macro_rules! declare_encoding_profile_has_restriction(
    ($name:ident) => {
        impl EncodingProfileHasRestrictionSetter for $name {
            fn set_restriction(&self, restriction: Option<&gst::Caps>) {
                let profile: &EncodingProfile = glib::object::Cast::upcast_ref(self);

                unsafe {
                    let restriction = match restriction {
                        Some(restriction) => restriction.to_glib_full(),
                        None => gst::ffi::gst_caps_new_any(),
                    };

                    ffi::gst_encoding_profile_set_restriction(
                        profile.to_glib_none().0,
                        restriction,
                    );
                }
            }
        }

        impl EncodingProfileHasRestrictionGetter for $name {
            fn restriction(&self) -> Option<gst::Caps> {
                let profile: &EncodingProfile = glib::object::Cast::upcast_ref(self);

                unsafe {
                   from_glib_full(ffi::gst_encoding_profile_get_restriction(
                       profile.to_glib_none().0,
                   ))
               }
            }
        }
    }
);

impl EncodingAudioProfile {
    fn new(
        format: &gst::Caps,
        preset: Option<&str>,
        restriction: Option<&gst::Caps>,
        presence: u32,
    ) -> EncodingAudioProfile {
        assert_initialized_main_thread!();
        let preset = preset.to_glib_none();
        let restriction = restriction.to_glib_none();
        unsafe {
            from_glib_full(ffi::gst_encoding_audio_profile_new(
                format.to_glib_none().0,
                preset.0,
                restriction.0,
                presence,
            ))
        }
    }
}

declare_encoding_profile_has_restriction!(EncodingAudioProfile);

impl EncodingVideoProfile {
    fn new(
        format: &gst::Caps,
        preset: Option<&str>,
        restriction: Option<&gst::Caps>,
        presence: u32,
    ) -> EncodingVideoProfile {
        assert_initialized_main_thread!();
        let preset = preset.to_glib_none();
        let restriction = restriction.to_glib_none();
        unsafe {
            from_glib_full(ffi::gst_encoding_video_profile_new(
                format.to_glib_none().0,
                preset.0,
                restriction.0,
                presence,
            ))
        }
    }

    fn set_pass(&self, pass: u32) {
        unsafe {
            ffi::gst_encoding_video_profile_set_pass(self.to_glib_none().0, pass);
        }
    }

    fn set_variableframerate(&self, variableframerate: bool) {
        unsafe {
            ffi::gst_encoding_video_profile_set_variableframerate(
                self.to_glib_none().0,
                variableframerate.to_glib(),
            );
        }
    }
}

declare_encoding_profile_has_restriction!(EncodingVideoProfile);

impl EncodingContainerProfile {
    fn new(
        name: Option<&str>,
        description: Option<&str>,
        format: &gst::Caps,
        preset: Option<&str>,
    ) -> EncodingContainerProfile {
        assert_initialized_main_thread!();
        let name = name.to_glib_none();
        let description = description.to_glib_none();
        let preset = preset.to_glib_none();
        unsafe {
            from_glib_full(ffi::gst_encoding_container_profile_new(
                name.0,
                description.0,
                format.to_glib_none().0,
                preset.0,
            ))
        }
    }

    fn add_profile<P: IsA<EncodingProfile>>(
        &self,
        profile: &P,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_encoding_container_profile_add_profile(
                    self.to_glib_none().0,
                    profile.as_ref().to_glib_full(),
                ),
                "Failed to add profile",
            )
        }
    }
}

#[derive(Debug, Clone, Error)]
#[error("failed to build encoding profile")]
pub struct EncodingProfileBuilderError(());

#[derive(Debug)]
struct EncodingProfileBuilderCommonData<'a> {
    name: Option<&'a str>,
    description: Option<&'a str>,
    format: Option<&'a gst::Caps>,
    preset: Option<&'a str>,
    preset_name: Option<&'a str>,
    presence: u32,
    allow_dynamic_output: bool,
    enabled: bool,
}

impl<'a> EncodingProfileBuilderCommonData<'a> {
    fn new() -> EncodingProfileBuilderCommonData<'a> {
        EncodingProfileBuilderCommonData {
            name: None,
            description: None,
            format: None,
            preset: None,
            preset_name: None,
            presence: 0,
            allow_dynamic_output: true,
            enabled: true,
        }
    }
}

pub trait EncodingProfileBuilder<'a>: Sized {
    fn name(self, name: &'a str) -> Self;
    fn description(self, description: &'a str) -> Self;
    fn format(self, format: &'a gst::Caps) -> Self;
    fn preset(self, preset: &'a str) -> Self;
    fn preset_name(self, preset_name: &'a str) -> Self;
    fn presence(self, presence: u32) -> Self;
    fn allow_dynamic_output(self, allow: bool) -> Self;
    fn enabled(self, enabled: bool) -> Self;
}

macro_rules! declare_encoding_profile_builder_common(
    ($name:ident) => {
        impl<'a> Default for $name<'a> {
            fn default() -> Self {
                Self::new()
            }
        }

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

fn set_common_fields<T: EncodingProfileBuilderCommon>(
    profile: &T,
    base_data: &EncodingProfileBuilderCommonData,
) {
    skip_assert_initialized!();
    profile.set_name(base_data.name);
    profile.set_description(base_data.description);
    profile.set_preset(base_data.preset);
    profile.set_preset_name(base_data.preset_name);
    profile.set_allow_dynamic_output(base_data.allow_dynamic_output);
    profile.set_enabled(base_data.enabled);
    profile.set_presence(base_data.presence);
}

#[derive(Debug)]
pub struct EncodingAudioProfileBuilder<'a> {
    base: EncodingProfileBuilderCommonData<'a>,
    restriction: Option<&'a gst::Caps>,
}

declare_encoding_profile_builder_common!(EncodingAudioProfileBuilder);

impl<'a> EncodingAudioProfileBuilder<'a> {
    pub fn new() -> Self {
        EncodingAudioProfileBuilder {
            base: EncodingProfileBuilderCommonData::new(),
            restriction: None,
        }
    }

    pub fn restriction(mut self, restriction: &'a gst::Caps) -> Self {
        self.restriction = Some(restriction);
        self
    }

    pub fn build(self) -> Result<EncodingAudioProfile, EncodingProfileBuilderError> {
        if self.base.format.is_none() {
            return Err(EncodingProfileBuilderError(()));
        }

        let profile = EncodingAudioProfile::new(
            self.base.format.unwrap(),
            self.base.preset,
            self.restriction,
            self.base.presence,
        );

        set_common_fields(&profile, &self.base);
        Ok(profile)
    }
}

#[derive(Debug)]
pub struct EncodingVideoProfileBuilder<'a> {
    base: EncodingProfileBuilderCommonData<'a>,
    restriction: Option<&'a gst::Caps>,
    pass: u32,
    variable_framerate: bool,
}

declare_encoding_profile_builder_common!(EncodingVideoProfileBuilder);

impl<'a> EncodingVideoProfileBuilder<'a> {
    pub fn new() -> Self {
        EncodingVideoProfileBuilder {
            base: EncodingProfileBuilderCommonData::new(),
            restriction: None,
            pass: 0,
            variable_framerate: false,
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

    pub fn restriction(mut self, restriction: &'a gst::Caps) -> Self {
        self.restriction = Some(restriction);
        self
    }

    pub fn build(self) -> Result<EncodingVideoProfile, EncodingProfileBuilderError> {
        if self.base.format.is_none() {
            return Err(EncodingProfileBuilderError(()));
        }

        let video_profile = EncodingVideoProfile::new(
            self.base.format.unwrap(),
            self.base.preset,
            self.restriction,
            self.base.presence,
        );

        video_profile.set_pass(self.pass);
        video_profile.set_variableframerate(self.variable_framerate);

        set_common_fields(&video_profile, &self.base);
        Ok(video_profile)
    }
}

#[derive(Debug)]
pub struct EncodingContainerProfileBuilder<'a> {
    base: EncodingProfileBuilderCommonData<'a>,
    profiles: Vec<EncodingProfile>,
}

declare_encoding_profile_builder_common!(EncodingContainerProfileBuilder);

impl<'a> EncodingContainerProfileBuilder<'a> {
    pub fn new() -> Self {
        EncodingContainerProfileBuilder {
            base: EncodingProfileBuilderCommonData::new(),
            profiles: Vec::new(),
        }
    }

    pub fn build(self) -> Result<EncodingContainerProfile, EncodingProfileBuilderError> {
        if self.base.format.is_none() {
            return Err(EncodingProfileBuilderError(()));
        }

        let container_profile = EncodingContainerProfile::new(
            self.base.name,
            self.base.description,
            self.base.format.unwrap(),
            self.base.preset,
        );

        for profile in self.profiles {
            container_profile
                .add_profile(&profile)
                .map_err(|_error| EncodingProfileBuilderError(()))?;
        }

        set_common_fields(&container_profile, &self.base);
        Ok(container_profile)
    }

    pub fn add_profile<P: IsA<EncodingProfile>>(mut self, profile: &P) -> Self {
        self.profiles.push(profile.as_ref().clone());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auto::EncodingContainerProfile;
    use crate::auto::EncodingVideoProfile;
    use crate::prelude::*;

    const AUDIO_PROFILE_NAME: &str = "audio-profile";
    const AUDIO_PROFILE_DESCRIPTION: &str = "audio-profile-description";
    const PRESET: &str = "preset";
    const PRESET_NAME: &str = "preset-name";
    const PRESENCE: u32 = 5;
    const ALLOW_DYNAMIC_OUTPUT: bool = false;
    const ENABLED: bool = false;

    const VIDEO_PROFILE_NAME: &str = "video-profile";
    const VIDEO_PROFILE_DESCRIPTION: &str = "video-profile-description";

    const CONTAINER_PROFILE_NAME: &str = "container-profile";
    const CONTAINER_PROFILE_DESCRIPTION: &str = "container-profile-description";

    // Video profile exclusive attributes
    const PASS: u32 = 8;
    const VARIABLE_FRAMERATE: bool = true;

    #[test]
    fn test_encoding_audio_profile_builder() {
        gst::init().unwrap();

        let caps = gst::Caps::new_simple("audio/x-raw", &[]);

        let restriction = gst::Caps::new_simple("audio/x-raw", &[("format", &"S32LE")]);

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
            .build()
            .unwrap();

        assert_eq!(audio_profile.name().unwrap(), AUDIO_PROFILE_NAME);
        assert_eq!(
            audio_profile.description().unwrap(),
            AUDIO_PROFILE_DESCRIPTION
        );
        assert_eq!(audio_profile.format(), caps);
        assert_eq!(audio_profile.preset().unwrap(), PRESET);
        assert_eq!(audio_profile.preset_name().unwrap(), PRESET_NAME);
        assert_eq!(audio_profile.restriction().unwrap(), restriction);
        assert_eq!(audio_profile.presence(), PRESENCE);
        assert_eq!(audio_profile.allows_dynamic_output(), ALLOW_DYNAMIC_OUTPUT);
        assert_eq!(audio_profile.is_enabled(), ENABLED);

        let restriction = gst::Caps::new_simple("audio/x-raw", &[("format", &"S32BE")]);
        audio_profile.set_restriction(Some(&restriction));
        assert_eq!(audio_profile.restriction().unwrap(), restriction);
    }

    #[test]
    fn test_encoding_video_profile_builder() {
        gst::init().unwrap();

        let caps = gst::Caps::new_simple("video/x-raw", &[]);

        let restriction = gst::Caps::new_simple("video/x-raw", &[("format", &"RGBA")]);

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
            .build()
            .unwrap();

        assert_eq!(video_profile.name().unwrap(), VIDEO_PROFILE_NAME);
        assert_eq!(
            video_profile.description().unwrap(),
            VIDEO_PROFILE_DESCRIPTION
        );
        assert_eq!(video_profile.format(), caps);
        assert_eq!(video_profile.preset().unwrap(), PRESET);
        assert_eq!(video_profile.preset_name().unwrap(), PRESET_NAME);
        assert_eq!(video_profile.restriction().unwrap(), restriction);
        assert_eq!(video_profile.presence(), PRESENCE);
        assert_eq!(video_profile.allows_dynamic_output(), ALLOW_DYNAMIC_OUTPUT);
        assert_eq!(video_profile.is_enabled(), ENABLED);

        let video_profile: EncodingVideoProfile =
            glib::object::Cast::downcast(video_profile).ok().unwrap();
        assert_eq!(video_profile.is_variableframerate(), VARIABLE_FRAMERATE);
        assert_eq!(video_profile.pass(), PASS);

        let restriction = gst::Caps::new_simple("video/x-raw", &[("format", &"NV12")]);
        video_profile.set_restriction(Some(&restriction));
        assert_eq!(video_profile.restriction().unwrap(), restriction);
    }

    #[test]
    fn test_encoding_container_profile_builder() {
        gst::init().unwrap();

        let container_caps = gst::Caps::new_simple("container/x-caps", &[]);
        let video_caps = gst::Caps::new_simple("video/x-raw", &[]);
        let audio_caps = gst::Caps::new_simple("audio/x-raw", &[]);

        let video_profile = EncodingVideoProfileBuilder::new()
            .name(VIDEO_PROFILE_NAME)
            .description(VIDEO_PROFILE_DESCRIPTION)
            .format(&video_caps)
            .build()
            .unwrap();
        let audio_profile = EncodingAudioProfileBuilder::new()
            .name(AUDIO_PROFILE_NAME)
            .description(AUDIO_PROFILE_DESCRIPTION)
            .format(&audio_caps)
            .build()
            .unwrap();

        let profile = EncodingContainerProfileBuilder::new()
            .name(CONTAINER_PROFILE_NAME)
            .description(CONTAINER_PROFILE_DESCRIPTION)
            .format(&container_caps)
            .preset(PRESET)
            .preset_name(PRESET_NAME)
            .presence(PRESENCE)
            .allow_dynamic_output(ALLOW_DYNAMIC_OUTPUT)
            .enabled(ENABLED)
            .add_profile(&audio_profile)
            .add_profile(&video_profile)
            .build()
            .unwrap();

        assert_eq!(profile.name().unwrap(), CONTAINER_PROFILE_NAME);
        assert_eq!(
            profile.description().unwrap(),
            CONTAINER_PROFILE_DESCRIPTION
        );
        assert_eq!(profile.format(), container_caps);
        assert_eq!(profile.preset().unwrap(), PRESET);
        assert_eq!(profile.preset_name().unwrap(), PRESET_NAME);
        assert_eq!(profile.presence(), PRESENCE);
        assert_eq!(profile.allows_dynamic_output(), ALLOW_DYNAMIC_OUTPUT);
        assert_eq!(profile.is_enabled(), ENABLED);

        let container_profile: EncodingContainerProfile =
            glib::object::Cast::downcast(profile).ok().unwrap();

        assert!(container_profile.contains_profile(&video_profile));
        assert!(container_profile.contains_profile(&audio_profile));
    }
}
