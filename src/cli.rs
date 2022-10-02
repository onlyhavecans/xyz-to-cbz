use clap::Parser;

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Name of comic to download.
    pub comic: String,
}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn clap_test() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
