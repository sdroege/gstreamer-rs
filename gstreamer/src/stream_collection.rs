// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib::translate::*;
use gst_sys;
use std::fmt;
use Stream;
use StreamCollection;

#[derive(Debug)]
pub struct Iter<'a> {
    collection: &'a StreamCollection,
    idx: u32,
    size: u32,
}

impl<'a> Iter<'a> {
    fn new(collection: &'a StreamCollection) -> Iter<'a> {
        skip_assert_initialized!();
        Iter {
            collection,
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

#[derive(Debug, Clone)]
pub struct StreamCollectionBuilder(StreamCollection);

impl StreamCollectionBuilder {
    pub fn stream(self, stream: &Stream) -> Self {
        unsafe {
            gst_sys::gst_stream_collection_add_stream(
                (self.0).to_glib_none().0,
                stream.to_glib_full(),
            );
        }

        self
    }

    pub fn streams<S: AsRef<Stream>>(self, streams: &[S]) -> Self {
        for stream in streams {
            unsafe {
                gst_sys::gst_stream_collection_add_stream(
                    (self.0).to_glib_none().0,
                    stream.as_ref().to_glib_full(),
                );
            }
        }

        self
    }

    pub fn build(self) -> StreamCollection {
        self.0
    }
}

impl StreamCollection {
    pub fn builder(upstream_id: Option<&str>) -> StreamCollectionBuilder {
        assert_initialized_main_thread!();
        let upstream_id = upstream_id.to_glib_none();
        let (major, minor, _, _) = ::version();
        let collection = if (major, minor) > (1, 12) {
            unsafe { from_glib_full(gst_sys::gst_stream_collection_new(upstream_id.0)) }
        } else {
            // Work-around for 1.14 switching from transfer-floating to transfer-full
            unsafe { from_glib_none(gst_sys::gst_stream_collection_new(upstream_id.0)) }
        };

        StreamCollectionBuilder(collection)
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }

    pub fn len(&self) -> usize {
        self.get_size() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn debug(&self) -> Debug {
        Debug(self)
    }
}

pub struct Debug<'a>(&'a StreamCollection);

impl<'a> fmt::Debug for Debug<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct Streams<'a>(&'a StreamCollection);

        impl<'a> fmt::Debug for Streams<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let mut f = f.debug_list();

                for stream in self.0.iter() {
                    f.entry(&stream.debug());
                }

                f.finish()
            }
        }

        let streams = Streams(self.0);

        f.debug_struct("StreamCollection")
            .field("streams", &streams)
            .finish()
    }
}
