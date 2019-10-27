-- Your SQL goes here
CREATE TABLE media(
       id BIGSERIAL,
       manga_id uuid NOT NULL REFERENCES mangas(id),
       relation_id uuid NOT NULL REFERENCES relations(id),
       CONSTRAINT media_pkey PRIMARY KEY(id)
)
