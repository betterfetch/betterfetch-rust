use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "betterfetch", about = "System fetcher written in Rust")]
pub struct Args {
    // Hide ASCII art
    #[arg(short, long)]
    pub no_ascii: bool,
}
