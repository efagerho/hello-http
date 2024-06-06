FROM rust:latest as builder

RUN mkdir /build
WORKDIR /build

COPY . /build

RUN cargo build --release

FROM debian:stable-slim as hello

RUN mkdir -p /app
WORKDIR /app

COPY --from=builder /build/target/release/hello-http /app/hello-http
CMD ["/app/hello-http"]
