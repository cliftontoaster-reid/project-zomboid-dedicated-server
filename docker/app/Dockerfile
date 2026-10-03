ARG BASE=debian:13-slim

# ---------- Stage 1: download the game once ----------
FROM steamcmd/steamcmd:alpine AS downloader

RUN mkdir -p /opt/zomboid
COPY docker/app/install_zomboid.txt /tmp/install_zomboid.txt

RUN steamcmd +login anonymous +quit
RUN for i in 1 2 3; do \
  steamcmd +runscript /tmp/install_zomboid.txt +quit && break; \
  echo "Attempt $i failed, retrying..."; sleep 10; \
  done; \
  test -f /opt/zomboid/start-server.sh

# ---------- Stage 2: runtime on whatever distro $BASE is ----------
FROM $BASE AS common-base
COPY --from=downloader /opt/zomboid /opt/zomboid

EXPOSE 16261/udp 16262/udp

FROM common-base AS root

FROM common-base AS rootless
RUN useradd -m steam && chown steam:steam /opt/zomboid
USER steam