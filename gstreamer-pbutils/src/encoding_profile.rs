// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, translate::*};

use crate::auto::{
    EncodingAudioProfile, EncodingContainerProfile, EncodingProfile, EncodingVideoProfile,
};
#[cfg(feature = "v1_20")]
use crate::ElementProperties;

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::EncodingProfile>> Sealed for T {}
}

pub trait EncodingProfileExtManual: sealed::Sealed + IsA<EncodingProfile> + 'static {
    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_encoding_profile_get_element_properties")]
    #[doc(alias = "get_element_properties")]
    fn element_properties(&self) -> Option<ElementProperties> {
        unsafe {
            from_glib_full::<_, Option<_>>(ffi::gst_encoding_profile_get_element_properties(
                self.as_ref().to_glib_none().0,
            ))
            .map(ElementProperties)
        }
    }
}

impl<O: IsA<EncodingProfile>> EncodingProfileExtManual for O {}

trait EncodingProfileBuilderCommon {
    fn set_allow_dynamic_output(&self, allow_dynamic_output: bool);

    fn set_description(&self, description: Option<&str>);

    fn set_enabled(&self, enabled: bool);

    fn set_format(&self, format: &gst::Caps);

    fn set_name(&self, name: Option<&str>);

    fn set_presence(&self, presence: u32);

    fn set_preset(&self, preset: Option<&str>);

    fn set_preset_name(&self, preset_name: Option<&str>);

    #[cfg(feature = "v1_18")]
    fn set_single_segment(&self, single_segment: bool);

    #[cfg(feature = "v1_20")]
    fn set_element_properties(&self, element_properties: ElementProperties);
}

impl<O: IsA<EncodingProfile>> EncodingProfileBuilderCommon for O {
    // checker-ignore-item
    fn set_allow_dynamic_output(&self, allow_dynamic_output: bool) {
        unsafe {
            ffi::gst_encoding_profile_set_allow_dynamic_output(
                self.as_ref().to_glib_none().0,
                allow_dynamic_output.into_glib(),
            );
        }
    }

    // checker-ignore-item
    fn set_description(&self, description: Option<&str>) {
        let description = description.to_glib_none();
        unsafe {
            ffi::gst_encoding_profile_set_description(
                self.as_ref().to_glib_none().0,
                description.0,
            );
        }
    }

    // checker-ignore-item
    fn set_enabled(&self, enabled: bool) {
        unsafe {
            ffi::gst_encoding_profile_set_enabled(
                self.as_ref().to_glib_none().0,
                enabled.into_glib(),
            );
        }
    }

    // checker-ignore-item
    fn set_format(&self, format: &gst::Caps) {
        unsafe {
            ffi::gst_encoding_profile_set_format(
                self.as_ref().to_glib_none().0,
                format.to_glib_none().0,
            );
        }
    }

    // checker-ignore-item
    fn set_name(&self, name: Option<&str>) {
        let name = name.to_glib_none();
        unsafe {
            ffi::gst_encoding_profile_set_name(self.as_ref().to_glib_none().0, name.0);
        }
    }

    // checker-ignore-item
    fn set_presence(&self, presence: u32) {
        unsafe {
            ffi::gst_encoding_profile_set_presence(self.as_ref().to_glib_none().0, presence);
        }
    }

    // checker-ignore-item
    fn set_preset(&self, preset: Option<&str>) {
        let preset = preset.to_glib_none();
        unsafe {
            ffi::gst_encoding_profile_set_preset(self.as_ref().to_glib_none().0, preset.0);
        }
    }

    // checker-ignore-item
    fn set_preset_name(&self, preset_name: Option<&str>) {
        let preset_name = preset_name.to_glib_none();
        unsafe {
            ffi::gst_encoding_profile_set_preset_name(
                self.as_ref().to_glib_none().0,
                preset_name.0,
            );
        }
    }

