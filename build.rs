fn main() {
    prost_build::compile_protos(
        &[
            "env-provider.proto",
            "kvp-provider.proto",
            "tpm-provider.proto",
            "actor-ra.proto",
            "actor-pinner.proto",
            "p2p.proto",
            "crypto-provider.proto",
        ],
        &["../tea-codec/proto"],
    )
    .unwrap();
}
