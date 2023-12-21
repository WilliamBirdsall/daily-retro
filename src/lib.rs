pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_entry(conn: &mut SqliteConnection, body: &str, created_at: &str) -> Entry {
    use crate::schema::entries;

    let new_entry = NewEntry { body, created_at };

    diesel::insert_into(entries::table)
        .values(&new_entry)
        .returning(Entry::as_returning())
        .get_result(conn)
        .expect("Error saving new entry")
}
