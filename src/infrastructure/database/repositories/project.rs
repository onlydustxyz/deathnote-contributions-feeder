use std::str::FromStr;

use crate::{
	domain::*,
	infrastructure::database::{
		models,
		schema::projects::{self, dsl::*},
		Client,
	},
};
use diesel::{prelude::*, query_dsl::BelongingToDsl};
use itertools::Itertools;
use starknet::core::types::FieldElement;

impl ProjectRepository for Client {
	fn find_all_with_contributions(&self) -> Result<Vec<ProjectWithContributions>> {
		let project_list = projects
			.load::<models::Project>(self.connection())
			.map_err(|e| Error::ProjectListingError(e.to_string()))?;

		let contribution_list = models::Contribution::belonging_to(&project_list)
			.load::<models::Contribution>(self.connection())
			.map_err(|e| Error::ProjectListingError(e.to_string()))?
			.grouped_by(&project_list);

		let result = project_list
			.into_iter()
			.zip(contribution_list)
			.map(ProjectWithContributions::from)
			.collect_vec();

		Ok(result)
	}

	fn store(&self, project: Project) -> Result<()> {
		let project: models::NewProject = project.into();
		diesel::insert_into(projects::table)
			.values(&project)
			.on_conflict(id)
			.do_update()
			.set(&project)
			.execute(self.connection())
			.map_err(|e| Error::ProjectListingError(e.to_string()))?;

		Ok(())
	}
}

impl From<(models::Project, Vec<models::Contribution>)> for ProjectWithContributions {
	fn from((project, contributions): (models::Project, Vec<models::Contribution>)) -> Self {
		Self {
			project: project.into(),
			contributions: contributions.into_iter().map_into().collect(),
		}
	}
}

impl From<models::Project> for Project {
	fn from(project: models::Project) -> Self {
		Self {
			id: project.id,
			name: project.name,
			owner: project.owner,
		}
	}
}

impl From<Project> for models::NewProject {
	fn from(project: Project) -> Self {
		Self {
			id: project.id,
			name: project.name,
			owner: project.owner,
		}
	}
}

impl From<models::Contribution> for Contribution {
	fn from(contribution: models::Contribution) -> Self {
		Self {
			id: contribution.id,
			onchain_id: contribution.onchain_id,
			contributor_id: {
				if contribution.contributor_id.is_empty() {
					None
				} else {
					Some(contribution.contributor_id.into())
				}
			},
			project_id: contribution.project_id,
			status: contribution.status.parse().unwrap_or(ContributionStatus::Open),
			// Safe to unwrap because the value stored can only come from an u8
			gate: contribution.gate.try_into().unwrap(),
			description: contribution.description,
			external_link: contribution.external_link.map(|link| url::Url::parse(&link).unwrap()),
			title: contribution.title,
			metadata: ContributionMetadata {
				difficulty: contribution.difficulty,
				technology: contribution.technology,
				duration: contribution.duration,
				context: contribution.context,
				r#type: contribution.type_,
			},
			// ok to unwrap because values in db are created by a call to FieldElement::ToString
			validator: FieldElement::from_str(&contribution.validator).unwrap(),
		}
	}
}
