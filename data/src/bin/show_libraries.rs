use data::models::library::*;
use data::*;
use diesel::prelude::*;

// cargo run --bin show_libraries
fn main() {
    use data::schema::library::dsl::*;

    let connection = &mut establish_connection();
    let results = library
        .limit(5)
        .select(LibraryEntity::as_select())
        .load(connection)
        .expect("Error loading libraries");

    println!("Displaying {} libraries", results.len());
    for lib in results {
        println!("{}", lib.name);
    }
}
