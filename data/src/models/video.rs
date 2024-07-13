use crate::models::library_path::LibraryPath;
use crate::schema::video;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = video)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(LibraryPath))]
pub struct Video {
    pub id: i32,
    pub library_path_id: i32,
    pub relative_path: String,
    pub title: String,
    pub file_name: String,
    pub height: i32,
    pub width: i32,
    pub runtime: i64,
    pub size: i64,
    pub checksum: Option<String>,
    pub added: SystemTime,
    pub deleted: bool,
    pub created: SystemTime,
    pub modified: SystemTime,
}

#[derive(Insertable)]
#[diesel(table_name = video)]
pub struct NewVideo<'a> {
    pub library_path_id: &'a i32,
    pub relative_path: String,
    pub title: String,
    pub file_name: String,
    pub height: i32,
    pub width: i32,
    pub runtime: i64,
    pub size: i64,
    pub checksum: Option<String>,
}
