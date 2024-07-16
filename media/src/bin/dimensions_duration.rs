use media::ffprobe::dimensions_and_duration;

fn main() {
    let file_path = String::from("");

    let (width, height, duration) = dimensions_and_duration(&file_path);
    println!(
        "Width: {}, Height: {}, Duration: {}",
        width, height, duration
    );
}
