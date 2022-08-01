use crate::domain::*;

pub trait Repository {
	fn by_id(&self, contributor_id: ContributorId) -> Result<Option<Contributor>>;
}
