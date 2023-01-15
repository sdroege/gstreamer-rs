// Take a look at the license at the top of the repository in the LICENSE file.

use std::ffi::CStr;

use glib::{prelude::*, translate::*};

use crate::{
    CapsRef, Element, ElementFactory, Rank, StaticPadTemplate, ELEMENT_METADATA_AUTHOR,
    ELEMENT_METADATA_DESCRIPTION, ELEMENT_METADATA_DOC_URI, ELEMENT_METADATA_ICON_NAME,
    ELEMENT_METADATA_KLASS, ELEMENT_METADATA_LONGNAME,
};

impl ElementFactory {
    #[doc(alias = "gst_element_factory_create_with_properties")]
    #[track_caller]
    pub fn create_with_properties(
        &self,
        properties: &[(&str, &dyn ToValue)],
    ) -> Result<Element, glib::BoolError> {
        let mut builder = self.create();
        builder.properties = properties
            .iter()
            .map(|(name, value)| (*name, ValueOrStr::Value(value.to_value())))
            .collect();
        builder.build()
    }

    #[doc(alias = "gst_element_factory_make_with_properties")]
    #[track_caller]
    pub fn make_with_properties(
        factoryname: &str,
        properties: &[(&str, &dyn ToValue)],
    ) -> Result<Element, glib::BoolError> {
        skip_assert_initialized!();
        let mut builder = Self::make(factoryname);
        builder.properties = properties
            .iter()
            .map(|(name, value)| (*name, ValueOrStr::Value(value.to_value())))
            .collect();
        builder.build()
    }

    #[doc(alias = "gst_element_factory_create")]
    #[doc(alias = "gst_element_factory_create_with_properties")]
    #[track_caller]
    pub fn create(&self) -> ElementBuilder {
        assert_initialized_main_thread!();

        ElementBuilder {
            name_or_factory: NameOrFactory::Factory(self),
            properties: Vec::new(),
        }
    }

    #[doc(alias = "gst_element_factory_make")]
    #[doc(alias = "gst_element_factory_make_with_properties")]
    #[track_caller]
    pub fn make(factoryname: &str) -> ElementBuilder {
        assert_initialized_main_thread!();

        ElementBuilder {
            name_or_factory: NameOrFactory::Name(factoryname),
            properties: Vec::new(),
        }
    }

    #[doc(alias = "gst_element_factory_create")]
    #[track_caller]
    pub fn create_with_name(&self, name: Option<&str>) -> Result<Element, glib::BoolError> {
        let mut builder = self.create();
        if let Some(name) = name {
            builder = builder.name(name);
        }
        builder.build()
    }

    #[doc(alias = "gst_element_factory_make")]
    #[track_caller]
    pub fn make_with_name(
        factoryname: &str,
        name: Option<&str>,
    ) -> Result<Element, glib::BoolError> {
        skip_assert_initialized!();
        let mut builder = Self::make(factoryname);
        if let Some(name) = name {
            builder = builder.name(name);
        }
        builder.build()
    }

    #[doc(alias = "gst_element_factory_get_static_pad_templates")]
    #[doc(alias = "get_static_pad_templates")]
    pub fn static_pad_templates(&self) -> glib::List<StaticPadTemplate> {
        unsafe {
            glib::List::from_glib_none(ffi::gst_element_factory_get_static_pad_templates(
                self.to_glib_none().0,
            ))
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
        self.metadata(ELEMENT_METADATA_LONGNAME).unwrap()
    }

    #[doc(alias = "get_klass")]
    #[doc(alias = "gst_element_factory_get_klass")]
    pub fn klass(&self) -> &str {
        self.metadata(ELEMENT_METADATA_KLASS).unwrap()
    }

    #[doc(alias = "get_description")]
    #[doc(alias = "gst_element_factory_get_description")]
    pub fn description(&self) -> &str {
        self.metadata(ELEMENT_METADATA_DESCRIPTION).unwrap()
    }

    #[doc(alias = "get_author")]
    #[doc(alias = "gst_element_factory_get_author")]
    pub fn author(&self) -> &str {
        self.metadata(ELEMENT_METADATA_AUTHOR).unwrap()
    }

    #[doc(alias = "get_documentation_uri")]
    #[doc(alias = "gst_element_factory_get_documentation_uri")]
    pub fn documentation_uri(&self) -> Option<&str> {
        self.metadata(ELEMENT_METADATA_DOC_URI)
    }

    #[doc(alias = "get_icon_name")]
    #[doc(alias = "gst_element_factory_get_icon_name")]
    pub fn icon_name(&self) -> Option<&str> {
        self.metadata(ELEMENT_METADATA_ICON_NAME)
    }

    #[doc(alias = "gst_element_factory_can_sink_all_caps")]
    pub fn can_sink_all_caps(&self, caps: &CapsRef) -> bool {
        unsafe {
            from_glib(ffi::gst_element_factory_can_sink_all_caps(
                self.to_glib_none().0,
                caps.as_ptr(),
            ))
        }
    }

    #[doc(alias = "gst_element_factory_can_sink_any_caps")]
    pub fn can_sink_any_caps(&self, caps: &CapsRef) -> bool {
        unsafe {
            from_glib(ffi::gst_element_factory_can_sink_any_caps(
                self.to_glib_none().0,
                caps.as_ptr(),
            ))
        }
    }

    #[doc(alias = "gst_element_factory_can_src_all_caps")]
    pub fn can_src_all_caps(&self, caps: &CapsRef) -> bool {
        unsafe {
            from_glib(ffi::gst_element_factory_can_src_all_caps(
                self.to_glib_none().0,
                caps.as_ptr(),
            ))
        }
    }

    #[doc(alias = "gst_element_factory_can_src_any_caps")]
    pub fn can_src_any_caps(&self, caps: &CapsRef) -> bool {
        unsafe {
            from_glib(ffi::gst_element_factory_can_src_any_caps(
                self.to_glib_none().0,
                caps.as_ptr(),
            ))
        }
    }
}

// rustdoc-stripper-ignore-next
/// Builder for `Element`s.
#[must_use = "The builder must be built to be used"]
pub struct ElementBuilder<'a> {
    name_or_factory: NameOrFactory<'a>,
    properties: Vec<(&'a str, ValueOrStr<'a>)>,
}

