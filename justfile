default: test

build:
	cargo build

run:
	cargo run -- github-ssh casey remote

test:
	cargo test
