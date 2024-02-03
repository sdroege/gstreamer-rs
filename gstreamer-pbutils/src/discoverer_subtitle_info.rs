// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

use glib::prelude::*;

use crate::{DiscovererStreamInfo, DiscovererSubtitleInfo};

pub struct Debug<'a>(&'a DiscovererSubtitleInfo);

impl<'a> fmt::Debug for Debug<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let info = self.0.upcast_ref::<DiscovererStreamInfo>();

        f.debug_struct("DiscovererSubtitleInfo")
            .field("language", &self.0.language())
            .field("stream", &info.debug())
            .finish()
    }
}

impl DiscovererSubtitleInfo {
    pub fn debug(&self) -> Debug {
        Debug(self)
    }
}
