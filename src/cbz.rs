use crate::yiffer::YifferComic;
use reqwest::Client;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use url::Url;
use zip::write::FileOptions;
use zip::ZipWriter;

pub struct Cbz {
    name: String,
    artist: String,
    urls: Vec<Url>,
}

impl Cbz {
    pub fn from(comic: YifferComic) -> Self {
        let name = sanitize_name(&comic.name);
        let artist = comic.artist;
        let urls = comic.pages;
        Self { name, artist, urls }
    }

    pub async fn write(self, directory: Option<String>) -> anyhow::Result<()> {
        let base_dir = match directory {
            Some(d) => d,
            None => "comics".into(),
        };
        let file = comic_file(&base_dir, &self.name, &self.artist);

        let client = Client::new();

        if let Err(e) = write_file(&file, self.urls, &client).await {
            // Ignore removal error
            let _ = fs::remove_file(file);
            eprintln!("Failed to write file: {}", e);
        }

        Ok(())
    }
}

fn sanitize_name(s: &str) -> String {
    s.replace(':', "")
        .replace('/', "")
        .replace('\\', "")
        // Keep this last to remove duplicate spaces
        .replace("  ", " ")
}

fn comic_file(base_dir: &str, name: &str, artist: &str) -> PathBuf {
    let base = PathBuf::from(base_dir);
    let comic_folder = format!("{} by {}", name, artist);
    let cbz = format!("{}.cbz", name);
    base.join(comic_folder).join(cbz)
}

fn filename_from_url(url: &Url) -> String {
    let segs = url.path_segments().unwrap();
    let name = segs.last().unwrap();
    name.into()
}

async fn write_file(file: &PathBuf, urls: Vec<Url>, client: &Client) -> anyhow::Result<()> {
    // Make the dir
    fs::create_dir_all(file.parent().unwrap())?;

    //Set up the zipfile
    let file = File::create(&file)?;
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default();

    // Write files in turm
    for url in urls {
        let filename = filename_from_url(&url);

        let res = client.get(url).send().await?;
        let bytes = res.bytes().await?;
        zip.start_file(filename, options)?;
        zip.write_all(&bytes)?;
    }

    zip.finish()?;
    Ok(())
}
