fn main() {
	prost_build::compile_protos(&["env-provider.proto"], &["../tea-runtime/proto"]).unwrap();
}