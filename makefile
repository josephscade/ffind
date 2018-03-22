install:
	cp target/release/ffind /usr/bin/ffind

all:
	cargo build --release
