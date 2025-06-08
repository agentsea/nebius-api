use anyhow::Result;
use prost::Message;
use prost_types::FileDescriptorSet;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<()> {
    // 1. Execute `buf build` to generate a FileDescriptorSet
    let buf_output = Command::new("buf")
        .arg("build")
        .arg("--output")
        .arg("-")
        .output()?;

    if !buf_output.status.success() {
        std::io::stderr().write_all(&buf_output.stderr)?;
        return Err(anyhow::anyhow!("`buf build` failed"));
    }

    // 2. Deserialize the FileDescriptorSet
    let fds = FileDescriptorSet::decode(buf_output.stdout.as_slice())?;

    // 3. Compile the FileDescriptorSet with tonic-build
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .file_descriptor_set_path(
            PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("descriptor.bin"),
        )
        .compile_fds(fds)?;

    Ok(())
}
