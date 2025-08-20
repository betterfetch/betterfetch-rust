use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "betterfetch",
    about = "System fetcher written in Rust.",
    version = env!("CARGO_PKG_VERSION"),
)]
pub struct Args {
    // Hide ASCII art
    #[arg(short, long)]
    pub no_ascii: bool,
}
