build:
	cargo build --release
	mkdir -p .next/standalone
	cp server.js target/release/rust-api .next/standalone

