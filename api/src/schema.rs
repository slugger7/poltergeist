// @generated automatically by Diesel CLI.

diesel::table! {
    library (id) {
        id -> Int4,
        name -> Varchar,
        created -> Timestamp,
        modified -> Timestamp,
    }
}
