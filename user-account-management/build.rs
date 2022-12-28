use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("user_descriptor.bin"))
        .compile(&["proto/user.proto"], &["proto"])
        .unwrap();
    Ok(())
}
