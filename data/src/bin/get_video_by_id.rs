use std::env::args;

use data::{establish_connection, repositories::video_repository::get_video_by_id};

// cargo run --bin get_video_by_id <id>
fn main() {
    let vid_id = args()
        .nth(1)
        .expect("get_video_by_id requires an id")
        .parse::<i32>()
        .expect("Invalid ID");

    let connection = &mut establish_connection();

    let video_option = get_video_by_id(connection, &vid_id).unwrap();

    println!("Video title: {}", video_option.title);
}
