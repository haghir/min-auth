bin_SCRIPTS  = target/release/min-auth-loader
bin_SCRIPTS += target/release/min-auth-web

confdir=$(sysconfdir)/min-auth
conf_DATA = config.toml

target/release/min-auth-genconfig: Makefile.am src/lib.rs src/bin/min-auth-genconfig.rs
	cargo build --release

target/release/min-auth-loader: Makefile.am src/lib.rs src/bin/min-auth-loader.rs
	cargo build --release

target/release/min-auth-web: Makefile.am src/lib.rs src/bin/min-auth-web.rs
	cargo build --release

config.toml: target/release/min-auth-genconfig
	target/release/min-auth-genconfig > config.toml
