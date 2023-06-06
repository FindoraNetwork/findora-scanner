FROM ghcr.io/findoranetwork/findora-scanner:latest AS fundation
FROM debian:stable-slim

COPY --from=fundation /etc/ssl/certs /etc/ssl/certs

RUN mkdir prismer
COPY --from=fundation /prismer /prismer/prismer

ENTRYPOINT ["/prismer/prismer"]
