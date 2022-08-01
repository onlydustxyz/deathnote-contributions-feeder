use crypto_bigint::Split;
use deathnote_contributions_feeder::domain;
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
pub struct Contributor {
	pub contributor_id: u128,
	pub github_userid: Option<String>,
	pub discord_handle: Option<String>,
}

impl From<domain::Contributor> for Contributor {
	fn from(contributor: domain::Contributor) -> Self {
		Self {
			contributor_id: contributor.id.split().1.into(), /* TODO: Refactor once we migrate to
			                                                  * uuid */
			github_userid: contributor.github_handle,
			discord_handle: contributor.discord_handle,
		}
	}
}
