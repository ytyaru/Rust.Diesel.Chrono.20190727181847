table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        is_published -> Bool,
        published -> chrono::DateTime<chrono::Local>,
    }
}
