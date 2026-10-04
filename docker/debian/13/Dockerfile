FROM debian:13-slim

# Enable i386 architecture, update lists, and install required runtime dependencies
RUN dpkg --add-architecture i386 && \
  apt-get update && \
  apt-get install -y --no-install-recommends \
  ca-certificates \
  lib32gcc-s1 \
  openjdk-25-jre-headless \
  curl \
  tar && \
  rm -rf /var/lib/apt/lists/*

ENV JAVA_HOME=/usr/lib/jvm/java-25-openjdk-amd64
ENV PATH="$JAVA_HOME/bin:$PATH"

RUN ln -sf "$JAVA_HOME"/bin/* /usr/local/bin/ && java -version

RUN useradd -m -d /home/steam steam

WORKDIR /home/steam/steamcmd
RUN curl -sqL "https://client-update.steamstatic.com/installer/steamcmd_linux.tar.gz" | tar zxvf - && \
  ln -s /home/steam/steamcmd/steamcmd.sh /usr/local/bin/steamcmd && \
  chown -R steam:steam /home/steam

CMD ["steamcmd", "+quit"]