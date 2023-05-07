use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};

fn collect_protobuf_files(dir: impl AsRef<Path>) -> Vec<PathBuf> {
    walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| match e.path().extension() {
            Some(ext) => ext == "proto",
            _ => false,
        })
        .map(|e| e.path().to_path_buf())
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let proto_root = Path::new("./proto");
    let proto_files = collect_protobuf_files(proto_root);

    // Rerun build.rs on changes to the script itself and to protobuf.
    println!("cargo:rerun-if-changed=build.rs");
    for file in &proto_files {
        println!("cargo:rerun-if-changed={}", file.display());
    }

    let original_out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let out_dir = "./src";

    tonic_build::configure()
        .out_dir(out_dir)
        .file_descriptor_set_path(original_out_dir.join("api.bin"))
        // .build_server(true)
        .compile(&proto_files, &[proto_root.to_path_buf()])?;

    Ok(())
}
