dev:
	cargo build && ./target/debug/earthmacs ~/Git/earthmacs/src/buffer.rs

all:
	cargo build --release
