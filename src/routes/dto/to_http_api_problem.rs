use deathnote_contributions_feeder::domain::Error;
use http_api_problem::{HttpApiProblem, StatusCode};

pub(crate) trait ToHttpApiProblem {
	fn to_http_api_problem(&self) -> HttpApiProblem;
}

impl ToHttpApiProblem for Error {
	fn to_http_api_problem(&self) -> HttpApiProblem {
		match self {
			Error::ParseStatusError(e) => HttpApiProblem::new(StatusCode::UNPROCESSABLE_ENTITY)
				.title("Failed to parse the contribution status")
				.detail(e),
			Error::ProjectListingError(e) => HttpApiProblem::new(StatusCode::INTERNAL_SERVER_ERROR)
				.title("Failed to list the projects")
				.detail(e),
			Error::ContributionStoreError(e) =>
				HttpApiProblem::new(StatusCode::INTERNAL_SERVER_ERROR)
					.title("Failed to store the contribution in database")
					.detail(e),
			Error::ApplicationStoreError(e) =>
				HttpApiProblem::new(StatusCode::INTERNAL_SERVER_ERROR)
					.title("Failed to store the application in database")
					.detail(e),
			Error::TransactionRevertedError(e) =>
				HttpApiProblem::new(StatusCode::FAILED_DEPENDENCY)
					.title("The on-chain batch transaction failed")
					.detail(e),
		}
	}
}
