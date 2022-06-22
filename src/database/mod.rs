use anyhow::{anyhow, Result};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use log::info;
use std::env;

use crate::{
    model::{pullrequest, repository},
    starknet::models::ContractUpdateStatus,
    traits::{fetcher::SyncFetcher, logger::SyncLogger},
};

use self::{
    models::{
        Project, ProjectIndexingStatusUpdateForm, PullRequest, PullRequestContractUpdateForm,
    },
    schema::{
        projects::{self, dsl::*},
        pull_requests::{self, dsl::*},
    },
};

pub mod models;
pub mod schema;

pub fn establish_connection() -> Result<PgConnection> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).map_err(|e| anyhow!(e))
}

pub struct API {
    connection: PgConnection,
}

impl API {
    pub fn new() -> Self {
        API {
            connection: establish_connection().unwrap(),
        }
    }

    fn insert_pullrequest(&self, pr: models::NewPullRequest) -> Result<()> {
        diesel::insert_into(pull_requests::table)
            .values(&pr)
            .get_result::<models::PullRequest>(&self.connection)?;

        Ok(())
    }

    fn insert_project(&self, project: models::NewProject) -> Result<()> {
        diesel::insert_into(projects::table)
            .values(&project)
            .get_result::<models::Project>(&self.connection)?;

        Ok(())
    }

    fn update_pullrequest(&self, pr: models::PullRequestForm) -> Result<()> {
        pr.save_changes::<models::PullRequest>(&self.connection)?;
        Ok(())
    }

    fn find_projects(&self, filter: repository::Filter) -> impl Iterator<Item = Project> {
        let mut query = projects.into_boxed();

        if let Some(owner) = filter.owner {
            query = query.filter(organisation.eq(owner));
        };

        if let Some(name) = filter.name {
            query = query.filter(repository.eq(name));
        };

        let results = query
            .limit(1)
            .load::<models::Project>(&self.connection)
            .expect("Error while fetching projects from database");

        results.into_iter()
    }

    fn find_pullrequests(
        &self,
        filter: pullrequest::Filter,
    ) -> Box<dyn Iterator<Item = PullRequest>> {
        let mut query = pull_requests.into_boxed();

        if let Some(repo) = filter.repository {
            let project = self
                .find_projects(repository::Filter {
                    owner: Some(repo.owner),
                    name: Some(repo.name),
                })
                .take(1)
                .collect::<Vec<Project>>()
                .pop();

            match project {
                Some(project) => query = query.filter(project_id.eq(project.id)),
                None => return Box::new(std::iter::empty::<PullRequest>()),
            }
        };

        let results = query
            .load::<models::PullRequest>(&self.connection)
            .expect("Error while fetching pullrequests from database");

        Box::new(results.into_iter())
    }
}

impl Default for API {
    fn default() -> Self {
        Self::new()
    }
}

impl From<pullrequest::PullRequest> for models::NewPullRequest {
    fn from(pr: pullrequest::PullRequest) -> Self {
        Self {
            id: pr.id,
            project_id: pr.repository_id,
            pr_status: pr.status.to_string(),
            author: pr.author,
        }
    }
}

impl From<pullrequest::PullRequest> for models::PullRequestForm {
    fn from(pr: pullrequest::PullRequest) -> Self {
        Self {
            id: pr.id,
            pr_status: pr.status.to_string(),
            author: pr.author,
        }
    }
}

impl From<ContractUpdateStatus> for models::PullRequestContractUpdateForm {
    fn from(status: ContractUpdateStatus) -> Self {
        Self {
            id: status.pr_id,
            smart_contract_update_time: status
                .last_update_time
                .elapsed()
                .expect("Invalid elapsed time")
                .as_secs()
                .to_string(),
        }
    }
}

impl From<models::Project> for repository::Repository {
    fn from(project: models::Project) -> Self {
        Self {
            id: project.id,
            name: project.repository,
            owner: project.organisation,
        }
    }
}

impl From<repository::Repository> for models::NewProject {
    fn from(repo: repository::Repository) -> Self {
        Self {
            id: repo.id,
            repository: repo.name,
            organisation: repo.owner,
        }
    }
}

impl SyncLogger<pullrequest::PullRequest, Result<()>> for API {
    fn log_sync(&self, pr: pullrequest::PullRequest) -> Result<()> {
        info!("Logging PR #{} by {} ({})", pr.id, pr.author, pr.status);

        let result: Result<models::PullRequest> = pull_requests
            .find(&pr.id)
            .first(&self.connection)
            .map_err(anyhow::Error::msg);

        match result {
            Ok(_) => self.update_pullrequest(pr.into()), // PR exists in DB => update
            Err(_) => self.insert_pullrequest(pr.into()), // PR does not exist in DB => insert
        }
    }
}

impl SyncLogger<repository::Repository, Result<()>> for API {
    fn log_sync(&self, repo: repository::Repository) -> Result<()> {
        info!("Logging repository {}/{}", repo.owner, repo.name);

        let filter = repository::Filter {
            owner: Some(repo.owner.clone()),
            name: Some(repo.name.clone()),
        };

        if self.find_projects(filter).count() == 0 {
            self.insert_project(repo.into())?;
        }

        Ok(())
    }
}

impl SyncLogger<ContractUpdateStatus, Result<()>> for API {
    fn log_sync(&self, status: ContractUpdateStatus) -> Result<()> {
        info!("Logging successful contract update for PR#{}", status.pr_id);

        PullRequestContractUpdateForm::from(status)
            .save_changes::<models::PullRequest>(&self.connection)?;
        Ok(())
    }
}

impl SyncFetcher<repository::Filter, repository::Repository> for API {
    fn fetch_sync(
        &self,
        filter: repository::Filter,
    ) -> Result<Box<dyn Iterator<Item = repository::Repository>>> {
        info!("Fetching repositories with filter: {:?}", filter);

        let results = self.find_projects(filter).map(|project| project.into());
        Ok(Box::new(results))
    }
}

impl From<repository::IndexingStatus> for ProjectIndexingStatusUpdateForm {
    fn from(status: repository::IndexingStatus) -> Self {
        Self {
            id: status.repository_id,
            last_indexed_time: status.last_update_time,
        }
    }
}

impl SyncLogger<repository::IndexingStatus, Result<()>> for API {
    fn log_sync(&self, status: repository::IndexingStatus) -> Result<()> {
        info!(
            "Logging successful syncing for project {} ",
            status.repository_id
        );

        ProjectIndexingStatusUpdateForm::from(status)
            .save_changes::<models::Project>(&self.connection)?;

        Ok(())
    }
}

impl From<models::PullRequest> for pullrequest::PullRequest {
    fn from(pr: models::PullRequest) -> Self {
        Self {
            id: pr.id,
            author: pr.author,
            repository_id: pr.project_id,
            status: pr.pr_status.parse().unwrap(),
        }
    }
}

impl SyncFetcher<pullrequest::Filter, pullrequest::PullRequest> for API {
    fn fetch_sync(
        &self,
        filter: pullrequest::Filter,
    ) -> Result<Box<dyn Iterator<Item = pullrequest::PullRequest>>> {
        info!("Fetching pull requests with filter {:?} ", filter);

        let pullrequests = self.find_pullrequests(filter).map(|pr| pr.into());

        Ok(Box::new(pullrequests))
    }
}
