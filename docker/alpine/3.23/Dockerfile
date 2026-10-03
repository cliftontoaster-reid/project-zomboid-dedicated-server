FROM alpine:3.23

# Alpine is musl-based and ships no 32-bit (i386) support, so the SteamCMD client
# cannot be installed or run here. gcompat supplies the glibc shims the
# ProjectZomboid64 launcher needs, and the musl OpenJDK 25 headless JRE is
# installed as the distro-provided JVM. It must be selected explicitly with
# -pzexejavacmd; ProjectZomboid64 otherwise uses its own bundled jre64.
RUN apk add --no-cache \
  ca-certificates \
  curl \
  tar \
  gcompat \
  openjdk25-jre-headless

ENV JAVA_HOME=/usr/lib/jvm/java-25-openjdk

RUN adduser -D -h /home/steam steam

WORKDIR /home/steam

CMD ["/bin/sh"]