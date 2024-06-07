use prost_build;

fn main() {
    prost_build::compile_protos(
        &[
            "src/proto/auth.proto",
            "src/proto/averaging.proto",
            "src/proto/crypto.proto",
            "src/proto/dht.proto",
            "src/proto/p2pd.proto",
            "src/proto/runtime.proto",
            "src/proto/test.proto",
        ],
        &["src/proto"],
    )
    .unwrap_or_else(|e| panic!("Failed to compile protos: {}", e));
}
