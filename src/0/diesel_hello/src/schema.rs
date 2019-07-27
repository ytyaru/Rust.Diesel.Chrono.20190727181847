table! {
    posts (id) {
        id -> Nullable<Integer>,
        title -> Text,
        body -> Text,
        is_published -> Text,
        published -> Nullable<Text>,
    }
}
