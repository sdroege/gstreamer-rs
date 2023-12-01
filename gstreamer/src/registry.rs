// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::{Plugin, PluginFeature, Registry};

impl Registry {
    #[doc(alias = "gst_registry_update")]
    pub fn update() -> Result<(), glib::BoolError> {
        crate::auto::functions::update_registry()
    }

    #[doc(alias = "gst_registry_feature_filter")]
    pub fn features_filtered<P: FnMut(&PluginFeature) -> bool>(
        &self,
        filter: P,
        first: bool,
    ) -> glib::List<PluginFeature> {
        let filter_data: P = filter;
        unsafe extern "C" fn filter_func<P: FnMut(&PluginFeature) -> bool>(
            feature: *mut ffi::GstPluginFeature,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let feature = from_glib_borrow(feature);
            let callback = user_data as *mut P;
            let res = (*callback)(&feature);
            res.into_glib()
        }
        let filter = Some(filter_func::<P> as _);
        let super_callback0: &P = &filter_data;
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_registry_feature_filter(
                self.to_glib_none().0,
                filter,
                first.into_glib(),
                super_callback0 as *const _ as *mut _,
            ))
        }
    }

    #[doc(alias = "gst_registry_get_feature_list")]
    #[doc(alias = "get_feature_list")]
    pub fn features(&self, type_: glib::types::Type) -> glib::List<PluginFeature> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_registry_get_feature_list(
                self.to_glib_none().0,
                type_.into_glib(),
            ))
        }
    }

    #[doc(alias = "gst_registry_get_feature_list_by_plugin")]
    #[doc(alias = "get_feature_list_by_plugin")]
    pub fn features_by_plugin(&self, name: &str) -> glib::List<PluginFeature> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_registry_get_feature_list_by_plugin(
                self.to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_registry_get_plugin_list")]
    #[doc(alias = "get_plugin_list")]
    pub fn plugins(&self) -> glib::List<Plugin> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_registry_get_plugin_list(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_registry_plugin_filter")]
    pub fn plugins_filtered<P: FnMut(&Plugin) -> bool>(
        &self,
        filter: P,
        first: bool,
    ) -> glib::List<Plugin> {
        let filter_data: P = filter;
        unsafe extern "C" fn filter_func<P: FnMut(&Plugin) -> bool>(
            plugin: *mut ffi::GstPlugin,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let plugin = from_glib_borrow(plugin);
            let callback = user_data as *const _ as *mut P;
            let res = (*callback)(&plugin);
            res.into_glib()
        }
        let filter = Some(filter_func::<P> as _);
        let super_callback0: &P = &filter_data;
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_registry_plugin_filter(
                self.to_glib_none().0,
                filter,
                first.into_glib(),
                super_callback0 as *const _ as *mut _,
            ))
        }
    }
}
