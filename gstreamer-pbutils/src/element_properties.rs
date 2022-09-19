use gst::prelude::*;

use std::ops::{Deref, DerefMut};

// rustdoc-stripper-ignore-next
/// Wrapper around `gst::Structure` for `element-properties`
/// property of `EncodingProfile`.
///
/// # Examples
///
/// ```rust
/// # use gstreamer_pbutils::ElementProperties;
/// # gst::init().unwrap();
/// ElementProperties::builder_general()
///     .field("threads", 16)
///     .build();
/// ```
///
/// ```rust
/// # use gstreamer_pbutils::{ElementProperties, ElementPropertiesMapItem};
/// # gst::init().unwrap();
/// ElementProperties::builder_map()
///     .item(
///         ElementPropertiesMapItem::builder("vp8enc")
///             .field("max-quantizer", 17)
///             .field("buffer-size", 20000)
///             .field("threads", 16)
///             .build(),
///     )
///     .build();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementProperties(pub(crate) gst::Structure);

impl Default for ElementProperties {
    fn default() -> Self {
        Self::builder_general().build()
    }
}

impl Deref for ElementProperties {
    type Target = gst::StructureRef;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl From<ElementProperties> for gst::Structure {
    fn from(e: ElementProperties) -> Self {
        skip_assert_initialized!();

        e.into_inner()
    }
}

impl ElementProperties {
    // rustdoc-stripper-ignore-next
    /// Creates an `ElementProperties` builder that build into
    /// something similar to the following:
    ///
    /// [element-properties, boolean-prop=true, string-prop="hi"]
    pub fn builder_general() -> ElementPropertiesGeneralBuilder {
        assert_initialized_main_thread!();

        ElementPropertiesGeneralBuilder {
            structure: gst::Structure::new_empty("element-properties"),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates an `ElementProperties` builder that build into
    /// something similar to the following:
    ///
    /// element-properties-map, map = {
    ///     [openh264enc, gop-size=32, ],
    ///     [x264enc, key-int-max=32, tune=zerolatency],
    /// }
    pub fn builder_map() -> ElementPropertiesMapBuilder {
        assert_initialized_main_thread!();

        ElementPropertiesMapBuilder { map: Vec::new() }
    }

    // rustdoc-stripper-ignore-next
    /// Returns true if self is built with `ElementPropertiesGeneralBuilder`.
    pub fn is_general(&self) -> bool {
        let structure_name = self.0.name();

        if structure_name != "element-properties" {
            assert_eq!(structure_name, "element-properties-map");
            return false;
        }

        true
    }

    // rustdoc-stripper-ignore-next
    /// Returns true if self is built with `ElementPropertiesMapBuilder`.
    pub fn is_map(&self) -> bool {
        !self.is_general()
    }

    // rustdoc-stripper-ignore-next
    /// Returns the inner vec of `ElementPropertiesMapItem` if self is_map()
    /// or `None` if self is_general().
    pub fn map(&self) -> Option<Vec<ElementPropertiesMapItem>> {
        if !self.is_map() {
            return None;
        }

        Some(
            self.0
                .get::<gst::List>("map")
                .unwrap()
                .as_slice()
                .iter()
                .map(|props_map| {
                    ElementPropertiesMapItem(props_map.get::<gst::Structure>().unwrap())
                })
                .collect::<Vec<_>>(),
        )
    }

    pub fn into_inner(self) -> gst::Structure {
        self.0
    }
}

#[must_use = "The builder must be built to be used"]
#[derive(Debug, Clone)]
pub struct ElementPropertiesGeneralBuilder {
    structure: gst::Structure,
}

impl ElementPropertiesGeneralBuilder {
    pub fn field<T>(mut self, property_name: &str, value: T) -> Self
    where
        T: ToSendValue + Sync,
    {
        self.structure.set(property_name, value);
        self
    }

    pub fn field_value(mut self, property_name: &str, value: glib::SendValue) -> Self {
        self.structure.set_value(property_name, value);
        self
    }

    pub fn build(self) -> ElementProperties {
        ElementProperties(self.structure)
    }
}

#[must_use = "The builder must be built to be used"]
#[derive(Debug, Clone)]
pub struct ElementPropertiesMapBuilder {
    map: Vec<glib::SendValue>,
}

impl ElementPropertiesMapBuilder {
    pub fn item(mut self, item: ElementPropertiesMapItem) -> Self {
        self.map.push(item.into_inner().to_send_value());
        self
    }

    pub fn build(self) -> ElementProperties {
        ElementProperties(
            gst::Structure::builder("element-properties-map")
                .field("map", gst::List::from(self.map))
                .build(),
        )
    }
}

// rustdoc-stripper-ignore-next
/// Wrapper around `gst::Structure` for `element-properties-map` map item.
///
/// # Examples
///
/// ```rust
/// # use gstreamer_pbutils::{ElementProperties, ElementPropertiesMapItem};
/// # gst::init().unwrap();
/// ElementProperties::builder_map()
///     .item(
///         ElementPropertiesMapItem::builder("vp8enc")
///             .field("max-quantizer", 17)
///             .field("buffer-size", 20000)
///             .field("threads", 16)
///             .build(),
///     )
///     .build();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementPropertiesMapItem(gst::Structure);

impl Deref for ElementPropertiesMapItem {
    type Target = gst::StructureRef;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl DerefMut for ElementPropertiesMapItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}

impl From<ElementPropertiesMapItem> for gst::Structure {
    fn from(e: ElementPropertiesMapItem) -> Self {
        skip_assert_initialized!();

        e.into_inner()
    }
}

impl ElementPropertiesMapItem {
    pub fn builder(factory_name: &str) -> ElementPropertiesMapItemBuilder {
        assert_initialized_main_thread!();

        ElementPropertiesMapItemBuilder {
            structure: gst::Structure::new_empty(factory_name),
        }
    }

    pub fn into_inner(self) -> gst::Structure {
        self.0
    }
}

#[must_use = "The builder must be built to be used"]
#[derive(Debug, Clone)]
pub struct ElementPropertiesMapItemBuilder {
    structure: gst::Structure,
}

impl ElementPropertiesMapItemBuilder {
    pub fn field<T>(mut self, property_name: &str, value: T) -> Self
    where
        T: ToSendValue + Sync,
    {
        self.structure.set(property_name, value);
        self
    }

    pub fn field_value(mut self, property_name: &str, value: glib::SendValue) -> Self {
        self.structure.set_value(property_name, value);
        self
    }

    pub fn build(self) -> ElementPropertiesMapItem {
        ElementPropertiesMapItem(self.structure)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn element_properties_getters() {
        gst::init().unwrap();

        let elem_props_general = ElementProperties::builder_general()
            .field("string-prop", "hi")
            .field("boolean-prop", true)
            .build();
        assert!(elem_props_general.is_general());
        assert!(!elem_props_general.is_map());
        assert_eq!(elem_props_general.map(), None);

        let elem_factory_props_map = ElementPropertiesMapItem::builder("vp8enc")
            .field("cq-level", 13)
            .field("resize-allowed", false)
            .build();
        let elem_props_map = ElementProperties::builder_map()
            .item(elem_factory_props_map.clone())
            .build();
        assert!(elem_props_map.is_map());
        assert!(!elem_props_map.is_general());
        assert_eq!(elem_props_map.map(), Some(vec![elem_factory_props_map]));
    }

    #[test]
    fn element_properties_general_builder() {
        gst::init().unwrap();

        let elem_props = ElementProperties::builder_general()
            .field("string-prop", "hi")
            .field("boolean-prop", true)
            .build();
        assert_eq!(elem_props.n_fields(), 2);
        assert_eq!(elem_props.name(), "element-properties");
        assert_eq!(elem_props.get::<String>("string-prop").unwrap(), "hi");
        assert!(elem_props.get::<bool>("boolean-prop").unwrap());
    }

    #[test]
    fn element_properties_map_builder() {
        gst::init().unwrap();

        let props_map = ElementPropertiesMapItem::builder("vp8enc")
            .field("cq-level", 13)
            .field("resize-allowed", false)
            .build();
        assert_eq!(props_map.n_fields(), 2);
        assert_eq!(props_map.name(), "vp8enc");
        assert_eq!(props_map.get::<i32>("cq-level").unwrap(), 13);
        assert!(!props_map.get::<bool>("resize-allowed").unwrap());

        let elem_props = ElementProperties::builder_map()
            .item(props_map.clone())
            .build();
        assert_eq!(elem_props.n_fields(), 1);

        let list = elem_props.map().unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list.get(0).unwrap(), &props_map);
    }
}
