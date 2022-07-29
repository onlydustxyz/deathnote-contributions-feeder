use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
	#[error("unable to connect to the indexing server")]
	Connection(String),
	#[error("unable to create the indexer `{id}`: {msg}")]
	CreateIndexer { id: String, msg: String },
	#[error("unable to get the indexer `{id}`: {msg}")]
	GetIndexer { id: String, msg: String },
	#[error("unable to delete the indexer `{id}`: {msg}")]
	DeleteIndexer { id: String, msg: String },
}

pub type Result<T> = std::result::Result<T, Error>;
