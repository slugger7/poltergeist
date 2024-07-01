use data::{
    establish_connection,
    repositories::video_repository::{create_video, show_videos},
};

fn main() {
    let conn = &mut establish_connection();
    create_video(
        conn,
        &1,
        "New video title",
        "new file name",
        &480,
        &720,
        &7000,
        &2000,
    );
    show_videos(conn);
}
