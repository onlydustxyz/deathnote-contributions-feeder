use super::{BoxedIndexingService, Result};
use crate::domain::*;
use async_trait::async_trait;
use mockall::automock;

/**
 * The client is responsible for interacting with the indexing server to perform CRUD operations on indexers
 */
#[automock]
#[async_trait]
pub trait IndexerRepository {
	async fn create(
		&mut self,
		indexer_id: String,
		network: BlockchainNetwork,
		index_from_block: u64,
		filters: Vec<BlockchainEventFilter>,
	) -> Result<BoxedIndexingService>;

	async fn by_id(&mut self, indexer_id: String) -> Result<Option<BoxedIndexingService>>;

	async fn delete(&mut self, indexer_id: String) -> Result<()>;
}
