ARG BASE=debian:13-slim
ARG BASE_IMAGE=$BASE
ARG DOWNLOADER_IMAGE=downloader

# ---------- Stage 1: download the game once ----------
FROM steamcmd/steamcmd:alpine AS downloader
ARG IS_UNSTABLE=false
RUN mkdir -p /opt/zomboid
COPY docker/app/install_zomboid.txt docker/app/install_zomboid_unstable.txt /tmp/

RUN steamcmd +login anonymous +quit
RUN SCRIPT=install_zomboid.txt; \
  if [ "$IS_UNSTABLE" = "true" ]; then SCRIPT=install_zomboid_unstable.txt; fi; \
  for i in 1 2 3; do \
  steamcmd +runscript /tmp/$SCRIPT +quit && break; \
  echo "Attempt $i failed, retrying..."; sleep 10; \
  done; \
  test -f /opt/zomboid/start-server.sh && \
  rm -rf /opt/zomboid/jre64

# ---------- Stage 1b: game source ----------
FROM $DOWNLOADER_IMAGE AS game-src

# ---------- Stage 2: runtime on whatever distro $BASE_IMAGE is ----------
FROM $BASE_IMAGE AS common-base

RUN set -e; \
  JAVA_BIN="${JAVA_HOME}/bin"; \
  if [ ! -x "$JAVA_BIN/java" ]; then \
  for d in /usr/lib/jvm/*/bin; do \
  if [ -x "$d/java" ]; then JAVA_BIN="$d"; break; fi; \
  done; \
  fi; \
  if [ ! -x "$JAVA_BIN/java" ]; then echo "ERROR: no Java found (BASE_IMAGE requires a zomboid-base image, got JAVA_HOME=${JAVA_HOME})" >&2; exit 1; fi; \
  ln -sf "$JAVA_BIN"/* /usr/local/bin/; \
  java -version; \
  ls -l /usr/local/bin/java
COPY --from=game-src --chown=steam:steam /opt/zomboid /opt/zomboid
COPY docker/app/entrypoint.sh /opt/zomboid/entrypoint.sh
RUN chmod 777 /opt/zomboid/entrypoint.sh

ENTRYPOINT [ "/bin/sh", "/opt/zomboid/entrypoint.sh" ]
CMD [ "/opt/zomboid/ProjectZomboid64" ]

EXPOSE 16261/udp 16262/udp

FROM common-base AS root
USER root

FROM common-base AS rootless
USER steam
