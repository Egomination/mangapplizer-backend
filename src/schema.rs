table! {
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
        title -> Text,
        description -> Text,
        total_chapters -> Nullable<Text>,
        volumes -> Nullable<Text>,
        genres -> Array<Text>,
        popularity -> Int8,
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
        banner_image -> Text,
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

joinable!(media -> mangas (manga_id));
joinable!(media -> relations (relation_id));
joinable!(series -> mangas (manga_id));
joinable!(series -> staffs (staff_id));

allow_tables_to_appear_in_same_query!(
    mangas,
    media,
    relations,
    series,
    staffs,
);
