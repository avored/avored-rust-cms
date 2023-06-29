// @generated automatically by Diesel CLI.

diesel::table! {
    admin_users (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 255]
        created_by -> Varchar,
        #[max_length = 255]
        updated_by -> Varchar,
    }
}
