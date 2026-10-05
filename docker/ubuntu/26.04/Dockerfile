FROM ubuntu:26.04

# Install runtime dependencies and OpenJDK 25 headless JRE
RUN apt-get update && \
  apt-get install -y --no-install-recommends \
  ca-certificates \
  openjdk-25-jre-headless && \
  rm -rf /var/lib/apt/lists/*

ENV JAVA_HOME=/usr/lib/jvm/java-25-openjdk-amd64

RUN useradd -m -d /home/steam steam

WORKDIR /home/steam

CMD ["/bin/bash"]