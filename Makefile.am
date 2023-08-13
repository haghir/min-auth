bin_SCRIPTS  = target/release/min-auth-loader
bin_SCRIPTS += target/release/min-auth-web
bin_SCRIPTS += target/release/min-auth-getauthuri
bin_SCRIPTS += min-auth-monitor

confdir = $(sysconfdir)/min-auth
conf_DATA = config-1.toml.default

sharedir = $(datadir)/min-auth
share_DATA = min-auth@.service

target/release/min-auth-genconfig: Makefile src/lib.rs src/bin/min-auth-genconfig.rs
	cargo build --release

target/release/min-auth-genmonitor: Makefile src/lib.rs src/bin/min-auth-genmonitor.rs
	cargo build --release

target/release/min-auth-genservice: Makefile src/lib.rs src/bin/min-auth-genservice.rs
	cargo build --release

target/release/min-auth-loader: Makefile src/lib.rs src/bin/min-auth-loader.rs
	cargo build --release

target/release/min-auth-web: Makefile src/lib.rs src/bin/min-auth-web.rs
	cargo build --release

target/release/min-auth-getauthuri: Makefile src/lib.rs src/bin/min-auth-getauthuri.rs
	cargo build --release

min-auth-monitor: Makefile target/release/min-auth-genmonitor
	target/release/min-auth-genmonitor --bindir "$(bindir)" --confdir "$(sysconfdir)/min-auth" > min-auth-monitor

config-1.toml.default: Makefile target/release/min-auth-genconfig
	target/release/min-auth-genconfig > config-1.toml.default

min-auth@.service: Makefile target/release/min-auth-genservice
	target/release/min-auth-genservice --bindir "$(bindir)" --confdir "$(sysconfdir)/min-auth" > min-auth@.service
