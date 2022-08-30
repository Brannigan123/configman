mod config;
mod fs;

use crate::config::load_config;

fn main() {
    let config = load_config("example/setup.cmf");
    match config {
        Ok(c) => println!("{c}"),
        Err(e) => println!("{e}"),
    };
}
