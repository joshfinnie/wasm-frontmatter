build:
	wasm-pack build --target nodejs

test:
	cargo test
	
publish:
	wasm-pack publish

fmt:
	cargo fmt

clippy:
	touch src/lib.rs ; cargo clippy
