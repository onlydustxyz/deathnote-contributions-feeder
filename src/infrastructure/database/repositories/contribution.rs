use crate::{
	domain::*,
	infrastructure::database::{models, schema, schema::contributions, Client},
};
use diesel::prelude::*;

impl ContributionRepository for Client {
	fn store(&self, contribution: Contribution, transaction_hash: String) -> Result<()> {
		let contribution = models::NewContribution::from((contribution, transaction_hash));
		diesel::insert_into(contributions::table)
			.values(&contribution)
			.execute(self.connection())
			.map_err(|e| Error::ContributionStoreError(e.to_string()))?;

		Ok(())
	}

	fn update_contributor_and_status(
		&self,
		contribution_id: ContributionOnChainId,
		contributor_id_: Option<ContributorId>,
		status_: ContributionStatus,
		transaction_hash_: String,
	) -> Result<()> {
		diesel::update(schema::contributions::dsl::contributions)
			.filter(contributions::onchain_id.eq(contribution_id))
			.set((
				schema::contributions::status.eq(status_.to_string()),
				schema::contributions::contributor_id.eq(match contributor_id_ {
					Some(id_) => id_.to_string(),
					None => String::new(),
				}),
				schema::contributions::transaction_hash.eq(transaction_hash_),
			))
			.execute(self.connection())
			.map_err(|e| Error::ContributionStoreError(e.to_string()))?;

		Ok(())
	}

	fn update_status(
		&self,
		contribution_id: ContributionOnChainId,
		status_: ContributionStatus,
		transaction_hash_: String,
	) -> Result<()> {
		diesel::update(schema::contributions::dsl::contributions)
			.filter(contributions::onchain_id.eq(contribution_id))
			.set((
				schema::contributions::status.eq(status_.to_string()),
				schema::contributions::transaction_hash.eq(transaction_hash_),
			))
			.execute(self.connection())
			.map_err(|e| Error::ContributionStoreError(e.to_string()))?;

		Ok(())
	}
}

impl From<(Contribution, String)> for models::NewContribution {
	fn from((contribution, transaction_hash): (Contribution, String)) -> Self {
		Self {
			id: contribution.id,
			onchain_id: contribution.onchain_id,
			project_id: contribution.project_id,
			status: contribution.status.to_string(),
			contributor_id: contribution.contributor_id.map_or(String::new(), |id| id.to_string()),
			gate: contribution.gate as i16,
			transaction_hash,
			title: contribution.title,
			description: contribution.description,
			external_link: contribution.external_link.map(|link| link.to_string()),
			difficulty: contribution.metadata.difficulty,
			technology: contribution.metadata.technology,
			duration: contribution.metadata.duration,
			context: contribution.metadata.context,
			type_: contribution.metadata.r#type,
			validator: format!("{:#x}", contribution.validator),
		}
	}
}
