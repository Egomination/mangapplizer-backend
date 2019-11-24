// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_json;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Manga {
    #[serde(rename = "anilistId")]
    anilist_id: i64,
    #[serde(rename = "mangaName")]
    manga_name: MangaName,
    status: String,
    description: String,
    #[serde(rename = "startDate")]
    start_date: Date,
    #[serde(rename = "endDate")]
    end_date: Date,
    chapters: Option<serde_json::Value>,
    volumes: Option<serde_json::Value>,
    #[serde(rename = "coverImage")]
    cover_image: CoverImage,
    #[serde(rename = "bannerImage")]
    banner_image: String,
    genres: Vec<String>,
    popularity: i64,
    #[serde(rename = "mangaLinkArray")]
    manga_link_array: Vec<HashMap<String, String>>,
    staff: Vec<Staff>,
    relations: Vec<Relation>,
}

#[derive(Serialize, Deserialize)]
pub struct CoverImage {
    #[serde(rename = "extraLarge")]
    extra_large: String,
    large: String,
    medium: String,
}

#[derive(Serialize, Deserialize)]
pub struct Date {
    year:  Option<i64>,
    month: Option<i64>,
    day:   Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct MangaName {
    romaji:  String,
    english: String,
    native:  String,
}

#[derive(Serialize, Deserialize)]
pub struct Relation {
    #[serde(rename = "anilistId")]
    anilist_id: i64,
    name: String,
    #[serde(rename = "type")]
    relation_type: String,
    #[serde(rename = "mediaType")]
    media_type: String,
    status: String,
    image: String,
}

#[derive(Serialize, Deserialize)]
pub struct Staff {
    #[serde(rename = "anilistId")]
    anilist_id: i64,
    position: String,
    name: String,
    picture: Picture,
    description: String,
}

#[derive(Serialize, Deserialize)]
pub struct Picture {
    large:  String,
    medium: String,
}
