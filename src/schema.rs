table! {
    library (id) {
        id -> Int4,
        name -> Varchar,
        location -> Varchar,
    }
}

table! {
    shows (id) {
        id -> Int4,
        library_id -> Nullable<Int4>,
        title -> Varchar,
        image -> Nullable<Varchar>,
        file_path -> Varchar,
    }
}

joinable!(shows -> library (library_id));

allow_tables_to_appear_in_same_query!(
    library,
    shows,
);
