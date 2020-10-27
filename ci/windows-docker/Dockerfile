# escape=`

FROM "registry.freedesktop.org/gstreamer/gstreamer/amd64/windows:2022-05-16.1-main"

# Make sure any failure in PowerShell is fatal
ENV ErrorActionPreference='Stop'
SHELL ["powershell","-NoLogo", "-NonInteractive", "-Command"]

ARG DEFAULT_BRANCH="main"
ARG RUST_VERSION="invalid"

COPY install_gst.ps1 C:\
RUN C:\install_gst.ps1
RUN choco install -y pkgconfiglite
ENV PKG_CONFIG_PATH="C:/lib/pkgconfig"

RUN Invoke-WebRequest -Uri https://win.rustup.rs/x86_64 -OutFile C:\rustup-init.exe
RUN C:\rustup-init.exe -y --profile minimal --default-toolchain $env:RUST_VERSION
