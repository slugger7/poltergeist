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

diesel::table! {
    video (id) {
        id -> Int4,
        library_path_id -> Int4,
        relative_path -> Varchar,
        title -> Varchar,
        file_name -> Varchar,
        height -> Int4,
        width -> Int4,
        runtime -> Int8,
        size -> Int8,
        #[max_length = 32]
        checksum -> Nullable<Bpchar>,
        added -> Timestamp,
        deleted -> Bool,
        created -> Timestamp,
        modified -> Timestamp,
    }
}

diesel::joinable!(library_path -> library (library_id));
diesel::joinable!(video -> library_path (library_path_id));

diesel::allow_tables_to_appear_in_same_query!(
    library,
    library_path,
    video,
);
