FROM debian:12-slim

# Enable i386 architecture, update lists, and install required runtime dependencies
RUN dpkg --add-architecture i386 && \
  apt-get update && \
  apt-get install -y --no-install-recommends \
  ca-certificates \
  lib32gcc-s1 \
  curl \
  tar \
  apt-transport-https \
  lsb-release && \
  rm -rf /var/lib/apt/lists/*

# Add Eclipse Temurin (Adoptium) GPG key and repository
RUN curl -fsSL https://packages.adoptium.net/artifactory/api/gpg/key/public -o /usr/share/keyrings/adoptium.asc && \
  echo "deb [signed-by=/usr/share/keyrings/adoptium.asc] https://packages.adoptium.net/artifactory/deb $(lsb_release -cs) main" \
  > /etc/apt/sources.list.d/adoptium.list && \
  apt-get update && \
  apt-get install -y --no-install-recommends \
  temurin-25-jdk && \
  rm -rf /var/lib/apt/lists/*

ENV JAVA_HOME=/usr/lib/jvm/temurin-25-jdk-amd64

RUN useradd -m -d /home/steam steam

WORKDIR /home/steam/steamcmd
RUN curl -sqL "https://client-update.steamstatic.com/installer/steamcmd_linux.tar.gz" | tar zxvf - && \
  ln -s /home/steam/steamcmd/steamcmd.sh /usr/local/bin/steamcmd && \
  chown -R steam:steam /home/steam

CMD ["steamcmd", "+quit"]