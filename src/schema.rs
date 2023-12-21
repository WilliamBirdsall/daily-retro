// @generated automatically by Diesel CLI.

diesel::table! {
    entries (id) {
        id -> Integer,
        body -> Text,
        created_at -> Text,
    }
}
