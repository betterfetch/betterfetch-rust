use clap::Parser;

const LONG_ABOUT: &str = "
A fast system information fetcher written in Rust.
github repo: https://github.com/betterfetch/betterfetch
websit: https://betterfetch.github.io
License: MIT
";

#[derive(Parser, Debug)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    about = "System fetcher written in Rust.",
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION"),
    long_about = LONG_ABOUT,
)]
pub struct Args {
    #[arg(short, long)]
    pub no_ascii: bool,
}