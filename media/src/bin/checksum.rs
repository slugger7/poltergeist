use media::extensions::checksum;

fn main() {
    let file_path = String::from("");

    let check = checksum(&file_path);

    println!("{}", check);
}
