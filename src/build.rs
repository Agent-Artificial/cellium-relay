use prost_build;
use std::io::Result;
use walkdir::WalkDir;

fn main() -> Result<()> {
    // Compile protobufs

    let dir = "src/proto";

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            println!("Converting file: {}", path.display());

            let mut config = prost_build::Config::new();
            config.btree_map(&[".proto"]);

            prost_build::compile_protos(&[path], &["src/proto"])
                .unwrap_or_else(|e| panic!("Failed to compile protos: {}", e));

            println!("Converted file: {}", path.display());
        }
    }

    Ok(())
}
