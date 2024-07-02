use diesel::prelude::*;
use diesel::{PgConnection, SelectableHelper};

use crate::models::library_path::LibraryPath;
use crate::models::video::{NewVideo, Video};
use crate::repositories::library_path_repository::get_library_path_entity_by_id;

pub fn show_videos(conn: &mut PgConnection) {
    use crate::schema::video::dsl::*;

    let results = video
        .limit(5)
        .select(Video::as_select())
        .load(conn)
        .expect("Error loading videos");
    println!("Displaying {} videos", results.len());

    for vid in results {
        println!("{}", vid.title)
    }
}

pub fn create_video(
    conn: &mut PgConnection,
    library_path_id: &i32,
    title: &str,
    file_name: &str,
    height: &i32,
    width: &i32,
    runtime: &i64,
    size: &i64,
) -> Option<Video> {
    use crate::schema::video;

    match get_library_path_entity_by_id(conn, *library_path_id) {
        Some(lib_path) => lib_path,
        None => {
            println!("No library path was found");
            return None;
        }
    };

    let new_video = NewVideo {
        library_path_id,
        title,
        file_name,
        height,
        width,
        runtime,
        size,
    };

    Some(
        diesel::insert_into(video::table)
            .values(&new_video)
            .returning(Video::as_returning())
            .get_result(conn)
            .expect("Error saving now video"),
    )
}

pub fn get_videos_in_library_path(conn: &mut PgConnection, lib_path: LibraryPath) -> Vec<Video> {
    let result = Video::belonging_to(&lib_path)
        .select(Video::as_select())
        .load(conn);

    match result {
        Ok(videos) => videos,
        Err(_) => {
            vec![]
        }
    }
}
