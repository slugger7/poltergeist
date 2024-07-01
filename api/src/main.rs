use data::{
    establish_connection,
    repositories::{
        library_path_repository::{create_library_path, show_library_paths},
        library_repository::{create_library, show_libraries},
        video_repository::show_videos,
    },
};

fn main() {
    let conn = &mut establish_connection();

    // let lib = create_library(conn, "Library name");

    // create_library_path(conn, "some path", &lib.id);

    // show_libraries(conn);
    // show_library_paths(conn);

    show_videos(conn)
}
