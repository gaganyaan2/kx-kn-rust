FROM rust:1.64.0-buster as build
# ARG arch="aarch64"
ARG arch="x86_64"
WORKDIR /opt/
RUN rustup target add $arch-unknown-linux-musl
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo update
COPY . .
RUN cargo build --target $arch-unknown-linux-musl --release

FROM alpine:3.15.0
WORKDIR /opt
# ARG arch="aarch64"
ARG arch="x86_64"
COPY --from=build /opt/target/$arch-unknown-linux-musl/release/kn .
ENTRYPOINT ["/opt/kn"]

# docker  buildx build --push --platform linux/arm64 --build-arg arch=aarch64 -t koolwithk/kn:0.1.0-arm64 .