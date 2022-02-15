FROM ghcr.io/FindoraNetwork/findora-scanner:latest AS fundation
FROM docker.io/busybox:latest

COPY --from=fundation /explorer /explorer

EXPOSE 8778
ENTRYPOINT ["/explorer"]
