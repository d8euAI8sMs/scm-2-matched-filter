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
        include_guard: Some(String::from("LIBMF_H")),
        ..Default::default()
    };

    // working around the lack of `post_build` script in cargo
    // 
    // if this script fails, no regular build output is generated
    // at all since no actual build will even be started!
    // as a result, there are no compiler error messages at all
    match cbindgen::generate_with_config(&crate_dir, config) {
        Ok(hpp) => {
            hpp.write_to_file(&output_file);
            ()
        },
        Err(err) => {
            // however, we MUST fail release build anyway
            if (env::var("PROFILE").unwrap() == "release") {
                panic!("failed to generate hpp file: {:?}", err);
            }
        },
    }
}

fn target_dir() -> PathBuf {
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("target")
        .join(env::var("PROFILE").unwrap())
}
