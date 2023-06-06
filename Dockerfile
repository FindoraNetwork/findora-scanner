FROM docker.io/rust:slim AS builder

RUN apt-get update -y && apt-get install -y musl-tools libssl-dev pkg-config make perl
ENV OPENSSL_LIB_DIR="/usr/lib/x86_64-linux-gnu"
ENV OPENSSL_INCLUDE_DIR="/usr/include/openssl"

ARG TARGETPLATFORM
RUN case "$TARGETPLATFORM" in \
    "linux/amd64") echo x86_64-unknown-linux-musl > /rust_targets ;; \
    *) exit 1 ;; \
    esac

RUN rustup target add $(cat /rust_targets)

COPY . ./findora-scanner
WORKDIR /findora-scanner

RUN cargo build --release --target $(cat /rust_targets)

RUN mkdir /findora-scanner-binaries
RUN cp target/$(cat /rust_targets)/release/explorer /findora-scanner-binaries
RUN cp target/$(cat /rust_targets)/release/scanner-cli /findora-scanner-binaries
RUN cp target/$(cat /rust_targets)/release/temp-server /findora-scanner-binaries
RUN cp target/$(cat /rust_targets)/release/prismer /findora-scanner-binaries
RUN strip --strip-all /findora-scanner-binaries/explorer
RUN strip --strip-all /findora-scanner-binaries/scanner-cli
RUN strip --strip-all /findora-scanner-binaries/temp-server
RUN strip --strip-all /findora-scanner-binaries/prismer
 
FROM docker.io/busybox:latest

COPY --from=builder /etc/ssl/certs /etc/ssl/certs
COPY --from=builder /findora-scanner-binaries/explorer /explorer
COPY --from=builder /findora-scanner-binaries/scanner-cli /scanner-cli
COPY --from=builder /findora-scanner-binaries/temp-server /temp-server
COPY --from=builder /findora-scanner-binaries/prismer /prismer
