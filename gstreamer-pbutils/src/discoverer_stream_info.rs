// Take a look at the license at the top of the repository in the LICENSE file.
use std::fmt;

use crate::{prelude::*, DiscovererStreamInfo};

#[derive(Debug)]
pub struct Iter {
    stream_info: Option<DiscovererStreamInfo>,
    direction_forward: bool,
}

impl Iterator for Iter {
    type Item = DiscovererStreamInfo;

    fn next(&mut self) -> Option<DiscovererStreamInfo> {
        let current = self.stream_info.take();
        self.stream_info = match current {
            Some(ref c) => {
                // Decide on the direction
                if self.direction_forward {
                    c.next()
                } else {
                    c.previous()
                }
            }
            None => None,
        };
        current
    }
}

impl std::iter::FusedIterator for Iter {}

pub trait DiscovererStreamInfoExtManual: 'static {
    fn next_iter(&self) -> Iter;
    fn previous_iter(&self) -> Iter;
}

impl<O: IsA<DiscovererStreamInfo>> DiscovererStreamInfoExtManual for O {
    fn next_iter(&self) -> Iter {
        Iter {
            stream_info: self.next(),
            direction_forward: true,
        }
    }

    fn previous_iter(&self) -> Iter {
        Iter {
            stream_info: self.previous(),
            direction_forward: false,
        }
    }
}

pub struct Debug<'a>(&'a DiscovererStreamInfo);

impl<'a> fmt::Debug for Debug<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut d = f.debug_struct("DiscovererStreamInfo");
        d.field("caps", &self.0.caps())
            .field("stream-id", &self.0.stream_id())
            .field("misc", &self.0.misc())
            .field("stream-type-nick", &self.0.stream_type_nick())
            .field("tags", &self.0.tags())
            .field("toc", &self.0.toc());

        #[cfg(feature = "v1_20")]
        d.field("stream-number", &self.0.stream_number());

        d.finish()
    }
}

impl DiscovererStreamInfo {
    pub fn debug(&self) -> Debug {
        Debug(self)
    }
}
