// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;
use glib::translate::IntoGlib;

use crate::MediaSource;

impl MediaSource {
    pub fn set_position(&self, position: impl Into<Option<gst::ClockTime>>) {
        ObjectExt::set_property(self, "position", position.into().into_glib())
    }
}
