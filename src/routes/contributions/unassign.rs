use std::sync::{Arc, RwLock};

use deathnote_contributions_feeder::domain::{Action, ContributionOnChainId};
use http_api_problem::{HttpApiProblem, StatusCode};
use rocket::{response::status, serde::json::Json, State};
use rocket_okapi::openapi;

use crate::{action_queue::ActionQueue, routes::api_key::ApiKey};

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