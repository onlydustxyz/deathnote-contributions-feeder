use super::dto;
use deathnote_contributions_feeder::application::{GetContributorById, GetContributorByIdUsecase};
use http_api_problem::{HttpApiProblem, StatusCode};
use rocket::{get, serde::json::Json};
use rocket_okapi::openapi;
use std::result::Result;

#[openapi(tag = "Contributors")]
#[get("/contributors/<contributor_id>")]
pub fn find_by_id(
	usecase: GetContributorById,
	contributor_id: u128,
) -> Result<Json<dto::Contributor>, Json<HttpApiProblem>> {
	find_by_id_impl(usecase, contributor_id)
}

fn find_by_id_impl<U: GetContributorByIdUsecase>(
	usecase: U,
	contributor_id: u128,
) -> Result<Json<dto::Contributor>, Json<HttpApiProblem>> {
	let contributor = usecase.execute(contributor_id.into()).map_err(|e| {
		HttpApiProblem::new(StatusCode::INTERNAL_SERVER_ERROR)
			.title("Error while fetching contributor")
			.detail(e.to_string())
	})?;

	match contributor {
		Some(contributor) => Ok(Json(contributor.into())),
		None =>
			Err(HttpApiProblem::new(StatusCode::NOT_FOUND).title("Contributor not found").into()),
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use deathnote_contributions_feeder::domain::*;
	use mockall::{mock, predicate::*};

	mock! {
		pub GetContributorById {}

		impl GetContributorByIdUsecase for GetContributorById{
			pub fn execute(
				&self,
				contributor_id: ContributorId,
			) -> deathnote_contributions_feeder::domain::Result<Option<Contributor>>;
		}
	}

	#[test]
	fn find_by_id_should_return_404_when_contributor_not_found() {
		let mut usecase = MockGetContributorById::new();

		usecase
			.expect_execute()
			.with(eq(ContributorId::from(123)))
			.returning(|_| Ok(None));

		let result = find_by_id_impl(usecase, 123);
		assert!(result.is_err());

		let problem = result.err().unwrap();
		assert_eq!(StatusCode::NOT_FOUND, problem.status.unwrap());
		assert_eq!("Contributor not found", problem.title.as_ref().unwrap());
	}

	#[test]
	fn find_by_id_should_forward_error_as_500() {
		let mut usecase = MockGetContributorById::new();

		usecase
			.expect_execute()
			.with(eq(ContributorId::from(123)))
			.returning(|_| Err(Error::GetContributorError(String::from("Oops"))));

		let result = find_by_id_impl(usecase, 123);
		assert!(result.is_err());

		let problem = result.err().unwrap();
		assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, problem.status.unwrap());
		assert_eq!(
			"Error while fetching contributor",
			problem.title.as_ref().unwrap()
		);
		assert_eq!("Oops", problem.detail.as_ref().unwrap());
	}

	#[test]
	fn find_by_id_should_return_contributor_if_found() {
		let mut usecase = MockGetContributorById::new();

		usecase.expect_execute().with(eq(ContributorId::from(123))).returning(|_| {
			Ok(Some(Contributor {
				id: ContributorId::from(123),
				github_username: None,
				github_handle: Some(String::from("github")),
				discord_handle: Some(String::from("discord")),
			}))
		});

		let result = find_by_id_impl(usecase, 123);
		assert!(result.is_ok(), "{:?}", result.err().unwrap());

		let contributor = result.unwrap();
		assert_eq!(123, contributor.contributor_id);
		assert_eq!(Some(String::from("github")), contributor.github_userid);
		assert_eq!(Some(String::from("discord")), contributor.discord_handle);
	}
}
