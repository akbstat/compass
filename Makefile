VERSION = v0.1.0

.PHONY: dev
dev:
	cargo run --bin compass

.PHONY: build
build:
	cargo build --release --bin compass && docker build -t mobius/compass:$(VERSION) .