dev:
	cargo build && ./target/debug/earthmacs

all:
	cargo build --release
