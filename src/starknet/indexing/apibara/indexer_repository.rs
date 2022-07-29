use crate::domain::*;

use super::{
	super::{BoxedIndexingService, Error, IndexerRepository, Result},
	apibara::{
		self, indexer_manager_client::IndexerManagerClient, CreateIndexerRequest,
		DeleteIndexerRequest, GetIndexerRequest,
	},
	ApibaraIndexer,
};
use async_trait::async_trait;
use itertools::Itertools;

/**
 * Wrapper around apibara objects
 */
struct ApibaraClient(IndexerManagerClient<tonic::transport::Channel>);

impl ApibaraClient {
	pub async fn default() -> Result<Self> {
		let client = IndexerManagerClient::connect(apibara_url())
			.await
			.map_err(|e| Error::Connection(e.to_string()))?;
		Ok(Self(client))
	}
}

fn apibara_url() -> String {
	std::env::var("APIBARA_URL").expect("APIBARA_URL must be set")
}

/**
 * Implementation of the Client trait for apibara
 */
#[async_trait]
impl IndexerRepository for ApibaraClient {
	async fn create(
		&mut self,
		indexer_id: String,
		network: BlockchainNetwork,
		index_from_block: u64,
		filters: Vec<BlockchainEventFilter>,
	) -> Result<BoxedIndexingService> {
		let response = self
			.0
			.create_indexer(CreateIndexerRequest {
				id: indexer_id.clone(),
				network_name: network.to_string(),
				index_from_block,
				filters: filters.into_iter().map_into().collect(),
			})
			.await
			.map_err(|status| Error::CreateIndexer {
				id: indexer_id.clone(),
				msg: status.to_string(),
			})?;

		response.into_inner().indexer.ok_or_else(|| Error::CreateIndexer {
			id: indexer_id.clone(),
			msg: String::from("Indexer not created"),
		})?;

		Ok(Box::new(ApibaraIndexer::new(indexer_id, self.0.clone())))
	}

	async fn by_id(&mut self, indexer_id: String) -> Result<Option<BoxedIndexingService>> {
		let response = self
			.0
			.get_indexer(GetIndexerRequest {
				id: indexer_id.clone(),
			})
			.await
			.map_err(|status| Error::GetIndexer {
				id: indexer_id.clone(),
				msg: status.to_string(),
			})?;

		if response.into_inner().indexer.is_some() {
			Ok(Some(Box::new(ApibaraIndexer::new(
				indexer_id,
				self.0.clone(),
			))))
		} else {
			Ok(None)
		}
	}

	async fn delete(&mut self, indexer_id: String) -> Result<()> {
		self.0
			.delete_indexer(DeleteIndexerRequest {
				id: indexer_id.clone(),
			})
			.await
			.map_err(|status| Error::DeleteIndexer {
				id: indexer_id,
				msg: status.to_string(),
			})?;

		Ok(())
	}
}

impl ToString for BlockchainNetwork {
	fn to_string(&self) -> String {
		match self {
			BlockchainNetwork::Starknet(chain) => chain.to_string(),
		}
	}
}

// Hardcoded strings are referenced in the server configuration.toml file
impl ToString for StarknetChain {
	fn to_string(&self) -> String {
		match self {
			StarknetChain::Devnet => "starknet-devnet",
			StarknetChain::Goerli => "starknet-goerli",
			StarknetChain::Mainnet => "starknet-mainnet",
		}
		.to_owned()
	}
}

impl From<BlockchainEventFilter> for apibara::EventFilter {
	fn from(filter: BlockchainEventFilter) -> Self {
		Self {
			address: filter.contract_address.into_bytes(),
			signature: filter.event_name,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn to_string() {
		assert_eq!(
			"starknet-devnet",
			BlockchainNetwork::Starknet(StarknetChain::Devnet).to_string()
		);

		assert_eq!(
			"starknet-goerli",
			BlockchainNetwork::Starknet(StarknetChain::Goerli).to_string()
		);

		assert_eq!(
			"starknet-mainnet",
			BlockchainNetwork::Starknet(StarknetChain::Mainnet).to_string()
		);
	}

	#[test]
	fn event_filter_from() {
		let filter = apibara::EventFilter::from(BlockchainEventFilter {
			contract_address: "0x04e16efc9bc2d8d40ecb73d3d69e3e2d6f0fc3e2e6e9b7601310fdfa7dd6c7cf"
				.to_owned(),
			event_name: "GithubUserRegistered".to_owned(),
		});

		// TODO: Check this is correct with end-to-end testing
		assert_eq!(
			vec![
				48, 120, 48, 52, 101, 49, 54, 101, 102, 99, 57, 98, 99, 50, 100, 56, 100, 52, 48,
				101, 99, 98, 55, 51, 100, 51, 100, 54, 57, 101, 51, 101, 50, 100, 54, 102, 48, 102,
				99, 51, 101, 50, 101, 54, 101, 57, 98, 55, 54, 48, 49, 51, 49, 48, 102, 100, 102,
				97, 55, 100, 100, 54, 99, 55, 99, 102
			],
			filter.address
		);
		assert_eq!("GithubUserRegistered", filter.signature);
	}
}
