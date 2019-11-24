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
      description text NOT NULL,
      total_chapters text,
      volumes text,
      english_title text NOT NULL,
      romaji_title text NOT NULL,
      native_title text NOT NULL,
      cover_extra_large text NOT NULL,
      cover_large text NOT NULL,
      cover_medium text NOT NULL,
      popularity BIGSERIAL NOT NULL,
      CONSTRAINT mangas_pkey PRIMARY KEY(id)
)
