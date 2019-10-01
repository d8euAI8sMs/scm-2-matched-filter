extern crate cbindgen;

use std::env;
use std::path::PathBuf;
use cbindgen::Config;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let package_name = env::var("CARGO_PKG_NAME").unwrap();
    let output_file = target_dir()
        .join(format!("{}.hpp", package_name))
        .display()
        .to_string();

    let config = Config {
        namespaces: Some(vec!(String::from("libmf"), String::from("ffi"))),
        ..Default::default()
    };

    cbindgen::generate_with_config(&crate_dir, config)
      .unwrap()
      .write_to_file(&output_file);
}

fn target_dir() -> PathBuf {
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("target")
        .join(env::var("PROFILE").unwrap())
}
