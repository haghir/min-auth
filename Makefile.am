RS = $(shell find src -type f -name "*.rs")

bin_SCRIPTS  = target/release/min-auth-loader
bin_SCRIPTS += target/release/min-auth
bin_SCRIPTS += target/release/min-auth-getauthuri

target/release/min-auth-loader: Makefile $(RS)
	cargo build --release

target/release/min-auth: Makefile $(RS)
	cargo build --release

target/release/min-auth-getauthuri: Makefile $(RS)
	cargo build --release
