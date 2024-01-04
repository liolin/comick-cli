use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ChapterResponse {
    pub chapter: Chapter,
    #[serde(rename(deserialize = "matureContent"))]
    pub mature_content: bool,
}

#[derive(Deserialize, Debug)]
pub struct Chapter {
    pub id: u32,
    pub chap: String,
    pub vol: Option<String>,
    pub title: Option<String>,
    pub hid: String,
    //pub group_name: Vec<>,
    pub chapter_id: Option<u32>,
    pub created_at: String, // TODO: Change to date type 
    pub updated_at: String, // TODO: Change to date type
    pub crawled_at: String, // TODO: Change to date type
    pub mdid: Option<String>,
    pub comment_count: u32,
    pub up_count: u32,
    pub down_count: u32,
    pub status: String, // TODO: Enum?
    pub adsense: bool,
    pub lang: String,
    //pub md_comics: Vec<>,
    pub md_images: Vec<Image>,
    //pub md_chapters_groups: Vec<>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Image {
    pub b2key: String,
}
