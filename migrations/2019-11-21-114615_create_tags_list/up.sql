-- Your SQL goes here
CREATE TABLE tags_lists(
       id BIGSERIAL,
       manga_id uuid NOT NULL REFERENCES mangas(id),
       tag_id BIGSERIAL NOT NULL REFERENCES tags(id),
       CONSTRAINT tags_lists_pkey PRIMARY KEY(id)
)
