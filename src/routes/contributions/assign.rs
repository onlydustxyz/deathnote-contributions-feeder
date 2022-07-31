use std::sync::{Arc, RwLock};

use crypto_bigint::U256;
use deathnote_contributions_feeder::domain::{Action, ContributionOnChainId};
use http_api_problem::{HttpApiProblem, StatusCode};
use log::info;
use rocket::{
	response::status,
	serde::{json::Json, Deserialize},
	State,
};
use rocket_okapi::{openapi, JsonSchema};

use crate::{action_queue::ActionQueue, routes::api_key::ApiKey};

#[derive(Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct AssignContributorDto {
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
	body: Json<AssignContributorDto>,
	queue: &State<Arc<RwLock<ActionQueue>>>,
) -> Result<status::Accepted<()>, Json<HttpApiProblem>> {
	let body = body.into_inner();
	info!("contributor_id={}", body.contributor_id);

	match queue.write() {
		Ok(mut queue) => queue.push(Action::AssignContributor {
			contribution_id,
			contributor_id: deathnote_contributions_feeder::domain::ContributorId(U256::from_u128(
				body.contributor_id,
			)),
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
