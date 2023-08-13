bin_SCRIPTS  = target/release/min-auth-loader
bin_SCRIPTS += target/release/min-auth-web

confdir=$(sysconfdir)/min-auth
conf_DATA = config.toml

target/release/min-auth-loader: cargo-build
target/release/min-auth-web: cargo-build
config.toml: cargo-build
	target/release/min-auth-genconfig > config.toml

.PHONY: cargo-build
cargo-build:
	cargo build --release
