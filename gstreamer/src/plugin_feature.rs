// Take a look at the license at the top of the repository in the LICENSE file.

use crate::PluginFeature;
use crate::Rank;

use glib::object::IsA;
use glib::translate::{from_glib, ToGlib, ToGlibPtr};

pub trait PluginFeatureExtManual: 'static {
    fn get_rank(&self) -> Rank;
    fn set_rank(&self, rank: Rank);
}

impl<O: IsA<PluginFeature>> PluginFeatureExtManual for O {
    fn get_rank(&self) -> Rank {
        unsafe {
            let rank = ffi::gst_plugin_feature_get_rank(self.as_ref().to_glib_none().0);
            from_glib(rank as i32)
        }
    }

    fn set_rank(&self, rank: Rank) {
        unsafe {
            ffi::gst_plugin_feature_set_rank(self.as_ref().to_glib_none().0, rank.to_glib() as u32);
        }
    }
}
