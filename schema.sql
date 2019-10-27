CREATE TABLE mangas(
      id uuid NOT NULL DEFAULT uuid_generate_v4(),
      created_at timestamp with time zone,
      updated_at timestamp with time zone,
      deleted_at timestamp with time zone,
      cover_image text NOT NULL,
      banner_image text NOT NULL,
      start_date text NOT NULL,
      end_date text NOT NULL,
      status text NOT NULL,
      title text NOT NULL,
      CONSTRAINT mangas_pkey PRIMARY KEY(id)
);

CREATE TABLE staffs(
       id uuid NOT NULL DEFAULT uuid_generate_v4(),
       created_at timestamp with time zone,
       updated_at timestamp with time zone,
       deleted_at timestamp with time zone,
       role text NOT NULL,
       name text NOT NULL UNIQUE,
       image text NOT NULL,
       CONSTRAINT staffs_pkey PRIMARY KEY(id)
);

CREATE TABLE relations(
       id uuid NOT NULL DEFAULT uuid_generate_v4(),
       created_at timestamp with time zone,
       updated_at timestamp with time zone,
       deleted_at timestamp with time zone,
       type text NOT NULL,
       relationship_type text NOT NULL,
       status text NOT NULL,
       title text NOT NULL,
       banner_image text NOT NULL,
       CONSTRAINT relations_pkey PRIMARY KEY(id)
);

CREATE TABLE series(
       manga_id uuid REFERENCES mangas(id),
       staff_id uuid REFERENCES staffs(id)
);

CREATE TABLE media(
       manga_id uuid REFERENCES mangas(id),
       relation_id uuid REFERENCES relations(id)
);
