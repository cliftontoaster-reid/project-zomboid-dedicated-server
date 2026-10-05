FROM debian:12-slim

# Install dependencies needed to add the Eclipse Temurin repository
RUN apt-get update && \
  apt-get install -y --no-install-recommends \
  ca-certificates \
  curl \
  apt-transport-https \
  lsb-release && \
  rm -rf /var/lib/apt/lists/*

# Add Eclipse Temurin (Adoptium) GPG key and repository, then install headless JRE
RUN curl -fsSL https://packages.adoptium.net/artifactory/api/gpg/key/public -o /usr/share/keyrings/adoptium.asc && \
  echo "deb [signed-by=/usr/share/keyrings/adoptium.asc] https://packages.adoptium.net/artifactory/deb $(lsb_release -cs) main" \
  > /etc/apt/sources.list.d/adoptium.list && \
  apt-get update && \
  apt-get install -y --no-install-recommends \
  temurin-25-jre && \
  rm -rf /var/lib/apt/lists/*

ENV JAVA_HOME=/usr/lib/jvm/temurin-25-jre-amd64

RUN useradd -m -d /home/steam steam

WORKDIR /home/steam

CMD ["/bin/bash"]