FROM debian:12-slim

# Enable i386 architecture, update lists, and install required runtime dependencies
RUN dpkg --add-architecture i386 && \
  apt-get update && \
  apt-get install -y --no-install-recommends \
  ca-certificates \
  lib32gcc-s1 \
  curl \
  tar && \
  rm -rf /var/lib/apt/lists/*

WORKDIR /home/steam/steamcmd
RUN curl -sqL "https://client-update.steamstatic.com/installer/steamcmd_linux.tar.gz" | tar zxvf -

RUN ln -s /home/steam/steamcmd/steamcmd.sh /usr/local/bin/steamcmd

CMD ["steamcmd", "+quit"]