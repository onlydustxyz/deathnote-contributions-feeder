use deathnote_contributions_feeder::infrastructure::{database, uuid_generator::UuidGenerator};
use http_api_problem::HttpApiProblem;
use rocket::{response::status, serde::json::Json};
use rocket_okapi::openapi;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::routes::{
	api_key::ApiKey, contributor_id::ContributorIdDto, to_http_api_problem::ToHttpApiProblem,
	uuid::UuidParam,
};

#[derive(Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct ApplyDto {
	contributor_id: ContributorIdDto,
}

#[openapi(tag = "Contributions")]
#[post(
	"/contributions/<contribution_id>/applications",
	format = "application/json",
	data = "<body>"
)]
pub async fn apply_to_contribution(
	_api_key: ApiKey,
	contribution_id: UuidParam,
	body: Json<ApplyDto>,
	connection: database::Connection,
) -> Result<status::Accepted<()>, Json<HttpApiProblem>> {
	let body = body.into_inner();
	let uuid_generator = UuidGenerator;
	let database = database::Client::new(connection);

	crate::application::apply_to_contribution(
		database,
		uuid_generator,
		contribution_id.into(),
		body.contributor_id.into(),
	)
	.map_err(|e| Json(e.to_http_api_problem()))?;

	Ok(status::Accepted(None))
}
