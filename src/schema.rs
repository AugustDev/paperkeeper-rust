table! {
    users (id) {
        id -> Int4,
        email -> Text,
        password -> Text,
        created_at -> Timestamptz,
        blocked -> Bool,
    }
}
