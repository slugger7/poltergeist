use data::*;
use diesel::prelude::*;
use std::env::args;

// cargo run --bin delete_library <name>
fn main() {
    use self::schema::library::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(library.filter(name.like(pattern)))
        .execute(connection)
        .expect("Error deleting libraries");

    println!("Deleted {} libraries", num_deleted);
}
