use crate::models::library::{LibraryEntity, NewLibrary};
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn create_library(conn: &mut PgConnection, name: &str) -> LibraryEntity {
    use crate::schema::library;

    let new_library = NewLibrary { name };

    diesel::insert_into(library::table)
        .values(&new_library)
        .returning(LibraryEntity::as_returning())
        .get_result(conn)
        .expect("Error saving new library")
}

pub fn show_libraries(conn: &mut PgConnection) {
    use crate::schema::library::dsl::*;

    let results = library
        .limit(5)
        .select(LibraryEntity::as_select())
        .load(conn)
        .expect("Error loading libraries");

    println!("Displaying {} libraries", results.len());
    for lib in results {
        println!("{}", lib.name);
    }
}
