-- Your SQL goes here
CREATE TABLE kissmanga_chapters(
       id BIGSERIAL,
       manga_id uuid NOT NULL REFERENCES mangas(id),
       source_name text NOT NULL,
       source_type text NOT NULL, -- like EN, JP
       chapter_no INT NOT NULL,
       pages JSONB NOT NULL,
       CONSTRAINT kissmanga_chapters_pkey PRIMARY KEY(id)
)
