dev:
	cargo build && ./target/debug/earthmacs ~/.emacs

all:
	cargo build --release
