// @generated automatically by Diesel CLI.

diesel::table! {
    characters (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        secrets -> Nullable<Text>,
    }
}
