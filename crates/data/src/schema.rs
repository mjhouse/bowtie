table! {
    friend_requests (id) {
        id -> Int4,
        view1 -> Int4,
        view2 -> Nullable<Int4>,
    }
}

table! {
    friends (id) {
        id -> Int4,
        view1 -> Int4,
        view2 -> Int4,
    }
}

table! {
    messages (id) {
        id -> Int4,
        sender -> Int4,
        receiver -> Int4,
        body -> Text,
        created -> Timestamp,
    }
}

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
        name -> Varchar,
    }
}

joinable!(posts -> views (view_id));
joinable!(views -> users (user_id));

allow_tables_to_appear_in_same_query!(
    friend_requests,
    friends,
    messages,
    posts,
    users,
    views,
);
