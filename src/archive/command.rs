use crate::{
    api::{chapter::ChapterResponse, comic::ComicResponse},
    Execute,
};

use anyhow::Result;
use tracing::{debug, info};

use tokio::{fs::File, io::AsyncWriteExt};

#[derive(Clone, Debug, clap::Parser)]
#[clap(about = "Download a chapter")]
#[command(arg_required_else_help(true))]
pub struct Archive {
    #[arg(help = "Name of the output file")]
    #[arg(long_help = "Name of the output file. \
                       {series_name}    -> Name of the series\n \
                       {volume_number}  -> Number of the Volume\n \
                       {chapter_number} -> Number of the Chapter\n \
                       {image_seq}      -> Sequence number of the image")]
    #[arg(
        short,
        long,
        default_value = "{series_name}/Vol-{volume_number}/Ch-{chapter_number}/{series_name}-{volume_number}-{chapter_number}-{image_seq}.jpg"
    )]
    pub(crate) output: String,

    #[arg(help = "Comic slug")]
    #[arg(required = true)]
    #[arg(short, long)]
    pub(crate) slug: String,

    #[arg(help = "Language")]
    #[arg(short, long, default_value = "en")]
    pub(crate) language: String,

    #[arg(help = "API URL", default_value = "https://api.comick.fun")]
    #[arg(short, long)]
    pub(crate) api_url: String,

    #[arg(help = "Image URL", default_value = "https://meo3.comick.pictures")]
    #[arg(short, long)]
    pub(crate) image_url: String,
}

#[async_trait::async_trait]
impl Execute for Archive {
    fn pre_check(&self) -> Result<()> {
        Ok(())
    }

    async fn execute(self) -> Result<()> {
        let comic_url = format!("{}/comic/{}", self.api_url, self.slug);
        info!("Start archiving comic {}", comic_url);
        let client = reqwest::Client::builder()
            .user_agent("comick-cli")
            .build()?;

        let comic_response = client
            .get(comic_url)
            .send()
            .await?
            .json::<crate::api::comic::ComicResponse>()
            .await?;
        let comic_hid = &comic_response.comic.hid;
        debug!("hid for {}: {}", self.slug, comic_hid);

        let chapters = client
            .get(format!("{}/comic/{}/chapters", self.api_url, comic_hid))
            .send()
            .await?
            .json::<crate::api::comic::ChaptersResponse>()
            .await?
            .chapters
            .into_iter()
            .filter(|c| c.lang == self.language)
            .collect::<Vec<_>>();

        for chapter in chapters {
            info!("Download chapter {} with hid {}", chapter.chap, chapter.hid);
            let chapter_response = client
                .get(format!("{}/chapter/{}", self.api_url, chapter.hid))
                .send()
                .await?
                .json::<crate::api::chapter::ChapterResponse>()
                .await?;
            let images = chapter_response
                .chapter
                .md_images
                .clone()
                .into_iter()
                .map(|i| i.b2key)
                .collect::<Vec<_>>();
            debug!("Image urls: {}", images.join(","));

            for (i, name) in images.iter().enumerate() {
                let filename = prepare_filename(&self.output, &comic_response, &chapter_response, i);
                create_folder_structure(&filename)?;
                download(
                    &filename,
                    &format!("{}/{}", self.image_url, name),
                )
                .await?;
            }
        }
        Ok(())
    }
}

async fn download(filename: &str, url: &str) -> Result<()> {
    debug!("Save image to {}", filename);
    let mut file = File::create(filename).await?;
    let mut response = reqwest::get(url).await?;
    while let Some(chunk) = response.chunk().await? {
        file.write(&chunk).await?;
    }

    Ok(())
}

fn prepare_filename(
    format: &str,
    comic_info: &ComicResponse,
    chapter_info: &ChapterResponse,
    seq: usize,
) -> String {
    let vol = chapter_info.chapter.vol.clone().unwrap_or("none".into());
    format
        .replace("{series_name}", &comic_info.comic.title)
        .replace("{volume_number}", &format!("{:0>3}", vol))
        .replace("{chapter_number}", &format!("{:0>3}", chapter_info.chapter.chap))
        .replace("{image_seq}", &format!("{:0>3}", seq.to_string()))
}

fn create_folder_structure(path: &str) -> Result<()> {
    if let Some((folders,_)) = path.rsplit_once('/') {
        if folders.len() > 0 {
            std::fs::create_dir_all(folders)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn create_blub() {
        let volumen_number = "1";
        let with_leading_zero = 

        assert_eq!("001", &with_leading_zero);
    }
}
