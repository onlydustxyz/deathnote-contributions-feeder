use diesel::RunQueryDsl;

use crate::{
	domain::*,
	infrastructure::database::{models, schema::applications, Client},
};

impl ApplicationRepository for Client {
	fn store(&self, application: Application) -> Result<()> {
		let application = models::NewApplication::from(application);
		diesel::insert_into(applications::table)
			.values(&application)
			.execute(self.connection())
			.map_err(|e| Error::ApplicationStoreError(e.to_string()))?;

		Ok(())
	}
}

impl From<Application> for models::NewApplication {
	fn from(application: crate::domain::Application) -> Self {
		Self {
			id: *application.id(),
			contribution_id: *application.contribution_id(),
			contributor_id: application.contributor_id().to_string(),
		}
	}
}
