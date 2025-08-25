use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    let repo = env::var("CARGO_PKG_REPOSITORY").unwrap_or_default();
    let homepage = env::var("CARGO_PKG_HOMEPAGE").unwrap_or_default();
    let license = env::var("CARGO_PKG_LICENSE").unwrap_or_default();

    let mut file = File::create("src/build_info.rs").unwrap();
    writeln!(
        file,
        "pub const REPOSITORY: &str = \"{}\";\npub const WEBSITE: &str = \"{}\";\npub const LICENSE: &str = \"{}\";",
        repo, homepage, license
    ).unwrap();
}