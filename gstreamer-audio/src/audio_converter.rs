// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;

use std::ops;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioConverterConfig(gst::Structure);

impl ops::Deref for AudioConverterConfig {
    type Target = gst::StructureRef;

    fn deref(&self) -> &gst::StructureRef {
        self.0.deref()
    }
}

impl ops::DerefMut for AudioConverterConfig {
    fn deref_mut(&mut self) -> &mut gst::StructureRef {
        self.0.deref_mut()
    }
}

impl AsRef<gst::StructureRef> for AudioConverterConfig {
    fn as_ref(&self) -> &gst::StructureRef {
        self.0.as_ref()
    }
}

impl AsMut<gst::StructureRef> for AudioConverterConfig {
    fn as_mut(&mut self) -> &mut gst::StructureRef {
        self.0.as_mut()
    }
}

impl Default for AudioConverterConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<gst::Structure> for AudioConverterConfig {
    type Error = glib::BoolError;

    fn try_from(v: gst::Structure) -> Result<Self, Self::Error> {
        skip_assert_initialized!();
        if v.name() == "GstAudioConverter" {
            Ok(Self(v))
        } else {
            Err(glib::bool_error!("Structure is no AudioConverterConfig"))
        }
    }
}

impl<'a> TryFrom<&'a gst::StructureRef> for AudioConverterConfig {
    type Error = glib::BoolError;

    fn try_from(v: &'a gst::StructureRef) -> Result<Self, Self::Error> {
        skip_assert_initialized!();
        Self::try_from(v.to_owned())
    }
}

impl From<AudioConverterConfig> for gst::Structure {
    fn from(v: AudioConverterConfig) -> Self {
        skip_assert_initialized!();
        v.0
    }
}

impl AudioConverterConfig {
    pub fn new() -> Self {
        Self(gst::Structure::new_empty("GstAudioConverter"))
    }

    pub fn set_dither_method(&mut self, v: crate::AudioDitherMethod) {
        self.0.set("GstAudioConverter.dither-method", &v);
    }

    #[doc(alias = "get_dither_method")]
    pub fn dither_method(&self) -> crate::AudioDitherMethod {
        self.0
            .get_optional("GstAudioConverter.dither-method")
            .expect("Wrong type")
            .unwrap_or(crate::AudioDitherMethod::None)
    }

    pub fn set_noise_shaping_method(&mut self, v: crate::AudioNoiseShapingMethod) {
        self.0.set("GstAudioConverter.noise-shaping-method", &v);
    }

    #[doc(alias = "get_noise_shaping_method")]
    pub fn noise_shaping_method(&self) -> crate::AudioNoiseShapingMethod {
        self.0
            .get_optional("GstAudioConverter.noise-shaping-method")
            .expect("Wrong type")
            .unwrap_or(crate::AudioNoiseShapingMethod::None)
    }

    pub fn set_quantization(&mut self, v: u32) {
        self.0.set("GstAudioConverter.quantization", &v);
    }

    #[doc(alias = "get_quantization")]
    pub fn quantization(&self) -> u32 {
        self.0
            .get_optional("GstAudioConverter.quantization")
            .expect("Wrong type")
            .unwrap_or(1)
    }

    pub fn set_resampler_method(&mut self, v: crate::AudioResamplerMethod) {
        self.0.set("GstAudioConverter.resampler-method", &v);
    }

    #[doc(alias = "get_resampler_method")]
    pub fn resampler_method(&self) -> crate::AudioResamplerMethod {
        self.0
            .get_optional("GstAudioConverter.resampler-method")
            .expect("Wrong type")
            .unwrap_or(crate::AudioResamplerMethod::BlackmanNuttall)
    }

    pub fn set_mix_matrix(&mut self, v: &[impl AsRef<[f32]>]) {
        let length = v.get(0).map(|v| v.as_ref().len()).unwrap_or(0);
        let array = gst::Array::from_values(v.iter().map(|val| {
            let val = val.as_ref();
            assert_eq!(val.len(), length);
            gst::Array::from_values(val.iter().map(|val| val.to_send_value())).to_send_value()
        }));
        self.0.set("GstAudioConverter.mix-matrix", &array);
    }

    #[doc(alias = "get_mix_matrix")]
    pub fn mix_matrix(&self) -> Vec<Vec<f32>> {
        self.0
            .get_optional::<gst::Array>("GstAudioConverter.mix-matrix")
            .expect("Wrong type")
            .map(|array| {
                array
                    .as_slice()
                    .iter()
                    .map(|val| {
                        let array = val.get::<gst::Array>().expect("Wrong type");

                        array
                            .as_slice()
                            .iter()
                            .map(|val| val.get::<f32>().expect("Wrong type"))
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_else(Vec::new)
    }

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    pub fn set_dither_threshold(&mut self, v: u32) {
        self.0.set("GstAudioConverter.dither-threshold", &v);
    }

    #[cfg(any(feature = "v1_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_22")))]
    #[doc(alias = "get_dither_threshold")]
    pub fn dither_threshold(&self) -> u32 {
        self.0
            .get_optional("GstAudioConverter.dither-threshold")
            .expect("Wrong type")
            .unwrap_or(20)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix_matrix() {
        const MATRIX: &[&[f32]] = &[&[1.2, 0.3], &[0.2, 0.8]];

        gst::init().unwrap();

        let mut config = AudioConverterConfig::new();
        config.set_mix_matrix(MATRIX);

        let matrix = config.mix_matrix();
        assert_eq!(matrix, MATRIX);

        config.set_mix_matrix(&matrix);
    }
}
