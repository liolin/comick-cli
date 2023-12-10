use crate::{ComickInformation, Execute};

use anyhow::{bail, Result};
use fantoccini::{Client, ClientBuilder, Locator};
use futures_util::TryStreamExt;
use thiserror::Error;
use tracing::{debug, info};

use futures::stream::{self, StreamExt};
use tokio::{fs::File, io::AsyncWriteExt};

#[derive(Clone, Debug, clap::Parser)]
#[clap(about = "Download a chapter")]
#[command(arg_required_else_help(true))]
pub struct Download {
    #[arg(help = "webdriver")]
    #[arg(required = true)]
    pub(crate) webdriver: String,

    #[arg(help = "Name of the output file")]
    #[arg(long_help = "Nam eof the output file. \
                       {series_name}    -> Name of the series\n \
                       {chapter_number} -> Number of the Chapter\n")]
    #[arg(short, long, default_value = "{series_name}-{chapter_number}.jpg")]
    pub(crate) output: String,

    #[arg(help = "comick book url(s)")]
    #[arg(required = true)]
    pub(crate) urls: Vec<String>,
}

#[async_trait::async_trait]
impl Execute for Download {
    fn pre_check(&self) -> Result<()> {
        if has_webdriver(&self.webdriver) {
            bail!("A webdriver is required to run this command");
        }

        Ok(())
    }

    async fn execute(self) -> Result<()> {
        start_webdriver(&self.webdriver)?;

        let c = ClientBuilder::native()
            .connect("http://localhost:4444")
            .await?;

        for url in self.urls {
            let info = parse_url(&url)?;
            c.goto(&url).await?;
            let img_urls = extract_urls(&c).await?;

            for (i, img) in img_urls.into_iter().enumerate() {
                download(&build_file_path(&info, i, &self.output), &img).await?;
            }
        }
        c.close().await?;
        Ok(())
    }
}
async fn extract_urls(client: &Client) -> Result<Vec<String>> {
    Ok(stream::iter(
        client
            .find_all(Locator::Css(".reader-container img"))
            .await?
            .into_iter(),
    )
    .then(|e| async move { e.attr("src").await })
    .try_collect::<Vec<_>>()
    .await?
    .into_iter()
    .flat_map(|e| e)
    .collect())
}

async fn download(file_name: &str, url: &str) -> Result<()> {
    info!("Create file: {file_name}");
    let mut file = File::create(file_name).await?;

    info!("Request image");
    let mut response = reqwest::get(url).await?;
    while let Some(chunk) = response.chunk().await? {
        file.write(&chunk).await?;
    }

    Ok(())
}

fn start_webdriver(driver: &str) -> Result<()> {
    Command::new(driver).spawn()?;
    Ok(())
}

fn parse_url(url: &str) -> Result<ComickInformation> {
    let mut splits = url.split('/');
    let title = splits.nth(4).ok_or(ComickError::InvalidUrl)?;
    let chapter_part = splits.next();
    let (slug, chapter, language) = chapter_part
        .and_then(|p| Some(p.split('-').collect::<Vec<_>>()))
        .map_or((None, None, None), |v| {
            (v.get(0).copied(), v.get(2).copied(), v.get(3).copied())
        });

    Ok(ComickInformation {
        series_name: title,
        slug,
        chapter_number: chapter,
        language,
    })
}

fn build_file_path(info: &ComickInformation, index: usize, format: &str) -> String {
    let series_name = &info.series_name;
    let chapter_number = &info.chapter_number;
    format
        .replace("{series_name}", series_name)
        .replace("{chapter_number}", chapter_number.unwrap_or_default())
        .replace(".jpg", &format!("-{index}.jpg"))
}

use std::io::ErrorKind;
use std::process::{Command, Stdio};
fn has_webdriver(path: &str) -> bool {
    if let Err(e) = Command::new(path).stderr(Stdio::null()).spawn() {
        if ErrorKind::NotFound != e.kind() {
            debug!(
                "unknown error occurred while checking if webdriver exists: {}",
                e.kind()
            )
        }
        false
    } else {
        true
    }
}

#[derive(Error, Debug)]
pub enum ComickError {
    #[error("The URL does seem not like a valid comick URL")]
    InvalidUrl,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_url_get_title_and_chapter() {
        let url = "https://comick.cc/comic/00-jujutsu-kaisen/9znVx-volume-0-en";
        let info = parse_url(url).unwrap();

        assert_eq!(info.series_name, "00-jujutsu-kaisen");
        assert_eq!(info.slug.unwrap(), "9znVx");
        assert_eq!(info.chapter_number.unwrap(), "0");
        assert_eq!(info.language.unwrap(), "en");
    }

    #[test]
    fn build_file_path_from_info() {
        let info = ComickInformation {
            series_name: "My Manga",
            chapter_number: Some("1"),
            language: Some("en"),
            slug: Some("LPl2r"),
        };

        let format = "{series_name}-c-{chapter_number}.jpg";
        assert_eq!(build_file_path(&info, 1, &format), "My Manga-c-1-1.jpg");
    }
}
