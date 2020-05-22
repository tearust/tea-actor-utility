fn main() {
	prost_build::compile_protos(&["env-provider.proto"], &["../tea-runtime/proto"]).unwrap();
	prost_build::compile_protos(&["kvp-provider.proto"], &["../tea-runtime/proto"]).unwrap();
}