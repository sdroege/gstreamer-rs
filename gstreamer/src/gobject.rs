// Take a look at the license at the top of the repository in the LICENSE file.

use std::marker::PhantomData;

use glib::{Type, object::IsClass, prelude::*};

use crate::{IdStr, value::GstValueExt};

impl crate::Object {
    // rustdoc-stripper-ignore-next
    /// Builds a `GObjectBuilder` targeting type `O`.
    #[inline]
    pub fn builder<'a, O>() -> GObjectBuilder<'a, O>
    where
        O: IsA<crate::Object> + IsClass,
    {
        assert_initialized_main_thread!();
        GObjectBuilder {
            type_: Some(O::static_type()),
            properties: smallvec::SmallVec::new(),
            phantom: PhantomData,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Builds a `GObjectBuilder` targeting base class of type `O` and concrete `type_`.
    #[inline]
    pub fn builder_for<'a, O>(type_: Type) -> GObjectBuilder<'a, O>
    where
        O: IsA<crate::Object> + IsClass,
    {
        assert_initialized_main_thread!();
        GObjectBuilder {
            type_: Some(type_),
            properties: smallvec::SmallVec::new(),
            phantom: PhantomData,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Builds a `GObjectBuilder` targeting base class of type `O`
    /// and a concrete `Type` that will be specified later.
    ///
    /// This is useful when the concrete type of the object is dynamically determined
    /// when calling the `build()` method of a wrapping builder.
    #[inline]
    pub fn builder_for_deferred_type<'a, O>() -> GObjectBuilder<'a, O>
    where
        O: IsA<crate::Object> + IsClass,
    {
        assert_initialized_main_thread!();
        GObjectBuilder {
            type_: None,
            properties: smallvec::SmallVec::new(),
            phantom: PhantomData,
        }
    }
}

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum GObjectError {
    #[error("property {property} for type {type_} not found")]
    PropertyNotFound { type_: Type, property: IdStr },

    #[error("property {property} for type {type_} can't be set from string {value}")]
    PropertyFromStr {
        type_: Type,
        property: IdStr,
        value: IdStr,
    },
}

fn value_from_property_str(
    pspec: glib::ParamSpec,
    value: &str,
) -> Result<glib::Value, GObjectError> {
    skip_assert_initialized!(); // Already checked transitively by caller

    if pspec.value_type() == crate::Structure::static_type() && value == "NULL" {
        Ok(None::<crate::Structure>.to_value())
    } else {
        cfg_if::cfg_if! {
            if #[cfg(feature = "v1_20")] {
                let res = glib::Value::deserialize_with_pspec(value, &pspec);
            } else {
                let res = glib::Value::deserialize(value, pspec.value_type());
            }
        }
        res.map_err(|_| GObjectError::PropertyFromStr {
            type_: pspec.owner_type(),
            property: pspec.name().into(),
            value: value.into(),
        })
    }
}

pub trait GObjectExtManualGst: IsA<glib::Object> + 'static {
    #[doc(alias = "gst_util_set_object_arg")]
    #[track_caller]
    fn set_property_from_str(&self, name: &str, value: &str) {
        let pspec = self.find_property(name).unwrap_or_else(|| {
            panic!("property '{}' of type '{}' not found", name, self.type_());
        });

        self.set_property(name, value_from_property_str(pspec, value).unwrap())
    }
}

impl<O: IsA<glib::Object>> GObjectExtManualGst for O {}

// rustdoc-stripper-ignore-next
/// Builder for `GObject`s.
#[must_use = "The builder must be built to be used"]
pub struct GObjectBuilder<'a, O> {
    type_: Option<Type>,
    properties: smallvec::SmallVec<[(&'a str, ValueOrStr<'a>); 16]>,
    phantom: PhantomData<O>,
}

enum ValueOrStr<'a> {
    Value(glib::Value),
    Str(&'a str),
}

impl<'a, O: IsA<crate::Object> + IsClass> GObjectBuilder<'a, O> {
    // rustdoc-stripper-ignore-next
    /// Sets the concrete `Type`.
    ///
    /// This should be used on an `GObjectBuilder` created with
    /// [`GObjectBuilder::for_deferred_type`].
    #[inline]
    pub fn type_(mut self, type_: Type) -> Self {
        self.type_ = Some(type_);
        self
    }

