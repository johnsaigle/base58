.PHONY: build
build:
	cargo build --release
.PHONY: lint
lint:
	cargo clippy --all -- -W clippy::all -W clippy::pedantic -W clippy::nursery -D warnings
