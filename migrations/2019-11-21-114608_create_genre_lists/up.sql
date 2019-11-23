-- Your SQL goes here
CREATE TABLE genre_lists(
       id BIGSERIAL,
       manga_id uuid NOT NULL REFERENCES mangas(id),
       genre_id BIGSERIAL NOT NULL REFERENCES genres(id),
       CONSTRAINT genre_lists_pkey PRIMARY KEY(id)
)
