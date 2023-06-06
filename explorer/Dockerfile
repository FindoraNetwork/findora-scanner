FROM ghcr.io/findoranetwork/findora-scanner:latest AS fundation
FROM debian:stable-slim

RUN mkdir -p /explorer/config
COPY --from=fundation /explorer /explorer/explorer
COPY --from=fundation /etc/ssl/certs /etc/ssl/certs

EXPOSE 8778
ENTRYPOINT ["/explorer/explorer"]
