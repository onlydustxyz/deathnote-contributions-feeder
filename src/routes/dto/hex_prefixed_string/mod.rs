#[cfg(test)]
mod tests;

use schemars::JsonSchema;
use serde::{
	de::{self, Unexpected, Visitor},
	Deserialize,
};

#[derive(Debug, JsonSchema, PartialEq, Eq)]
pub struct HexPrefixedString(String);

impl HexPrefixedString {
	pub fn as_string(&self) -> &String {
		&self.0
	}
}

struct HexPrefixedStringVisitor;

impl<'de> Visitor<'de> for HexPrefixedStringVisitor {
	type Value = HexPrefixedString;

	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		formatter.write_str("a \"0x\" prefixed string representation of an hexadecimal number")
	}

	fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		if v.len() < 3 {
			return Err(de::Error::invalid_length(v.len(), &self));
		}

		let lowercase_value = v.to_lowercase();

		if &lowercase_value[0..2] != "0x"
			|| !lowercase_value[2..].chars().all(|c| c.is_numeric() || ('a'..='f').contains(&c))
		{
			Err(de::Error::invalid_value(Unexpected::Str(v), &self))
		} else {
			Ok(HexPrefixedString(lowercase_value))
		}
	}
}

impl<'de> Deserialize<'de> for HexPrefixedString {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		deserializer.deserialize_str(HexPrefixedStringVisitor)
	}
}
