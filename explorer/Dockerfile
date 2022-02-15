FROM findora-scanner:latest AS store
FROM docker.io/busybox:latest

COPY --from=store /explorer /explorer

EXPOSE 8778
ENTRYPOINT ["/explorer"]
