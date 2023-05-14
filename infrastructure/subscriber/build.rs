use std::io::Result;
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

fn main() -> Result<()> {
    let proto_root = Path::new("./src/proto");
    let proto_files = collect_protobuf_files(proto_root);

    let out_dir = "./src";

    prost_build::Config::new()
        .out_dir(out_dir)
        .compile_protos(&proto_files, &["src/"])?;
    Ok(())
}
