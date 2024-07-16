use data::{
    establish_connection, repositories::video_repository::get_videos_without_checksum_limited,
};

// cargo run --bin get_videos_without_checksum_limited
fn main() {
    let connection = &mut establish_connection();

    get_videos_without_checksum_limited(connection, 10);
}
