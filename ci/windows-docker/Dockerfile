# escape=`

FROM "registry.freedesktop.org/gstreamer/gstreamer/amd64/windows:2022-12-10.0-main"

# Make sure any failure in PowerShell is fatal
ENV ErrorActionPreference='Stop'
SHELL ["powershell","-NoLogo", "-NonInteractive", "-Command"]

ARG DEFAULT_BRANCH="main"
ARG RUST_VERSION="invalid"

RUN choco install -y pkgconfiglite nasm llvm

# https://stackoverflow.com/a/50716450
RUN setx PATH '%PATH%;C:\Program Files\NASM;C:\gst-install\bin'
ENV PKG_CONFIG_PATH="C:\gst-install\lib\pkgconfig"

COPY install_pango.ps1 install_gst.ps1 install_gtk.ps1 install_dav1d.ps1 C:\
RUN C:\install_pango.ps1
RUN C:\install_gst.ps1
RUN C:\install_gtk.ps1
RUN C:\install_dav1d.ps1

RUN Invoke-WebRequest -Uri https://win.rustup.rs/x86_64 -OutFile C:\rustup-init.exe
RUN C:\rustup-init.exe -y --profile minimal --default-toolchain $env:RUST_VERSION
