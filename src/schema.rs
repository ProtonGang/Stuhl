table! {
    comments (id) {
        id -> Int4,
        body -> Text,
        user_id -> Int4,
        post_url -> Varchar,
        approved -> Bool,
        date -> Timestamp,
        last_edit -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Int4,
        access_token -> Varchar,
        id_token -> Varchar,
        date -> Timestamp,
        provider -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    comments,
    users,
);
