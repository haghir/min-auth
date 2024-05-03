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

    print!(r#"#!/bin/sh

AUTH_URI="$({bindir}/min-auth-getauthuri -c {confdir}/config.toml -p ${{1}})"

if ! curl -m 5 "${{AUTH_URI}}" ; then
	systemctl restart min-auth@${{1}}
fi
"#);
}