use std::{
	str::FromStr,
	sync::{Arc, RwLock},
};

use deathnote_contributions_feeder::{
	domain::{self, Action, ContributionOnChainId},
	github,
};
use http_api_problem::{HttpApiProblem, StatusCode};
use log::info;
use rocket::{
	http::Status,
	response::status,
	serde::{json::Json, Deserialize},
	State,
};
use rocket_okapi::{openapi, JsonSchema};
use starknet::core::types::FieldElement;
use uuid::Uuid;

use crate::action_queue::ActionQueue;

use super::{api_key::ApiKey, hex_prefixed_string::HexPrefixedString};

#[derive(Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct CreateContributionDto {
	github_issue_number: u128,
	project_id: u128,
	gate: u8,
	validator: HexPrefixedString,
}

#[openapi(tag = "Contributions")]
#[post("/contributions/github", format = "application/json", data = "<body>")]
pub async fn create_contribution(
	_api_key: ApiKey,
	body: Json<CreateContributionDto>,
	github_api: &State<github::API>,
	queue: &State<Arc<RwLock<ActionQueue>>>,
) -> Result<Status, Json<HttpApiProblem>> {
	let body = body.into_inner();
	let validator = FieldElement::from_str(body.validator.as_string()).map_err(|e| {
		Json(
			HttpApiProblem::new(StatusCode::BAD_REQUEST)
				.title("Invalid validator address")
				.detail(e.to_string()),
		)
	})?;

	let github_issue = github_api.issue(body.project_id, body.github_issue_number).await;
	let github_issue = match github_issue {
		Ok(github_issue) => github_issue,
		Err(error) =>
			return Err(Json(
				HttpApiProblem::new(StatusCode::BAD_REQUEST)
					.title("Unable to get GitHub issue data")
					.detail(error.to_string()),
			)),
	};

	let metadata = github::extract_metadata(github_issue.clone());

	let contribution = domain::Contribution {
		id: Uuid::new_v4(),
		onchain_id: (body.project_id * 1_000_000 + body.github_issue_number).to_string(),
		project_id: body.project_id.to_string(),
		contributor_id: None,
		title: Some(github_issue.title),
		description: Some(github_issue.body.unwrap_or_default()),
		status: domain::ContributionStatus::Open,
		external_link: Some(github_issue.html_url),
		gate: body.gate,
		metadata,
		validator,
	};

	match queue.write() {
		Ok(mut queue) => queue.push(Action::CreateContribution {
			contribution: Box::new(contribution),
		}),
		Err(error) =>
			return Err(Json(
				HttpApiProblem::new(StatusCode::INTERNAL_SERVER_ERROR)
					.title("Unable to add contribution to the queue")
					.detail(error.to_string()),
			)),
	}

	Ok(Status::Accepted)
}

#[derive(Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct AssignContributorBody {
	contributor_id: u128,
}

#[openapi(tag = "Contributions")]
#[post(
	"/contributions/<contribution_id>/contributor",
	format = "application/json",
	data = "<body>"
)]
pub async fn assign_contributor(
	_api_key: ApiKey,
	contribution_id: ContributionOnChainId,
	body: Json<AssignContributorBody>,
	queue: &State<Arc<RwLock<ActionQueue>>>,
) -> Result<status::Accepted<()>, Json<HttpApiProblem>> {
	let body = body.into_inner();
	info!("contributor_id={}", body.contributor_id);

	match queue.write() {
		Ok(mut queue) => queue.push(Action::AssignContributor {
			contribution_id,
			contributor_id: body.contributor_id.into(),
		}),
		Err(error) =>
			return Err(Json(
				HttpApiProblem::new(StatusCode::INTERNAL_SERVER_ERROR)
					.title("Unable to add assignation to the queue")
					.detail(error.to_string()),
			)),
	}

	Ok(status::Accepted(None))
}

#[openapi(tag = "Contributions")]
#[post("/contributions/<contribution_id>/validate")]
pub async fn validate_contribution(
	_api_key: ApiKey,
	contribution_id: ContributionOnChainId,
	queue: &State<Arc<RwLock<ActionQueue>>>,
) -> Result<status::Accepted<()>, Json<HttpApiProblem>> {
	match queue.write() {
		Ok(mut queue) => queue.push(Action::ValidateContribution { contribution_id }),
		Err(error) =>
			return Err(Json(
				HttpApiProblem::new(StatusCode::INTERNAL_SERVER_ERROR)
					.title("Unable to add contribution validation to the queue")
					.detail(error.to_string()),
			)),
	}

	Ok(status::Accepted(None))
}

#[openapi(tag = "Contributions")]
#[delete("/contributions/<contribution_id>/contributor")]
pub async fn unassign_contributor(
	_api_key: ApiKey,
	contribution_id: ContributionOnChainId,
	queue: &State<Arc<RwLock<ActionQueue>>>,
) -> Result<status::Accepted<()>, Json<HttpApiProblem>> {
	match queue.write() {
		Ok(mut queue) => queue.push(Action::UnassignContributor { contribution_id }),
		Err(error) =>
			return Err(Json(
				HttpApiProblem::new(StatusCode::INTERNAL_SERVER_ERROR)
					.title("Unable to add unassignation to the queue")
					.detail(error.to_string()),
			)),
	}

	Ok(status::Accepted(None))
}
