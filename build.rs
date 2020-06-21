fn main() {
	prost_build::compile_protos(&["env-provider.proto"], &["../tea-codec/proto"]).unwrap();
	prost_build::compile_protos(&["kvp-provider.proto"], &["../tea-codec/proto"]).unwrap();
}