-- Your SQL goes here
CREATE TABLE staffs(
       id uuid NOT NULL DEFAULT uuid_generate_v4(),
       created_at timestamp DEFAULT NOW(),
       updated_at timestamp DEFAULT NOW(),
       deleted_at timestamp DEFAULT NULL,
       anilist_id BIGSERIAL NOT NULL,
       staff_role text NOT NULL,
       staff_name text NOT NULL UNIQUE,
       image text NOT NULL,
       description text NOT NULL,
       CONSTRAINT staffs_pkey PRIMARY KEY(id)
)
