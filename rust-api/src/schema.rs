// @generated automatically by Diesel CLI.

diesel::table! {
    admin_users (id) {
        id -> Uuid,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    posts (id) {
        id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        body -> Text,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    admin_users,
    posts,
);
