use data::{establish_connection, repositories::library_repository::show_libraries};

fn main() {
    let conn = &mut establish_connection();
    show_libraries(conn);
}
