use crate::yiffer::{YifferClient, YifferComic};
use crate::Cbz;
use log::info;

/// The bulk of run logic
pub async fn run(comic: String) -> anyhow::Result<()> {
    let client = YifferClient::default();
    info!("aquiring comic page");
    let body = client.comic_page(&comic).await?;
    info!("parsing page");
    let comic = YifferComic::parse(&body)?;
    info!("writing cbz");
    Cbz::from(comic).write(None).await?;
    info!("done");
    Ok(())
}
