FROM ghcr.io/findoranetwork/findora-scanner:latest AS fundation
FROM docker.io/busybox:latest

COPY --from=fundation /scanner /scanner

ENTRYPOINT ["/scanner"]
