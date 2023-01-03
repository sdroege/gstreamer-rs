// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;

use crate::value::GstValueExt;

pub trait GObjectExtManualGst: 'static {
    #[doc(alias = "gst_util_set_object_arg")]
    fn set_property_from_str(&self, name: &str, value: &str);
}

impl<O: IsA<glib::Object>> GObjectExtManualGst for O {
    #[track_caller]
    fn set_property_from_str(&self, name: &str, value: &str) {
        let pspec = self.find_property(name).unwrap_or_else(|| {
            panic!("property '{}' of type '{}' not found", name, self.type_());
        });

        let value = {
            if pspec.value_type() == crate::Structure::static_type() && value == "NULL" {
                None::<crate::Structure>.to_value()
            } else {
                #[cfg(feature = "v1_20")]
                {
                    glib::Value::deserialize_with_pspec(value, &pspec).unwrap_or_else(|_| {
                        panic!(
                            "property '{}' of type '{}' can't be set from string '{}'",
                            name,
                            self.type_(),
                            value,
                        )
                    })
                }
                #[cfg(not(feature = "v1_20"))]
                {
                    glib::Value::deserialize(value, pspec.value_type()).unwrap_or_else(|_| {
                        panic!(
                            "property '{}' of type '{}' can't be set from string '{}'",
                            name,
                            self.type_(),
                            value,
                        )
                    })
                }
            }
        };

        self.set_property(name, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_property_from_str() {
        crate::init().unwrap();

        let fakesink = crate::ElementFactory::make("fakesink").build().unwrap();
        fakesink.set_property_from_str("state-error", "ready-to-paused");
        let v = fakesink.property_value("state-error");
        let (_klass, e) = glib::EnumValue::from_value(&v).unwrap();
        assert_eq!(e.nick(), "ready-to-paused");
    }
}
