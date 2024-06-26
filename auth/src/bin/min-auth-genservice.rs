use getopts::Options;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("b", "bindir", "path to the bin directory.", "BINDIR");
    opts.optopt("c", "confdir", "path to the config directory", "CONFDIR");
    let matches = opts.parse(&args[1..]).unwrap();
    let bindir = matches.opt_str("b").unwrap();
    let confdir = matches.opt_str("c").unwrap();

    print!(r#"[Unit]
Description=Mini Authenticator on %I
After=mysql.service redis-server.service

[Service]
Type=simple
User=min-auth
Group=min-auth
Environment=RUST_LOG=info
ExecStartPre={bindir}/min-auth-loader -c {confdir}/config.toml
ExecStart={bindir}/min-auth-auth -c {confdir}/config.toml -p %i
StandardOutput=journal
StandardError=journal
SyslogIdentifier=min-auth
KillSignal=SIGTERM
TimeoutSec=10

[Install]
WantedBy=multi-user.target
"#);
}
