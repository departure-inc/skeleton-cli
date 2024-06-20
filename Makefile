fmt:
	cargo fmt

lint:
	cargo clippy

lint.fix:
	cargo clippy --fix

fix:
	cargo fix

build:
	cargo build
	cp target/debug/skeleton bin/skeleton
	cp target/debug/skeleton tmp/skeleton
