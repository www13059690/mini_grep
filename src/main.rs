use mini_grep;
use std::{env, error::Error, fs, io, process};
fn main() {
    let config: mini_grep::Config = mini_grep::parse_config(env::args()).unwrap_or_else(|err| {
        eprintln!("problem parsing args: {err}");
        process::exit(12);
    });
    if let Err(e) = mini_grep::run(config) {
        panic!("{}", e.to_string());
    }
}
