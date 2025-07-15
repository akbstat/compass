VERSION = v0.1.2

.PHONY: dev
dev:
	cargo run --bin compass

.PHONY: build
build:
	cargo build --release --bin compass && docker build -t mobius/compass:$(VERSION) .