    // checker-ignore-item
    #[cfg(feature = "v1_18")]
    fn set_single_segment(&self, single_segment: bool) {
        unsafe {
            ffi::gst_encoding_profile_set_single_segment(
                self.as_ref().to_glib_none().0,
                single_segment.into_glib(),
            );
        }
    }

    // checker-ignore-item
    #[cfg(feature = "v1_20")]
    fn set_element_properties(&self, element_properties: ElementProperties) {
        unsafe {
            ffi::gst_encoding_profile_set_element_properties(
                self.as_ref().to_glib_none().0,
                element_properties.into_inner().into_glib_ptr(),
            );
        }
    }
}

// Split the trait as only the getter is public
trait EncodingProfileHasRestrictionSetter {
    fn set_restriction(&self, restriction: Option<gst::Caps>);
}

pub trait EncodingProfileHasRestrictionGetter {
    #[doc(alias = "get_restriction")]
    #[doc(alias = "gst_encoding_profile_get_restriction")]
    fn restriction(&self) -> Option<gst::Caps>;
}

macro_rules! declare_encoding_profile_has_restriction(
    ($name:ident) => {
        impl EncodingProfileHasRestrictionSetter for $name {
            // checker-ignore-item
            fn set_restriction(&self, restriction: Option<gst::Caps>) {
                let profile: &EncodingProfile = glib::object::Cast::upcast_ref(self);

                unsafe {
                    let restriction = match restriction {
                        Some(restriction) => restriction.into_glib_ptr(),
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
            // checker-ignore-item
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
    // checker-ignore-item
    fn new(
        format: &gst::Caps,
        preset: Option<&str>,
        restriction: Option<&gst::Caps>,
        presence: u32,
    ) -> EncodingAudioProfile {
        skip_assert_initialized!();
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

    #[doc(alias = "gst_encoding_audio_profile_new")]
    pub fn builder(format: &gst::Caps) -> EncodingAudioProfileBuilder {
        assert_initialized_main_thread!();
        EncodingAudioProfileBuilder::new(format)
    }
}

declare_encoding_profile_has_restriction!(EncodingAudioProfile);

impl EncodingVideoProfile {
    // checker-ignore-item
    fn new(
        format: &gst::Caps,
        preset: Option<&str>,
        restriction: Option<&gst::Caps>,
        presence: u32,
    ) -> EncodingVideoProfile {
        skip_assert_initialized!();
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

    #[doc(alias = "gst_encoding_video_profile_new")]
    pub fn builder(format: &gst::Caps) -> EncodingVideoProfileBuilder {
        assert_initialized_main_thread!();
        EncodingVideoProfileBuilder::new(format)
    }

    // checker-ignore-item
    fn set_pass(&self, pass: u32) {
        unsafe {
            ffi::gst_encoding_video_profile_set_pass(self.to_glib_none().0, pass);
        }
    }

    // checker-ignore-item
    fn set_variableframerate(&self, variableframerate: bool) {
        unsafe {
            ffi::gst_encoding_video_profile_set_variableframerate(
                self.to_glib_none().0,
                variableframerate.into_glib(),
            );
        }
    }
}

declare_encoding_profile_has_restriction!(EncodingVideoProfile);

impl EncodingContainerProfile {
    // checker-ignore-item
    fn new(
        name: Option<&str>,
        description: Option<&str>,
        format: &gst::Caps,
        preset: Option<&str>,
    ) -> EncodingContainerProfile {
        skip_assert_initialized!();
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

    #[doc(alias = "gst_encoding_container_profile_new")]
    pub fn builder(format: &gst::Caps) -> EncodingContainerProfileBuilder {
        assert_initialized_main_thread!();
        EncodingContainerProfileBuilder::new(format)
    }

    // checker-ignore-item
    fn add_profile(&self, profile: impl IsA<EncodingProfile>) {
        unsafe {
            let res = ffi::gst_encoding_container_profile_add_profile(
                self.to_glib_none().0,
                profile.upcast().into_glib_ptr(),
            );
            // Can't possibly fail unless we pass random pointers
            debug_assert_ne!(res, glib::ffi::GFALSE);
        }
    }
}

#[derive(Debug)]
struct EncodingProfileBuilderCommonData<'a> {
    format: &'a gst::Caps,
    name: Option<&'a str>,
    description: Option<&'a str>,
    preset: Option<&'a str>,
    preset_name: Option<&'a str>,
    presence: u32,
    allow_dynamic_output: bool,
    enabled: bool,
    #[cfg(feature = "v1_18")]
    single_segment: bool,
    #[cfg(feature = "v1_20")]
    element_properties: Option<ElementProperties>,
}

impl<'a> EncodingProfileBuilderCommonData<'a> {
    fn new(format: &'a gst::Caps) -> EncodingProfileBuilderCommonData<'a> {
        skip_assert_initialized!();
        EncodingProfileBuilderCommonData {
            name: None,
            description: None,
            format,
            preset: None,
            preset_name: None,
            presence: 0,
            allow_dynamic_output: true,
            enabled: true,
            #[cfg(feature = "v1_18")]
            single_segment: false,
            #[cfg(feature = "v1_20")]
            element_properties: None,
        }
    }
}

pub trait EncodingProfileBuilder<'a>: Sized {
    #[doc(alias = "gst_encoding_profile_set_name")]
    #[must_use]
    fn name(self, name: &'a str) -> Self;
    #[doc(alias = "gst_encoding_profile_set_description")]
    #[must_use]
    fn description(self, description: &'a str) -> Self;
    #[doc(alias = "gst_encoding_profile_set_preset")]
    #[must_use]
    fn preset(self, preset: &'a str) -> Self;
    #[doc(alias = "gst_encoding_profile_set_preset_name")]
    #[must_use]
    fn preset_name(self, preset_name: &'a str) -> Self;
    #[doc(alias = "gst_encoding_profile_set_presence")]
    #[must_use]
    fn presence(self, presence: u32) -> Self;
    #[doc(alias = "gst_encoding_profile_set_allow_dynamic_output")]
    #[must_use]
    fn allow_dynamic_output(self, allow: bool) -> Self;
    #[doc(alias = "gst_encoding_profile_set_enabled")]
    #[must_use]
    fn enabled(self, enabled: bool) -> Self;
    #[cfg(feature = "v1_18")]
    #[doc(alias = "gst_encoding_profile_set_single_segment")]
    #[must_use]
    fn single_segment(self, single_segment: bool) -> Self;
    #[cfg(feature = "v1_20")]
    #[doc(alias = "gst_encoding_profile_set_element_properties")]
    #[must_use]
    fn element_properties(self, element_properties: ElementProperties) -> Self;
}

macro_rules! declare_encoding_profile_builder_common(
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

            #[cfg(feature = "v1_18")]
            fn single_segment(mut self, single_segment: bool) -> $name<'a> {
                self.base.single_segment = single_segment;
                self
            }

            #[cfg(feature = "v1_20")]
            fn element_properties(mut self, element_properties: ElementProperties) -> $name<'a> {
                self.base.element_properties = Some(element_properties);
                self
            }
        }
    }
);

fn set_common_fields<T: EncodingProfileBuilderCommon>(
    profile: &T,
    base_data: EncodingProfileBuilderCommonData,
) {
    skip_assert_initialized!();
    profile.set_name(base_data.name);
    profile.set_description(base_data.description);
    profile.set_preset(base_data.preset);
    profile.set_preset_name(base_data.preset_name);
    profile.set_allow_dynamic_output(base_data.allow_dynamic_output);
    profile.set_enabled(base_data.enabled);
    profile.set_presence(base_data.presence);
    #[cfg(feature = "v1_18")]
    {
        profile.set_single_segment(base_data.single_segment);
    }
    #[cfg(feature = "v1_20")]
    {
        let mut base_data = base_data;
        if let Some(element_properties) = base_data.element_properties.take() {
            profile.set_element_properties(element_properties);
        }
    }
}

#[derive(Debug)]
#[must_use = "The builder must be built to be used"]
pub struct EncodingAudioProfileBuilder<'a> {
    base: EncodingProfileBuilderCommonData<'a>,
    restriction: Option<&'a gst::Caps>,
}

declare_encoding_profile_builder_common!(EncodingAudioProfileBuilder);

impl<'a> EncodingAudioProfileBuilder<'a> {
    fn new(format: &'a gst::Caps) -> Self {
        skip_assert_initialized!();
        EncodingAudioProfileBuilder {
            base: EncodingProfileBuilderCommonData::new(format),
            restriction: None,
        }
    }

    #[doc(alias = "gst_encoding_profile_set_restriction")]
    pub fn restriction(mut self, restriction: &'a gst::Caps) -> Self {
        self.restriction = Some(restriction);
        self
    }

    #[must_use = "Building the profile without using it has no effect"]
    pub fn build(self) -> EncodingAudioProfile {
        let profile = EncodingAudioProfile::new(
            self.base.format,
            self.base.preset,
            self.restriction,
            self.base.presence,
        );

        set_common_fields(&profile, self.base);

        profile
    }
}

#[derive(Debug)]
#[must_use = "The builder must be built to be used"]
pub struct EncodingVideoProfileBuilder<'a> {
    base: EncodingProfileBuilderCommonData<'a>,
    restriction: Option<&'a gst::Caps>,
    pass: u32,
    variable_framerate: bool,
}

declare_encoding_profile_builder_common!(EncodingVideoProfileBuilder);

impl<'a> EncodingVideoProfileBuilder<'a> {
    fn new(format: &'a gst::Caps) -> Self {
        skip_assert_initialized!();
        EncodingVideoProfileBuilder {
            base: EncodingProfileBuilderCommonData::new(format),
            restriction: None,
            pass: 0,
            variable_framerate: false,
        }
    }

    #[doc(alias = "gst_encoding_video_profile_set_pass")]
    pub fn pass(mut self, pass: u32) -> Self {
        self.pass = pass;
        self
    }

    #[doc(alias = "gst_encoding_video_profile_set_variableframerate")]
    pub fn variable_framerate(mut self, variable_framerate: bool) -> Self {
        self.variable_framerate = variable_framerate;
        self
    }

    #[doc(alias = "gst_encoding_profile_set_restriction")]
    pub fn restriction(mut self, restriction: &'a gst::Caps) -> Self {
        self.restriction = Some(restriction);
        self
    }

    #[must_use = "Building the profile without using it has no effect"]
    pub fn build(self) -> EncodingVideoProfile {
        let video_profile = EncodingVideoProfile::new(
            self.base.format,
            self.base.preset,
            self.restriction,
            self.base.presence,
        );

        video_profile.set_pass(self.pass);
        video_profile.set_variableframerate(self.variable_framerate);

        set_common_fields(&video_profile, self.base);

        video_profile
    }
}

#[derive(Debug)]
#[must_use = "The builder must be built to be used"]
pub struct EncodingContainerProfileBuilder<'a> {
    base: EncodingProfileBuilderCommonData<'a>,
    profiles: Vec<EncodingProfile>,
}

declare_encoding_profile_builder_common!(EncodingContainerProfileBuilder);

impl<'a> EncodingContainerProfileBuilder<'a> {
    fn new(format: &'a gst::Caps) -> Self {
        skip_assert_initialized!();
        EncodingContainerProfileBuilder {
            base: EncodingProfileBuilderCommonData::new(format),
            profiles: Vec::new(),
        }
    }

    #[must_use = "Building the profile without using it has no effect"]
    pub fn build(self) -> EncodingContainerProfile {
        let container_profile = EncodingContainerProfile::new(
            self.base.name,
            self.base.description,
            self.base.format,
            self.base.preset,
        );

        for profile in self.profiles {
            container_profile.add_profile(profile);
        }

        set_common_fields(&container_profile, self.base);

        container_profile
    }

    #[doc(alias = "gst_encoding_container_profile_add_profile")]
    pub fn add_profile(mut self, profile: impl IsA<EncodingProfile>) -> Self {
        self.profiles.push(profile.upcast());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        auto::{EncodingContainerProfile, EncodingVideoProfile},
        prelude::*,
    };

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

        let caps = gst::Caps::builder("audio/x-raw").build();

        let restriction = gst_audio::AudioCapsBuilder::new()
            .format(gst_audio::AudioFormat::S32le)
            .build();

        let audio_profile = EncodingAudioProfile::builder(&caps)
            .name(AUDIO_PROFILE_NAME)
            .description(AUDIO_PROFILE_DESCRIPTION)
            .preset(PRESET)
            .preset_name(PRESET_NAME)
            .restriction(&restriction)
            .presence(PRESENCE)
            .allow_dynamic_output(ALLOW_DYNAMIC_OUTPUT)
            .enabled(ENABLED)
            .build();

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

        let restriction = gst_audio::AudioCapsBuilder::new()
            .format(gst_audio::AudioFormat::S32be)
            .build();
        audio_profile.set_restriction(Some(restriction.clone()));
        assert_eq!(audio_profile.restriction().unwrap(), restriction);
    }

    #[test]
    fn test_encoding_video_profile_builder() {
        gst::init().unwrap();

        let caps = gst::Caps::builder("video/x-raw").build();

        let restriction = gst_video::VideoCapsBuilder::new()
            .format(gst_video::VideoFormat::Rgba)
            .build();

        let video_profile = EncodingVideoProfile::builder(&caps)
            .name(VIDEO_PROFILE_NAME)
            .description(VIDEO_PROFILE_DESCRIPTION)
            .preset(PRESET)
            .preset_name(PRESET_NAME)
            .restriction(&restriction)
            .presence(PRESENCE)
            .allow_dynamic_output(ALLOW_DYNAMIC_OUTPUT)
            .enabled(ENABLED)
            .pass(PASS)
            .variable_framerate(VARIABLE_FRAMERATE)
            .build();

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

        let restriction = gst_video::VideoCapsBuilder::new()
            .format(gst_video::VideoFormat::Nv12)
            .build();
        video_profile.set_restriction(Some(restriction.clone()));
        assert_eq!(video_profile.restriction().unwrap(), restriction);
    }

    #[test]
    fn test_encoding_container_profile_builder() {
        gst::init().unwrap();

        let container_caps = gst::Caps::builder("container/x-caps").build();
        let video_caps = gst::Caps::builder("video/x-raw").build();
        let audio_caps = gst::Caps::builder("audio/x-raw").build();

        let video_profile = EncodingVideoProfile::builder(&video_caps)
            .name(VIDEO_PROFILE_NAME)
            .description(VIDEO_PROFILE_DESCRIPTION)
            .build();
        let audio_profile = EncodingAudioProfile::builder(&audio_caps)
            .name(AUDIO_PROFILE_NAME)
            .description(AUDIO_PROFILE_DESCRIPTION)
            .build();

        let profile = EncodingContainerProfile::builder(&container_caps)
            .name(CONTAINER_PROFILE_NAME)
            .description(CONTAINER_PROFILE_DESCRIPTION)
            .preset(PRESET)
            .preset_name(PRESET_NAME)
            .presence(PRESENCE)
            .allow_dynamic_output(ALLOW_DYNAMIC_OUTPUT)
            .enabled(ENABLED)
            .add_profile(audio_profile.clone())
            .add_profile(video_profile.clone())
            .build();

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
