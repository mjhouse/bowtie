table! {
    posts (id) {
        id -> Int4,
        user_id -> Int4,
        title -> Varchar,
        body -> Text,
        created -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Nullable<Varchar>,
        username -> Varchar,
        passhash -> Varchar,
    }
}

table! {
    views (id) {
        id -> Int4,
        user_id -> Int4,
    }
}

joinable!(posts -> users (user_id));
joinable!(views -> users (user_id));

allow_tables_to_appear_in_same_query!(
    posts,
    users,
    views,
);
