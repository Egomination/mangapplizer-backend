use crate::errors::MangapplizerError;
use crate::errors::MangapplizerResult;
use crate::models::manga;
use crate::schema::kissmanga_chapters;
use diesel::PgConnection;
use std::collections::HashMap;

type Page = HashMap<String, String>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chapter {
    pub manga_name: String,
    pub chapters:   Vec<Page>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QueryData {
    pub source_name: String,
    pub source_type: String,
}

#[derive(
    Queryable,
    QueryableByName,
    Identifiable,
    Associations,
    Debug,
    Serialize,
    Deserialize,
)]
#[table_name = "kissmanga_chapters"]
pub struct KmChapter {
    pub id:          i64,
    pub manga_id:    uuid::Uuid,
    pub source_name: String,
    pub source_type: String,
    pub chapter_no:  i32,
    pub pages:       serde_json::Value,
}

#[derive(Insertable, Debug, Deserialize)]
#[table_name = "kissmanga_chapters"]
pub struct NewKmChapter<'a> {
    pub manga_id:    uuid::Uuid,
    pub source_name: &'a str,
    pub source_type: &'a str,
    pub chapter_no:  i32,
    pub pages:       serde_json::Value,
}

impl<'a> NewKmChapter<'a> {
    pub fn create(
        &self,
        connection: &PgConnection,
    ) -> MangapplizerResult<KmChapter> {
        use diesel::RunQueryDsl;

        let res = diesel::insert_into(kissmanga_chapters::table)
            .values(self)
            .get_result::<KmChapter>(connection)?;

        Ok(res)
    }

    pub fn insert_chapter(
        json_data: &Chapter,
        query_data: &QueryData,
        connection: &PgConnection,
    ) -> MangapplizerResult<&'a str> {
        use diesel::RunQueryDsl;

        let search = &json_data.manga_name;
        let search_result = manga::MangaList::list(&connection, search);

        // Early return if result is not singular
        if search_result.len() > 1 {
            return Err(MangapplizerError::TooManyMangas);
        } else if search_result.is_empty() {
            return Err(MangapplizerError::EmptySearch);
        }
        // chapter_data.chapters.iter().for_each(|m| print_pages(m));
        let mut ch_no = Self::latest(search_result.0[0].id, connection);
        ch_no += 1;

        let sname = query_data.source_name.to_owned();
        let stype = query_data.source_type.to_owned();
        json_data.chapters.iter().for_each(|c| {
            // I am going to store Page pairs as Json in Postgres
            let chapter_json_data = serde_json::to_value(&c);
            let chapter = Self {
                manga_id:    search_result.0[0].id,
                source_name: &sname,
                source_type: &stype,
                chapter_no:  ch_no,
                pages:       chapter_json_data.unwrap(),
            };

            diesel::insert_into(kissmanga_chapters::table)
                .values(&chapter)
                .get_result::<KmChapter>(connection)
                .unwrap();
            ch_no += 1;
        });
        Ok("Chapters inserted!")
    }

    pub fn latest(
        manga_id: uuid::Uuid,
        connection: &PgConnection,
    ) -> i32 {
        use crate::diesel::ExpressionMethods;
        use crate::diesel::QueryDsl;
        use crate::diesel::RunQueryDsl;
        use std::convert::TryInto;

        kissmanga_chapters::table
            .filter(kissmanga_chapters::manga_id.eq(&manga_id))
            .get_results::<KmChapter>(connection)
            .unwrap()
            .len()
            .try_into()
            .unwrap()

        // manga.len().try_into().unwrap()
    }
}
