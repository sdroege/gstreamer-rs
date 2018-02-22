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

pub struct DiscovererStreamInfoIterator {
    stream_info: Option<DiscovererStreamInfo>
}

impl Iterator for DiscovererStreamInfoIterator {
    type Item = DiscovererStreamInfo;

    fn next(&mut self) -> Option<DiscovererStreamInfo> {
        let current = self.stream_info.take();
        self.stream_info = match &current {
            &Some(ref c) => c.get_next(),
            &None => None
        };
        current
    }
}

impl DiscovererStreamInfo {
    pub fn next_iter(&self) -> DiscovererStreamInfoIterator {
        DiscovererStreamInfoIterator {
            stream_info: self.get_next()
        }
    }
}
