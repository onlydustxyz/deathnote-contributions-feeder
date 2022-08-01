mod connection;
pub use connection::{init_pool, DbConn as Connection, Pool as ConnectionPool};

mod contribution_repository;
mod contributor_repository;
mod models;
mod project_repository;
mod schema;

use diesel::PgConnection;

pub fn run_migrations(pool: &ConnectionPool) {
	let connection = Connection::from_pool(pool);
	diesel_migrations::run_pending_migrations(&*connection).expect("diesel migration failure");
}

fn database_url() -> String {
	std::env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub struct Client {
	connection: Connection,
}

impl Client {
	pub fn new(connection: Connection) -> Self {
		Self { connection }
	}

	fn connection(&self) -> &PgConnection {
		&self.connection
	}
}
