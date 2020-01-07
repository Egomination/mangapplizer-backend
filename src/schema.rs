table! {
    genres (id) {
        id -> Int8,
        genre_name -> Text,
        description -> Nullable<Text>,
    }
}

table! {
    genres_lists (id) {
        id -> Int8,
        manga_id -> Uuid,
        genre_id -> Int8,
    }
}

table! {
    kissmanga_chapters (id) {
        id -> Int8,
        manga_id -> Uuid,
        source_name -> Text,
        source_type -> Text,
        chapter_no -> Int4,
        pages -> Jsonb,
    }
}

table! {
    use diesel::sql_types::Uuid;
    use diesel_full_text_search::TsVector;
    use diesel::sql_types::Int8;
    use diesel::sql_types::Timestamp;
    use diesel::sql_types::Text;
    use diesel::sql_types::Nullable;
    mangas (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        anilist_id -> Int8,
        cover_image -> Text,
        banner_image -> Text,
        start_date -> Text,
        end_date -> Text,
        status -> Text,
        description -> Text,
        total_chapters -> Nullable<Text>,
        volumes -> Nullable<Text>,
        english_title -> Text,
        romaji_title -> Text,
        native_title -> Text,
        cover_extra_large -> Text,
        cover_large -> Text,
        cover_medium -> Text,
        popularity -> Int8,
        text_searchable_mangas -> TsVector,
    }
}

table! {
    media (id) {
        id -> Int8,
        manga_id -> Uuid,
        relation_id -> Uuid,
    }
}

table! {
    relations (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        media_type -> Text,
        anilist_id -> Int8,
        relationship_type -> Text,
        status -> Text,
        title -> Text,
        banner_image -> Nullable<Text>,
    }
}

table! {
    series (id) {
        id -> Int8,
        manga_id -> Uuid,
        staff_id -> Uuid,
    }
}

table! {
    staffs (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        anilist_id -> Int8,
        staff_role -> Text,
        staff_name -> Text,
        image -> Text,
        description -> Text,
    }
}

table! {
    tags (id) {
        id -> Int8,
        tag_name -> Text,
        category -> Text,
        is_spoiler -> Bool,
        description -> Text,
    }
}

table! {
    tags_lists (id) {
        id -> Int8,
        manga_id -> Uuid,
        tag_id -> Int8,
    }
}

joinable!(genres_lists -> genres (genre_id));
joinable!(genres_lists -> mangas (manga_id));
joinable!(kissmanga_chapters -> mangas (manga_id));
joinable!(media -> mangas (manga_id));
joinable!(media -> relations (relation_id));
joinable!(series -> mangas (manga_id));
joinable!(series -> staffs (staff_id));
joinable!(tags_lists -> mangas (manga_id));
joinable!(tags_lists -> tags (tag_id));

allow_tables_to_appear_in_same_query!(
    genres,
    genres_lists,
    kissmanga_chapters,
    mangas,
    media,
    relations,
    series,
    staffs,
    tags,
    tags_lists,
);
