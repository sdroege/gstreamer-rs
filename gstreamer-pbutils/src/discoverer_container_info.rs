// Take a look at the license at the top of the repository in the LICENSE file.
use std::fmt;

use crate::{prelude::*, DiscovererContainerInfo};

pub struct Debug<'a>(&'a DiscovererContainerInfo);

impl<'a> fmt::Debug for Debug<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let streams = self.0.streams();

        let mut d = f.debug_struct("DiscovererContainerInfo");

        d.field("tags", &self.0.tags()).field(
            "streams",
            &streams.iter().map(|info| info.debug()).collect::<Vec<_>>(),
        );

        #[cfg(feature = "v1_20")]
        d.field("stream-number", &self.0.stream_number());
        #[cfg(feature = "v1_20")]
        d.field("tags", &self.0.tags());

        d.finish()
    }
}

impl DiscovererContainerInfo {
    pub fn debug(&self) -> Debug {
        Debug(self)
    }
}
