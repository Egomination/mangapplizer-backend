-- Your SQL goes here
CREATE TABLE series(
       id BIGSERIAL,
       manga_id uuid NOT NULL REFERENCES mangas(id),
       staff_id uuid NOT NULL REFERENCES staffs(id),
       CONSTRAINT series_pkey PRIMARY KEY(id)
)