#[derive(Copy, Clone)]
enum NameOrFactory<'a> {
    Name(&'a str),
    Factory(&'a ElementFactory),
}

enum ValueOrStr<'a> {
    Value(glib::Value),
    Str(&'a str),
}

impl<'a> ElementBuilder<'a> {
    // rustdoc-stripper-ignore-next
    /// Sets the name property to the given `name`.
    pub fn name(self, name: &'a str) -> Self {
        self.property("name", name)
    }

    // rustdoc-stripper-ignore-next
    /// Set property `name` to the given value `value`.
    pub fn property(self, name: &'a str, value: impl Into<glib::Value> + 'a) -> Self {
        Self {
            name_or_factory: self.name_or_factory,
            properties: {
                let mut properties = self.properties;
                properties.push((name, ValueOrStr::Value(value.into())));
                properties
            },
        }
    }

    // rustdoc-stripper-ignore-next
    /// Set property `name` to the given string value `value`.
    pub fn property_from_str(self, name: &'a str, value: &'a str) -> Self {
        Self {
            name_or_factory: self.name_or_factory,
            properties: {
                let mut properties = self.properties;
                properties.push((name, ValueOrStr::Str(value)));
                properties
            },
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the element with the provided properties.
    ///
    /// This fails if there is no such element factory or the element factory can't be loaded.
    ///
    /// # Panics
    ///
    /// This panics if the element is not instantiable, doesn't have all the given properties or
    /// property values of the wrong type are provided.
    #[track_caller]
    #[must_use = "Building the element without using it has no effect"]
    pub fn build(self) -> Result<Element, glib::BoolError> {
        let mut _factory_found = None;
        let factory = match self.name_or_factory {
            NameOrFactory::Name(name) => {
                let factory = ElementFactory::find(name).ok_or_else(|| {
                    crate::warning!(crate::CAT_RUST, "element factory '{}' not found", name);
                    glib::bool_error!(
                        "Failed to find element factory with name '{}' for creating element",
                        name
                    )
                })?;
                _factory_found = Some(factory);
                _factory_found.as_ref().unwrap()
            }
            NameOrFactory::Factory(factory) => factory,
        };

        // The below is basically a reimplementation of the C function. We want to call
        // glib::Object::with_type() ourselves here for checking properties and their values
        // correctly and to provide consistent behaviour.
        use crate::prelude::{
            ElementExtManual, GstObjectExt, GstObjectExtManual, PluginFeatureExtManual,
        };

        let factory = factory.load().map_err(|_| {
            crate::warning!(
                crate::CAT_RUST,
                obj: factory,
                "loading element factory '{}' failed",
                factory.name(),
            );
            glib::bool_error!(
                "Failed to load element factory '{}' for creating element",
                factory.name()
            )
        })?;

        let element_type = factory.element_type();
        if !element_type.is_valid() {
            crate::warning!(
                crate::CAT_RUST,
                obj: &factory,
                "element factory '{}' has no type",
                factory.name()
            );
            return Err(glib::bool_error!(
                "Failed to create element from factory '{}'",
                factory.name()
            ));
        }

        let mut properties = Vec::with_capacity(self.properties.len());
        let klass = glib::Class::<Element>::from_type(element_type).unwrap();
        for (name, value) in self.properties {
            match value {
                ValueOrStr::Value(value) => {
                    properties.push((name, value));
                }
                ValueOrStr::Str(value) => {
                    use crate::value::GstValueExt;

                    let pspec = match klass.find_property(name) {
                        Some(pspec) => pspec,
                        None => {
                            panic!(
                                "property '{}' of element factory '{}' not found",
                                name,
                                factory.name()
                            );
                        }
                    };

                    let value = {
                        if pspec.value_type() == crate::Structure::static_type() && value == "NULL"
                        {
                            None::<crate::Structure>.to_value()
                        } else {
                            #[cfg(feature = "v1_20")]
                            {
                                glib::Value::deserialize_with_pspec(value, &pspec)
                                    .unwrap_or_else(|_| {
                                        panic!(
                                            "property '{}' of element factory '{}' can't be set from string '{}'",
                                            name,
                                            factory.name(),
                                            value,
                                        )
                                    })
                            }
                            #[cfg(not(feature = "v1_20"))]
                            {
                                glib::Value::deserialize(value, pspec.value_type())
                                    .unwrap_or_else(|_| {
                                        panic!(
                                            "property '{}' of element factory '{}' can't be set from string '{}'",
                                            name,
                                            factory.name(),
                                            value,
                                        )
                                    })
                            }
                        }
                    };

                    properties.push((name, value));
                }
            }
        }

        let element = glib::Object::with_values(element_type, &properties)
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
            obj: &factory,
            "created element \"{}\"",
            factory.name()
        );

        Ok(element)
    }
}
