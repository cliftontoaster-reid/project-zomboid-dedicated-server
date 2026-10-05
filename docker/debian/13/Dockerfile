FROM debian:13-slim

# Install runtime dependencies and OpenJDK 25 headless JRE
RUN apt-get update && \
  apt-get install -y --no-install-recommends \
  ca-certificates \
  openjdk-25-jre-headless && \
  rm -rf /var/lib/apt/lists/*

ENV JAVA_HOME=/usr/lib/jvm/java-25-openjdk-amd64
ENV PATH="$JAVA_HOME/bin:$PATH"

RUN ln -sf "$JAVA_HOME"/bin/* /usr/local/bin/ && java -version

RUN useradd -m -d /home/steam steam

WORKDIR /home/steam

CMD ["/bin/bash"]