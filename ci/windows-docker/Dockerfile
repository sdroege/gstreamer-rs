# escape=`

FROM "registry.freedesktop.org/gstreamer/gstreamer/amd64/windows:2023-07-17.0-main"

SHELL ["powershell", "-Command", "$ErrorActionPreference = 'Stop'; $ProgressPreference = 'SilentlyContinue';"]

ARG DEFAULT_BRANCH="1.24"
ARG RUST_VERSION="invalid"

RUN choco install -y pkgconfiglite nasm llvm openssl

# https://stackoverflow.com/a/50716450
RUN setx PATH '%PATH%;C:\Program Files\NASM;C:\gst-install\bin'
ENV PKG_CONFIG_PATH="C:\gst-install\lib\pkgconfig"

COPY install_gst.ps1 install_dav1d.ps1 install_rust.ps1 install_cargo_utils.ps1 C:\
RUN C:\install_gst.ps1
RUN C:\install_dav1d.ps1
RUN C:\install_rust.ps1
RUN C:\install_cargo_utils.ps1
