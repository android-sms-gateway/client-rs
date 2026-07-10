.PHONY: all fmt lint test coverage clean help

all: fmt lint test

fmt:
	cargo fmt --check

lint:
	cargo clippy --all-features -- -D warnings

test:
	cargo test --all-features

coverage: test
	cargo llvm-cov --all-features

clean:
	cargo clean

help:
	@awk 'BEGIN {FS = ":.*## "} /^[a-zA-Z_-]+:.*## / {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)
