
use gst;

use std::error;
use std::fmt;

use auto::EncodingAudioProfile;


#[derive(Debug, Clone)]
pub struct EncodingAudioProfileBuilderError;

impl fmt::Display for EncodingAudioProfileBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to build encoding audio profile")
    }
}

impl error::Error for EncodingAudioProfileBuilderError {
    fn description(&self) -> &str {
        "invalid parameters to build encoding audio profile"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

pub struct EncodingAudioProfileBuilder<'a> {
    format: Option<& 'a gst::Caps>,
    preset: Option<& 'a str>,
    restriction: Option<& 'a gst::Caps>,
    presence: u32
}

impl<'a> EncodingAudioProfileBuilder<'a> {
    pub fn new() -> Self {
        EncodingAudioProfileBuilder {
            format: None,
            preset: None,
            restriction: None,
            presence: 0,
        }
    }

    pub fn build(self) -> Result<EncodingAudioProfile, EncodingAudioProfileBuilderError> {
        if self.format.is_none() {
            return Err(EncodingAudioProfileBuilderError);
        }

        let profile = EncodingAudioProfile::new(
            self.format.unwrap(), self.preset, self.restriction, self.presence);

        Ok(profile)
    }

    pub fn format(self, format: & 'a gst::Caps) -> Self {
        Self {
            format: Some(format),
            ..self
        }
    }

    pub fn restriction(self, restriction: & 'a gst::Caps) -> Self {
        Self {
            restriction: Some(restriction),
            ..self
        }
    }

    pub fn preset(self, preset: & 'a str) -> Self {
        Self {
            preset: Some(preset),
            ..self
        }
    }

    pub fn presence(self, presence: u32) -> Self {
        Self {
            presence: presence,
            ..self
        }
    }
}