    // rustdoc-stripper-ignore-next
    /// Sets property `name` to the given value `value`.
    ///
    /// Overrides any default or previously defined value for `name`.
    #[inline]
    pub fn property(self, name: &'a str, value: impl Into<glib::Value> + 'a) -> Self {
        Self {
            properties: {
                let mut properties = self.properties;
                properties.push((name, ValueOrStr::Value(value.into())));
                properties
            },
            ..self
        }
    }

    // rustdoc-stripper-ignore-next
    /// Sets property `name` to the given string value `value`.
    #[inline]
    pub fn property_from_str(self, name: &'a str, value: &'a str) -> Self {
        Self {
            properties: {
                let mut properties = self.properties;
                properties.push((name, ValueOrStr::Str(value)));
                properties
            },
            ..self
        }
    }

    impl_builder_gvalue_extra_setters!(property_and_name);

    // rustdoc-stripper-ignore-next
    /// Builds the [`Object`] with the provided properties.
    ///
    /// This fails if there is no such element factory or the element factory can't be loaded.
    ///
    /// # Panics
    ///
    /// This panics if:
    ///
    /// * The [`Object`] is not instantiable, doesn't have all the given properties or
    ///   property values of the wrong type are provided.
    /// * The [`GObjectBuilder`] was created for a deferred concrete `Type` but
    ///   the `Type` was not set.
    ///
    /// [`Object`]: crate::Object
    #[track_caller]
    #[must_use = "Building the element without using it has no effect"]
    pub fn build(self) -> Result<O, GObjectError> {
        let type_ = self.type_.expect("Deferred Type must be set");

        let mut properties = smallvec::SmallVec::<[_; 16]>::with_capacity(self.properties.len());
        let klass = glib::Class::<O>::from_type(type_).unwrap();
        for (name, value) in self.properties {
            let pspec =
                klass
                    .find_property(name)
                    .ok_or_else(|| GObjectError::PropertyNotFound {
                        type_,
                        property: name.into(),
                    })?;

            match value {
                ValueOrStr::Value(value) => properties.push((name, value)),
                ValueOrStr::Str(value) => {
                    properties.push((name, value_from_property_str(pspec, value)?));
                }
            }
        }

        let object =
            unsafe { glib::Object::with_mut_values(type_, &mut properties).unsafe_cast::<O>() };

        Ok(object)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Bin, Element, ElementFactory, Object, prelude::*};

    #[test]
    fn test_set_property_from_str() {
        crate::init().unwrap();

        let fakesink = ElementFactory::make("fakesink").build().unwrap();
        fakesink.set_property_from_str("state-error", "ready-to-paused");
        let v = fakesink.property_value("state-error");
        let (_klass, e) = glib::EnumValue::from_value(&v).unwrap();
        assert_eq!(e.nick(), "ready-to-paused");
    }

    #[test]
    fn builder() {
        crate::init().unwrap();

        let msg_fwd = "message-forward";
        let bin = Object::builder::<Bin>()
            .name("test-bin")
            .property("async-handling", true)
            .property_from_str(msg_fwd, "True")
            .build()
            .unwrap();

        assert_eq!(bin.name(), "test-bin");
        assert!(bin.property::<bool>("async-handling"));
        assert!(bin.property::<bool>("message-forward"));
    }

    #[test]
    fn builder_err() {
        crate::init().unwrap();

        assert_eq!(
            Object::builder::<Bin>()
                .property("not-a-prop", true)
                .build(),
            Err(GObjectError::PropertyNotFound {
                type_: Bin::static_type(),
                property: idstr!("not-a-prop")
            })
        );

        assert_eq!(
            Object::builder::<Bin>()
                .property_from_str("async-handling", "not-a-bool")
                .build(),
            Err(GObjectError::PropertyFromStr {
                type_: Bin::static_type(),
                property: idstr!("async-handling"),
                value: idstr!("not-a-bool")
            })
        );
    }

    #[test]
    fn builder_for() {
        crate::init().unwrap();

        let fakesink = ElementFactory::make("fakesink").build().unwrap();

        let fakesink = Object::builder_for::<Element>(fakesink.type_())
            .name("test-fakesink")
            .property("can-activate-pull", true)
            .property_from_str("state-error", "ready-to-paused")
            .build()
            .unwrap();

        assert_eq!(fakesink.name(), "test-fakesink");
        assert!(fakesink.property::<bool>("can-activate-pull"));
        let v = fakesink.property_value("state-error");
        let (_klass, e) = glib::EnumValue::from_value(&v).unwrap();
        assert_eq!(e.nick(), "ready-to-paused");
    }
}
