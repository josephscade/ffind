all:
	cargo build --release

install:
	cp target/release/ffind /usr/bin/ffind
