#!/bin/sh

{bindir}/min-auth-auth -c "{confdir}/auth.toml" -u | while read -r ADDR; do
    if ! curl -m 5 "http://${ADDR}/auth" ; then
        systemctl restart min-auth-auth
        exit 1
    fi
done
