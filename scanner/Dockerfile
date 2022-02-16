FROM ghcr.io/findoranetwork/findora-scanner:latest AS fundation
FROM docker.io/busybox:latest

RUN mkdir scanner
COPY --from=fundation /scanner /scanner/scanner

ENTRYPOINT ["/scanner/scanner"]
