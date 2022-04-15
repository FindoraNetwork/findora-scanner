FROM ghcr.io/findoranetwork/findora-scanner:latest AS fundation
FROM docker.io/busybox:latest

COPY --from=fundation /etc/ssl/certs /etc/ssl/certs

RUN mkdir scanner
COPY --from=fundation /scanner-cli /scanner/scanner-cli

ENTRYPOINT ["/scanner/scanner-cli"]
