// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

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

    fn stream_id(&self) -> glib::GString {
        unsafe {
            let ptr = ffi::gst_discoverer_stream_info_get_stream_id(self.as_ref().to_glib_none().0);

            if ptr.is_null() {
                glib::GString::new()
            } else {
                from_glib_none(ptr)
            }
        }
    }
}

impl std::iter::FusedIterator for Iter {}
