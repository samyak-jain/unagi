table! {
    episodes (id) {
        id -> Int4,
        show_id -> Int4,
        name -> Varchar,
        thumbnail -> Nullable<Varchar>,
        file_path -> Varchar,
        locator_id -> Uuid,
        episode_number -> Nullable<Int4>,
    }
}

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
        library_id -> Int4,
        title -> Varchar,
        file_path -> Varchar,
        description -> Nullable<Varchar>,
        cover_image -> Nullable<Varchar>,
        banner_image -> Nullable<Varchar>,
        season -> Int8,
        parent_season -> Int8,
    }
}

joinable!(episodes -> shows (show_id));
joinable!(shows -> library (library_id));

allow_tables_to_appear_in_same_query!(
    episodes,
    library,
    shows,
);
