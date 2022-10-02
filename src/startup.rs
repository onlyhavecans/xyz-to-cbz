use crate::{
    yiffer::{YifferClient, YifferComic},
    Cbz,
};

pub async fn run(comic: String) -> anyhow::Result<()> {
    let client = YifferClient::default();
    let body = client.comic_page(&comic).await?;
    let comic = YifferComic::parse(&body)?;
    Cbz::from(comic).write(None).await?;
    Ok(())
}
