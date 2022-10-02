use crate::yiffer::{YifferClient, YifferComic};

pub async fn run(comic: String) -> anyhow::Result<()> {
    let client = YifferClient::default();
    let body = client.comic_page(&comic).await?;
    let comic = YifferComic::parse(&body)?;
    dbg!(comic);
    Ok(())
}
