// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{
    prelude::*,
    translate::{FromGlibPtrFull, IntoGlib, ToGlibPtr, from_glib},
};

use crate::{PluginFeature, Rank, ffi};

pub trait PluginFeatureExtManual: IsA<PluginFeature> + 'static {
    #[doc(alias = "get_rank")]
    #[doc(alias = "gst_plugin_feature_get_rank")]
    fn rank(&self) -> Rank {
        unsafe {
            let rank = ffi::gst_plugin_feature_get_rank(self.as_ref().to_glib_none().0);
            from_glib(rank as i32)
        }
    }

    #[doc(alias = "gst_plugin_feature_set_rank")]
    fn set_rank(&self, rank: Rank) {
        unsafe {
            ffi::gst_plugin_feature_set_rank(
                self.as_ref().to_glib_none().0,
                rank.into_glib() as u32,
            );
        }
    }

    #[doc(alias = "gst_plugin_feature_load")]
    fn load(&self) -> Result<Self, glib::BoolError> {
        unsafe {
            let loaded = Option::<PluginFeature>::from_glib_full(ffi::gst_plugin_feature_load(
                self.as_ref().to_glib_none().0,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to load plugin feature"))?;
            Ok(loaded.unsafe_cast())
        }
    }
}

impl<O: IsA<PluginFeature>> PluginFeatureExtManual for O {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        crate::init().unwrap();

        let factory = crate::ElementFactory::find("identity").unwrap();
        let loaded = factory.load().unwrap();
        assert_eq!(factory.type_(), loaded.type_());
        let _element = loaded.create().build().unwrap();
    }
}
