use data::*;
use std::io::stdin;

// cargo run --bin create_library
fn main() {
    let connection = &mut establish_connection();

    let mut name = String::new();

    println!("What would you like your library name to be?");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim_end(); // Remove the trailing newline

    let library = create_library(connection, name);
    println!("\nSaved library {} with id {}", name, library.id);
}
