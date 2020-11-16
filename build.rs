/*! Bundle mybin.rs and the crate libraries into singlefile.rs */

use std::path::Path;
extern crate rustsourcebundler;
use rustsourcebundler::Bundler;

fn main() {
    let mut bundler: Bundler = Bundler::new(Path::new("src/main.rs"), Path::new("full.rs"));
    bundler.crate_name("codingames_fall_2020");
    bundler.run();
}
