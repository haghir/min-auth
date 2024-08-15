svcdir = $(datadir)/min-auth/service
svc_DATA = min-auth@.service

min-auth@.service: min-auth@.service.t Makefile
	sed -e "s|{bindir}|$(bindir)|g" "$<" | \
	sed -e "s|{confdir}|$(sysconfdir)/min-auth|g" > "$@"
