SUBDIRS = etc scripts share

DEP  = ../common/Cargo.toml
DEP += $(shell find ../common/src -type f -name "*.rs")
RS = $(shell find src -type f -name "*.rs")

bin_SCRIPTS  = ../target/release/min-auth
bin_SCRIPTS += ../target/release/min-auth-loader
bin_SCRIPTS += ../target/release/min-auth-getauthuri

../target/release/min-auth-loader: Makefile $(RS) $(DEP)
	cargo build --release

../target/release/min-auth: Makefile $(RS) $(DEP)
	cargo build --release

../target/release/min-auth-getauthuri: Makefile $(RS) $(DEP)
	cargo build --release

install-data-hook:
	mkdir -p "$(prefix)/var/tickets"
	mkdir -p "$(prefix)/var/requests"
