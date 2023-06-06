FROM ghcr.io/findoranetwork/findora-scanner:latest AS fundation
FROM debian:stable-slim

COPY --from=fundation /etc/ssl/certs /etc/ssl/certs

RUN mkdir temp-server
COPY --from=fundation /temp-server /temp-server/temp-server

ENTRYPOINT ["/temp-server/temp-server"]
