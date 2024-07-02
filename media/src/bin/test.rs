use media::{get_directories, get_files_by_extension, get_files_by_extensions_recursive};

fn main() {
    let dirs = get_directories("./");

    println!("Directories\n");
    for dir in dirs {
        println!("{}", dir.file_name().into_string().unwrap());
    }

    let toml_files = get_files_by_extension("./", &vec!["toml"]);
    println!("\ntoml files\n");

    for file in toml_files {
        println!("{}", file.file_name().into_string().unwrap());
    }

    let all_toml_files = get_files_by_extensions_recursive("./", &vec!["toml", "rs"]);
    println!("\nall toml files\n");

    for file in all_toml_files {
        println!("{}", file.file_name().into_string().unwrap())
    }
}
