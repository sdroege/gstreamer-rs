[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12;
# Make sure powershell exits on errors
$ErrorActionPreference = "Stop"

# Download gstreamer and all its subprojects
git clone -b $env:DEFAULT_BRANCH --depth 1 https://gitlab.freedesktop.org/gstreamer/gstreamer.git C:\gstreamer

Set-Location C:\gstreamer

# Copy the cache we already have in the image to avoid massive redownloads
Move-Item C:/subprojects/*  C:\gstreamer\subprojects

# Update the subprojects cache
Write-Output "Running meson subproject reset"
meson subprojects update --reset

$env:MESON_ARGS = "--prefix=C:\gst-install\ " +
    "-Dglib:installed_tests=false " +
    "-Dlibnice:tests=disabled " +
    "-Dlibnice:examples=disabled " +
    "-Dffmpeg:tests=disabled " +
    "-Dopenh264:tests=disabled " +
    "-Dpygobject:tests=false " +
    "-Dgpl=enabled " +
    "-Dugly=enabled " +
    "-Dbad=enabled " +
    "-Dges=enabled " +
    "-Drtsp_server=enabled " +
    "-Ddevtools=enabled " +
    "-Dsharp=disabled " +
    "-Dpython=disabled " +
    "-Dlibav=disabled " +
    "-Dvaapi=disabled " +
    "-Dgst-plugins-base:pango=enabled " +
    "-Dgst-plugins-good:cairo=enabled " +
    "-Dgst-plugins-good:lame=disabled "

Write-Output "Building gst"
cmd.exe /C "C:\BuildTools\Common7\Tools\VsDevCmd.bat -host_arch=amd64 -arch=amd64 && meson setup _build $env:MESON_ARGS && meson compile -C _build && meson install -C _build"


cd c:\
Remove-Item -LiteralPath "C:\gstreamer" -Force -Recurse
