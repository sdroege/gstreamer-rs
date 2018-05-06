
use gst;

use std::error;
use std::fmt;

use auto::EncodingVideoProfile;


#[derive(Debug, Clone)]
pub struct EncodingVideoProfileBuilderError;

impl fmt::Display for EncodingVideoProfileBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to build encoding video profile")
    }
}

impl error::Error for EncodingVideoProfileBuilderError {
    fn description(&self) -> &str {
        "invalid parameters to build encoding video profile"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

pub struct EncodingVideoProfileBuilder<'a> {
    format: Option<& 'a gst::Caps>,
    preset: Option<& 'a str>,
    restriction: Option<& 'a gst::Caps>,
    presence: u32,
    pass: u32,
    variable_framerate: bool,
}

impl<'a> EncodingVideoProfileBuilder<'a> {
    pub fn new() -> Self {
        EncodingVideoProfileBuilder {
            format: None,
            preset: None,
            restriction: None,
            presence: 0,
            pass: 0,
            variable_framerate: false,
        }
    }

    pub fn build(self) -> Result<EncodingVideoProfile, EncodingVideoProfileBuilderError> {
        if self.format.is_none() {
            return Err(EncodingVideoProfileBuilderError);
        }

        let profile = EncodingVideoProfile::new(
            self.format.unwrap(), self.preset, self.restriction, self.presence);

        profile.set_pass(self.pass);
        profile.set_variableframerate(self.variable_framerate);

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

    pub fn pass(self, pass: u32) -> Self {
        Self {
            pass: pass,
            ..self
        }
    }

    pub fn variable_framerate(self, variable_framerate: bool) -> Self {
        Self {
            variable_framerate: variable_framerate,
            ..self
        }
    }
}
