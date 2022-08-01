use uuid::Uuid;

pub trait Service {
	fn new_random(&self) -> Uuid;
}
