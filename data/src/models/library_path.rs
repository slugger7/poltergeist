use crate::models::library::Library;
use crate::schema::library_path;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = library_path)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Library))]
pub struct LibraryPath {
    pub id: i32,
    pub path: String,
    pub library_id: i32,
    pub created: SystemTime,
    pub modified: SystemTime,
}

#[derive(Insertable)]
#[diesel(table_name = library_path)]
pub struct NewLibraryPath<'a> {
    pub path: &'a str,
    pub library_id: &'a i32,
}
