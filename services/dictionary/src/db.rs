use std::env;

use diesel::{Connection, PgConnection};

pub fn establish_connection() -> PgConnection {
	let database_url_env_key = "DATABASE_URL";

	let database_url = env::var(database_url_env_key).expect(&format!(
		"Read environment variable {}",
		database_url_env_key
	));

	PgConnection::establish(&database_url)
		.expect(&format!("Establish connection to {}", database_url))
}
