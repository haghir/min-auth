DEP  = ../../../common/Cargo.toml
DEP += $(shell find ../../../common/src -type f -name "*.rs")
RS = $(shell find ../../src -type f -name "*.rs")

confdir = $(sysconfdir)/min-auth
conf_DATA = backend.toml.default

backend.toml.default: Makefile $(RS)
	cargo run --release --bin min-auth-genconfig -- -d "$(prefix)" > backend.toml.default
