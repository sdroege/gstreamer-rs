// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;

use std::convert;
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
        AudioConverterConfig::new()
    }
}

impl convert::TryFrom<gst::Structure> for AudioConverterConfig {
    type Error = glib::BoolError;

    fn try_from(v: gst::Structure) -> Result<AudioConverterConfig, Self::Error> {
        skip_assert_initialized!();
        if v.get_name() == "GstAudioConverter" {
            Ok(AudioConverterConfig(v))
        } else {
            Err(glib_bool_error!("Structure is no AudioConverterConfig"))
        }
    }
}

impl<'a> convert::TryFrom<&'a gst::StructureRef> for AudioConverterConfig {
    type Error = glib::BoolError;

    fn try_from(v: &'a gst::StructureRef) -> Result<AudioConverterConfig, Self::Error> {
        skip_assert_initialized!();
        AudioConverterConfig::try_from(v.to_owned())
    }
}

impl From<AudioConverterConfig> for gst::Structure {
    fn from(v: AudioConverterConfig) -> gst::Structure {
        skip_assert_initialized!();
        v.0
    }
}

impl AudioConverterConfig {
    pub fn new() -> Self {
        AudioConverterConfig(gst::Structure::new_empty("GstAudioConverter"))
    }

    pub fn set_dither_method(&mut self, v: crate::AudioDitherMethod) {
        self.0.set("GstAudioConverter.dither-method", &v);
    }

    pub fn get_dither_method(&self) -> crate::AudioDitherMethod {
        self.0
            .get_optional("GstAudioConverter.dither-method")
            .expect("Wrong type")
            .unwrap_or(crate::AudioDitherMethod::None)
    }

    pub fn set_noise_shaping_method(&mut self, v: crate::AudioNoiseShapingMethod) {
        self.0.set("GstAudioConverter.noise-shaping-method", &v);
    }

    pub fn get_noise_shaping_method(&self) -> crate::AudioNoiseShapingMethod {
        self.0
            .get_optional("GstAudioConverter.noise-shaping-method")
            .expect("Wrong type")
            .unwrap_or(crate::AudioNoiseShapingMethod::None)
    }

    pub fn set_quantization(&mut self, v: u32) {
        self.0.set("GstAudioConverter.quantization", &v);
    }

    pub fn get_quantization(&self) -> u32 {
        self.0
            .get_optional("GstAudioConverter.quantization")
            .expect("Wrong type")
            .unwrap_or(1)
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    pub fn set_resampler_method(&mut self, v: crate::AudioResamplerMethod) {
        self.0.set("GstAudioConverter.resampler-method", &v);
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    pub fn get_resampler_method(&self) -> crate::AudioResamplerMethod {
        self.0
            .get_optional("GstAudioConverter.resampler-method")
            .expect("Wrong type")
            .unwrap_or(crate::AudioResamplerMethod::BlackmanNuttall)
    }

    pub fn set_mix_matrix(&mut self, v: &[&[f64]]) {
        let length = v.get(0).map(|v| v.len()).unwrap_or(0);
        let array = gst::Array::from_owned(
            v.iter()
                .map(|val| {
                    assert_eq!(val.len(), length);
                    gst::Array::from_owned(
                        val.iter()
                            .map(|val| val.to_send_value())
                            .collect::<Vec<_>>(),
                    )
                    .to_send_value()
                })
                .collect::<Vec<_>>(),
        );
        self.0.set("GstAudioConverter.mix-matrix", &array);
    }

    pub fn get_mix_matrix(&self) -> Vec<Vec<f64>> {
        self.0
            .get_optional::<gst::Array>("GstAudioConverter.mix-matrix")
            .expect("Wrong type")
            .map(|array| {
                array
                    .as_slice()
                    .iter()
                    .map(|val| {
                        let array = val
                            .get::<gst::Array>()
                            .expect("Wrong type")
                            .unwrap_or_else(|| gst::Array::from_owned(Vec::new()));

                        array
                            .as_slice()
                            .iter()
                            .map(|val| val.get_some::<f64>().expect("Wrong type"))
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_else(Vec::new)
    }
}
