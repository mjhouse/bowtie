table! {
    users (id) {
        id -> Int4,
        email -> Nullable<Varchar>,
        username -> Varchar,
        passhash -> Varchar,
    }
}
