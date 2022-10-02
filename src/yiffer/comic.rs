use anyhow::anyhow;
use itertools::Itertools;
use scraper::{Html, Selector};
use url::Url;

#[derive(Debug)]
pub struct YifferComic {
    pub name: String,
    pub artist: String,
    pub pages: Vec<Url>,
}

impl YifferComic {
    pub fn parse(body: &str) -> anyhow::Result<Self> {
        let document = Html::parse_document(body);

        // Pull the title
        let title_selector = Selector::parse("h1.loadedComicHeader").unwrap();
        let name = document
            .select(&title_selector)
            .map(|x| x.inner_html())
            .unique()
            .next()
            .ok_or_else(|| anyhow!("Comic Title Not Found"))?;

        // Pull the author
        let artist_selector = Selector::parse("a.artistNameLink").unwrap();
        let artist = document
            .select(&artist_selector)
            .map(|x| x.inner_html())
            .unique()
            .next()
            .ok_or_else(|| anyhow!("Comic Artist Not Found"))?
            .trim()
            .into();

        // Pull the pages
        let page_selector = Selector::parse("img.comic-page").unwrap();
        let pages: Vec<Url> = document
            .select(&page_selector)
            .filter_map(|x| x.value().attr("src"))
            .filter_map(|x| Url::parse(x).ok())
            .unique()
            .collect();

        let comic = YifferComic {
            name,
            artist,
            pages,
        };
        Ok(comic)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_page() {
        let body = std::fs::read_to_string("test/Kissy Cousin - Yiffer.html").unwrap();
        let comic = YifferComic::parse(&body).unwrap();
        assert_eq!("Kissy Cousin", comic.name);
        assert_eq!("Kamicheetah", comic.artist);

        let pages =
            vec![Url::parse("https://static.yiffer.xyz/comics/Kissy Cousin/001.jpg").unwrap()];
        assert_eq!(pages[0], comic.pages[0]);

        let pages =
            vec![Url::parse("https://static.yiffer.xyz/comics/Kissy Cousin/042.jpg").unwrap()];

        assert_eq!(&pages[0], comic.pages.last().unwrap());

        assert_eq!(42, comic.pages.len());
    }
}
