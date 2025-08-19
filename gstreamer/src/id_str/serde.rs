// Take a look at the license at the top of the repository in the LICENSE file.

use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

use crate::IdStr;

impl Serialize for IdStr {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_str().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for IdStr {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        skip_assert_initialized!();
        <&str>::deserialize(deserializer).map(IdStr::from)
    }
}

#[cfg(test)]
mod tests {
    use crate::idstr;

    #[test]
    fn ser_de() {
        assert_eq!(
            ron::ser::to_string(&idstr!("my IdStr")),
            Ok("\"my IdStr\"".to_owned())
        );

        assert_eq!(ron::de::from_str("\"my IdStr\""), Ok(idstr!("my IdStr")));
    }
}
