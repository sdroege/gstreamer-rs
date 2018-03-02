// Copyright (C) 2018 Thiago Santos <thiagossantos@gmail.com>
//                    Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use DiscovererStreamInfo;
use DiscovererStreamInfoExt;

pub struct Iter {
    stream_info: Option<DiscovererStreamInfo>,
    direction_forward: bool
}

impl Iterator for Iter {
    type Item = DiscovererStreamInfo;

    fn next(&mut self) -> Option<DiscovererStreamInfo> {
        let current = self.stream_info.take();
        self.stream_info = match &current {
            &Some(ref c) => {
                // Decide on the direction
                if self.direction_forward {
                    c.get_next()
                } else {
                    c.get_previous()
                }
            },
            &None => None
        };
        current
    }
}

impl DiscovererStreamInfo {
    pub fn next_iter(&self) -> Iter {
        Iter {
            stream_info: self.get_next(),
            direction_forward: true
        }
    }

    pub fn previous_iter(&self) -> Iter {
        Iter {
            stream_info: self.get_previous(),
            direction_forward: false
        }
    }
}
