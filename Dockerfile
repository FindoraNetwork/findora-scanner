FROM docker.io/rust:slim AS builder

RUN apt-get update -y && apt-get install -y musl-tools libssl-dev pkg-config make perl
ENV OPENSSL_LIB_DIR="/usr/lib/x86_64-linux-gnu"
ENV OPENSSL_INCLUDE_DIR="/usr/include/openssl"

COPY . ./findora-scanner
WORKDIR /findora-scanner

RUN cargo build --release

RUN mkdir /findora-scanner-binaries
RUN cp target/$(cat /rust_targets)/release/explorer /findora-scanner-binaries
RUN cp target/$(cat /rust_targets)/release/scanner-cli /findora-scanner-binaries
RUN cp target/$(cat /rust_targets)/release/temp-server /findora-scanner-binaries
RUN cp target/$(cat /rust_targets)/release/prismer /findora-scanner-binaries
RUN strip --strip-all /findora-scanner-binaries/explorer
RUN strip --strip-all /findora-scanner-binaries/scanner-cli
RUN strip --strip-all /findora-scanner-binaries/temp-server
RUN strip --strip-all /findora-scanner-binaries/prismer
 
FROM debian:stable-slim

COPY --from=builder /etc/ssl/certs /etc/ssl/certs
COPY --from=builder /findora-scanner-binaries/explorer /explorer
COPY --from=builder /findora-scanner-binaries/scanner-cli /scanner-cli
COPY --from=builder /findora-scanner-binaries/temp-server /temp-server
COPY --from=builder /findora-scanner-binaries/prismer /prismer
