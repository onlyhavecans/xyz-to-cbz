use std::process;
use yiffer_xyz_to_cbz::run;
use yiffer_xyz_to_cbz::Cli;

#[tokio::main]
async fn main() {
    let cli = Cli::new();
    let comic = cli.comic;

    if let Err(e) = run(comic).await {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}
