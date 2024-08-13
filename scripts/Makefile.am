bin_SCRIPTS = min-auth-monitor

min-auth-monitor: min-auth-monitor.t Makefile
	sed -e "s|{bindir}|$(bindir)|g" "$<" | \
	sed -e "s|{confdir}|$(sysconfdir)/min-auth|g" > "$@"
