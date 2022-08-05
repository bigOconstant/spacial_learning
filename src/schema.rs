table! {
    user_sessions (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        uuid -> Varchar,
        created_on -> Timestamp,
    }
}

table! {
    users (user_id) {
        user_id -> Int4,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
        created_on -> Timestamp,
        last_login -> Timestamp,
    }
}

joinable!(user_sessions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    user_sessions,
    users,
);
