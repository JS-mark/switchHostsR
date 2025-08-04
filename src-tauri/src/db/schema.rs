// @generated automatically by Diesel CLI.

diesel::table! {
    host_group_relations (id) {
        id -> Integer,
        group_id -> Integer,
        host_id -> Integer,
        created_at -> Integer,
    }
}

diesel::table! {
    host_groups (id) {
        id -> Integer,
        user_id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        created_at -> Integer,
        updated_at -> Integer,
    }
}

diesel::table! {
    hosts (id) {
        id -> Integer,
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
    logs (id) {
        id -> Integer,
        user_id -> Integer,
        action -> Text,
        target_type -> Text,
        target_id -> Nullable<Integer>,
        details -> Nullable<Text>,
        created_at -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
        email -> Nullable<Text>,
        avatar -> Nullable<Text>,
        is_admin -> Nullable<Bool>,
        created_at -> Integer,
        updated_at -> Integer,
    }
}

diesel::joinable!(host_group_relations -> host_groups (group_id));
diesel::joinable!(host_group_relations -> hosts (host_id));
diesel::joinable!(host_groups -> users (user_id));
diesel::joinable!(hosts -> users (user_id));
diesel::joinable!(logs -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    host_group_relations,
    host_groups,
    hosts,
    logs,
    users,
);
