use crate::schema::video;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = video)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(LibraryPathEntity))]
pub struct VideoEntity {
    pub id: i32,
    pub library_path_id: i32,
    pub title: String,
    pub file_name: String,
    pub height: i32,
    pub width: i32,
    pub runtime: i64,
    pub size: i64,
    pub added: SystemTime,
    pub deleted: bool,
    pub created: SystemTime,
    pub modified: SystemTime,
}
