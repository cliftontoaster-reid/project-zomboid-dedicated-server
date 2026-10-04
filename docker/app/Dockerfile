ARG BASE=debian:13-slim
ARG BASE_IMAGE=$BASE
ARG DOWNLOADER_IMAGE=downloader
ARG RUST_IMAGE=rust:alpine

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

# ---------- Stage 2: build the entrypoint ----------
# Statically linked musl binary so a single artifact runs on the alpine images
# and on the glibc ones.
FROM $RUST_IMAGE AS entrypoint-build
WORKDIR /build
RUN apk add --no-cache musl-dev
COPY Cargo.toml Cargo.lock ./
COPY crates/builder crates/builder
COPY crates/entrypoint crates/entrypoint
# Building for an explicit target keeps proc-macro crates dynamic while the
# binary itself links statically against musl.
RUN target="$(rustc -vV | sed -n 's/^host: //p')" && \
  cargo build --release --locked -p entrypoint --target "$target" && \
  cp "target/$target/release/entrypoint" /entrypoint

# ---------- Stage 3: runtime on whatever distro $BASE_IMAGE is ----------
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
COPY --from=entrypoint-build --chmod=0755 /entrypoint /usr/local/bin/zomboid-entrypoint

# ---------- Server configuration (see crates/entrypoint/src/config.rs) ----------
# check=skip=SecretsUsedInArgOrEnv
ENV SERVER_NAME=servertest \
  ADMIN_PASSWORD=ChangeMe123 \
  SERVER_PASSWORD="" \
  PORT=16261 \
  STEAM_PORT=16262 \
  RCON_PORT=27015 \
  RCON_PASSWORD=ChangeMe123 \
  MEMORY=4g \
  MAX_PLAYERS=16 \
  PAUSE_EMPTY=true \
  STEAM_VAC=true \
  UPNP=false \
  SOFT_RESET=false \
  DATA_PATH=/home/steam/ZomboidServer \
  GAME_PATH=/opt/zomboid

# The entrypoint takes a SIGTERM, writes "save" to $DATA_PATH/zomboid.control,
# waits 15s and then writes "quit" so the world is saved before exit. That needs
# more than docker's 10s default, so stop with `docker stop -t 30` or set
# stop_grace_period: 30s.
ENTRYPOINT [ "/usr/local/bin/zomboid-entrypoint" ]
CMD [ ]

# RCON round trip against the local server, see crates/entrypoint/src/healthcheck.rs
HEALTHCHECK --interval=30s --timeout=10s --start-period=120s --retries=3 \
  CMD [ "/usr/local/bin/zomboid-entrypoint", "--healthcheck" ]

EXPOSE 16261/udp 16262/udp 27015/tcp

FROM common-base AS root
USER root

FROM common-base AS rootless
USER steam
