pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use models::{Library, NewLibrary};
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn show_libraries(conn: &mut PgConnection) {
    use self::schema::library::dsl::*;

    let results = library
        .limit(5)
        .select(Library::as_select())
        .load(conn)
        .expect("Error loading posts");

    println!("Displaying {} libraries", results.len());

    for lib in results {
        println!("Id {}, {}", lib.id, lib.name)
    }
}

fn create_library(conn: &mut PgConnection, name: &str) -> Library {
    use crate::schema::library;

    let new_library = NewLibrary { name };

    diesel::insert_into(library::table)
        .values(&new_library)
        .returning(Library::as_returning())
        .get_result(conn)
        .expect("Error saving library")
}

fn update_library(conn: &mut PgConnection, id: i32, new_name: &str) {
    use self::schema::library::dsl::{library, name};

    let updated_lib = diesel::update(library.find(id))
        .set(name.eq(new_name))
        .returning(Library::as_returning())
        .get_result(conn)
        .unwrap();

    println!("Updated library {}, {}", updated_lib.id, updated_lib.name)
}

fn delete_library(conn: &mut PgConnection, id: i32) {
    use self::schema::library::dsl::*;

    let num_deleted = diesel::delete(library.find(id))
        .execute(conn)
        .expect("Error deleting posts");

    println!("Deleted {} librarie(s)", num_deleted)
}

fn main() {
    let connection = &mut establish_connection();

    show_libraries(connection);
    let lib = create_library(connection, "Kevins lib");
    show_libraries(connection);
    update_library(connection, lib.id, "New Lib Name");
    show_libraries(connection);
    delete_library(connection, lib.id)
}
