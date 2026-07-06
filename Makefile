BIN := raytracing
OUT := image.ppm

.PHONY: build release run open test fmt clippy check clean

build:
	cargo build

release:
	cargo build --release

run: release
	./target/release/$(BIN) > $(OUT)

open: run
	open $(OUT)

test:
	cargo test

fmt:
	cargo fmt

clippy:
	cargo clippy -- -D warnings

check:
	cargo check

clean:
	cargo clean
	rm -f $(OUT)
