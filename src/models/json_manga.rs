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

#[derive(Serialize, Deserialize, Clone)]
pub struct Manga {
    #[serde(rename = "anilistId")]
    pub anilist_id: i64,
    #[serde(rename = "mangaName")]
    pub manga_name: MangaName,
    pub status: String,
    pub description: String,
    #[serde(rename = "startDate")]
    pub start_date: Date,
    #[serde(rename = "endDate")]
    pub end_date: Date,
    #[serde(rename = "chapters")]
    pub total_chapters: Option<String>,
    pub volumes: Option<String>,
    #[serde(rename = "coverImage")]
    pub cover_image: CoverImage,
    #[serde(rename = "bannerImage")]
    pub banner_image: String,
    pub genres: Vec<String>,
    pub popularity: i64,
    #[serde(rename = "mangaLinkArray")]
    pub manga_link_array: Vec<HashMap<String, String>>,
    pub staff: Vec<Staff>,
    pub relations: Vec<Relation>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CoverImage {
    #[serde(rename = "extraLarge")]
    pub extra_large: String,
    pub large: String,
    pub medium: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Date {
    pub year:  Option<i64>,
    pub month: Option<i64>,
    pub day:   Option<i64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MangaName {
    pub romaji:  String,
    pub english: String,
    pub native:  String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Relation {
    #[serde(rename = "anilistId")]
    pub anilist_id: i64,
    pub name: String,
    #[serde(rename = "type")]
    pub relation_type: String,
    #[serde(rename = "mediaType")]
    pub media_type: String,
    pub status: String,
    pub image: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Staff {
    #[serde(rename = "anilistId")]
    pub anilist_id: i64,
    pub position: String,
    pub name: String,
    pub picture: Picture,
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Picture {
    pub large:  String,
    pub medium: String,
}

impl std::fmt::Display for Date {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        write!(f, "{:?}/{:?}/{:?}", self.year, self.month, self.day)
    }
}
