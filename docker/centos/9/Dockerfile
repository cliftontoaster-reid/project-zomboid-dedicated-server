FROM quay.io/centos/centos:stream9

# Install runtime dependencies and OpenJDK 25 headless JRE
RUN dnf install -y --setopt=install_weak_deps=False \
  ca-certificates \
  java-25-openjdk-headless && \
  dnf clean all && \
  rm -rf /var/cache/dnf

ENV JAVA_HOME=/usr/lib/jvm/java-25-openjdk

RUN useradd -m -d /home/steam steam

WORKDIR /home/steam

CMD ["/bin/bash"]