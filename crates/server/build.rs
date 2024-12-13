#![forbid(unsafe_code)]

use std::fs::remove_dir_all;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    // Remove the output directory for TS bindings.
    let typings = PathBuf::from("../../packages/client/typing/");
    let _ = remove_dir_all(typings.as_path());

    // Specify the output path for TS bindings.
    // Run `cargo test export` to emit TS bindings.
    let typings = typings.to_str().unwrap();
    println!("cargo:rustc-env=TS_RS_EXPORT_DIR={}", typings);

    Ok(())
}
