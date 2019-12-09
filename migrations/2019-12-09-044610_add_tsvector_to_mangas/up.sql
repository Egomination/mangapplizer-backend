-- Your SQL goes here

ALTER TABLE mangas ADD COLUMN text_searchable_mangas tsvector NOT NULL;

UPDATE mangas SET text_searchable_mangas = to_tsvector('english', coalesce(english_title, '') || ' ' || coalesce(romaji_title, '') || ' ' || coalesce(native_title, ''));

CREATE INDEX textsearch_idx ON mangas USING rum (text_searchable_mangas rum_tsvector_ops);

CREATE TRIGGER tsvectorupdatemangas BEFORE INSERT OR UPDATE
ON mangas FOR EACH ROW EXECUTE PROCEDURE
tsvector_update_trigger(text_searchable_mangas, 'pg_catalog.english', english_title, romaji_title, native_title);

