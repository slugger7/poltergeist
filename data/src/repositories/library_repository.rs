use crate::models::library::{Library, NewLibrary};
use crate::schema::library::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn create_library(conn: &mut PgConnection, new_name: &str) -> Library {
    use crate::schema::library;

    let new_library = NewLibrary { name: new_name };

    diesel::insert_into(library::table)
        .values(&new_library)
        .returning(Library::as_returning())
        .get_result(conn)
        .expect("Error saving new library")
}

pub fn show_libraries(conn: &mut PgConnection) {
    let results = library
        .limit(5)
        .select(Library::as_select())
        .load(conn)
        .expect("Error loading libraries");

    println!("Displaying {} libraries", results.len());
    for lib in results {
        println!("{}", lib.name);
    }
}

pub fn get_library_entity_by_id(conn: &mut PgConnection, lib_id: i32) -> Option<Library> {
    let result = library
        .find(lib_id)
        .select(Library::as_select())
        .first(conn)
        .optional();

    match result {
        Ok(lib) => Some(lib?),
        Err(error) => {
            eprintln!("Error encountered getting a library by id: {}", error);
            None
        }
    }
}
