FROM docker.io/rust:slim AS builder

RUN apt-get update -y && apt-get install -y musl-tools
ARG TARGETPLATFORM
RUN case "$TARGETPLATFORM" in \
	"linux/amd64") echo x86_64-unknown-linux-musl > /rust_targets ;; \
	*) exit 1 ;; \
    esac

RUN rustup target add $(cat /rust_targets)

COPY . ./findora-scanner
WORKDIR /findora-scanner
RUN cargo build --release --target $(cat /rust_targets)

RUN cp target/$(cat /rust_targets)/release/explorer ./
RUN cp target/$(cat /rust_targets)/release/scanner ./
RUN strip --strip-all ./explorer
RUN strip --strip-all ./scanner

FROM docker.io/busybox:latest

COPY --from=builder /findora-scanner/explorer /explorer
COPY --from=builder /findora-scanner/scanner /scanner
