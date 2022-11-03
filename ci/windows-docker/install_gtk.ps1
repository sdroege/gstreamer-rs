[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12;

$env:MESON_ARGS = "--prefix=C:\gst-install\"

# Download gtk and all its subprojects
git clone -b 4.8.2 --depth 1 https://gitlab.gnome.org/gnome/gtk.git C:\gtk
if (!$?) {
  Write-Host "Failed to clone gtk"
  Exit 1
}

Set-Location C:\gtk

Write-Output "Building gtk"
cmd.exe /C "C:\BuildTools\Common7\Tools\VsDevCmd.bat -host_arch=amd64 -arch=amd64 && meson _build $env:MESON_ARGS && meson compile -C _build && ninja -C _build install"

if (!$?) {
  Write-Host "Failed to build and install gtk"
  Exit 1
}

cd C:\
cmd /c rmdir /s /q  C:\gtk
if (!$?) {
  Write-Host "Failed to remove gtk checkout"
  Exit 1
}
