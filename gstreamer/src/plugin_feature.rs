// Copyright (C) 2019 Guillaume Desmottes <guillaume.desmottes@collabora.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use PluginFeature;
use Rank;

use glib::object::IsA;
use glib::translate::{from_glib, ToGlib, ToGlibPtr};

pub trait PluginFeatureExtManual: 'static {
    fn get_rank(&self) -> Rank;
    fn set_rank(&self, rank: Rank);
}

impl<O: IsA<PluginFeature>> PluginFeatureExtManual for O {
    fn get_rank(&self) -> Rank {
        unsafe {
            let rank = gst_sys::gst_plugin_feature_get_rank(self.as_ref().to_glib_none().0);
            from_glib(rank as i32)
        }
    }

    fn set_rank(&self, rank: Rank) {
        unsafe {
            gst_sys::gst_plugin_feature_set_rank(
                self.as_ref().to_glib_none().0,
                rank.to_glib() as u32,
            );
        }
    }
}
