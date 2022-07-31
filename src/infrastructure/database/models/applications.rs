use crate::infrastructure::database::schema::*;
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Application {
	pub id: Uuid,
	pub contribution_id: Uuid,
	pub contributor_id: String,
}

#[derive(Insertable)]
#[table_name = "applications"]
pub struct NewApplication {
	pub id: Uuid,
	pub contribution_id: Uuid,
	pub contributor_id: String,
}
