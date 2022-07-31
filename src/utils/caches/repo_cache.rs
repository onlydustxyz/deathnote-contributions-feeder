use octocrab::models::Repository;
use std::{env, time::Duration};

use super::Cache;

#[derive(Clone)]
pub struct RepoCache(Cache<String, Repository>);

fn ttl() -> Duration {
	if let Ok(ttl) = env::var("REPO_CACHE_TTL") {
		if let Ok(ttl) = ttl.parse() {
			return Duration::from_secs(ttl);
		}
	}
	Duration::from_secs(5 * 60)
}

impl Default for RepoCache {
	fn default() -> Self {
		Self(Cache::new(10_000, 1, ttl()))
	}
}

impl RepoCache {
	pub fn inner_ref(&self) -> &Cache<String, Repository> {
		&self.0
	}
}
