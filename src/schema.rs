// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Integer,
        author_id -> Integer,
        title -> Text,
        content -> Text,
        published_date -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        email -> Text,
        password_hash -> Text,
    }
}

diesel::joinable!(posts -> users (author_id));

diesel::allow_tables_to_appear_in_same_query!(posts, users,);
