use data::{
    establish_connection,
    repositories::{
        library_path_repository::create_library_path,
        library_repository::create_library,
        video_repository::{create_video, get_videos_in_library_path, show_videos},
    },
};

fn main() {
    let conn = &mut establish_connection();
    let lib = create_library(conn, "Main Library unique 1");
    let lib_path = create_library_path(conn, "some path again", &lib.id).unwrap();
    create_video(
        conn,
        &lib_path.id,
        "New video title",
        "new file name",
        &480,
        &720,
        &7000,
        &2000,
    );
    create_video(
        conn,
        &lib_path.id,
        "New video title number 2",
        "new file name another file",
        &480,
        &720,
        &7000,
        &2000,
    );
    show_videos(conn);
    let videos = get_videos_in_library_path(conn, lib_path);
    println!("{} videos found", videos.len());

    for vid in videos {
        println!("Title: {}, File Name: {}", vid.title, vid.file_name)
    }
}
