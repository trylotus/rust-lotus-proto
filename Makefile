gen-proto:
	buf generate
	cp -r gen/rust/lotus/* src && rm -rf gen && rm src/mod.rs

protoc-gen-rust:
	cargo install --git https://github.com/trylotus/protobuf-codegen.git
