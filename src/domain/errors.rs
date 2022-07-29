#[derive(Debug)]
pub enum Error {
	ParseStatusError(String),
	ProjectListingError(String),
	ContributionStoreError(String),
	TransactionRevertedError(String),
	IndexerError(String),
}

impl ToString for Error {
	fn to_string(&self) -> String {
		match self {
			Error::ParseStatusError(e) => e.to_owned(),
			Error::ProjectListingError(e) => e.to_owned(),
			Error::ContributionStoreError(e) => e.to_owned(),
			Error::TransactionRevertedError(e) => e.to_owned(),
			Error::IndexerError(e) => e.to_owned(),
		}
	}
}

pub type Result<T> = std::result::Result<T, Error>;
