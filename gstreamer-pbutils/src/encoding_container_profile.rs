
use gst;

use std::error;
use std::fmt;

use auto::EncodingProfile;
use auto::EncodingContainerProfile;
use auto::EncodingContainerProfileExt;

use std::collections::LinkedList;

#[derive(Debug, Clone)]
pub struct EncodingContainerProfileBuilderError;

impl fmt::Display for EncodingContainerProfileBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to build encoding video profile")
    }
}

impl error::Error for EncodingContainerProfileBuilderError {
    fn description(&self) -> &str {
        "invalid parameters to build encoding container profile"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

pub struct EncodingContainerProfileBuilder<'a> {
    name: Option<& 'a str>,
    description: Option<& 'a str>,
    format: Option<& 'a gst::Caps>,
    preset: Option<& 'a str>,
    profiles: LinkedList<& 'a EncodingProfile>
}

impl<'a> EncodingContainerProfileBuilder<'a> {
    pub fn new() -> Self {
        EncodingContainerProfileBuilder {
            name: None,
            description: None,
            format: None,
            preset: None,
            profiles: LinkedList::new()
        }
    }

    pub fn build(self) -> Result<EncodingContainerProfile, EncodingContainerProfileBuilderError> {
        if self.format.is_none() {
            return Err(EncodingContainerProfileBuilderError);
        }

        let container_profile = EncodingContainerProfile::new(
            self.name, self.description, self.format.unwrap(), self.preset);

        for profile in self.profiles {
            container_profile.add_profile(profile).or_else(|_error| Err(EncodingContainerProfileBuilderError))?;
        }

        Ok(container_profile)
    }

    pub fn name(self, name: & 'a str) -> Self {
        Self {
            name: Some(name),
            ..self
        }
    }

    pub fn description(self, description: & 'a str) -> Self {
        Self {
            description: Some(description),
            ..self
        }
    }

    pub fn format(self, format: & 'a gst::Caps) -> Self {
        Self {
            format: Some(format),
            ..self
        }
    }

    pub fn preset(self, preset: & 'a str) -> Self {
        Self {
            preset: Some(preset),
            ..self
        }
    }

    pub fn add_profile(mut self, profile: & 'a EncodingProfile) -> Self {
        self.profiles.push_back(profile);
        self
    }

}
