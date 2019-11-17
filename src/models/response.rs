use diesel::PgConnection;

#[derive(Serialize)]
pub struct Response {
    pub manga:     crate::models::manga::Manga,
    pub staffs:    Vec<crate::models::staff::Staff>,
    pub relations: Vec<crate::models::relation::Relation>,
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
            manga::Manga,
            media::Media,
            relation::Relation,
            series::Series,
            staff::Staff,
        };
        use crate::schema::{
            mangas,
            media,
            relations,
            series,
            staffs,
        };

        let uid = uuid::Uuid::parse_str(&mid).unwrap();
        let manga: Manga = mangas::table
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

        Ok(Response {
            manga:     manga,
            staffs:    staff_vec,
            relations: relation_vec,
        })
    }
}
