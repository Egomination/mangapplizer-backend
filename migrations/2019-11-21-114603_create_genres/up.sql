-- Your SQL goes here
CREATE TABLE genres(
       id BIGSERIAL,
       genre_name text NOT NULL,
       description text,
       CONSTRAINT genres_pkey PRIMARY KEY(id)
)
