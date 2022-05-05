// Take a look at the license at the top of the repository in the LICENSE file.

#[macro_export]
macro_rules! bitflags_serialize_impl {
    // this implementation serializes only flags using only one bit,
    // ignoring all other flags
    ($type:ty, single_bit_flags$(, $feature:expr)?) => {
        $(#[cfg(any(feature = $feature, feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = $feature)))])?
        impl serde::Serialize for $type {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let class = FlagsClass::new(Self::static_type()).unwrap();
                let this = self.to_value();

                let mut handled = Self::empty().to_value();
                let mut res = String::new();

                for v in class.values() {
                    let value = v.value();
                    if value.count_ones() == 1 && class.is_set(&this, value) && !class.is_set(&handled, value) {
                        if !res.is_empty() {
                            res.push('+');
                        }
                        res.push_str(v.nick());
                        handled = class.set(handled, value).expect("Failed to set flag");
                    }
                }

                serializer.serialize_str(&res)
            }
        }
    };

    // considers the flags using the most bits first
    ($type:ty, by_ones_decreasing$(, $feature:expr)?) => {
        $(#[cfg(any(feature = $feature, feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = $feature)))])?
        impl serde::Serialize for $type {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                use once_cell::sync::Lazy;

                let mut handled = Self::empty();
                let mut res = String::new();

                static SORTED_VALUES: Lazy<Vec<(u32, String)>> = Lazy::new(|| {
                    let class = FlagsClass::new(<$type>::static_type()).unwrap();
                    let mut sorted_values: Vec<(u32, String)> =
                        class.values().iter()
                            .map(|f| (f.value(), f.nick().to_owned()))
                            .collect();

                    sorted_values.sort_by(|(a, _), (b, _)| {
                        b.count_ones().cmp(&a.count_ones())
                    });

                    sorted_values
                });

                for (bits, nick) in SORTED_VALUES.iter() {
                    // not all values defined in the class are always also defined
                    // in the bitflags struct, see RTPBufferFlags for example
                    if let Some(value) = Self::from_bits(*bits) {
                        if !value.is_empty() && self.contains(value) && !handled.intersects(value) {
                            if !res.is_empty() {
                                res.push('+');
                            }
                            res.push_str(nick);
                            handled.insert(value);
                        }
                    }
                }

                serializer.serialize_str(&res)
            }
        }
    };
}

#[macro_export]
macro_rules! bitflags_deserialize_impl {
    ($type:ty$(, $feature:expr)?) => {
        $(#[cfg(any(feature = $feature, feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = $feature)))])?
        impl<'de> serde::Deserialize<'de> for $type {
            fn deserialize<D: serde::de::Deserializer<'de>>(
                deserializer: D,
            ) -> std::result::Result<Self, D::Error> {
                skip_assert_initialized!();

                struct FlagsVisitor;

                impl<'de> serde::de::Visitor<'de> for FlagsVisitor {
                    type Value = $type;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str(
                            "one or more mask names separated by plus signs, or the empty string",
                        )
                    }

                    fn visit_str<E: serde::de::Error>(
                        self,
                        value: &str,
                    ) -> std::result::Result<Self::Value, E> {
                        if value.is_empty() {
                            return Ok(Self::Value::empty());
                        }

                        let mut gvalue = glib::Value::from_type(Self::Value::static_type());
                        let tokens = value.split('+');
                        let class = FlagsClass::new(Self::Value::static_type()).unwrap();

                        for token in tokens {
                            gvalue = class.set_by_nick(gvalue, token).map_err(|_| {
                                serde::de::Error::custom(&format!("Invalid value: {}", token))
                            })?;
                        }

                        Ok(unsafe {
                            from_glib(glib::gobject_ffi::g_value_get_flags(
                                gvalue.to_glib_none().0,
                            ))
                        })
                    }
                }

                deserializer.deserialize_str(FlagsVisitor)
            }
        }
    };
}

#[macro_export]
macro_rules! bitflags_serde_impl {
    ($type:ty$(, $feature:expr)?) => {
        $crate::bitflags_serialize_impl!($type, single_bit_flags$(, $feature)?);
        $crate::bitflags_deserialize_impl!($type$(, $feature)?);
    };
}
