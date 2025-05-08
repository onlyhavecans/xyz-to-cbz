use log::error;
use std::process;
use xyz_to_cbz::Cli;
use xyz_to_cbz::run;

#[tokio::main]
async fn main() {
    env_logger::init();

    let cli = Cli::new();
    let comic = cli.comic;

    if let Err(e) = run(comic).await {
        error!("application error: {}", e);
        process::exit(1);
    }
}
