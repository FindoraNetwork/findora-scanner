FROM ghcr.io/FindoraNetwork/findora-scanner:latest AS fundation
FROM docker.io/busybox:latest

COPY --from=fundation /scanner /scanner

ENTRYPOINT ["/scanner"]
