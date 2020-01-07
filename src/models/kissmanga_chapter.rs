use crate::schema::kissmanga_chapters;
use diesel::PgConnection;

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
    ) -> Result<KmChapter, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::insert_into(kissmanga_chapters::table)
            .values(self)
            .get_result::<KmChapter>(connection)
    }

    pub fn increment_ch(
        manga_id: uuid::Uuid,
        connection: &PgConnection,
    ) -> i32 {
        use crate::diesel::ExpressionMethods;
        use crate::diesel::QueryDsl;
        use crate::diesel::RunQueryDsl;
        let manga: Vec<KmChapter> = kissmanga_chapters::table
            .filter(kissmanga_chapters::manga_id.eq(&manga_id))
            .get_results(connection)
            .unwrap();

        let mut ch_no = 1;
        manga.iter().for_each(|ch| {
            if ch.chapter_no > ch_no {
                ch_no = ch.chapter_no
            } else if ch.chapter_no == ch_no {
                ch_no += 1
            }
        });
        ch_no
    }

    pub fn latest(
        manga_id: uuid::Uuid,
        connection: &PgConnection,
    ) -> i32 {
        use crate::diesel::ExpressionMethods;
        use crate::diesel::QueryDsl;
        use crate::diesel::RunQueryDsl;
        let manga: Vec<KmChapter> = kissmanga_chapters::table
            .filter(kissmanga_chapters::manga_id.eq(&manga_id))
            .get_results(connection)
            .unwrap();

        let mut ch_no = 1;
        manga.iter().for_each(|ch| {
            if ch.chapter_no > ch_no {
                ch_no = ch.chapter_no
            }
        });
        ch_no
    }
}
