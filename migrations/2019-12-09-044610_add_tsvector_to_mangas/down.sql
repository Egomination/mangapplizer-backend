-- This file should undo anything in `up.sql`

ALTER TABLE mangas DROP COLUMN text_searchable_mangas;

DROP TRIGGER tsvectorupdatemangas ON mangas;
