#!/bin/sh

AUTH_URI="$({bindir}/min-auth-getauthuri -c {confdir}/config.toml -p ${{1}})"

if ! curl -m 5 "${{AUTH_URI}}" ; then
	systemctl restart min-auth@${1}
fi
