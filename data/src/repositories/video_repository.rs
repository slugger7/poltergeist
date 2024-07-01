use diesel::prelude::*;
use diesel::{PgConnection, SelectableHelper};

use crate::models::video::VideoEntity;

pub fn show_videos(conn: &mut PgConnection) {
    use crate::schema::video::dsl::*;

    let results = video
        .limit(5)
        .select(VideoEntity::as_select())
        .load(conn)
        .expect("Error loading videos");
    println!("Displaying {} videos", results.len());

    for vid in results {
        println!("{}", vid.title)
    }
}
