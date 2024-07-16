use data::{establish_connection, repositories::video_repository::add_checksum_to_video_by_id};
use media::extensions::checksum;

// cargo run --bin update_video_checksum
fn main() {
    let connection = &mut establish_connection();

    let video_checksum = checksum("");

    let result = add_checksum_to_video_by_id(connection, &3, &video_checksum);

    match result {
        Ok(s) => println!("It worked: {}", s),
        Err(err) => eprintln!("It did not work: {}", err),
    }
}
