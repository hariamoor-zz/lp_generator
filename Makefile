RUST_FLAGS=--release

deps:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
	source $HOME/.cargo/env

install:
	cargo build $(RUST_FLAGS)
	cargo install --path .
