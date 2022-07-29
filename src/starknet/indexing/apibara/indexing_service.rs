use super::{super::IndexingService, apibara::indexer_manager_client::IndexerManagerClient};

/**
 * Implementation of the Indexer trait for apibara
 */
pub struct ApibaraIndexer {
	id: String,
	client: IndexerManagerClient<tonic::transport::Channel>,
}

impl ApibaraIndexer {
	pub fn new(id: String, client: IndexerManagerClient<tonic::transport::Channel>) -> Self {
		Self { id, client }
	}
}

impl IndexingService for ApibaraIndexer {
	fn fetch_new_events(&self) {}
}
