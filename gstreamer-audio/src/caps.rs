use crate::{AudioFormat, AudioLayout};
use gst::prelude::*;
use gst::Caps;
use std::ops::Bound::*;
use std::ops::RangeBounds;

pub struct AudioCapsBuilder<T> {
    builder: gst::caps::Builder<T>,
}

impl AudioCapsBuilder<gst::caps::NoFeature> {
    pub fn new() -> Self {
        let builder = Caps::builder("audio/x-raw");
        let builder = AudioCapsBuilder { builder };
        builder
            .rate_range(..)
            .channels_range(..)
            .layout_list([AudioLayout::Interleaved, AudioLayout::NonInterleaved])
            .format_list(AudioFormat::iter_raw())
    }

    pub fn new_interleaved() -> Self {
        AudioCapsBuilder::new().layout(AudioLayout::Interleaved)
    }

    pub fn any_features(self) -> AudioCapsBuilder<gst::caps::HasFeatures> {
        AudioCapsBuilder {
            builder: self.builder.any_features(),
        }
    }

    pub fn features(self, features: &[&str]) -> AudioCapsBuilder<gst::caps::HasFeatures> {
        AudioCapsBuilder {
            builder: self.builder.features(features),
        }
    }
}

impl Default for AudioCapsBuilder<gst::caps::NoFeature> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> AudioCapsBuilder<T> {
    pub fn format(self, format: AudioFormat) -> Self {
        Self {
            builder: self.builder.field("format", format.to_str()),
        }
    }

    pub fn format_list(self, formats: impl IntoIterator<Item = AudioFormat>) -> Self {
        Self {
            builder: self.builder.field(
                "format",
                gst::List::new(formats.into_iter().map(|f| f.to_str())),
            ),
        }
    }

    pub fn rate(self, rate: i32) -> Self {
        Self {
            builder: self.builder.field("rate", rate),
        }
    }

    pub fn rate_range(self, rates: impl RangeBounds<i32>) -> Self {
        let (start, end) = range_bounds_i32_start_end(rates);
        let gst_rates = gst::IntRange::<i32>::new(start, end);
        Self {
            builder: self.builder.field("rate", gst_rates),
        }
    }

    pub fn rate_list(self, rates: impl IntoIterator<Item = i32>) -> Self {
        Self {
            builder: self.builder.field("rate", gst::List::new(rates)),
        }
    }

    pub fn channels(self, channels: i32) -> Self {
        Self {
            builder: self.builder.field("channels", channels),
        }
    }

    pub fn channels_range(self, channels: impl RangeBounds<i32>) -> Self {
        let (start, end) = range_bounds_i32_start_end(channels);
        let gst_channels: gst::IntRange<i32> = gst::IntRange::new(start, end);
        Self {
            builder: self.builder.field("channels", gst_channels),
        }
    }

    pub fn channels_list(self, channels: impl IntoIterator<Item = i32>) -> Self {
        Self {
            builder: self.builder.field("channels", gst::List::new(channels)),
        }
    }

    pub fn layout(self, layout: AudioLayout) -> Self {
        Self {
            builder: self.builder.field("layout", layout_str(layout)),
        }
    }

    pub fn layout_list(self, layouts: impl IntoIterator<Item = AudioLayout>) -> Self {
        Self {
            builder: self.builder.field(
                "layout",
                gst::List::new(layouts.into_iter().map(layout_str)),
            ),
        }
    }

    pub fn channel_mask(self, channel_mask: u64) -> Self {
        Self {
            builder: self
                .builder
                .field("channel-mask", gst::Bitmask::new(channel_mask)),
        }
    }

    pub fn fallback_channel_mask(self) -> Self {
        let channels = self.builder.structure().get::<i32>("channels");
        match channels {
            Ok(channels) => Self {
                builder: self.builder.field(
                    "channel-mask",
                    gst::Bitmask::new(crate::AudioChannelPosition::fallback_mask(channels as u32)),
                ),
            },
            Err(e) => panic!("{:?}", e),
        }
    }

    pub fn field<V: ToSendValue + Sync>(self, name: &str, value: V) -> Self {
        Self {
            builder: self.builder.field(name, value),
        }
    }

    #[must_use]
    pub fn build(self) -> gst::Caps {
        self.builder.build()
    }
}

fn range_bounds_i32_start_end(range: impl RangeBounds<i32>) -> (i32, i32) {
    skip_assert_initialized!();
    let start = match range.start_bound() {
        Unbounded => 1,
        Excluded(n) => n + 1,
        Included(n) => *n,
    };
    let end = match range.end_bound() {
        Unbounded => i32::MAX,
        Excluded(n) => n - 1,
        Included(n) => *n,
    };
    (start, end)
}

fn layout_str(layout: AudioLayout) -> &'static str {
    skip_assert_initialized!();
    match layout {
        crate::AudioLayout::Interleaved => "interleaved",
        crate::AudioLayout::NonInterleaved => "non-interleaved",
        crate::AudioLayout::__Unknown(_) => "unknown",
    }
}
