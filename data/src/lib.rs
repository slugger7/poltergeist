pub mod models;
pub mod schema;

use self::models::{Library, NewLibrary};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_library(conn: &mut PgConnection, name: &str) -> Library {
    use crate::schema::library;

    let new_library = NewLibrary { name };

    diesel::insert_into(library::table)
        .values(&new_library)
        .returning(Library::as_returning())
        .get_result(conn)
        .expect("Error saving new library")
}
