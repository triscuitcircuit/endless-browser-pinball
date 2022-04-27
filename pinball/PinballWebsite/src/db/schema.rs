table! {
    images (id) {
        id -> Int4,
        image -> Nullable<Bytea>,
    }
}

table! {
    scores (id) {
        id -> Int4,
        users -> Int4,
        epoch -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        images -> Int4,
    }
}

joinable!(scores -> users (users));
joinable!(users -> images (images));

allow_tables_to_appear_in_same_query!(
    images,
    scores,
    users,
);
