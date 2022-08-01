use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crypto_bigint::U256;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash, Default)]
pub struct Id(pub U256);

#[derive(Clone, Debug)]
pub struct Contributor {
	pub id: Id,
	pub github_username: Option<String>,
	pub github_handle: Option<String>,
	pub discord_handle: Option<String>,
}

impl Deref for Id {
	type Target = U256;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl From<String> for Id {
	fn from(s: String) -> Self {
		Self(U256::from_be_hex(&s))
	}
}

impl From<u128> for Id {
	fn from(id: u128) -> Self {
		Self(U256::from_u128(id))
	}
}
