#                                               -*- Autoconf -*-
# Process this file with autoconf to produce a configure script.

AC_PREREQ([2.71])
AC_INIT([min-auth], [3.0.0], [])
AM_INIT_AUTOMAKE([foreign])
AC_PREFIX_DEFAULT([/opt/$PACKAGE_NAME/$PACKAGE_VERSION])

# Checks for programs.
AC_CHECK_PROG([CARGO_FOUND], [cargo], [yes])
AS_IF([test x"$CARGO_FOUND" != x"yes"], AC_MSG_ERROR(['cargo' is not found.]))

AC_CONFIG_FILES([
Makefile
etc/Makefile
etc/min-auth/Makefile
scripts/Makefile
share/Makefile
share/min-auth/Makefile
share/min-auth/apparmor/Makefile
share/min-auth/service/Makefile
])
AC_OUTPUT
