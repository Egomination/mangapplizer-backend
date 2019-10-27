-- Your SQL goes here
CREATE TABLE mangas(
      id uuid NOT NULL DEFAULT uuid_generate_v4(),
      created_at timestamp DEFAULT NOW(),
      updated_at timestamp DEFAULT NOW(),
      deleted_at timestamp DEFAULT NULL,
      anilist_id BIGSERIAL NOT NULL,
      cover_image text NOT NULL,
      banner_image text NOT NULL,
      start_date text NOT NULL,
      end_date text NOT NULL,
      status text NOT NULL,
      title text NOT NULL,
      CONSTRAINT mangas_pkey PRIMARY KEY(id)
)
