FROM ghcr.io/findoranetwork/findora-scanner:latest AS fundation
FROM docker.io/busybox:latest

RUN mkdir -p /explorer/config
COPY --from=fundation /explorer /explorer/explorer

EXPOSE 8778
ENTRYPOINT ["/explorer/explorer"]
