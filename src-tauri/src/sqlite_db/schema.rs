// @generated automatically by Diesel CLI.

diesel::table! {
    host_group_relations (id) {
        id -> Nullable<Integer>,
        group_id -> Integer,
        host_id -> Integer,
        created_at -> Integer,
    }
}

diesel::table! {
    host_groups (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        created_at -> Integer,
        updated_at -> Integer,
    }
}

diesel::table! {
    hosts (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        content -> Text,
        is_active -> Integer,
        is_system -> Integer,
        created_at -> Integer,
        updated_at -> Integer,
    }
}

diesel::table! {
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
    logs (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        action -> Text,
        target_type -> Text,
        target_id -> Nullable<Integer>,
        details -> Nullable<Text>,
        created_at -> Integer,
    }
}

diesel::table! {
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
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
        password -> Text,
        email -> Nullable<Text>,
        avatar -> Nullable<Text>,
        created_at -> Integer,
        updated_at -> Integer,
    }
}

diesel::joinable!(host_group_relations -> host_groups (group_id));
diesel::joinable!(host_group_relations -> hosts (host_id));
diesel::joinable!(host_groups -> users (user_id));
diesel::joinable!(hosts -> users (user_id));
diesel::joinable!(hosts_data -> users (created_by));
diesel::joinable!(hosts_refresh_data -> hosts_data (hosts_id));
diesel::joinable!(hosts_refresh_data -> users (created_by));
diesel::joinable!(logs -> users (user_id));
diesel::joinable!(permission_list -> users (created_by));
diesel::joinable!(third_account -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    host_group_relations,
    host_groups,
    hosts,
    hosts_data,
    hosts_refresh_data,
    logs,
    permission_list,
    third_account,
    users,
);
