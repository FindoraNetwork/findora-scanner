FROM findora-scanner:latest AS store
FROM docker.io/busybox:latest

COPY --from=store /scanner /scanner

ENTRYPOINT ["/scanner"]
