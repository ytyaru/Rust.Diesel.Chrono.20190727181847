table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        is_published -> Bool,
        published -> Nullable<Timestamp>,
    }
}
