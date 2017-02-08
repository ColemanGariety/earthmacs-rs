dev:
	cargo build && ./target/debug/earthmacs ~/Git/earthmacs/src/main.rs

all:
	cargo build --release
