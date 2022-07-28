use uuid::Uuid;

pub trait Repository {
	fn new_random(&self) -> Uuid;
}
