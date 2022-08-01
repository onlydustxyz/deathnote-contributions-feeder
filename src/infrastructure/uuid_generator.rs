use crate::domain::*;
use uuid::Uuid;

pub struct UuidGenerator;

impl UuidService for UuidGenerator {
	fn new_random(&self) -> Uuid {
		Uuid::new_v4()
	}
}
