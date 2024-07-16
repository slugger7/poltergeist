use data::{
    establish_connection,
    models::video::NewVideo,
    repositories::{
        library_path_repository::{create_library_path, get_library_path_entity_by_id},
        library_repository::create_library,
        video_repository::{
            add_checksum_to_video_by_id, create_videos, get_videos_without_checksum_limited,
        },
    },
};
use media::{
    extensions::{checksum, create_relative_path, file_name_without_extension, file_size},
    ffprobe::dimensions_and_duration,
    get_files_by_extensions_recursive,
};

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
        let length = videos.len();
        let mut i = 1;
        for vid in videos {
            let progress = (i as f64 / length as f64) * 100.00;
            println!("{}%", progress as i32);
            i = i + 1;
            match vid.file_name().into_string() {
                Ok(filename) => {
                    if let Some(absolute_path) = vid.path().to_str() {
                        match create_relative_path(lib_path.path.clone(), absolute_path.to_string())
                        {
                            Ok(relative_path) => {
                                let full_path = [lib_path.path.clone(), relative_path.clone()]
                                    .clone()
                                    .join("/");
                                let size = file_size(full_path.clone().as_str()) as i64;

                                let (width, height, duration) =
                                    dimensions_and_duration(full_path.as_str());

                                new_videos.push(NewVideo {
                                    library_path_id: &lib_path.id,
                                    relative_path: relative_path,
                                    file_name: filename.clone(),
                                    title: file_name_without_extension(&filename),
                                    height: height as i32,
                                    width: width as i32,
                                    runtime: duration as i64,
                                    size: size,
                                    checksum: None,
                                })
                            }
                            Err(err) => print!("{}", err),
                        }
                    }
                }
                Err(_) => println!("Could not turn file name into a string"),
            }
        }

        println!("New video entities created");

        create_videos(conn, &new_videos);

        loop {
            let checksum_vids = get_videos_without_checksum_limited(conn, 10);

            if checksum_vids.len() == 0 {
                break;
            }

            println!("Processing batch");
            for (vid, lib_path) in checksum_vids {
                let full_path = [lib_path.path, vid.relative_path].join("/");
                let calculated_checksum = checksum(full_path.as_str());
                println!("Updating checksum: {} {}", full_path, calculated_checksum);
                let result = add_checksum_to_video_by_id(conn, &vid.id, &calculated_checksum);

                if let Err(error) = result {
                    eprintln!("Something went wrong updating the checksum: {}", error);
                }
            }
        }
    }
    println!("Done");
}
