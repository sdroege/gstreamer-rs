// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

use glib::Cast;

use crate::{DiscovererAudioInfo, DiscovererStreamInfo};

pub struct Debug<'a>(&'a DiscovererAudioInfo);

impl<'a> fmt::Debug for Debug<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let info = self.0.upcast_ref::<DiscovererStreamInfo>();

        f.debug_struct("DiscovererAudioInfo")
            .field("channels", &self.0.channels())
            .field("sample-rate", &self.0.sample_rate())
            .field("depth", &self.0.depth())
            .field("bitrate", &self.0.bitrate())
            .field("max-bitrate", &self.0.max_bitrate())
            .field("channel-mask", &self.0.channel_mask())
            .field("language", &self.0.language())
            .field("stream", &info.debug())
            .finish()
    }
}

impl DiscovererAudioInfo {
    pub fn debug(&self) -> Debug {
        Debug(self)
    }
}
