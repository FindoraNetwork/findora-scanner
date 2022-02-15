FROM localhost/findora-scanner AS store
FROM docker.io/busybox:latest

COPY --from=store /scanner /scanner

ENTRYPOINT ["/scanner"]
