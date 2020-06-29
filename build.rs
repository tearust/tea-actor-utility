fn main() {
	prost_build::compile_protos(&[
		"env-provider.proto", 
		"kvp-provider.proto", 
		"tpm-provider.proto",
		], 
		&["../tea-codec/proto"]).unwrap();
}