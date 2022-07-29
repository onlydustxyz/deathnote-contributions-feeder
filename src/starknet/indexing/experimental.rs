/**
 * This file is purely experimental and contains functions to fetch events, to be migrated to the ApibaraIndexer
 */
pub struct ExperimentalIndexer {
	client: IndexerManagerClient<tonic::transport::Channel>,
	indexer: indexer_service::Indexer,
	stream: Option<Streaming<ConnectIndexerResponse>>,
}

impl ExperimentalIndexer {
	pub async fn connect(&mut self) -> Result<()> {
		let request = ConnectIndexerRequest {
			message: Some(connect_indexer_request::Message::Connect(ConnectIndexer {
				id: self.indexer.id.clone(),
			})),
		};

		let stream = stream::iter(vec![request]); // TODO: Find a better way to create stream from single element

		self.stream = Some(
			self.client
				.connect_indexer(stream)
				.await
				.map_err(|e| Error::IndexerError(e.to_string()))?
				.into_inner(),
		);

		Ok(())
	}

	pub async fn process(&mut self) -> Result<()> {
		self.stream
			.as_mut()
			.unwrap()
			.try_filter_map(|msg| async {
				match msg.message.unwrap() {
					indexer_service::connect_indexer_response::Message::Connected(msg) => {
						println!("Indexer connected: {:?}", msg)
					},
					indexer_service::connect_indexer_response::Message::NewBlock(msg) => {
						println!("New block: {:?}", msg)
					},
					indexer_service::connect_indexer_response::Message::Reorg(msg) => {
						println!("Reorg: {:?}", msg)
					},
					indexer_service::connect_indexer_response::Message::NewEvents(msg) => {
						println!("New events: {:?}", msg)
					},
				};

				Ok(Some(()))
			})
			.collect::<Vec<_>>()
			.await;

		Ok(())
	}
}
