#![forbid(unsafe_code)]

use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let builder = tonic_build::configure()
        .build_transport(true)
        .build_client(true)
        .build_server(false);

    let dir = PathBuf::from("./protobuf/");
    let instance = dir.join("./runtime.proto");

    let protos = [instance.as_path()];
    let includes = [dir.as_path()];
    builder.compile_protos(&protos, &includes)?;

    Ok(())
}
