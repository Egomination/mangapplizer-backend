-- Your SQL goes here
CREATE TABLE relations(
       id uuid NOT NULL DEFAULT uuid_generate_v4(),
       created_at timestamp DEFAULT NOW(),
       updated_at timestamp DEFAULT NOW(),
       deleted_at timestamp DEFAULT NULL,
       media_type text NOT NULL,
       anilist_id BIGSERIAL NOT NULL,
       relationship_type text NOT NULL,
       status text NOT NULL,
       title text NOT NULL,
       banner_image text,
       CONSTRAINT relations_pkey PRIMARY KEY(id)
)
