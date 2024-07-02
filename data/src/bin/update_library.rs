use self::models::library::Library;
use data::*;
use diesel::prelude::*;
use std::env::args;
use std::io::stdin;

// cargo run --bin update_library <id>
fn main() {
    use self::schema::library::dsl::{library, name};

    let id = args()
        .nth(1)
        .expect("update_library requires a library id")
        .parse::<i32>()
        .expect("Invalid ID");

    let mut new_name = String::new();

    println!("What would you like your library name to be?");
    stdin().read_line(&mut new_name).unwrap();
    let new_name = new_name.trim_end(); // Remove the trailing newline

    let connection = &mut establish_connection();

    let lib = diesel::update(library.find(id))
        .set(name.eq(new_name))
        .returning(Library::as_returning())
        .get_result(connection)
        .unwrap();
    println!("Library update to {}", lib.name);
}
