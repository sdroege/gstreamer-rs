[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12;

$env:MESON_ARGS = "--prefix=C:\gst-install\"

# Download pango all its subprojects
git clone -b main --depth 1 https://gitlab.gnome.org/gnome/pango.git C:\pango
if (!$?) {
  Write-Host "Failed to clone pango"
  Exit 1
}

Set-Location C:\pango

Write-Output "Building pango"
cmd.exe /C "C:\BuildTools\Common7\Tools\VsDevCmd.bat -host_arch=amd64 -arch=amd64 && meson _build $env:MESON_ARGS && meson compile -C _build && ninja -C _build install"

if (!$?) {
  Write-Host "Failed to build and install pango"
  Exit 1
}

cd C:\
cmd /c rmdir /s /q  C:\pango
if (!$?) {
  Write-Host "Failed to remove gtk checkout"
  Exit 1
}
