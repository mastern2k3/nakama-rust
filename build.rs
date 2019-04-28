extern crate protoc_rust_grpc;

fn main() {
    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: "src",
        includes: &["proto"],
        input: &[
            "proto/google/protobuf/empty.proto",
            "proto/github.com/heroiclabs/nakama/api/api.proto",
            "proto/github.com/heroiclabs/nakama/apigrpc/apigrpc.proto",
        ],
        rust_protobuf: true,
        ..Default::default()
    }).expect("protoc-rust-grpc");
}
