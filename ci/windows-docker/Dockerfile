# escape=`

FROM "registry.freedesktop.org/gstreamer/gstreamer/amd64/windows:2025-09-13.0-main"

SHELL ["powershell", "-Command", "$ErrorActionPreference = 'Stop'; $ProgressPreference = 'SilentlyContinue';"]

# These arguments are always required to be specified with --build-arg
# when building the image.
# See DOCKER_BUILD_ARGS in .gitlab-ci.yml for an example
ARG DEFAULT_BRANCH="invalid"
ARG RUST_VERSION="invalid"

RUN choco install -y pkgconfiglite nasm protoc
RUN choco install -y openssl --version=3.6.1
ENV SSL_CERT_FILE="C:\Program Files\OpenSSL-Win64\ca-bundle.crt"
RUN Invoke-WebRequest -Uri "https://curl.se/ca/cacert.pem" -OutFile $env:SSL_CERT_FILE
ENV OPENSSL_DIR="C:\Program Files\OpenSSL-Win64"
ENV OPENSSL_LIB_DIR="C:\Program Files\OpenSSL-Win64\lib\VC\x64\MD"

# https://stackoverflow.com/a/50716450
RUN setx PATH '%PATH%;C:\Program Files\NASM;C:\gst-install\bin'
ENV PKG_CONFIG_PATH="C:\gst-install\lib\pkgconfig"

COPY install_gst.ps1 install_dav1d.ps1 install_vvdec.ps1 install_rust.ps1 install_cargo_utils.ps1 install_librice.ps1 C:\
RUN C:\install_gst.ps1
RUN C:\install_dav1d.ps1
RUN C:\install_vvdec.ps1
RUN C:\install_rust.ps1
RUN C:\install_cargo_utils.ps1
RUN C:\install_librice.ps1

ENV LIBCLANG_PATH="C:\gst-install\bin"
