// @generated automatically by Diesel CLI.

diesel::table! {
    theorems (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
    }
}
