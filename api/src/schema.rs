// @generated automatically by Diesel CLI.

diesel::table! {
    library (id) {
        id -> Int4,
        name -> Varchar,
        created -> Timestamp,
        modified -> Timestamp,
    }
}

diesel::table! {
    library_path (id) {
        id -> Int4,
        library_id -> Int4,
        path -> Varchar,
        created -> Timestamp,
        modified -> Timestamp,
    }
}

diesel::joinable!(library_path -> library (library_id));

diesel::allow_tables_to_appear_in_same_query!(
    library,
    library_path,
);
