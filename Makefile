dev:
	cargo build && ./target/debug/earthmacs ~/Git/earthmacs/src/mode.rs

all:
	cargo build --release
