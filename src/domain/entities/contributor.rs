use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crypto_bigint::U256;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash, Default)]
pub struct Id(pub U256);

#[derive(Clone, Debug)]
pub struct Contributor {
	pub id: Id,
	pub github_username: Option<String>,
	pub github_handle: Option<String>,
}

impl From<String> for Id {
	fn from(s: String) -> Self {
		Self(U256::from_be_hex(&s))
	}
}

impl From<U256> for Id {
	fn from(v: U256) -> Self {
		Self(v)
	}
}

impl Display for Id {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		Display::fmt(&self.0, f)
	}
}
