DEP  = ../../../common/Cargo.toml ../../Cargo.toml ../../build.rs
DEP += $(shell find ../../../common/src -type f -name "*.rs")
DEP += $(shell find ../../src -type f -name "*.rs")

confdir = $(sysconfdir)/min-auth
conf_DATA = auth.toml.example

auth.toml.example: Makefile $(DEP)
	cargo run --release --bin min-auth-genconfig -- -d "$(prefix)" > auth.toml.default
