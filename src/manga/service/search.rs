use crate::database::{
    db_connection,
    Pool,
};
use crate::errors::MangapplizerResult;
use crate::manga::model::{
    Manga,
    MangaList,
    MANGAS_COLUMNS,
};
use actix_web::web;

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub title: String,
}

pub fn search(
    search_query: SearchQuery,
    pool: web::Data<Pool>,
) -> MangapplizerResult<MangaList> {
    use crate::schema;
    use crate::schema::mangas::dsl::*;
    use diesel::pg::Pg;
    use diesel::QueryDsl;
    use diesel::RunQueryDsl;
    use diesel_full_text_search::{
        plainto_tsquery,
        TsVectorExtensions,
    };

    let conn = &db_connection(&pool)?;
    let mut query = schema::mangas::table.into_boxed::<Pg>();

    if !search_query.title.is_empty() {
        query = query.filter(
            text_searchable_mangas.matches(plainto_tsquery(search_query.title)),
        );
    }

    let res = query.select(MANGAS_COLUMNS).load::<Manga>(conn)?;

    Ok(MangaList(res))
}
