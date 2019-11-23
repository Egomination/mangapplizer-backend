-- Your SQL goes here
CREATE TABLE tags(
       id BIGSERIAL,
       tag_name text NOT NULL UNIQUE,
       category text NOT NULL,
       is_spoiler boolean NOT NULL,
       description text NOT NULL,
       CONSTRAINT tags_pkey PRIMARY KEY(id)
)
