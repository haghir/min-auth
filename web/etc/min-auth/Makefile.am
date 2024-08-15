DEP  = ../../../common/Cargo.toml
DEP += $(shell find ../../../common/src -type f -name "*.rs")
RS = $(shell find ../../src -type f -name "*.rs")

confdir = $(sysconfdir)/min-auth
conf_DATA = web.toml.default

web.toml.default: Makefile $(RS)
	cargo run --release --bin min-auth-genconfig -- -p "$(prefix)" > web.toml.default
