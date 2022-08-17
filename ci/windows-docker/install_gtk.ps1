[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12;

$env:Path += ";C:\gst-install\bin\"

# Download gstreamer and all its subprojects
git clone -b gtk-4-6 --depth 1 https://gitlab.gnome.org/gnome/gtk.git C:\gtk
if (!$?) {
  Write-Host "Failed to clone gtk"
  Exit 1
}

Set-Location C:\gtk

$env:MESON_ARGS = "--prefix=C:\gst-install\"

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
