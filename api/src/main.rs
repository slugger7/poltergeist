use data::{establish_connection, show_libraries};

fn main() {
    let conn = &mut establish_connection();
    show_libraries(conn);
}
