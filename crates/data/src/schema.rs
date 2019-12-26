table! {
    comments (id) {
        id -> Int4,
        author -> Int4,
        post -> Int4,
        parent -> Nullable<Int4>,
        body -> Text,
        created -> Timestamp,
    }
}

table! {
    friends (id) {
        id -> Int4,
        sender -> Int4,
        receiver -> Int4,
        accepted -> Bool,
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

joinable!(comments -> posts (post));
joinable!(comments -> views (author));
joinable!(posts -> views (view_id));
joinable!(views -> users (user_id));

allow_tables_to_appear_in_same_query!(
    comments,
    friends,
    messages,
    posts,
    users,
    views,
);
