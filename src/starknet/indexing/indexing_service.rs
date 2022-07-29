use mockall::automock;

/**
 * The indexer is responsible for fetching new events from the indexing server
 */
#[automock]
pub trait IndexingService {
	fn fetch_new_events(&self);
}

pub type BoxedIndexingService = Box<dyn IndexingService>;
