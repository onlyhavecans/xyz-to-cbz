use clap::Parser;

/// The command line agruments passed to the program
#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Name of comic to download.
    pub comic: String,
}

impl Cli {
    /// Parse command line aguments and return a pupulated struct
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
