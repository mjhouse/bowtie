table! {
    posts (id) {
        id -> Int4,
        user_id -> Int4,
        title -> Varchar,
        body -> Text,
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

joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
