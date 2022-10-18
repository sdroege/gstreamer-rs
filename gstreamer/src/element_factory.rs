// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ELEMENT_METADATA_AUTHOR;
use crate::ELEMENT_METADATA_DESCRIPTION;
use crate::ELEMENT_METADATA_DOC_URI;
use crate::ELEMENT_METADATA_ICON_NAME;
use crate::ELEMENT_METADATA_KLASS;
use crate::ELEMENT_METADATA_LONGNAME;
use std::ffi::CStr;

#[cfg(any(feature = "v1_20", feature = "dox"))]
use crate::Element;
use crate::ElementFactory;
use crate::Rank;
use crate::StaticPadTemplate;

#[cfg(any(feature = "v1_20", feature = "dox"))]
use glib::prelude::*;
use glib::translate::*;

impl ElementFactory {
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_element_factory_create_with_properties")]
    #[track_caller]
    pub fn create_with_properties(
        &self,
        properties: &[(&str, &dyn ToValue)],
    ) -> Result<Element, glib::BoolError> {
        assert_initialized_main_thread!();

        // The below is basically a reimplementation of the C function. We want to call
        // glib::Object::with_type() ourselves here for checking properties and their values
        // correctly and to provide consistent behaviour.
        use crate::prelude::{
            ElementExtManual, GstObjectExt, GstObjectExtManual, PluginFeatureExtManual,
        };

        let factory = self.load().map_err(|_| {
            crate::warning!(crate::CAT_RUST, obj: self, "loading plugin returned None");
            glib::bool_error!("Failed to create element from factory")
        })?;

        let element_type = factory.element_type();
        if !element_type.is_valid() {
            crate::warning!(crate::CAT_RUST, obj: self, "factory has no type");
            return Err(glib::bool_error!("Failed to create element from factory"));
        }

        let element = glib::Object::with_type(element_type, properties)
            .downcast::<crate::Element>()
            .unwrap();
        unsafe {
            use std::sync::atomic;

            let klass = element.element_class();
            let factory_ptr: &atomic::AtomicPtr<ffi::GstElementFactory> =
                &*(&klass.as_ref().elementfactory as *const *mut ffi::GstElementFactory
                    as *const atomic::AtomicPtr<ffi::GstElementFactory>);
            if factory_ptr
                .compare_exchange(
                    std::ptr::null_mut(),
                    factory.as_ptr(),
                    atomic::Ordering::SeqCst,
                    atomic::Ordering::SeqCst,
                )
                .is_ok()
            {
                factory.set_object_flags(crate::ObjectFlags::MAY_BE_LEAKED);
            }

            if glib::gobject_ffi::g_object_is_floating(factory.as_ptr() as *mut _)
                != glib::ffi::GFALSE
            {
                glib::g_critical!(
                    "GStreamer",
                    "The created element should be floating, this is probably caused by faulty bindings",
                );
            }
        }

        crate::log!(
            crate::CAT_RUST,
            obj: self,
            "created element \"{}\"",
            factory.name()
        );

        Ok(element)
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_element_factory_make_with_properties")]
    #[track_caller]
    pub fn make_with_properties(
        factoryname: &str,
        properties: &[(&str, &dyn ToValue)],
    ) -> Result<Element, glib::BoolError> {
        assert_initialized_main_thread!();

        crate::log!(
            crate::CAT_RUST,
            "gstelementfactory: make \"{}\"",
            factoryname
        );

        let factory = Self::find(factoryname).ok_or_else(|| {
            crate::warning!(
                crate::CAT_RUST,
                "gstelementfactory: make \"{}\"",
                factoryname
            );
            glib::bool_error!("Failed to create element from factory name")
        })?;

        factory.create_with_properties(properties)
    }

    #[doc(alias = "gst_element_factory_get_static_pad_templates")]
    #[doc(alias = "get_static_pad_templates")]
    pub fn static_pad_templates(&self) -> glib::List<StaticPadTemplate> {
        unsafe {
            glib::List::from_glib_none_static(ffi::gst_element_factory_get_static_pad_templates(
                self.to_glib_none().0,
            ) as *mut _)
        }
    }

    #[doc(alias = "gst_element_factory_list_is_type")]
    pub fn has_type(&self, type_: crate::ElementFactoryType) -> bool {
        unsafe {
            from_glib(ffi::gst_element_factory_list_is_type(
                self.to_glib_none().0,
                type_.into_glib(),
            ))
        }
    }

    #[doc(alias = "gst_element_factory_list_get_elements")]
    pub fn factories_with_type(
        type_: crate::ElementFactoryType,
        minrank: Rank,
    ) -> glib::List<ElementFactory> {
        assert_initialized_main_thread!();
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_element_factory_list_get_elements(
                type_.into_glib(),
                minrank.into_glib(),
            ))
        }
    }

    #[doc(alias = "gst_element_factory_get_metadata")]
    #[doc(alias = "get_metadata")]
    pub fn metadata(&self, key: &str) -> Option<&str> {
        unsafe {
            let ptr =
                ffi::gst_element_factory_get_metadata(self.to_glib_none().0, key.to_glib_none().0);

            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap())
            }
        }
    }

    #[doc(alias = "get_longname")]
    #[doc(alias = "gst_element_factory_get_longname")]
    pub fn longname(&self) -> &str {
        self.metadata(&ELEMENT_METADATA_LONGNAME).unwrap()
    }

    #[doc(alias = "get_klass")]
    #[doc(alias = "gst_element_factory_get_klass")]
    pub fn klass(&self) -> &str {
        self.metadata(&ELEMENT_METADATA_KLASS).unwrap()
    }

    #[doc(alias = "get_description")]
    #[doc(alias = "gst_element_factory_get_description")]
    pub fn description(&self) -> &str {
        self.metadata(&ELEMENT_METADATA_DESCRIPTION).unwrap()
    }

    #[doc(alias = "get_author")]
    #[doc(alias = "gst_element_factory_get_author")]
    pub fn author(&self) -> &str {
        self.metadata(&ELEMENT_METADATA_AUTHOR).unwrap()
    }

    #[doc(alias = "get_documentation_uri")]
    #[doc(alias = "gst_element_factory_get_documentation_uri")]
    pub fn documentation_uri(&self) -> Option<&str> {
        self.metadata(&ELEMENT_METADATA_DOC_URI)
    }

    #[doc(alias = "get_icon_name")]
    #[doc(alias = "gst_element_factory_get_icon_name")]
    pub fn icon_name(&self) -> Option<&str> {
        self.metadata(&ELEMENT_METADATA_ICON_NAME)
    }
}
