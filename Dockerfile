FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM ubuntu:latest as runner
COPY --from=builder /usr/local/cargo/bin/min-auth /usr/local/bin/min-auth
ENV RUST_LOG=info
EXPOSE 3000
CMD ["/usr/local/bin/min-auth"]
