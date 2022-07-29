use super::{BoxedIndexingService, IndexerRepository, Result};
use crate::domain::*;

/**
 * The Builder is responsible for creating an Indexer from parameters
 */
pub struct Builder<'a, R> {
	indexer_repository: R,
	id: &'a str,
	network: BlockchainNetwork,
	filters: Vec<BlockchainEventFilter>,
	start_at_block: u64,
	on_conflict: OnConflictAction,
}

#[derive(Clone, Debug, PartialEq)]
enum OnConflictAction {
	DoNothing,
	Recreate,
}

impl<'a, R> Builder<'a, R>
where
	R: IndexerRepository,
{
	/**
	 * Create a new Builder object
	 */
	pub fn new(indexer_repository: R, id: &'a str) -> Self {
		Self {
			indexer_repository,
			id,
			network: BlockchainNetwork::Starknet(StarknetChain::Devnet),
			filters: vec![],
			start_at_block: 0,
			on_conflict: OnConflictAction::DoNothing,
		}
	}

	/**
	 * Modify the network
	 */
	pub fn network(&mut self, network: BlockchainNetwork) -> &mut Self {
		self.network = network;
		self
	}

	/**
	 * Start indexing at a given block_number
	 */
	pub fn start_at_block(&mut self, block_number: u64) -> &mut Self {
		self.start_at_block = block_number;
		self
	}

	/**
	 * Modify the network
	 */
	pub fn filter(&mut self, contract_address: String, event_name: String) -> &mut Self {
		self.filters.push(BlockchainEventFilter {
			contract_address,
			event_name,
		});
		self
	}

	/**
	 * If the indexer we are creating already exists, do nothing
	 */
	pub fn on_conflict_do_nothing(&mut self) -> &mut Self {
		self.on_conflict = OnConflictAction::DoNothing;
		self
	}

	/**
	 * If the indexer we are creating already exists, delete it and re-create it
	 */
	pub fn on_conflict_recreate(&mut self) -> &mut Self {
		self.on_conflict = OnConflictAction::Recreate;
		self
	}

	/**
	 * Build the indexer
	 */
	pub async fn build(&mut self) -> Result<BoxedIndexingService> {
		let mut existing = self.indexer_repository.by_id(self.id.to_owned()).await?;

		if existing.is_some() {
			match self.on_conflict {
				OnConflictAction::Recreate => {
					self.indexer_repository.delete(self.id.to_owned()).await?;
					existing = None
				},
				OnConflictAction::DoNothing => (),
			}
		};

		if let Some(indexer) = existing {
			return Ok(indexer);
		}

		self.indexer_repository
			.create(
				self.id.to_owned(),
				self.network.clone(),
				self.start_at_block,
				self.filters.clone(),
			)
			.await
	}
}

#[cfg(test)]
mod tests {
	use super::{
		super::{
			indexer_repository::MockIndexerRepository, indexing_service::MockIndexingService, Error,
		},
		*,
	};
	use mockall::predicate::*;

	#[test]
	fn builder_can_be_created() {
		let client = MockIndexerRepository::new();
		Builder::new(client, "ID");
	}

	#[tokio::test]
	async fn builder_can_create_an_indexer_with_default_values() {
		let mut client = MockIndexerRepository::new();

		client.expect_by_id().with(eq("ID".to_owned())).returning(|_| Ok(None));

		client
			.expect_create()
			.with(
				eq("ID".to_string()),
				eq(BlockchainNetwork::Starknet(StarknetChain::Devnet)),
				eq(0),
				eq(Vec::<BlockchainEventFilter>::new()),
			)
			.returning(|_, _, _, _| Result::Ok(Box::new(MockIndexingService::new())));

		let result = Builder::new(client, "ID").build().await;
		assert!(result.is_ok(), "{}", result.err().unwrap());
	}

	#[tokio::test]
	async fn builder_can_create_an_indexer_with_parameters() {
		let mut client = MockIndexerRepository::new();

		client.expect_by_id().with(eq("ID".to_owned())).returning(|_| Ok(None));

		client
			.expect_create()
			.with(
				eq("ID".to_string()),
				eq(BlockchainNetwork::Starknet(StarknetChain::Mainnet)),
				eq(1234),
				eq(vec![
					BlockchainEventFilter {
						contract_address: "0x123".to_owned(),
						event_name: "Event1".to_owned(),
					},
					BlockchainEventFilter {
						contract_address: "0x456".to_owned(),
						event_name: "Event2".to_owned(),
					},
				]),
			)
			.returning(|_, _, _, _| Result::Ok(Box::new(MockIndexingService::new())));

		let result = Builder::new(client, "ID")
			.network(BlockchainNetwork::Starknet(StarknetChain::Mainnet))
			.start_at_block(1234)
			.filter("0x123".to_owned(), "Event1".to_owned())
			.filter("0x456".to_owned(), "Event2".to_owned())
			.build()
			.await;
		assert!(result.is_ok(), "{}", result.err().unwrap());
	}

	#[tokio::test]
	async fn on_conflict_do_nothing() {
		let mut client = MockIndexerRepository::new();

		client
			.expect_by_id()
			.with(eq("ID".to_owned()))
			.returning(|_| Ok(Some(Box::new(MockIndexingService::new()))));

		let result = Builder::new(client, "ID").on_conflict_do_nothing().build().await;

		assert!(result.is_ok(), "{}", result.err().unwrap());
	}

	#[tokio::test]
	async fn on_conflict_recreate() {
		let mut client = MockIndexerRepository::new();

		client
			.expect_by_id()
			.with(eq("ID".to_owned()))
			.returning(|_| Ok(Some(Box::new(MockIndexingService::new()))));

		client.expect_delete().with(eq("ID".to_owned())).returning(|_| Ok(()));

		client
			.expect_create()
			.returning(|_, _, _, _| Result::Ok(Box::new(MockIndexingService::new())));

		let result = Builder::new(client, "ID").on_conflict_recreate().build().await;

		assert!(result.is_ok(), "{}", result.err().unwrap());
	}

	#[tokio::test]
	async fn builder_forward_client_error_on_get() {
		let mut client = MockIndexerRepository::new();

		client.expect_by_id().with(eq("ID".to_owned())).returning(|_| {
			Err(Error::GetIndexer {
				id: "ID".to_owned(),
				msg: "Oops".to_owned(),
			})
		});

		let result = Builder::new(client, "ID").build().await;
		assert!(result.is_err());
	}

	#[tokio::test]
	async fn builder_forward_client_error_on_create() {
		let mut client = MockIndexerRepository::new();

		client.expect_by_id().with(eq("ID".to_owned())).returning(|_| Ok(None));

		client.expect_create().returning(|_, _, _, _| {
			Err(Error::CreateIndexer {
				id: "ID".to_owned(),
				msg: "Oops".to_owned(),
			})
		});

		let result = Builder::new(client, "ID").build().await;
		assert!(result.is_err());
	}

	#[tokio::test]
	async fn builder_forward_client_error_on_delete() {
		let mut client = MockIndexerRepository::new();

		client
			.expect_by_id()
			.with(eq("ID".to_owned()))
			.returning(|_| Ok(Some(Box::new(MockIndexingService::new()))));

		client.expect_delete().returning(|_| {
			Err(Error::DeleteIndexer {
				id: "ID".to_owned(),
				msg: "Oops".to_owned(),
			})
		});

		let result = Builder::new(client, "ID").on_conflict_recreate().build().await;
		assert!(result.is_err());
	}
}
