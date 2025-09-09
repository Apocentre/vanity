use clap::Parser;
use vanity::{Command, grind, verify};

fn main() {
    rayon::ThreadPoolBuilder::new().build_global().unwrap();

    // Parse command line arguments
    let command = Command::parse();
    match command {
        Command::Grind(args) => {
            grind(args);
        }

        Command::Verify(args) => {
            verify(args);
        }

        #[cfg(feature = "deploy")]
        Command::Deploy(args) => {
            deploy(args);
        }
    }
}
