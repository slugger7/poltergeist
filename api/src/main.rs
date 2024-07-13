use data::{
    establish_connection,
    models::video::NewVideo,
    repositories::{
        library_path_repository::{create_library_path, get_library_path_entity_by_id},
        library_repository::create_library,
        video_repository::create_videos,
    },
};
use media::{create_relative_path, file_name_without_extension, get_files_by_extensions_recursive};

fn main() {
    let conn = &mut establish_connection();
    let lib = create_library(conn, "Default");
    create_library_path(conn, "", &lib.id);

    if let Some(lib_path) = get_library_path_entity_by_id(conn, 1) {
        println!("Fetching videos from disc");

        let videos = get_files_by_extensions_recursive(
            &lib_path.path,
            &vec![
                "mp4", "m4v", "mkv", "avi", "wmv", "flv", "webm", "f4v", "mpg", "m2ts", "mov",
            ],
        );

        println!("Length {}", videos.len());

        println!("Done fetching from disc\n");

        println!("Adding videos to database");
        let mut new_videos: Vec<NewVideo> = Vec::new();
        for vid in videos {
            match vid.file_name().into_string() {
                Ok(filename) => {
                    if let Some(absolute_path) = vid.path().to_str() {
                        match create_relative_path(lib_path.path.clone(), absolute_path.to_string())
                        {
                            Ok(relative_path) => new_videos.push(NewVideo {
                                library_path_id: &lib_path.id,
                                relative_path: relative_path,
                                file_name: filename.clone(),
                                title: file_name_without_extension(&filename),
                                height: &480,
                                width: &480,
                                runtime: &480,
                                size: &480,
                                checksum: None,
                            }),
                            Err(err) => print!("{}", err),
                        }
                    }
                }
                Err(_) => println!("Could not turn file name into a string"),
            }
        }

        println!("New video entities created");

        create_videos(conn, &new_videos);
    }
    println!("Done");
}
