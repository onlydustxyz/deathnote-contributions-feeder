use crate::domain::*;

pub trait Repository {
	fn store(&self, application: Application) -> Result<()>;
}
