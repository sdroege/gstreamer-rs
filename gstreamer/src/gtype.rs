// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use std::ffi::c_void;

pub trait PluginApiExt {
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_type_mark_as_plugin_api")]
    fn mark_as_plugin_api(self, flags: crate::PluginAPIFlags);
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_type_is_plugin_api")]
    fn plugin_api_flags(self) -> Option<crate::PluginAPIFlags>;
    #[doc(alias = "gst_element_type_set_skip_documentation")]
    fn set_skip_documentation(self);
    #[doc(alias = "gst_element_factory_get_skip_documentation")]
    fn skip_documentation(self) -> bool;
}

impl PluginApiExt for glib::Type {
    fn set_skip_documentation(self) {
        let quark = glib::Quark::from_str("GST_ELEMENTCLASS_SKIP_DOCUMENTATION");
        unsafe {
            crate::glib::gobject_ffi::g_type_set_qdata(
                self.into_glib(),
                quark.into_glib(),
                1 as *mut c_void,
            );
        }
    }

    fn skip_documentation(self) -> bool {
        let quark = glib::Quark::from_str("GST_ELEMENTCLASS_SKIP_DOCUMENTATION");
        unsafe {
            !crate::glib::gobject_ffi::g_type_get_qdata(self.into_glib(), quark.into_glib())
                .is_null()
        }
    }

    fn plugin_api_flags(self) -> Option<crate::PluginAPIFlags> {
        assert_initialized_main_thread!();
        unsafe {
            use std::mem;

            let mut flags = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_type_is_plugin_api(
                self.into_glib(),
                flags.as_mut_ptr(),
            ));
            if ret {
                Some(from_glib(flags.assume_init()))
            } else {
                None
            }
        }
    }

    fn mark_as_plugin_api(self, flags: crate::PluginAPIFlags) {
        assert_initialized_main_thread!();

        unsafe { ffi::gst_type_mark_as_plugin_api(self.into_glib(), flags.into_glib()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glib::StaticType;

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, glib::Enum)]
    #[repr(u32)]
    #[enum_type(name = "GstTestEnum")]
    pub enum TestEnum {
        #[enum_value(name = "test", nick = "test")]
        Test,
    }

    #[test]
    fn test_gtype_mark_as_api() {
        crate::init().unwrap();

        assert!(TestEnum::static_type().plugin_api_flags().is_none());
        assert!(!TestEnum::static_type().skip_documentation());

        TestEnum::static_type().mark_as_plugin_api(crate::PluginAPIFlags::empty());
        TestEnum::static_type().set_skip_documentation();

        assert!(
            TestEnum::static_type().plugin_api_flags().unwrap() == crate::PluginAPIFlags::empty()
        );
        assert!(TestEnum::static_type().skip_documentation());
    }
}
