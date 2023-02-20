build:
	cargo build --release

unix-install: build
	cp target/release/rd /usr/local/bin