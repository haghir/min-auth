RS = $(shell find ../../src -type f -name "*.rs")

confdir = $(sysconfdir)/min-auth
conf_DATA = config.toml.default

config.toml.default: Makefile $(RS)
	cargo run --release --bin min-auth-genconfig -- -p "$(prefix)" > config.toml.default
