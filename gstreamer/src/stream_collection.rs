// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use Stream;
use StreamCollection;

pub struct Iter<'a> {
    collection: &'a StreamCollection,
    idx: u32,
    size: u32,
}

impl<'a> Iter<'a> {
    fn new(collection: &'a StreamCollection) -> Iter<'a> {
        skip_assert_initialized!();
        Iter {
            collection: collection,
            idx: 0,
            size: collection.len() as u32,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Stream;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.size {
            return None;
        }

        let item = self.collection.get_stream(self.idx);
        self.idx += 1;

        item
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.idx == self.size {
            return (0, Some(0));
        }

        let remaining = (self.size - self.idx) as usize;

        (remaining, Some(remaining))
    }
}

impl<'a> DoubleEndedIterator for Iter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.idx == self.size {
            return None;
        }

        self.size -= 1;
        self.collection.get_stream(self.size)
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {}

impl StreamCollection {
    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }

    pub fn len(&self) -> usize {
        self.get_size() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
