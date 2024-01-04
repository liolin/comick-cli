use serde::Deserialize;
use serde_repr::Deserialize_repr;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct ComicResponse {
    #[serde(rename(deserialize = "firstChap"))]
    pub first_chap: FirstChapter,
    pub comic: Comic,
    pub artists: Vec<Person>,
    pub authors: Vec<Person>,
    #[serde(rename(deserialize = "langList"))]
    pub lang_list: Vec<String>,
    pub demographic: Option<Demographic>,
    #[serde(rename(deserialize = "englishLink"))]
    pub english_link: Option<String>,
    #[serde(rename(deserialize = "matureContent"))]
    pub mature_content: bool,
    //checkVol2Chap1: bool,
}

#[derive(Deserialize, Debug)]
pub struct FirstChapter {}

#[derive(Deserialize, Debug)]
pub struct Comic {
    // pub id: u32,
    pub hid: String,
    pub title: String,
    pub country: String,
    pub status: Status,
    pub links: HashMap<String, String>,
    pub last_chapter: f32,
    pub chapter_count: u32,
    pub demographic: Option<Demographic>,
    pub hentai: bool,
    pub user_follow_count: u32,
    pub follow_rank: u32,
    pub comment_count: u32,
    pub follow_count: u32,
    pub desc: String,
    pub parsed: String,
    pub slug: String,
    // pub mismatch: null, // What is this?
    pub year: u32,
    pub bayesian_rating: String,
    pub rating_count: u32,
    pub content_rating: String,
    pub translation_completed: bool,
    pub chapter_numbers_reset_on_new_volume_manual: bool,
    pub final_chapter: String,
    pub final_volume: String,
    pub noindex: bool,
    // pub relate_from: Vec<RelateFrom>,
}

#[derive(Deserialize, Debug)]
pub struct Person {
    pub name: String,
    pub slug: String,
}

// Which states exists?
#[derive(Deserialize_repr, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Status {
    Ongoing = 1,
    Completed = 2,
    Cancelled = 3,
    Hiatus = 4,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub enum Demographic {
    Shounen,
    Shoujo,
    Seinen,
    Josei,
}

#[derive(Deserialize, Debug)]
pub struct RelateFrom {
    pub relate_to: Name,
    pub md_relates: Relation,
}

#[derive(Deserialize, Debug)]
pub struct Name {
    pub slug: String,
    pub title: String,
}

#[derive(Deserialize, Debug)]
pub struct Relation {
    pub name: String, // TODO: Change to Enum
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub enum RelationType {
    Coloured,
    Sequel,
    Prequel,
    SpinOff,
}

#[derive(Deserialize, Debug)]
pub struct ChaptersResponse {
    pub chapters: Vec<Chapter>,
    pub total: u32,
    pub limit: u32,
}

#[derive(Deserialize, Debug)]
pub struct Chapter {
    pub id: u32,
    pub chap: String,
    // pub title: String,
    // pub vol: Option<String>,
    pub lang: String,
    // pub created_at: String, // TODO: Change to a Date Type
    // pub updated_at: String, // TODO: Change to a Date Type
    // pub up_count: u32,
    // pub down_count: u32,
    // pub group_name: Vec<Option<String>>,
    pub hid: String,
    //pub identities: Option<???>
    //pub md_chapters_groups: ???
}
