use crate::models::library_path::{LibraryPath, NewLibraryPath};
use crate::repositories::library_repository::get_library_entity_by_id;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn show_library_paths(conn: &mut PgConnection) {
    use crate::schema::library_path::dsl::*;

    let results = library_path
        .limit(5)
        .select(LibraryPath::as_select())
        .load(conn)
        .expect("Error loading library paths");

    println!("Displaying {} library paths", results.len());

    for lib_path in results {
        println!("{}", lib_path.path)
    }
}

pub fn create_library_path(
    conn: &mut PgConnection,
    path: &str,
    library_id: &i32,
) -> Option<LibraryPath> {
    use crate::schema::library_path;

    match get_library_entity_by_id(conn, *library_id) {
        Some(lib) => lib,
        None => {
            println!("No library was found");
            return None;
        }
    };

    let new_library_path = NewLibraryPath { path, library_id };

    Some(
        diesel::insert_into(library_path::table)
            .values(&new_library_path)
            .returning(LibraryPath::as_returning())
            .get_result(conn)
            .expect("Error saving new library path"),
    )
}

pub fn get_library_path_entity_by_id(
    conn: &mut PgConnection,
    lib_path_id: i32,
) -> Option<LibraryPath> {
    use crate::schema::library_path::dsl::*;

    let result = library_path
        .find(lib_path_id)
        .select(LibraryPath::as_select())
        .first(conn)
        .optional();

    match result {
        Ok(lib_path) => Some(lib_path?),
        Err(error) => {
            eprintln!("Error encountered getting library path by id: {}", error);
            None
        }
    }
}
