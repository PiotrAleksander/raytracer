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
	@open -n -a Preview "$(OUT)"; \
	sleep 1; \
	pid=$$(pgrep -nx Preview); \
	trap "kill $$pid 2>/dev/null" EXIT INT TERM HUP; \
	echo "Preview open (pid $$pid) — stop this task to close it."; \
	while kill -0 $$pid 2>/dev/null; do sleep 1; done

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
