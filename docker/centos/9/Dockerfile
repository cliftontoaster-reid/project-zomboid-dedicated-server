FROM quay.io/centos/centos:stream9

# Install required runtime dependencies, including the 32-bit (i686) glibc and
# libstdc++ that the SteamCMD client needs. curl-minimal is requested instead of
# curl because the base image already ships it and full curl conflicts with it.
RUN dnf install -y --setopt=install_weak_deps=False \
  ca-certificates \
  glibc.i686 \
  libstdc++.i686 \
  java-25-openjdk-headless \
  curl-minimal \
  tar && \
  dnf clean all && \
  rm -rf /var/cache/dnf

ENV JAVA_HOME=/usr/lib/jvm/java-25-openjdk

RUN useradd -m -d /home/steam steam

WORKDIR /home/steam/steamcmd
RUN curl -sqL "https://client-update.steamstatic.com/installer/steamcmd_linux.tar.gz" | tar zxvf - && \
  ln -s /home/steam/steamcmd/steamcmd.sh /usr/local/bin/steamcmd && \
  chown -R steam:steam /home/steam

CMD ["steamcmd", "+quit"]