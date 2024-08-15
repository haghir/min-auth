[Unit]
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
