// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::{Integer, Nullable, Text};
    hosts_data (id) {
        id -> Nullable<Integer>,
        name -> Text,
        hosts_type -> Integer,
        hosts_path -> Nullable<Text>,
        content -> Text,
        status -> Integer,
        is_del -> Integer,
        is_readonly -> Integer,
        created_by -> Integer,
        updated_at -> Text,
        created_at -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::{Integer, Nullable, Text};
    hosts_refresh_data (id) {
        id -> Nullable<Integer>,
        hosts_id -> Integer,
        created_by -> Integer,
        created_at -> Text,
        updated_at -> Text,
        hosts_refresh_time -> Integer,
        last_refresh_time -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::{Integer, Nullable, Text};
    logs (id) {
        id -> Nullable<Integer>,
        log_type -> Integer,
        content -> Text,
        created_at -> Text,
        created_by -> Nullable<Integer>,
    }
}

diesel::table! {
    use diesel::sql_types::{Integer, Nullable, Text};
    permission_list (id) {
        id -> Nullable<Integer>,
        name -> Text,
        menu_list -> Text,
        status -> Integer,
        is_del -> Integer,
        created_by -> Nullable<Integer>,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::{Integer, Nullable, Text};
    third_account (id) {
        id -> Nullable<Integer>,
        uid -> Text,
        user_id -> Nullable<Integer>,
        account -> Text,
        email -> Text,
        avatar_url -> Text,
        nickname -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::{Integer, Nullable, Text};
    users (id) {
        id -> Nullable<Integer>,
        name -> Text,
        email -> Text,
        avatar_url -> Text,
        status -> Integer,
        user_level -> Integer,
        password -> Text,
        is_del -> Integer,
        is_third -> Integer,
        third_account_uid -> Nullable<Text>,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::joinable!(hosts_data -> users (created_by));
diesel::joinable!(hosts_refresh_data -> hosts_data (hosts_id));
diesel::joinable!(hosts_refresh_data -> users (created_by));
diesel::joinable!(logs -> users (created_by));
diesel::joinable!(permission_list -> users (created_by));
diesel::joinable!(third_account -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    hosts_data,
    hosts_refresh_data,
    logs,
    permission_list,
    third_account,
    users,
);
