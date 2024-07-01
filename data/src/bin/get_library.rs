use self::models::library::Library;
use data::*;
use diesel::prelude::*;
use std::env::args;

// cargo run --bin get_library <id>
fn main() {
    use self::schema::library::dsl::library;

    let library_id = args()
        .nth(1)
        .expect("get_library requires a library id")
        .parse::<i32>()
        .expect("Invalid ID");

    let connection = &mut establish_connection();

    let lib = library
        .find(library_id)
        .select(Library::as_select())
        .first(connection)
        .optional(); // This allows for returning an Option<Library>, otherwise it will throw an error

    match lib {
        Ok(Some(lib)) => println!("Library with id: {} has a name: {}", lib.id, lib.name),
        Ok(None) => println!("Unable to find library {}", library_id),
        Err(_) => println!("An error occured while fetching library {}", library_id),
    }
}
