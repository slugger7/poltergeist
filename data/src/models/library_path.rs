use crate::schema::library_path;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = library_path)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(LibraryEntity))]
pub struct LibraryPathEntity {
    pub id: i32,
    pub path: String,
    pub created: SystemTime,
    pub modified: SystemTime,
}

#[derive(Insertable)]
#[diesel(table_name = library_path)]
pub struct NewLibraryPath<'a> {
    pub path: &'a str,
    pub library_id: &'a i32,
}
