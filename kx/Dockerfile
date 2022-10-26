FROM rust:1.64.0-buster as build
WORKDIR /opt/
RUN rustup target add x86_64-unknown-linux-musl
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo update
COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:3.15.0
WORKDIR /opt
COPY --from=build /opt/target/x86_64-unknown-linux-musl/release/kx .
ENTRYPOINT ["/opt/kx"]