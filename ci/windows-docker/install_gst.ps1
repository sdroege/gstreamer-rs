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

$MESON_ARGS = @(`
  "--prefix=C:\gst-install", `
  "-Dglib:installed_tests=false", `
  "-Dlibnice:tests=disabled", `
  "-Dlibnice:examples=disabled", `
  "-Dffmpeg:tests=disabled", `
  "-Dopenh264:tests=disabled", `
  "-Dpygobject:tests=false", `
  "-Dgpl=enabled", `
  "-Dugly=enabled", `
  "-Dbad=enabled", `
  "-Dges=enabled", `
  "-Drtsp_server=enabled", `
  "-Ddevtools=enabled", `
  "-Dsharp=disabled", `
  "-Dpython=disabled", `
  "-Dlibav=disabled", `
  "-Dvaapi=disabled", `
  "-Dgst-plugins-base:pango=enabled", `
  "-Dgst-plugins-good:cairo=enabled", `
  "-Dgst-plugins-good:lame=disabled"
)

$PSDefaultParameterValues['Out-File:Encoding'] = 'utf8'
echo "subproject('gtk')" >> meson.build

Write-Output "Building gstreamer"
meson setup --vsenv $MESON_ARGS _build
meson compile -C _build
# meson install does a spurious rebuild sometimes that then fails
meson install --no-rebuild -C _build

cd c:\
Remove-Item -LiteralPath "C:\gstreamer" -Force -Recurse
