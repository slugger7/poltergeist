use std::vec;

use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::{PgConnection, SelectableHelper};

use crate::models::library_path::LibraryPath;
use crate::models::video::{NewVideo, Video};
use crate::repositories::library_path_repository::get_library_path_entity_by_id;
use crate::schema::video::{checksum, id, table as video_table};

pub fn create_videos(conn: &mut PgConnection, videos: &Vec<NewVideo>) -> Vec<Video> {
    println!("{}", videos.len());

    let mut created_vids: Vec<Video> = Vec::new();
    for chunk in videos.chunks(5000) {
        let mut fresh_vids = diesel::insert_into(video_table)
            .values(chunk)
            .get_results(conn)
            .expect("Error saving videos");
        created_vids.append(&mut fresh_vids)
    }

    return created_vids;
}

pub fn create_video(
    conn: &mut PgConnection,
    library_path_id: &i32,
    title: String,
    file_name: String,
    height: &i32,
    width: &i32,
    runtime: &i64,
    size: &i64,
) -> Option<Video> {
    match get_library_path_entity_by_id(conn, *library_path_id) {
        Some(lib_path) => lib_path,
        None => {
            println!("No library path was found");
            return None;
        }
    };

    let new_video = NewVideo {
        library_path_id,
        relative_path: String::new(),
        title,
        file_name,
        height: *height,
        width: *width,
        runtime: *runtime,
        size: *size,
        checksum: Some(String::from("01234567890123456789012345678901")),
    };

    Some(
        diesel::insert_into(video_table)
            .values(&new_video)
            .returning(Video::as_returning())
            .get_result(conn)
            .expect("Error saving new video"),
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

pub fn add_checksum_to_video_by_id<'a>(
    conn: &mut PgConnection,
    vid_id: &i32,
    vid_checksum: &str,
) -> Result<usize, &'a str> {
    let result = diesel::update(video_table)
        .filter(id.eq(vid_id))
        .set(checksum.eq(vid_checksum))
        .execute(conn);
    match result {
        Ok(num) => return Ok(num),
        Err(err) => {
            eprintln!(
                "Unable to update checksum for video: {} {}\n {}",
                vid_id, vid_checksum, err
            );
            return Err("Could not update checksum for video");
        }
    }
}

pub fn get_video_by_id(conn: &mut PgConnection, vid_id: &i32) -> Option<Video> {
    let result = crate::schema::video::dsl::video
        .find(vid_id)
        .select(Video::as_select())
        .first(conn)
        .optional();
    match result {
        Ok(vid_option) => return vid_option,
        Err(err) => {
            eprintln!("Colud not find video by id: {} {}", vid_id, err);
            return None;
        }
    }
}

pub fn get_videos_without_checksum_limited(
    conn: &mut PgConnection,
    limit: i32,
) -> Vec<(Video, LibraryPath)> {
    use crate::schema::video::dsl::{checksum as checksum_field, video};

    let query = video::table()
        .inner_join(crate::schema::library_path::dsl::library_path::table())
        .filter(checksum_field.is_null())
        .limit(limit as i64)
        .select((Video::as_select(), LibraryPath::as_select()));

    let results: Result<Vec<(Video, LibraryPath)>, _> = query.load::<(Video, LibraryPath)>(conn);

    match results {
        Ok(videos) => {
            return videos;
        }
        Err(err) => {
            eprintln!(
                "Could not get videos without checksums:\nlimit: {}\nerr: {}",
                limit, err
            );
            return vec![];
        }
    };
}
