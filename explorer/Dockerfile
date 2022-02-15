FROM ghcr.io/findoranetwork/findora-scanner:latest AS fundation
FROM docker.io/busybox:latest

COPY --from=fundation /explorer /explorer

EXPOSE 8778
ENTRYPOINT ["/explorer"]
