// Take a look at the license at the top of the repository in the LICENSE file.

use crate::DiscovererStreamInfo;
use crate::DiscovererStreamInfoExt;

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

impl DiscovererStreamInfo {
    pub fn next_iter(&self) -> Iter {
        Iter {
            stream_info: self.next(),
            direction_forward: true,
        }
    }

    pub fn previous_iter(&self) -> Iter {
        Iter {
            stream_info: self.previous(),
            direction_forward: false,
        }
    }
}
