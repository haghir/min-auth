FROM ubuntu:latest as builder
ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ENV PATH=/usr/local/cargo/bin:$PATH
RUN apt update && apt upgrade -y && apt install -y curl build-essential pkg-config libssl-dev && apt autoremove -y
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN mkdir /usr/src/min-auth
WORKDIR /usr/src/min-auth
COPY . .
RUN cargo install --path .

FROM ubuntu:latest as runner
COPY --from=builder /usr/local/cargo/bin/min-auth /usr/local/bin/min-auth
ENV RUST_LOG=info
EXPOSE 3000
CMD ["/usr/local/bin/min-auth"]
