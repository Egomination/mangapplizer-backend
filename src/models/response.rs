use diesel::PgConnection;

#[derive(Serialize)]
pub struct Response {
    pub manga:     crate::models::manga::Manga,
    pub staffs:    Vec<crate::models::staff::Staff>,
    pub relations: Vec<crate::models::relation::Relation>,
    pub genres:    Vec<crate::models::genre::Genre>,
}

impl Response {
    pub fn full(
        mid: String,
        connection: &PgConnection,
    ) -> Result<Response, diesel::result::Error> {
        use crate::diesel::ExpressionMethods;
        use crate::diesel::QueryDsl;
        use crate::diesel::RunQueryDsl;
        use crate::models::{
            genre::Genre,
            genre_lists::GenreList,
            manga::Manga,
            manga::MANGAS_COLUMNS,
            media::Media,
            relation::Relation,
            series::Series,
            staff::Staff,
        };
        use crate::schema::{
            genres,
            genres_lists,
            mangas,
            media,
            relations,
            series,
            staffs,
        };

        let uid = uuid::Uuid::parse_str(&mid).unwrap();
        let manga: Manga = mangas::table
            .select(MANGAS_COLUMNS)
            .filter(mangas::id.eq(uid))
            .get_result(connection)?;
        let series: Result<Vec<Series>, diesel::result::Error> = series::table
            .filter(series::manga_id.eq(uid))
            .get_results(connection);
        let staff_vec: Vec<Staff> = series
            .into_iter()
            .flatten()
            .flat_map(|s| {
                staffs::table
                    .filter(staffs::id.eq(s.staff_id))
                    .get_results(connection)
                    .unwrap()
            })
            .collect::<Vec<Staff>>();
        let relations: Result<Vec<Media>, diesel::result::Error> = media::table
            .filter(media::manga_id.eq(uid))
            .get_results(connection);
        let relation_vec: Vec<Relation> = relations
            .into_iter()
            .flatten()
            .flat_map(|r| {
                relations::table
                    .filter(relations::id.eq(r.relation_id))
                    .get_results(connection)
                    .unwrap()
            })
            .collect::<Vec<Relation>>();
        let genres: Result<Vec<GenreList>, diesel::result::Error> =
            genres_lists::table
                .filter(genres_lists::manga_id.eq(uid))
                .get_results(connection);
        let genre_vec: Vec<Genre> = genres
            .into_iter()
            .flatten()
            .flat_map(|g| {
                genres::table
                    .filter(genres::id.eq(g.genre_id))
                    .get_results(connection)
                    .unwrap()
            })
            .collect::<Vec<Genre>>();

        Ok(Response {
            manga:     manga,
            staffs:    staff_vec,
            relations: relation_vec,
            genres:    genre_vec,
        })
    }
}
