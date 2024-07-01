use crate::schema::library;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = library)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LibraryEntity {
    pub id: i32,
    pub name: String,
    pub created: SystemTime,
    pub modified: SystemTime,
}

#[derive(Insertable)]
#[diesel(table_name = library)]
pub struct NewLibrary<'a> {
    pub name: &'a str,
}
