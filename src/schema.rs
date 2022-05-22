table! {
    comments (id) {
        id -> Int4,
        body -> Text,
        user_id -> Int4,
        approved -> Bool,
        date -> Timestamp,
    }
}
