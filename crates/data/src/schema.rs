table! {
    posts (id) {
        id -> Int4,
        view_id -> Int4,
        title -> Varchar,
        body -> Text,
        created -> Timestamp,
    }
}

table! {
    session (id) {
        id -> Int4,
        user_key -> Varchar,
        user_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Nullable<Varchar>,
        username -> Varchar,
        passhash -> Varchar,
        view -> Nullable<Int4>,
    }
}

table! {
    views (id) {
        id -> Int4,
        user_id -> Int4,
    }
}

joinable!(posts -> views (view_id));
joinable!(session -> users (user_id));
joinable!(views -> users (user_id));

allow_tables_to_appear_in_same_query!(
    posts,
    session,
    users,
    views,
);
